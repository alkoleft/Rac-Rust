use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

#[derive(Debug, Serialize)]
pub struct ProcessListResp {
    pub processes: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ProcessInfoResp {
    pub process: Uuid16,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn process_list(client: &mut RacClient, cluster: Uuid16) -> Result<ProcessListResp> {
    let reply = client.call(RacRequest::ProcessList { cluster })?;
    let body = rpc_body(&reply)?;
    let processes = parse_process_list(body)?;
    Ok(ProcessListResp {
        processes,
        raw_payload: Some(reply),
    })
}

pub fn process_info(
    client: &mut RacClient,
    cluster: Uuid16,
    process: Uuid16,
) -> Result<ProcessInfoResp> {
    let reply = client.call(RacRequest::ProcessInfo { cluster, process })?;
    let body = rpc_body(&reply)?;
    let mut cursor = RecordCursor::new(body, 0);
    let record = parse_process_record(&mut cursor)?;
    Ok(ProcessInfoResp {
        process: record.uuid,
        fields: record.strings,
        raw_payload: Some(reply),
    })
}

struct ProcessRecordParts {
    uuid: Uuid16,
    strings: Vec<String>,
}

fn parse_process_list(body: &[u8]) -> Result<Vec<Uuid16>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut processes = Vec::with_capacity(count);
    for _ in 0..count {
        let record = parse_process_record(&mut cursor)?;
        processes.push(record.uuid);
    }
    Ok(processes)
}

fn parse_process_record(cursor: &mut RecordCursor<'_>) -> Result<ProcessRecordParts> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("process record truncated"));
    }
    let uuid = cursor.take_uuid()?;
    let mut strings = Vec::new();

    let _gap_0 = cursor.take_bytes(8)?;
    let _avg_call_time = cursor.take_f64_be()?;
    let _avg_db_call_time = cursor.take_f64_be()?;
    let _avg_lock_call_time = cursor.take_f64_be()?;
    let _avg_server_call_time = cursor.take_f64_be()?;
    let _avg_threads = cursor.take_f64_be()?;
    let _capacity = cursor.take_u32_be()?;
    let _connections = cursor.take_u32_be()?;

    let host = cursor.take_str8()?;
    push_if_nonempty(&mut strings, &host);

    let license_count = cursor.take_u8()?;
    for _ in 0..license_count {
        let _gap_license_0 = cursor.take_u8()?;
        let file_name = cursor.take_str8()?;
        push_if_nonempty(&mut strings, &file_name);
        let full_len = take_u14_len(cursor)?;
        let full_bytes = cursor.take_bytes(full_len)?;
        let full_presentation = String::from_utf8_lossy(&full_bytes).to_string();
        push_if_nonempty(&mut strings, &full_presentation);
        let _issued_by_server = cursor.take_bool()?;
        let _license_type = cursor.take_u32_be()?;
        let _max_users_all = cursor.take_u32_be()?;
        let _max_users_current = cursor.take_u32_be()?;
        let _network_key = cursor.take_bool()?;
        let server_address = cursor.take_str8()?;
        push_if_nonempty(&mut strings, &server_address);
        let process_id = cursor.take_str8()?;
        push_if_nonempty(&mut strings, &process_id);
        let _server_port = cursor.take_u32_be()?;
        let key_series = cursor.take_str8()?;
        push_if_nonempty(&mut strings, &key_series);
        let brief_presentation = cursor.take_str8()?;
        push_if_nonempty(&mut strings, &brief_presentation);
    }

    let _port = cursor.take_u16_be()?;
    let _memory_excess_time = cursor.take_u32_be()?;
    let _memory_size = cursor.take_u32_be()?;
    let pid = cursor.take_str8()?;
    push_if_nonempty(&mut strings, &pid);
    let _use = cursor.take_u32_be()?;
    let _selection_size = cursor.take_u32_be()?;
    let _started_at = cursor.take_u64_be()?;
    let _running = cursor.take_u32_be()?;
    let _available_performance = cursor.take_u32_be()?;
    let _reserve = cursor.take_u8()?;

    Ok(ProcessRecordParts { uuid, strings })
}

fn take_u14_len(cursor: &mut RecordCursor<'_>) -> Result<usize> {
    let b0 = cursor.take_u8()? as usize;
    let b1 = cursor.take_u8()? as usize;
    Ok((b0 & 0x3f) | (b1 << 6))
}

fn push_if_nonempty(out: &mut Vec<String>, value: &str) {
    if !value.is_empty() {
        out.push(value.to_string());
    }
}
