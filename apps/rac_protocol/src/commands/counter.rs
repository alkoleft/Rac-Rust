use serde::Serialize;

use crate::client::RacClient;
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::protocol::ProtocolCodec;
use crate::rpc::{Meta, Request, Response};
use crate::rpc::decode_utils::{parse_ack_payload, rpc_body};
use crate::rac_wire::{
    METHOD_COUNTER_ACCUMULATED_VALUES_REQ, METHOD_COUNTER_ACCUMULATED_VALUES_RESP,
    METHOD_COUNTER_CLEAR_REQ, METHOD_COUNTER_INFO_REQ, METHOD_COUNTER_INFO_RESP,
    METHOD_COUNTER_LIST_REQ, METHOD_COUNTER_LIST_RESP, METHOD_COUNTER_REMOVE_REQ,
    METHOD_COUNTER_UPDATE_REQ, METHOD_COUNTER_VALUES_REQ, METHOD_COUNTER_VALUES_RESP,
};
use crate::rac_wire::encode_with_len_u8;

use crate::commands::cluster_auth;

mod generated {
    include!("counter_generated.rs");
}

pub use generated::{CounterRecord, CounterValuesRecord};

#[derive(Debug, Serialize)]
pub struct CounterListResp {
    pub records: Vec<CounterRecord>,
}

#[derive(Debug, Serialize)]
pub struct CounterInfoResp {
    pub record: CounterRecord,
}

#[derive(Debug, Serialize, Clone)]
pub struct CounterUpdateReq {
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

#[derive(Debug, Serialize)]
pub struct CounterUpdateResp {
    pub acknowledged: bool,
}

#[derive(Debug, Serialize)]
pub struct CounterRemoveResp {
    pub acknowledged: bool,
}

#[derive(Debug, Serialize)]
pub struct CounterClearResp {
    pub acknowledged: bool,
}

#[derive(Debug, Serialize)]
pub struct CounterValuesResp {
    pub records: Vec<CounterValuesRecord>,
}

#[derive(Debug, Serialize)]
pub struct CounterAccumulatedValuesResp {
    pub records: Vec<CounterValuesRecord>,
}

struct CounterListRpc {
    cluster: crate::Uuid16,
}

impl Request for CounterListRpc {
    type Response = CounterListResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_COUNTER_LIST_REQ,
            method_resp: Some(METHOD_COUNTER_LIST_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        Ok(self.cluster.to_vec())
    }
}

impl Response for CounterListResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let records = parse_counter_list_body(body)?;
        Ok(Self { records })
    }
}

struct CounterInfoRpc {
    cluster: crate::Uuid16,
    counter: String,
}

impl Request for CounterInfoRpc {
    type Response = CounterInfoResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_COUNTER_INFO_REQ,
            method_resp: Some(METHOD_COUNTER_INFO_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut body = Vec::with_capacity(16 + 1 + self.counter.len());
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        Ok(body)
    }
}

impl Response for CounterInfoResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let record = parse_counter_info_body(body)?;
        Ok(Self { record })
    }
}

struct CounterUpdateRpc {
    cluster: crate::Uuid16,
    req: CounterUpdateReq,
}

impl Request for CounterUpdateRpc {
    type Response = Vec<u8>;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_COUNTER_UPDATE_REQ,
            method_resp: None,
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let req = &self.req;
        let mut body = Vec::with_capacity(16 + 32 + req.name.len() + req.filter.len() + req.descr.len());
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&encode_with_len_u8(req.name.as_bytes())?);
        body.extend_from_slice(&req.collection_time.to_be_bytes());
        body.push(req.group);
        body.push(req.filter_type);
        body.extend_from_slice(&encode_with_len_u8(req.filter.as_bytes())?);
        body.push(req.duration);
        body.push(req.cpu_time);
        body.push(req.duration_dbms);
        body.push(req.service);
        body.push(req.memory);
        body.push(req.read);
        body.push(req.write);
        body.push(req.dbms_bytes);
        body.push(req.call);
        body.push(req.number_of_active_sessions);
        body.push(req.number_of_sessions);
        body.extend_from_slice(&encode_with_len_u8(req.descr.as_bytes())?);
        Ok(body)
    }
}

struct CounterRemoveRpc {
    cluster: crate::Uuid16,
    name: String,
}

impl Request for CounterRemoveRpc {
    type Response = Vec<u8>;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_COUNTER_REMOVE_REQ,
            method_resp: None,
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut body = Vec::with_capacity(16 + 1 + self.name.len());
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        Ok(body)
    }
}

struct CounterClearRpc {
    cluster: crate::Uuid16,
    counter: String,
    object: String,
}

impl Request for CounterClearRpc {
    type Response = Vec<u8>;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_COUNTER_CLEAR_REQ,
            method_resp: None,
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut body = Vec::with_capacity(16 + 2 + self.counter.len() + self.object.len());
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        body.extend_from_slice(&encode_with_len_u8(self.object.as_bytes())?);
        Ok(body)
    }
}

struct CounterValuesRpc {
    cluster: crate::Uuid16,
    counter: String,
    object: String,
}

impl Request for CounterValuesRpc {
    type Response = CounterValuesResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_COUNTER_VALUES_REQ,
            method_resp: Some(METHOD_COUNTER_VALUES_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut body = Vec::with_capacity(16 + 2 + self.counter.len() + self.object.len());
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        body.extend_from_slice(&encode_with_len_u8(self.object.as_bytes())?);
        Ok(body)
    }
}

impl Response for CounterValuesResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let records = parse_counter_values_body(body)?;
        Ok(Self { records })
    }
}

struct CounterAccumulatedValuesRpc {
    cluster: crate::Uuid16,
    counter: String,
    object: String,
}

impl Request for CounterAccumulatedValuesRpc {
    type Response = CounterAccumulatedValuesResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_COUNTER_ACCUMULATED_VALUES_REQ,
            method_resp: Some(METHOD_COUNTER_ACCUMULATED_VALUES_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut body = Vec::with_capacity(16 + 2 + self.counter.len() + self.object.len());
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&encode_with_len_u8(self.counter.as_bytes())?);
        body.extend_from_slice(&encode_with_len_u8(self.object.as_bytes())?);
        Ok(body)
    }
}

impl Response for CounterAccumulatedValuesResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let records = parse_counter_accumulated_values_body(body)?;
        Ok(Self { records })
    }
}

impl CounterRecord {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut out = Vec::new();
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

pub fn counter_list(client: &mut RacClient, cluster: crate::Uuid16) -> Result<CounterListResp> {
    client.call_typed(CounterListRpc { cluster })
}

pub fn counter_info(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    counter: &str,
) -> Result<CounterInfoResp> {
    client.call_typed(CounterInfoRpc {
        cluster,
        counter: counter.to_string(),
    })
}

pub fn counter_update(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    req: CounterUpdateReq,
) -> Result<CounterUpdateResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    let reply = client.call(CounterUpdateRpc { cluster, req })?;
    let acknowledged = parse_ack_payload(&reply, "counter update expected ack")?;
    Ok(CounterUpdateResp { acknowledged })
}

pub fn counter_clear(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    counter: &str,
    object: &str,
) -> Result<CounterClearResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    let reply = client.call(CounterClearRpc {
        cluster,
        counter: counter.to_string(),
        object: object.to_string(),
    })?;
    let acknowledged = parse_ack_payload(&reply, "counter clear expected ack")?;
    Ok(CounterClearResp { acknowledged })
}

pub fn counter_remove(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    name: &str,
) -> Result<CounterRemoveResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    let reply = client.call(CounterRemoveRpc {
        cluster,
        name: name.to_string(),
    })?;
    let acknowledged = parse_ack_payload(&reply, "counter remove expected ack")?;
    Ok(CounterRemoveResp { acknowledged })
}

pub fn counter_values(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    counter: &str,
    object: &str,
) -> Result<CounterValuesResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    client.call_typed(CounterValuesRpc {
        cluster,
        counter: counter.to_string(),
        object: object.to_string(),
    })
}

pub fn counter_accumulated_values(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    counter: &str,
    object: &str,
) -> Result<CounterAccumulatedValuesResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    client.call_typed(CounterAccumulatedValuesRpc {
        cluster,
        counter: counter.to_string(),
        object: object.to_string(),
    })
}

fn parse_counter_list_body(body: &[u8]) -> Result<Vec<CounterRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body);
    let count = cursor.take_u8()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(CounterRecord::decode(&mut cursor)?);
    }
    Ok(records)
}

fn parse_counter_info_body(body: &[u8]) -> Result<CounterRecord> {
    let mut cursor = RecordCursor::new(body);
    CounterRecord::decode(&mut cursor)
}

fn parse_counter_values_body(body: &[u8]) -> Result<Vec<CounterValuesRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body);
    let count = cursor.take_u8()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(CounterValuesRecord::decode(&mut cursor)?);
    }
    Ok(records)
}

fn parse_counter_accumulated_values_body(body: &[u8]) -> Result<Vec<CounterValuesRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body);
    let count = cursor.take_u8()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(CounterValuesRecord::decode(&mut cursor)?);
    }
    Ok(records)
}

#[cfg(test)]
fn parse_counter_update_ack(payload: &[u8]) -> Result<bool> {
    let mut cursor = RecordCursor::new(payload);
    if cursor.remaining_len() < 4 {
        return Err(crate::error::RacError::Decode("counter update ack truncated"));
    }
    let ack = cursor.take_u32_be()?;
    Ok(ack == 0x01000000)
}

#[cfg(test)]
fn parse_counter_clear_ack(payload: &[u8]) -> Result<bool> {
    let mut cursor = RecordCursor::new(payload);
    if cursor.remaining_len() < 4 {
        return Err(crate::error::RacError::Decode("counter clear ack truncated"));
    }
    let ack = cursor.take_u32_be()?;
    Ok(ack == 0x01000000)
}

#[cfg(test)]
fn parse_counter_remove_ack(payload: &[u8]) -> Result<bool> {
    let mut cursor = RecordCursor::new(payload);
    if cursor.remaining_len() < 4 {
        return Err(crate::error::RacError::Decode("counter remove ack truncated"));
    }
    let ack = cursor.take_u32_be()?;
    Ok(ack == 0x01000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ProtocolVersion;
    use crate::rpc::Request;
    use crate::rac_wire::parse_frames;
    use crate::rac_wire::parse_uuid;
    use crate::commands::rpc_body;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn parse_counter_list_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_list_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let records = parse_counter_list_body(body).expect("counter list parse");

        assert_eq!(records.len(), 11);
        assert_eq!(records[0].name, "Вызовы");
        assert_eq!(records[0].collection_time, 5);
        assert_eq!(records[0].group, 0);
        assert_eq!(records[0].filter_type, 2);
        assert_eq!(records[0].filter, "2");
        assert_eq!(records[0].duration, 1);
        assert_eq!(records[0].descr, "");

        assert_eq!(records[1].name, "cpu");
        assert_eq!(records[1].collection_time, 6);
        assert_eq!(records[1].cpu_time, 1);
        assert_eq!(records[1].descr, "cpu desc");

        assert_eq!(records[8].name, "sessions");
        assert_eq!(records[8].collection_time, 2000);
        assert_eq!(records[8].number_of_sessions, 1);
        assert_eq!(records[8].descr, "sessions d");

        assert_eq!(records[10].name, "serv call");
        assert_eq!(records[10].call, 1);
    }

    #[test]
    fn parse_counter_info_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_info_codex_tmp_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let record = parse_counter_info_body(body).expect("counter info parse");

        assert_eq!(record.name, "codex_tmp");
        assert_eq!(record.collection_time, 12);
        assert_eq!(record.group, 0);
        assert_eq!(record.filter_type, 2);
        assert_eq!(record.filter, "1");
        assert_eq!(record.duration, 1);
        assert_eq!(record.cpu_time, 0);
        assert_eq!(record.duration_dbms, 0);
        assert_eq!(record.service, 0);
        assert_eq!(record.memory, 1);
        assert_eq!(record.read, 0);
        assert_eq!(record.write, 1);
        assert_eq!(record.dbms_bytes, 1);
        assert_eq!(record.call, 1);
        assert_eq!(record.number_of_active_sessions, 0);
        assert_eq!(record.number_of_sessions, 1);
        assert_eq!(record.descr, "codex_tmp");
    }

    #[test]
    fn parse_counter_update_ack_payload() {
        let payload = decode_hex_str("01000000");
        let acknowledged = parse_counter_update_ack(&payload).expect("parse ack");
        assert!(acknowledged);
    }

    #[test]
    fn parse_counter_update_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_update_codex_tmp_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let acknowledged = parse_counter_update_ack(&frames[3].payload).expect("parse ack");
        assert!(acknowledged);
    }

    #[test]
    fn parse_counter_clear_ack_payload() {
        let payload = decode_hex_str("01000000");
        let acknowledged = parse_counter_clear_ack(&payload).expect("parse ack");
        assert!(acknowledged);
    }

    #[test]
    fn parse_counter_clear_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_clear_codex_tmp_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let acknowledged = parse_counter_clear_ack(&frames[3].payload).expect("parse ack");
        assert!(acknowledged);
    }

    #[test]
    fn parse_counter_remove_ack_payload() {
        let payload = decode_hex_str("01000000");
        let acknowledged = parse_counter_remove_ack(&payload).expect("parse ack");
        assert!(acknowledged);
    }

    #[test]
    fn parse_counter_remove_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_remove_codex_tmp_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[2].opcode, 0x0e);
        assert_eq!(frames[3].opcode, 0x0e);
        let acknowledged = parse_counter_remove_ack(&frames[2].payload).expect("parse ack");
        assert!(acknowledged);
        let acknowledged = parse_counter_remove_ack(&frames[3].payload).expect("parse ack");
        assert!(acknowledged);
    }

    #[test]
    fn encode_counter_update_request() {
        let expected = decode_hex_str(
            "010000017a1619820ad36f4d8aa7161516b1dea07709636f6465785f746d70000000000000000c00020131010000000100010101000109636f6465785f746d70",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let req = CounterUpdateReq {
            name: "codex_tmp".to_string(),
            collection_time: 12,
            group: 0,
            filter_type: 2,
            filter: "1".to_string(),
            duration: 1,
            cpu_time: 0,
            duration_dbms: 0,
            service: 0,
            memory: 1,
            read: 0,
            write: 1,
            dbms_bytes: 1,
            call: 1,
            number_of_active_sessions: 0,
            number_of_sessions: 1,
            descr: "codex_tmp".to_string(),
        };
        let rpc = CounterUpdateRpc { cluster, req };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_counter_remove_request() {
        let expected = decode_hex_str(
            "010000017b1619820ad36f4d8aa7161516b1dea07709636f6465785f746d70",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = CounterRemoveRpc {
            cluster,
            name: "codex_tmp".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_counter_clear_request() {
        let expected = decode_hex_str(
            "01000001841619820ad36f4d8aa7161516b1dea07709636f6465785f746d7000",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = CounterClearRpc {
            cluster,
            counter: "codex_tmp".to_string(),
            object: "".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn parse_counter_values_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_values_codex_tmp_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let records = parse_counter_values_body(body).expect("counter values parse");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].object, "infobase=yaxunit;user=DefUser");
        assert_eq!(records[0].collection_time, 12);
        assert_eq!(records[0].duration, 1006);
        assert_eq!(records[0].cpu_time, 0);
        assert_eq!(records[0].memory, 0);
        assert_eq!(records[0].read, 0);
        assert_eq!(records[0].write, 0);
        assert_eq!(records[0].duration_dbms, 0);
        assert_eq!(records[0].dbms_bytes, 0);
        assert_eq!(records[0].service, 0);
        assert_eq!(records[0].call, 0);
        assert_eq!(records[0].number_of_active_sessions, 0);
        assert_eq!(records[0].number_of_sessions, 1);
        assert_eq!(records[0].time, "2026-02-17T19:42:41");
    }

    #[test]
    fn encode_counter_values_request() {
        let expected = decode_hex_str(
            "01000001821619820ad36f4d8aa7161516b1dea07709636f6465785f746d7000",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = CounterValuesRpc {
            cluster,
            counter: "codex_tmp".to_string(),
            object: "".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x83));
    }

    #[test]
    fn parse_counter_accumulated_values_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_accumulated_values_codex_tmp_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let records = parse_counter_accumulated_values_body(body)
            .expect("counter accumulated values parse");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].object, "infobase=yaxunit;user=DefUser");
        assert_eq!(records[0].collection_time, 10000);
        assert_eq!(records[0].duration, 1001);
        assert_eq!(records[0].cpu_time, 0);
        assert_eq!(records[0].memory, 0);
        assert_eq!(records[0].read, 0);
        assert_eq!(records[0].write, 0);
        assert_eq!(records[0].duration_dbms, 0);
        assert_eq!(records[0].dbms_bytes, 0);
        assert_eq!(records[0].service, 0);
        assert_eq!(records[0].call, 0);
        assert_eq!(records[0].number_of_active_sessions, 0);
        assert_eq!(records[0].number_of_sessions, 0);
        assert_eq!(records[0].time, "2026-02-17T19:42:45");
    }

    #[test]
    fn encode_counter_accumulated_values_request() {
        let expected = decode_hex_str(
            "01000001851619820ad36f4d8aa7161516b1dea07709636f6465785f746d7000",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = CounterAccumulatedValuesRpc {
            cluster,
            counter: "codex_tmp".to_string(),
            object: "".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x86));
    }
}
