use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use super::rpc_body;
use crate::Uuid16;

#[derive(Debug, Serialize)]
pub struct LimitListResp {
    pub limits: Vec<LimitRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct LimitInfoResp {
    pub record: LimitRecord,
    pub raw_payload: Option<Vec<u8>>,
}

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

pub fn limit_list(client: &mut RacClient, cluster: Uuid16) -> Result<LimitListResp> {
    let reply = client.call(RacRequest::LimitList { cluster })?;
    Ok(LimitListResp {
        limits: parse_limit_list_body(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn limit_info(client: &mut RacClient, cluster: Uuid16, limit: &str) -> Result<LimitInfoResp> {
    let reply = client.call(RacRequest::LimitInfo {
        cluster,
        limit: limit.to_string(),
    })?;
    Ok(LimitInfoResp {
        record: parse_limit_info_body(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

fn parse_limit_list_body(body: &[u8]) -> Result<Vec<LimitRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut limits = Vec::with_capacity(count);
    for _ in 0..count {
        limits.push(parse_limit_record(&mut cursor)?);
    }
    Ok(limits)
}

fn parse_limit_info_body(body: &[u8]) -> Result<LimitRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("limit info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    parse_limit_record(&mut cursor)
}

fn parse_limit_record(cursor: &mut RecordCursor<'_>) -> Result<LimitRecord> {
    let name = cursor.take_str8()?;
    let counter = cursor.take_str8()?;
    let action = cursor
        .take_u8()
        .map_err(|_| RacError::Decode("limit record action truncated"))?;
    let duration = take_u64_required(cursor)?;
    let cpu_time = take_u64_required(cursor)?;
    let memory = take_u64_required(cursor)?;
    let read = take_u64_required(cursor)?;
    let write = take_u64_required(cursor)?;
    let duration_dbms = take_u64_required(cursor)?;
    let dbms_bytes = take_u64_required(cursor)?;
    let service = take_u64_required(cursor)?;
    let call = take_u64_required(cursor)?;
    let number_of_active_sessions = take_u64_required(cursor)?;
    let number_of_sessions = take_u64_required(cursor)?;
    let error_message = cursor.take_str8()?;
    let descr = cursor.take_str8()?;
    Ok(LimitRecord {
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

fn take_u64_required(cursor: &mut RecordCursor<'_>) -> Result<u64> {
    cursor
        .take_u64_be_opt()?
        .ok_or(RacError::Decode("limit record u64 truncated"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rac_wire::parse_frames;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn parse_limit_list_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/limit_list_nonempty_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let limits = parse_limit_list_body(body).expect("limit list parse");

        assert_eq!(limits.len(), 2);
        assert_eq!(limits[0].name, "limit_codex_a");
        assert_eq!(limits[0].counter, "cpu");
        assert_eq!(limits[0].action, 2);
        assert_eq!(limits[0].cpu_time, 11);
        assert_eq!(limits[0].error_message, "limit_a");
        assert_eq!(limits[0].descr, "limit_a");

        assert_eq!(limits[1].name, "limit_codex_b");
        assert_eq!(limits[1].counter, "call");
        assert_eq!(limits[1].action, 3);
        assert_eq!(limits[1].call, 7);
        assert_eq!(limits[1].error_message, "limit_b");
        assert_eq!(limits[1].descr, "limit_b");
    }

    #[test]
    fn parse_limit_info_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/limit_info_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert!(!frames.is_empty());
        let frame = frames.last().expect("frame");
        assert_eq!(frame.opcode, 0x0e);
        let body = rpc_body(&frame.payload).expect("rpc body");
        let record = parse_limit_info_body(body).expect("limit info parse");

        assert_eq!(record.name, "limit_codex_tmp");
        assert_eq!(record.counter, "cpu");
        assert_eq!(record.action, 2);
        assert_eq!(record.cpu_time, 10);
        assert_eq!(record.error_message, "limit_tmp");
        assert_eq!(record.descr, "limit_tmp");
    }
}
