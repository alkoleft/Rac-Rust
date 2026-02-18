use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::rac_wire::encode_with_len_u8;

use super::rpc_body;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rac_wire::parse_frames;

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
}
