use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::Uuid16;
use crate::rac_wire::encode_with_len_u8;

pub const METHOD_LIMIT_LIST_REQ: u8 = 0x7c;
pub const METHOD_LIMIT_LIST_RESP: u8 = 0x7d;
pub const METHOD_LIMIT_INFO_REQ: u8 = 0x7e;
pub const METHOD_LIMIT_INFO_RESP: u8 = 0x7f;
pub const METHOD_LIMIT_UPDATE_REQ: u8 = 0x80;
pub const METHOD_LIMIT_REMOVE_REQ: u8 = 0x81;

#[derive(Debug, Serialize, Clone)]
pub struct LimitRecord {
    pub name: String,
    pub counter: String,
    pub action: u8,
    pub duration: u64,
    pub cpu_time: u64,
    pub memory: u64,
    pub read: u64,
    pub write: u64,
    pub duration_dbms: u64,
    pub dbms_bytes: u64,
    pub service: u64,
    pub call: u64,
    pub number_of_active_sessions: u64,
    pub number_of_sessions: u64,
    pub error_message: String,
    pub descr: String,
}

impl LimitRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let name = cursor.take_str8()?;
        let counter = cursor.take_str8()?;
        let action = cursor.take_u8()?;
        let duration = cursor.take_u64_be()?;
        let cpu_time = cursor.take_u64_be()?;
        let memory = cursor.take_u64_be()?;
        let read = cursor.take_u64_be()?;
        let write = cursor.take_u64_be()?;
        let duration_dbms = cursor.take_u64_be()?;
        let dbms_bytes = cursor.take_u64_be()?;
        let service = cursor.take_u64_be()?;
        let call = cursor.take_u64_be()?;
        let number_of_active_sessions = cursor.take_u64_be()?;
        let number_of_sessions = cursor.take_u64_be()?;
        let error_message = cursor.take_str8()?;
        let descr = cursor.take_str8()?;
        Ok(Self {
            name,
            counter,
            action,
            duration,
            cpu_time,
            memory,
            read,
            write,
            duration_dbms,
            dbms_bytes,
            service,
            call,
            number_of_active_sessions,
            number_of_sessions,
            error_message,
            descr,
        })
    }
}

pub struct LimitListRpc {
    pub cluster: Uuid16,
}

impl crate::rpc::Request for LimitListRpc {
    type Response = LimitListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_LIMIT_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16);
        out.extend_from_slice(&self.cluster);
        Ok(out)
    }
}

pub struct LimitInfoRpc {
    pub cluster: Uuid16,
    pub name: String,
}

impl crate::rpc::Request for LimitInfoRpc {
    type Response = LimitInfoResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_LIMIT_INFO_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 1 + self.name.len());
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        Ok(out)
    }
}

pub struct LimitUpdateRpc {
    pub cluster: Uuid16,
    pub name: String,
    pub counter: String,
    pub action: u8,
    pub duration: u64,
    pub cpu_time: u64,
    pub memory: u64,
    pub read: u64,
    pub write: u64,
    pub duration_dbms: u64,
    pub dbms_bytes: u64,
    pub service: u64,
    pub call: u64,
    pub number_of_active_sessions: u64,
    pub number_of_sessions: u64,
    pub error_message: String,
    pub descr: String,
}

impl crate::rpc::Request for LimitUpdateRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_LIMIT_UPDATE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 1 + self.name.len() + 1 + self.counter.len() + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + self.error_message.len() + 1 + self.descr.len());
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        out.push(self.action);
        out.extend_from_slice(&self.duration.to_be_bytes());
        out.extend_from_slice(&self.cpu_time.to_be_bytes());
        out.extend_from_slice(&self.memory.to_be_bytes());
        out.extend_from_slice(&self.read.to_be_bytes());
        out.extend_from_slice(&self.write.to_be_bytes());
        out.extend_from_slice(&self.duration_dbms.to_be_bytes());
        out.extend_from_slice(&self.dbms_bytes.to_be_bytes());
        out.extend_from_slice(&self.service.to_be_bytes());
        out.extend_from_slice(&self.call.to_be_bytes());
        out.extend_from_slice(&self.number_of_active_sessions.to_be_bytes());
        out.extend_from_slice(&self.number_of_sessions.to_be_bytes());
        out.extend_from_slice(&encode_with_len_u8(self.error_message.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.descr.as_bytes())?);
        Ok(out)
    }
}

pub struct LimitRemoveRpc {
    pub cluster: Uuid16,
    pub name: String,
}

impl crate::rpc::Request for LimitRemoveRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_LIMIT_REMOVE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 1 + self.name.len());
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        Ok(out)
    }
}


#[derive(Debug, Serialize)]
pub struct LimitListResp {
    pub limits: Vec<LimitRecord>,
}

impl crate::rpc::Response for LimitListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        Ok(Self {
            limits: crate::commands::parse_list_u8(body, LimitRecord::decode)?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct LimitInfoResp {
    pub record: LimitRecord,
}

impl crate::rpc::Response for LimitInfoResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let record = parse_limit_info_body(body)?;
        Ok(Self {
            record: record,
        })
    }
}


pub fn parse_limit_info_body(body: &[u8]) -> Result<LimitRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("limit info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    LimitRecord::decode(&mut cursor)
}


pub const RPC_LIMIT_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_LIMIT_LIST_REQ,
    method_resp: Some(METHOD_LIMIT_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_LIMIT_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_LIMIT_INFO_REQ,
    method_resp: Some(METHOD_LIMIT_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_LIMIT_UPDATE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_LIMIT_UPDATE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_LIMIT_REMOVE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_LIMIT_REMOVE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};


