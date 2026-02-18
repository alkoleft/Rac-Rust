use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::{v8_datetime_to_iso, RecordCursor};
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

#[derive(Debug, Serialize, Default, Clone)]
pub struct ProcessLicense {
    pub file_name: String,
    pub key_series: String,
    pub issued_by_server: bool,
    pub license_type: u32,
    pub network_key: bool,
    pub max_users_all: u32,
    pub max_users_current: u32,
    pub server_address: String,
    pub server_port: u32,
    pub process_id: String,
    pub brief_presentation: String,
    pub full_presentation: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProcessRecord {
    pub process: Uuid16,
    pub host: String,
    pub port: u16,
    pub pid: String,
    pub turned_on: bool,
    pub running: bool,
    pub started_at: String,
    pub use_status: u32,
    pub available_performance: u32,
    pub capacity: u32,
    pub connections: u32,
    pub memory_size: u32,
    pub memory_excess_time: u32,
    pub selection_size: u32,
    pub avg_call_time: f64,
    pub avg_db_call_time: f64,
    pub avg_lock_call_time: f64,
    pub avg_server_call_time: f64,
    pub avg_threads: f64,
    pub reserve: bool,
    pub licenses: Vec<ProcessLicense>,
}

#[derive(Debug, Serialize)]
pub struct ProcessListResp {
    pub processes: Vec<Uuid16>,
    pub records: Vec<ProcessRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ProcessInfoResp {
    pub process: Uuid16,
    pub record: ProcessRecord,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn process_list(client: &mut RacClient, cluster: Uuid16) -> Result<ProcessListResp> {
    let reply = client.call(RacRequest::ProcessList { cluster })?;
    let body = rpc_body(&reply)?;
    let records = parse_process_list_records(body)?;
    Ok(ProcessListResp {
        processes: records.iter().map(|r| r.process).collect(),
        records,
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
    let record = parse_process_record_1cv8c(body)?;
    Ok(ProcessInfoResp {
        process: record.process,
        record,
        raw_payload: Some(reply),
    })
}

fn parse_process_list_records(body: &[u8]) -> Result<Vec<ProcessRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let expected = cursor.take_u8()? as usize;
    if expected == 0 {
        return Ok(Vec::new());
    }
    let mut records = Vec::with_capacity(expected);
    for _ in 0..expected {
        records.push(parse_process_record(&mut cursor)?);
    }
    Ok(records)
}

fn parse_process_record_1cv8c(data: &[u8]) -> Result<ProcessRecord> {
    let mut cursor = RecordCursor::new(data, 0);
    parse_process_record(&mut cursor)
}

fn parse_process_record(cursor: &mut RecordCursor<'_>) -> Result<ProcessRecord> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("process record truncated"));
    }
    let process = cursor.take_uuid()?;

    let _gap_0 = cursor.take_bytes(8)?;
    let avg_call_time = cursor.take_f64_be()?;
    let avg_db_call_time = cursor.take_f64_be()?;
    let avg_lock_call_time = cursor.take_f64_be()?;
    let avg_server_call_time = cursor.take_f64_be()?;
    let avg_threads = cursor.take_f64_be()?;
    let capacity = cursor.take_u32_be()?;
    let connections = cursor.take_u32_be()?;

    let host = cursor.take_str8()?;

    let license_count = cursor.take_u8()? as usize;
    let mut licenses = Vec::with_capacity(license_count);
    for _ in 0..license_count {
        let _gap_license_0 = cursor.take_u8()?;
        let file_name = cursor.take_str8()?;
        let full_len = take_u14_len(cursor)?;
        let full_bytes = cursor.take_bytes(full_len)?;
        let full_presentation = String::from_utf8_lossy(&full_bytes).to_string();
        let issued_by_server = cursor.take_bool()?;
        let license_type = cursor.take_u32_be()?;
        let max_users_all = cursor.take_u32_be()?;
        let max_users_current = cursor.take_u32_be()?;
        let network_key = cursor.take_bool()?;
        let server_address = cursor.take_str8()?;
        let process_id = cursor.take_str8()?;
        let server_port = cursor.take_u32_be()?;
        let key_series = cursor.take_str8()?;
        let brief_presentation = cursor.take_str8()?;
        licenses.push(ProcessLicense {
            file_name,
            key_series,
            issued_by_server,
            license_type,
            network_key,
            max_users_all,
            max_users_current,
            server_address,
            server_port,
            process_id,
            brief_presentation,
            full_presentation,
        });
    }

    let port = cursor.take_u16_be()?;
    let memory_excess_time = cursor.take_u32_be()?;
    let memory_size = cursor.take_u32_be()?;
    let pid = cursor.take_str8()?;
    let use_status = cursor.take_u32_be()?;
    let selection_size = cursor.take_u32_be()?;
    let started_raw = cursor.take_u64_be()?;
    let running = cursor.take_u32_be()? != 0;
    let available_performance = cursor.take_u32_be()?;
    let reserve = cursor.take_u8()? != 0;
    let started_at = v8_datetime_to_iso(started_raw).unwrap_or_default();
    let turned_on = use_status != 0;

    Ok(ProcessRecord {
        process,
        host,
        port,
        pid,
        turned_on,
        running,
        started_at,
        use_status,
        available_performance,
        capacity,
        connections,
        memory_size,
        memory_excess_time,
        selection_size,
        avg_call_time,
        avg_db_call_time,
        avg_lock_call_time,
        avg_server_call_time,
        avg_threads,
        reserve,
        licenses,
    })
}

fn take_u14_len(cursor: &mut RecordCursor<'_>) -> Result<usize> {
    let b0 = cursor.take_u8()? as usize;
    let b1 = cursor.take_u8()? as usize;
    Ok((b0 & 0x3f) | (b1 << 6))
}

#[cfg(test)]
mod tests {
    use super::{parse_process_list_records, parse_process_record_1cv8c};
    use crate::commands::rpc_body;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        let s = input.trim();
        assert!(s.len() % 2 == 0, "hex length must be even");
        let mut out = Vec::with_capacity(s.len() / 2);
        let bytes = s.as_bytes();
        for i in (0..bytes.len()).step_by(2) {
            let hi = (bytes[i] as char).to_digit(16).expect("hex hi");
            let lo = (bytes[i + 1] as char).to_digit(16).expect("hex lo");
            out.push(((hi << 4) | lo) as u8);
        }
        out
    }

    #[test]
    fn process_list_response_decodes_fields() {
        let hex = include_str!("../../../../artifacts/process_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let records = parse_process_list_records(body).expect("process list");
        assert_eq!(records.len(), 1);
        let record = &records[0];
        assert_eq!(
            record.process,
            *b"\xf7\x7f,\x1d\x1e[HU\xa0\xb9\x949\x0c\xcdL\xe5"
        );
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.port, 1560);
        assert_eq!(record.pid, "314150");
        assert_eq!(record.turned_on, true);
        assert_eq!(record.running, true);
        assert_eq!(record.started_at, "2026-02-14T22:47:19");
        assert_eq!(record.use_status, 1);
        assert_eq!(record.available_performance, 153);
        assert_eq!(record.capacity, 1000);
        assert_eq!(record.connections, 7);
        assert_eq!(record.memory_size, 682224);
        assert_eq!(record.memory_excess_time, 0);
        assert_eq!(record.selection_size, 21944);
        assert!((record.avg_call_time - 4.115422347794385).abs() < 1e-9);
        assert!((record.avg_db_call_time - 0.0).abs() < 1e-12);
        assert!((record.avg_lock_call_time - 0.0001695679912504557).abs() < 1e-12);
        assert!((record.avg_server_call_time - 4.115252779803135).abs() < 1e-9);
        assert!((record.avg_threads - 1.0139867691851958).abs() < 1e-9);
        assert_eq!(record.reserve, false);
        assert_eq!(record.licenses.len(), 1);
        let license = &record.licenses[0];
        assert_eq!(
            license.file_name,
            "file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"
        );
        assert_eq!(license.key_series, "500000025347");
        assert_eq!(license.issued_by_server, true);
        assert_eq!(license.license_type, 0);
        assert_eq!(license.network_key, false);
        assert_eq!(license.max_users_all, 4);
        assert_eq!(license.max_users_current, 4);
        assert_eq!(license.server_address, "alko-home");
        assert_eq!(license.server_port, 1560);
        assert_eq!(license.process_id, "314150");
        assert!(license
            .full_presentation
            .contains("file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"));
    }

    #[test]
    fn process_info_response_decodes_fields() {
        let hex = include_str!("../../../../artifacts/process_info_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let record = parse_process_record_1cv8c(body).expect("process info");
        assert_eq!(
            record.process,
            *b"\xf7\x7f,\x1d\x1e[HU\xa0\xb9\x949\x0c\xcdL\xe5"
        );
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.port, 1560);
        assert_eq!(record.pid, "314150");
        assert_eq!(record.running, true);
        assert_eq!(record.started_at, "2026-02-14T22:47:19");
    }

    #[test]
    fn process_list_licenses_response_decodes_license_fields() {
        let hex = include_str!("../../../../artifacts/process_list_licenses_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let records = parse_process_list_records(body).expect("process list");
        assert_eq!(records.len(), 1);
        let record = &records[0];
        assert_eq!(
            record.process,
            *b"\xf7\x7f,\x1d\x1e[HU\xa0\xb9\x949\x0c\xcdL\xe5"
        );
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.port, 1560);
        assert_eq!(record.pid, "314150");
        assert_eq!(record.licenses.len(), 1);
        let license = &record.licenses[0];
        assert_eq!(
            license.file_name,
            "file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"
        );
        assert_eq!(license.key_series, "500000025347");
        assert_eq!(license.issued_by_server, true);
        assert_eq!(license.license_type, 0);
        assert_eq!(license.network_key, false);
        assert_eq!(license.max_users_all, 4);
        assert_eq!(license.max_users_current, 4);
        assert_eq!(license.server_address, "alko-home");
        assert_eq!(license.server_port, 1560);
        assert_eq!(license.process_id, "314150");
        assert!(license
            .brief_presentation
            .contains("500000025347 4 4"));
        assert!(license
            .full_presentation
            .contains("file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"));
    }

    #[test]
    fn process_info_licenses_response_decodes_license_fields() {
        let hex = include_str!("../../../../artifacts/process_info_licenses_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let record = parse_process_record_1cv8c(body).expect("process info licenses");
        assert_eq!(
            record.process,
            *b"\xf7\x7f,\x1d\x1e[HU\xa0\xb9\x949\x0c\xcdL\xe5"
        );
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.port, 1560);
        assert_eq!(record.pid, "314150");
        assert_eq!(record.licenses.len(), 1);
        let license = &record.licenses[0];
        assert_eq!(
            license.file_name,
            "file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"
        );
        assert_eq!(license.key_series, "500000025347");
        assert_eq!(license.issued_by_server, true);
        assert_eq!(license.license_type, 0);
        assert_eq!(license.network_key, false);
        assert_eq!(license.max_users_all, 4);
        assert_eq!(license.max_users_current, 4);
        assert_eq!(license.server_address, "alko-home");
        assert_eq!(license.server_port, 1560);
        assert_eq!(license.process_id, "314150");
        assert!(license
            .brief_presentation
            .contains("500000025347 4 4"));
        assert!(license
            .full_presentation
            .contains("file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"));
    }
}
