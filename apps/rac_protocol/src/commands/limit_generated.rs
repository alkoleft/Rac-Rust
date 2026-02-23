use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::metadata::RpcMethodMeta;
use crate::Uuid16;
use crate::rac_wire::encode_with_len_u8;

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

pub const RPC_LIMIT_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_LIMIT_LIST_REQ,
    method_resp: Some(crate::rac_wire::METHOD_LIMIT_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_LIMIT_INFO_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_LIMIT_INFO_REQ,
    method_resp: Some(crate::rac_wire::METHOD_LIMIT_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_LIMIT_UPDATE_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_LIMIT_UPDATE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_LIMIT_REMOVE_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_LIMIT_REMOVE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};


pub fn parse_limit_info_body(body: &[u8]) -> Result<LimitRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("limit info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    LimitRecord::decode(&mut cursor)
}

#[derive(Debug, Clone)]
pub struct LimitListRequest {
    pub cluster: Uuid16,
}

impl LimitListRequest {
    pub fn encoded_len(&self) -> usize {
        16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LimitInfoRequest {
    pub cluster: Uuid16,
    pub name: String,
}

impl LimitInfoRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.name.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LimitUpdateRequest {
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

impl LimitUpdateRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.name.len() + 1 + self.counter.len() + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + self.error_message.len() + 1 + self.descr.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
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
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LimitRemoveRequest {
    pub cluster: Uuid16,
    pub name: String,
}

impl LimitRemoveRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.name.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        Ok(())
    }
}




