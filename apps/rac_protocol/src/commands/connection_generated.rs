use crate::Uuid16;
use crate::error::RacError;
use crate::codec::v8_datetime_to_iso;
use crate::protocol::ProtocolVersion;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;

pub const METHOD_CONNECTION_LIST_REQ: u8 = 0x32;
pub const METHOD_CONNECTION_LIST_RESP: u8 = 0x33;
pub const METHOD_CONNECTION_LIST_BY_INFOBASE_REQ: u8 = 0x34;
pub const METHOD_CONNECTION_LIST_BY_INFOBASE_RESP: u8 = 0x35;
pub const METHOD_CONNECTION_INFO_REQ: u8 = 0x36;
pub const METHOD_CONNECTION_INFO_RESP: u8 = 0x37;
pub const METHOD_CONNECTION_DISCONNECT_REQ: u8 = 0x40;

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
    pub fn decode(cursor: &mut RecordCursor<'_>, protocol_version: ProtocolVersion) -> Result<Self> {
        let _ = protocol_version;
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

pub struct ConnectionListRpc {
    pub cluster: Uuid16,
}

impl crate::rpc::Request for ConnectionListRpc {
    type Response = ConnectionListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_CONNECTION_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ConnectionList unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        Ok(out)
    }
}

pub struct ConnectionListByInfobaseRpc {
    pub cluster: Uuid16,
    pub infobase: Uuid16,
}

impl crate::rpc::Request for ConnectionListByInfobaseRpc {
    type Response = ConnectionListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_CONNECTION_LIST_BY_INFOBASE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ConnectionListByInfobase unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.infobase);
        }
        Ok(out)
    }
}

pub struct ConnectionInfoRpc {
    pub cluster: Uuid16,
    pub connection: Uuid16,
}

impl crate::rpc::Request for ConnectionInfoRpc {
    type Response = ConnectionInfoResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_CONNECTION_INFO_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ConnectionInfo unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.connection);
        }
        Ok(out)
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
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ConnectionDisconnect unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.connection);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.process);
        }
        Ok(out)
    }
}


#[derive(Debug, Serialize)]
pub struct ConnectionListResp {
    pub records: Vec<ConnectionRecord>,
}

impl crate::rpc::Response for ConnectionListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        Ok(Self {
            records: crate::commands::parse_list_u8(body, |cursor| ConnectionRecord::decode(cursor, protocol_version))?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ConnectionInfoResp {
    pub record: ConnectionRecord,
}

impl crate::rpc::Response for ConnectionInfoResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        let record = parse_connection_info_body(body, protocol_version)?;
        Ok(Self {
            record: record,
        })
    }
}


pub fn parse_connection_info_body(body: &[u8], protocol_version: ProtocolVersion) -> Result<ConnectionRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("connection info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    ConnectionRecord::decode(&mut cursor, protocol_version)
}


pub const RPC_CONNECTION_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_CONNECTION_LIST_REQ,
    method_resp: Some(METHOD_CONNECTION_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_CONNECTION_LIST_BY_INFOBASE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_CONNECTION_LIST_BY_INFOBASE_REQ,
    method_resp: Some(METHOD_CONNECTION_LIST_BY_INFOBASE_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_CONNECTION_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_CONNECTION_INFO_REQ,
    method_resp: Some(METHOD_CONNECTION_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_CONNECTION_DISCONNECT_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_CONNECTION_DISCONNECT_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};


