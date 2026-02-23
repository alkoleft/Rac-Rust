use crate::error::RacError;
use crate::codec::v8_datetime_to_iso;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::metadata::RpcMethodMeta;
use crate::Uuid16;
use crate::rac_wire::encode_with_len_u8;

#[derive(Debug, Serialize, Clone)]
pub struct CounterRecord {
    pub name: String,
    pub collection_time: u64,
    pub group: u8,
    pub filter_type: u8,
    pub filter: String,
    pub duration: u8,
    pub cpu_time: u8,
    pub duration_dbms: u8,
    pub service: u8,
    pub memory: u8,
    pub read: u8,
    pub write: u8,
    pub dbms_bytes: u8,
    pub call: u8,
    pub number_of_active_sessions: u8,
    pub number_of_sessions: u8,
    pub descr: String,
}

impl CounterRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let name = cursor.take_str8()?;
        let collection_time = cursor.take_u64_be()?;
        let group = cursor.take_u8()?;
        let filter_type = cursor.take_u8()?;
        let filter = cursor.take_str8()?;
        let duration = cursor.take_u8()?;
        let cpu_time = cursor.take_u8()?;
        let duration_dbms = cursor.take_u8()?;
        let service = cursor.take_u8()?;
        let memory = cursor.take_u8()?;
        let read = cursor.take_u8()?;
        let write = cursor.take_u8()?;
        let dbms_bytes = cursor.take_u8()?;
        let call = cursor.take_u8()?;
        let number_of_active_sessions = cursor.take_u8()?;
        let number_of_sessions = cursor.take_u8()?;
        let descr = cursor.take_str8()?;
        Ok(Self {
            name,
            collection_time,
            group,
            filter_type,
            filter,
            duration,
            cpu_time,
            duration_dbms,
            service,
            memory,
            read,
            write,
            dbms_bytes,
            call,
            number_of_active_sessions,
            number_of_sessions,
            descr,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CounterValuesRecord {
    pub object: String,
    pub collection_time: u64,
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
    pub time: String,
}

impl CounterValuesRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let object = cursor.take_str8()?;
        let collection_time = cursor.take_u64_be()?;
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
        let time = v8_datetime_to_iso(cursor.take_u64_be()?).unwrap_or_default();
        Ok(Self {
            object,
            collection_time,
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
            time,
        })
    }
}

pub const RPC_COUNTER_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 118,
    method_resp: Some(119),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_INFO_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 120,
    method_resp: Some(121),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_UPDATE_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 122,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_REMOVE_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 123,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_CLEAR_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 132,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_VALUES_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 130,
    method_resp: Some(131),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_ACCUMULATED_VALUES_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 133,
    method_resp: Some(134),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


pub fn parse_counter_info_body(body: &[u8]) -> Result<CounterRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("counter info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    CounterRecord::decode(&mut cursor)
}

#[derive(Debug, Clone)]
pub struct CounterListRequest {
    pub cluster: Uuid16,
}

impl CounterListRequest {
    pub fn encoded_len(&self) -> usize {
        16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CounterInfoRequest {
    pub cluster: Uuid16,
    pub counter: String,
}

impl CounterInfoRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.counter.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CounterUpdateRequest {
    pub cluster: Uuid16,
    pub name: String,
    pub collection_time: u64,
    pub group: u8,
    pub filter_type: u8,
    pub filter: String,
    pub duration: u8,
    pub cpu_time: u8,
    pub duration_dbms: u8,
    pub service: u8,
    pub memory: u8,
    pub read: u8,
    pub write: u8,
    pub dbms_bytes: u8,
    pub call: u8,
    pub number_of_active_sessions: u8,
    pub number_of_sessions: u8,
    pub descr: String,
}

impl CounterUpdateRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.name.len() + 8 + 1 + 1 + 1 + self.filter.len() + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + self.descr.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        out.extend_from_slice(&self.collection_time.to_be_bytes());
        out.push(self.group);
        out.push(self.filter_type);
        out.extend_from_slice(&encode_with_len_u8(self.filter.as_bytes())?);
        out.push(self.duration);
        out.push(self.cpu_time);
        out.push(self.duration_dbms);
        out.push(self.service);
        out.push(self.memory);
        out.push(self.read);
        out.push(self.write);
        out.push(self.dbms_bytes);
        out.push(self.call);
        out.push(self.number_of_active_sessions);
        out.push(self.number_of_sessions);
        out.extend_from_slice(&encode_with_len_u8(self.descr.as_bytes())?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CounterRemoveRequest {
    pub cluster: Uuid16,
    pub name: String,
}

impl CounterRemoveRequest {
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
pub struct CounterClearRequest {
    pub cluster: Uuid16,
    pub counter: String,
    pub object: String,
}

impl CounterClearRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.counter.len() + 1 + self.object.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.object.as_bytes())?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CounterValuesRequest {
    pub cluster: Uuid16,
    pub counter: String,
    pub object: String,
}

impl CounterValuesRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.counter.len() + 1 + self.object.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.object.as_bytes())?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CounterAccumulatedValuesRequest {
    pub cluster: Uuid16,
    pub counter: String,
    pub object: String,
}

impl CounterAccumulatedValuesRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.counter.len() + 1 + self.object.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.object.as_bytes())?);
        Ok(())
    }
}


