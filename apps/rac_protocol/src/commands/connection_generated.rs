use crate::Uuid16;
use crate::error::RacError;
use crate::codec::v8_datetime_to_iso;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ConnectionRecord {
    pub connection: Uuid16,
    pub application: String,
    pub blocked_by_ls: u32,
    pub connected_at: String,
    pub conn_id: u32,
    pub host: String,
    pub infobase: Uuid16,
    pub process: Uuid16,
    pub session_number: u32,
}

impl ConnectionRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let connection = cursor.take_uuid()?;
        let application = cursor.take_str8()?;
        let blocked_by_ls = cursor.take_u32_be()?;
        let connected_at = v8_datetime_to_iso(cursor.take_u64_be()?).unwrap_or_default();
        let conn_id = cursor.take_u32_be()?;
        let host = cursor.take_str8()?;
        let infobase = cursor.take_uuid()?;
        let process = cursor.take_uuid()?;
        let session_number = cursor.take_u32_be()?;
        Ok(Self {
            connection,
            application,
            blocked_by_ls,
            connected_at,
            conn_id,
            host,
            infobase,
            process,
            session_number,
        })
    }
}

pub struct ConnectionDisconnectRpc {
    pub cluster: Uuid16,
    pub connection: Uuid16,
    pub process: Uuid16,
}

impl crate::rpc::Request for ConnectionDisconnectRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_CONNECTION_DISCONNECT_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 16 + 16);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.connection);
        out.extend_from_slice(&self.process);
        Ok(out)
    }
}



pub fn parse_connection_info_body(body: &[u8]) -> Result<ConnectionRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("connection info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    ConnectionRecord::decode(&mut cursor)
}


pub const RPC_CONNECTION_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x32,
    method_resp: Some(0x33),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_CONNECTION_LIST_BY_INFOBASE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x34,
    method_resp: Some(0x35),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_CONNECTION_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x36,
    method_resp: Some(0x37),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_CONNECTION_DISCONNECT_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x40,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};


