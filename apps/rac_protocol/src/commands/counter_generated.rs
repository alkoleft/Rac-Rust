use crate::error::RacError;
use crate::codec::v8_datetime_to_iso;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::Uuid16;
use crate::rac_wire::encode_with_len_u8;

pub const METHOD_COUNTER_LIST_REQ: u8 = 0x76;
pub const METHOD_COUNTER_LIST_RESP: u8 = 0x77;
pub const METHOD_COUNTER_INFO_REQ: u8 = 0x78;
pub const METHOD_COUNTER_INFO_RESP: u8 = 0x79;
pub const METHOD_COUNTER_UPDATE_REQ: u8 = 0x7a;
pub const METHOD_COUNTER_REMOVE_REQ: u8 = 0x7b;
pub const METHOD_COUNTER_CLEAR_REQ: u8 = 0x84;
pub const METHOD_COUNTER_VALUES_REQ: u8 = 0x82;
pub const METHOD_COUNTER_VALUES_RESP: u8 = 0x83;
pub const METHOD_COUNTER_ACCUMULATED_VALUES_REQ: u8 = 0x85;
pub const METHOD_COUNTER_ACCUMULATED_VALUES_RESP: u8 = 0x86;

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

pub struct CounterListRpc {
    pub cluster: Uuid16,
}

impl crate::rpc::Request for CounterListRpc {
    type Response = CounterListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_COUNTER_LIST_META
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

pub struct CounterInfoRpc {
    pub cluster: Uuid16,
    pub counter: String,
}

impl crate::rpc::Request for CounterInfoRpc {
    type Response = CounterInfoResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_COUNTER_INFO_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 1 + self.counter.len());
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        Ok(out)
    }
}

pub struct CounterUpdateRpc {
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

impl crate::rpc::Request for CounterUpdateRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_COUNTER_UPDATE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 1 + self.name.len() + 8 + 1 + 1 + 1 + self.filter.len() + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + self.descr.len());
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
        Ok(out)
    }
}

pub struct CounterRemoveRpc {
    pub cluster: Uuid16,
    pub name: String,
}

impl crate::rpc::Request for CounterRemoveRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_COUNTER_REMOVE_META
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

pub struct CounterClearRpc {
    pub cluster: Uuid16,
    pub counter: String,
    pub object: String,
}

impl crate::rpc::Request for CounterClearRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_COUNTER_CLEAR_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 1 + self.counter.len() + 1 + self.object.len());
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.object.as_bytes())?);
        Ok(out)
    }
}

pub struct CounterValuesRpc {
    pub cluster: Uuid16,
    pub counter: String,
    pub object: String,
}

impl crate::rpc::Request for CounterValuesRpc {
    type Response = CounterValuesResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_COUNTER_VALUES_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 1 + self.counter.len() + 1 + self.object.len());
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.object.as_bytes())?);
        Ok(out)
    }
}

pub struct CounterAccumulatedValuesRpc {
    pub cluster: Uuid16,
    pub counter: String,
    pub object: String,
}

impl crate::rpc::Request for CounterAccumulatedValuesRpc {
    type Response = CounterAccumulatedValuesResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_COUNTER_ACCUMULATED_VALUES_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 1 + self.counter.len() + 1 + self.object.len());
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.object.as_bytes())?);
        Ok(out)
    }
}


#[derive(Debug, Serialize)]
pub struct CounterListResp {
    pub records: Vec<CounterRecord>,
}

impl crate::rpc::Response for CounterListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        Ok(Self {
            records: crate::commands::parse_list_u8(body, CounterRecord::decode)?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CounterInfoResp {
    pub record: CounterRecord,
}

impl crate::rpc::Response for CounterInfoResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let record = parse_counter_info_body(body)?;
        Ok(Self {
            record: record,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CounterValuesResp {
    pub records: Vec<CounterValuesRecord>,
}

impl crate::rpc::Response for CounterValuesResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        Ok(Self {
            records: crate::commands::parse_list_u8(body, CounterValuesRecord::decode)?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct CounterAccumulatedValuesResp {
    pub records: Vec<CounterValuesRecord>,
}

impl crate::rpc::Response for CounterAccumulatedValuesResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        Ok(Self {
            records: crate::commands::parse_list_u8(body, CounterValuesRecord::decode)?,
        })
    }
}


pub fn parse_counter_info_body(body: &[u8]) -> Result<CounterRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("counter info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    CounterRecord::decode(&mut cursor)
}


pub const RPC_COUNTER_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_COUNTER_LIST_REQ,
    method_resp: Some(METHOD_COUNTER_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_COUNTER_INFO_REQ,
    method_resp: Some(METHOD_COUNTER_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_UPDATE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_COUNTER_UPDATE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_REMOVE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_COUNTER_REMOVE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_CLEAR_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_COUNTER_CLEAR_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_VALUES_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_COUNTER_VALUES_REQ,
    method_resp: Some(METHOD_COUNTER_VALUES_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_COUNTER_ACCUMULATED_VALUES_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_COUNTER_ACCUMULATED_VALUES_REQ,
    method_resp: Some(METHOD_COUNTER_ACCUMULATED_VALUES_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


