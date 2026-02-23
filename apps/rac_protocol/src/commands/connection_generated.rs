use crate::Uuid16;
use crate::error::RacError;
use crate::codec::v8_datetime_to_iso;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::metadata::RpcMethodMeta;

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

pub const RPC_CONNECTION_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 50,
    method_resp: Some(51),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_CONNECTION_INFO_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 54,
    method_resp: Some(55),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


pub fn parse_connection_info_body(body: &[u8]) -> Result<ConnectionRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("connection info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    ConnectionRecord::decode(&mut cursor)
}

#[derive(Debug, Clone)]
pub struct ConnectionListRequest {
    pub cluster: Uuid16,
}

impl ConnectionListRequest {
    pub fn encoded_len(&self) -> usize {
        16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionInfoRequest {
    pub cluster: Uuid16,
    pub connection: Uuid16,
}

impl ConnectionInfoRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.connection);
        Ok(())
    }
}


