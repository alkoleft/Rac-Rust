use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::rac_wire::encode_with_len_u8;

use super::rpc_body;

mod generated {
    include!("counter_generated.rs");
}

pub use generated::{CounterRecord, CounterValuesRecord};

#[derive(Debug, Serialize)]
pub struct CounterListResp {
    pub records: Vec<CounterRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct CounterInfoResp {
    pub record: CounterRecord,
    pub raw_payload: Option<Vec<u8>>,
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
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct CounterRemoveResp {
    pub acknowledged: bool,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct CounterClearResp {
    pub acknowledged: bool,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct CounterValuesResp {
    pub records: Vec<CounterValuesRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct CounterAccumulatedValuesResp {
    pub records: Vec<CounterValuesRecord>,
    pub raw_payload: Option<Vec<u8>>,
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
    let reply = client.call(RacRequest::CounterList { cluster })?;
    let body = rpc_body(&reply)?;
    let records = parse_counter_list_body(body)?;
    Ok(CounterListResp {
        records,
        raw_payload: Some(reply),
    })
}

pub fn counter_info(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    counter: &str,
) -> Result<CounterInfoResp> {
    let reply = client.call(RacRequest::CounterInfo {
        cluster,
        counter: counter.to_string(),
    })?;
    let body = rpc_body(&reply)?;
    let record = parse_counter_info_body(body)?;
    Ok(CounterInfoResp {
        record,
        raw_payload: Some(reply),
    })
}

pub fn counter_update(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    req: CounterUpdateReq,
) -> Result<CounterUpdateResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::CounterUpdate {
        cluster,
        name: req.name,
        collection_time: req.collection_time,
        group: req.group,
        filter_type: req.filter_type,
        filter: req.filter,
        duration: req.duration,
        cpu_time: req.cpu_time,
        duration_dbms: req.duration_dbms,
        service: req.service,
        memory: req.memory,
        read: req.read,
        write: req.write,
        dbms_bytes: req.dbms_bytes,
        call: req.call,
        number_of_active_sessions: req.number_of_active_sessions,
        number_of_sessions: req.number_of_sessions,
        descr: req.descr,
    })?;
    let acknowledged = parse_counter_update_ack(&reply)?;
    if !acknowledged {
        return Err(crate::error::RacError::Decode("counter update expected ack"));
    }
    Ok(CounterUpdateResp {
        acknowledged,
        raw_payload: Some(reply),
    })
}

pub fn counter_clear(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    counter: &str,
    object: &str,
) -> Result<CounterClearResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::CounterClear {
        cluster,
        counter: counter.to_string(),
        object: object.to_string(),
    })?;
    let acknowledged = parse_counter_clear_ack(&reply)?;
    if !acknowledged {
        return Err(crate::error::RacError::Decode("counter clear expected ack"));
    }
    Ok(CounterClearResp {
        acknowledged,
        raw_payload: Some(reply),
    })
}

pub fn counter_remove(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    name: &str,
) -> Result<CounterRemoveResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::CounterRemove {
        cluster,
        name: name.to_string(),
    })?;
    let acknowledged = parse_counter_remove_ack(&reply)?;
    if !acknowledged {
        return Err(crate::error::RacError::Decode("counter remove expected ack"));
    }
    Ok(CounterRemoveResp {
        acknowledged,
        raw_payload: Some(reply),
    })
}

pub fn counter_values(
    client: &mut RacClient,
    cluster: crate::Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    counter: &str,
    object: &str,
) -> Result<CounterValuesResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::CounterValues {
        cluster,
        counter: counter.to_string(),
        object: object.to_string(),
    })?;
    let body = rpc_body(&reply)?;
    let records = parse_counter_values_body(body)?;
    Ok(CounterValuesResp {
        records,
        raw_payload: Some(reply),
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
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::CounterAccumulatedValues {
        cluster,
        counter: counter.to_string(),
        object: object.to_string(),
    })?;
    let body = rpc_body(&reply)?;
    let records = parse_counter_accumulated_values_body(body)?;
    Ok(CounterAccumulatedValuesResp {
        records,
        raw_payload: Some(reply),
    })
}

fn parse_counter_list_body(body: &[u8]) -> Result<Vec<CounterRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(CounterRecord::decode(&mut cursor)?);
    }
    Ok(records)
}

fn parse_counter_info_body(body: &[u8]) -> Result<CounterRecord> {
    let mut cursor = RecordCursor::new(body, 0);
    CounterRecord::decode(&mut cursor)
}

fn parse_counter_values_body(body: &[u8]) -> Result<Vec<CounterValuesRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
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
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(CounterValuesRecord::decode(&mut cursor)?);
    }
    Ok(records)
}

fn parse_counter_update_ack(payload: &[u8]) -> Result<bool> {
    let mut cursor = RecordCursor::new(payload, 0);
    if cursor.remaining_len() < 4 {
        return Err(crate::error::RacError::Decode("counter update ack truncated"));
    }
    let ack = cursor.take_u32_be()?;
    Ok(ack == 0x01000000)
}

fn parse_counter_clear_ack(payload: &[u8]) -> Result<bool> {
    let mut cursor = RecordCursor::new(payload, 0);
    if cursor.remaining_len() < 4 {
        return Err(crate::error::RacError::Decode("counter clear ack truncated"));
    }
    let ack = cursor.take_u32_be()?;
    Ok(ack == 0x01000000)
}

fn parse_counter_remove_ack(payload: &[u8]) -> Result<bool> {
    let mut cursor = RecordCursor::new(payload, 0);
    if cursor.remaining_len() < 4 {
        return Err(crate::error::RacError::Decode("counter remove ack truncated"));
    }
    let ack = cursor.take_u32_be()?;
    Ok(ack == 0x01000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::RacProtocolVersion;
    use crate::rac_wire::parse_frames;
    use crate::rac_wire::parse_uuid;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn parse_counter_list_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/counter_list_response.hex");
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
        let hex = include_str!("../../../../artifacts/counter_info_codex_tmp_response.hex");
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
        let hex = include_str!("../../../../artifacts/counter_update_codex_tmp_response.hex");
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
        let hex = include_str!("../../../../artifacts/counter_clear_codex_tmp_response.hex");
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
        let hex = include_str!("../../../../artifacts/counter_remove_codex_tmp_response.hex");
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
        let req = RacRequest::CounterUpdate {
            cluster,
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
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_counter_remove_request() {
        let expected = decode_hex_str(
            "010000017b1619820ad36f4d8aa7161516b1dea07709636f6465785f746d70",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let req = RacRequest::CounterRemove {
            cluster,
            name: "codex_tmp".to_string(),
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_counter_clear_request() {
        let expected = decode_hex_str(
            "01000001841619820ad36f4d8aa7161516b1dea07709636f6465785f746d7000",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let req = RacRequest::CounterClear {
            cluster,
            counter: "codex_tmp".to_string(),
            object: "".to_string(),
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn parse_counter_values_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/counter_values_codex_tmp_response.hex");
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
        let req = RacRequest::CounterValues {
            cluster,
            counter: "codex_tmp".to_string(),
            object: "".to_string(),
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x83));
    }

    #[test]
    fn parse_counter_accumulated_values_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/counter_accumulated_values_codex_tmp_response.hex");
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
        let req = RacRequest::CounterAccumulatedValues {
            cluster,
            counter: "codex_tmp".to_string(),
            object: "".to_string(),
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x86));
    }
}
