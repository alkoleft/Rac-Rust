use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
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



pub fn parse_limit_info_body(body: &[u8]) -> Result<LimitRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("limit info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    LimitRecord::decode(&mut cursor)
}


pub const RPC_LIMIT_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x7c,
    method_resp: Some(0x7d),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_LIMIT_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x7e,
    method_resp: Some(0x7f),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_LIMIT_UPDATE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x80,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_LIMIT_REMOVE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x81,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};


