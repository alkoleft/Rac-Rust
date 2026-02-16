use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::error::{RacError, Result};
use crate::rac_wire::{scan_len_prefixed_strings, take_str8, uuid_from_slice};
use crate::Uuid16;

use super::record_cursor::{v8_datetime_to_iso, RecordCursor};
use super::{first_uuid, rpc_body};

#[derive(Debug, Serialize, Default, Clone)]
pub struct SessionCounters {
    pub blocked_by_dbms: Option<u32>,
    pub blocked_by_ls: Option<u32>,
    pub bytes_all: Option<u64>,
    pub bytes_last_5min: Option<u64>,
    pub calls_all: Option<u32>,
    pub calls_last_5min: Option<u64>,
    pub dbms_bytes_all: Option<u64>,
    pub dbms_bytes_last_5min: Option<u64>,
    pub db_proc_took: Option<u32>,
    pub duration_all: Option<u32>,
    pub duration_all_dbms: Option<u32>,
    pub duration_current: Option<u32>,
    pub duration_current_dbms: Option<u32>,
    pub duration_last_5min: Option<u64>,
    pub duration_last_5min_dbms: Option<u64>,
    pub passive_session_hibernate_time: Option<u32>,
    pub hibernate_session_terminate_time: Option<u32>,
    pub memory_current: Option<u64>,
    pub memory_last_5min: Option<u64>,
    pub memory_total: Option<u64>,
    pub read_current: Option<u64>,
    pub read_last_5min: Option<u64>,
    pub read_total: Option<u64>,
    pub write_current: Option<u64>,
    pub write_last_5min: Option<u64>,
    pub write_total: Option<u64>,
    pub duration_current_service: Option<u32>,
    pub duration_last_5min_service: Option<u64>,
    pub duration_all_service: Option<u32>,
    pub cpu_time_current: Option<u64>,
    pub cpu_time_last_5min: Option<u64>,
    pub cpu_time_total: Option<u64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SessionLicense {
    pub license_type: Option<u32>,
    pub server_address: Option<String>,
    pub process_id: Option<String>,
    pub file_name: Option<String>,
    pub brief_presentation: Option<String>,
    pub max_users_all: Option<u32>,
    pub max_users_current: Option<u32>,
    pub full_presentation: Option<String>,
    pub issued_by_server: Option<bool>,
    pub server_port: Option<u32>,
    pub software_license: Option<bool>,
    pub key_series: Option<String>,
    pub network_key: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SessionRecord {
    pub session: Uuid16,
    pub app_id: Option<String>,
    pub connection: Option<Uuid16>,
    pub process: Option<Uuid16>,
    pub infobase: Option<Uuid16>,
    pub host: Option<String>,
    pub hibernate: Option<bool>,
    pub locale: Option<String>,
    pub user_name: Option<String>,
    pub started_at: Option<String>,
    pub last_active_at: Option<String>,
    pub client_ip: Option<String>,
    pub retrieved_by_server: Option<bool>,
    pub software_license: Option<bool>,
    pub network_key: Option<bool>,
    pub license: Option<SessionLicense>,
    pub db_proc_info: Option<String>,
    pub db_proc_took_at: Option<String>,
    pub current_service_name: Option<String>,
    pub data_separation: Option<String>,
    pub session_id: Option<u32>,
    pub counters: SessionCounters,
}

#[derive(Debug, Serialize)]
pub struct SessionListResp {
    pub sessions: Vec<Uuid16>,
    pub records: Vec<SessionRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct SessionInfoResp {
    pub session: Uuid16,
    pub record: SessionRecord,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn session_list(client: &mut RacClient, cluster: Uuid16) -> Result<SessionListResp> {
    let reply = client.call(RacRequest::SessionList { cluster })?;
    let body = rpc_body(&reply)?;
    let records = parse_session_list_records(body)?;
    Ok(SessionListResp {
        sessions: records.iter().map(|r| r.session).collect(),
        records,
        raw_payload: Some(reply),
    })
}

pub fn session_info(
    client: &mut RacClient,
    cluster: Uuid16,
    session: Uuid16,
) -> Result<SessionInfoResp> {
    let reply = client.call(RacRequest::SessionInfo { cluster, session })?;
    let body = rpc_body(&reply)?;
    let record = match parse_session_record_for_info(body, session) {
        Ok(record) => record,
        Err(_) => SessionRecord {
            session: first_uuid(body)?,
            app_id: None,
            connection: None,
            process: None,
            infobase: None,
            host: None,
            hibernate: None,
            locale: None,
            user_name: None,
            started_at: None,
            last_active_at: None,
            client_ip: None,
            retrieved_by_server: None,
            software_license: None,
            network_key: None,
            license: None,
            db_proc_info: None,
            db_proc_took_at: None,
            current_service_name: None,
            data_separation: None,
            session_id: None,
            counters: SessionCounters::default(),
        },
    };
    Ok(SessionInfoResp {
        session: record.session,
        record,
        fields: scan_len_prefixed_strings(body)
            .into_iter()
            .map(|(_, s)| s)
            .collect(),
        raw_payload: Some(reply),
    })
}

fn parse_session_record_for_info(data: &[u8], requested_session: Uuid16) -> Result<SessionRecord> {
    let record = parse_session_record_1cv8c(data)?;
    if record.session != requested_session {
        return Err(RacError::Decode(
            "session info record does not match requested session",
        ));
    }
    Ok(record)
}

fn parse_session_list_records(body: &[u8]) -> Result<Vec<SessionRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }

    let expected = body[0] as usize;
    if expected == 0 {
        return Ok(Vec::new());
    }
    if body.len() < 1 + 16 {
        return Err(RacError::Decode("session list body truncated"));
    }

    let mut starts = Vec::with_capacity(expected);
    let mut cursor = 1usize;

    // Prefer the canonical first record offset, but fall back to scanning if needed.
    if parse_session_record_start(body, cursor).is_some() {
        starts.push(cursor);
        cursor = cursor.saturating_add(16 + 1);
    } else if let Some((off, _)) = find_next_session_record_start(body, cursor) {
        starts.push(off);
        cursor = off.saturating_add(16 + 1);
    } else {
        return Err(RacError::Decode("failed to locate first session record"));
    }

    while starts.len() < expected {
        match find_next_session_record_start(body, cursor) {
            Some((off, _)) => {
                starts.push(off);
                cursor = off.saturating_add(16 + 1);
            }
            None => break,
        }
    }

    if starts.len() != expected {
        return Err(RacError::Decode("failed to decode all session records"));
    }

    let mut records = Vec::with_capacity(starts.len());
    for (idx, start) in starts.iter().copied().enumerate() {
        let end = starts.get(idx + 1).copied().unwrap_or(body.len());
        if end <= start || end > body.len() {
            return Err(RacError::Decode("invalid session list record boundaries"));
        }
        records.push(parse_session_record_1cv8c(&body[start..end])?);
    }
    Ok(records)
}

fn find_next_session_record_start(data: &[u8], start: usize) -> Option<(usize, Uuid16)> {
    if data.len() < 1 + 16 + 2 || start >= data.len() {
        return None;
    }
    let last = data.len().saturating_sub(16 + 2);
    for off in start..=last {
        if let Some((_, uuid)) = parse_session_record_start(data, off) {
            return Some((off, uuid));
        }
    }
    None
}

fn parse_session_record_start(data: &[u8], offset: usize) -> Option<(String, Uuid16)> {
    // Session record header (docs/rac_message_formats_session.md):
    // - uuid[16] (session) at +0x00
    // - str8 app-id at +0x10 (len byte), bytes start at +0x11
    if offset + 16 + 1 > data.len() {
        return None;
    }
    let uuid = uuid_from_slice(&data[offset..offset + 16]).ok()?;
    if !is_probable_rfc4122_uuid(&uuid) {
        return None;
    }
    let app_off = offset + 16;
    let (app_id, _next) = take_str8(data, app_off).ok()?;
    if !is_reasonable_app_id(&app_id) {
        return None;
    }
    Some((app_id, uuid))
}

fn parse_session_record_1cv8c(data: &[u8]) -> Result<SessionRecord> {
    if data.len() < 16 {
        return Err(RacError::Decode("session record: truncated uuid"));
    }
    println!("Binary data: {:?}", data);
    let mut cursor = RecordCursor::new(data, 0);
    let session = cursor.take_uuid()?;

    let mut rec = SessionRecord {
        session,
        app_id: None,
        connection: None,
        process: None,
        infobase: None,
        host: None,
        hibernate: None,
        locale: None,
        user_name: None,
        started_at: None,
        last_active_at: None,
        client_ip: None,
        retrieved_by_server: None,
        software_license: None,
        network_key: None,
        license: None,
        db_proc_info: None,
        db_proc_took_at: None,
        current_service_name: None,
        data_separation: None,
        session_id: None,
        counters: SessionCounters::default(),
    };

    rec.app_id = Some(cursor.take_str8()?);
    rec.counters.blocked_by_dbms = cursor.take_u32_be_opt();
    rec.counters.blocked_by_ls = cursor.take_u32_be_opt();

    rec.counters.bytes_all = cursor.take_u64_be_opt();
    rec.counters.bytes_last_5min = cursor.take_u64_be_opt();

    rec.counters.calls_all = cursor.take_u32_be_opt();
    rec.counters.calls_last_5min = cursor.take_u64_be_opt();

    rec.connection = Some(cursor.take_uuid()?);

    rec.counters.dbms_bytes_all = cursor.take_u64_be_opt();
    rec.counters.dbms_bytes_last_5min = cursor.take_u64_be_opt();

    rec.db_proc_info = cursor.take_str8_opt();
    rec.counters.db_proc_took = cursor.take_u32_be_opt();
    rec.db_proc_took_at = cursor.take_datetime_opt();

    rec.counters.duration_all = cursor.take_u32_be_opt();
    rec.counters.duration_all_dbms = cursor.take_u32_be_opt();
    rec.counters.duration_current = cursor.take_u32_be_opt();
    rec.counters.duration_current_dbms = cursor.take_u32_be_opt();
    rec.counters.duration_last_5min = cursor.take_u64_be_opt();
    rec.counters.duration_last_5min_dbms = cursor.take_u64_be_opt();

    rec.host = cursor.take_str8_opt();
    rec.infobase = cursor.take_uuid_opt();

    rec.last_active_at = cursor.take_datetime_opt();

    rec.hibernate = cursor.take_bool_opt(); // TODO
    rec.counters.passive_session_hibernate_time = cursor.take_u32_be_opt();
    rec.counters.hibernate_session_terminate_time = cursor.take_u32_be_opt();

    rec.license = parse_licenses(&mut cursor);

    rec.locale = cursor.take_str8_opt();
    rec.process = cursor.take_uuid_opt();
    rec.session_id = cursor.take_u32_be_opt();
    rec.started_at = cursor.take_datetime_opt();
    rec.user_name = cursor.take_str8_opt();

    rec.counters.memory_current = cursor.take_u64_be_opt();
    rec.counters.memory_last_5min = cursor.take_u64_be_opt();
    rec.counters.memory_total = cursor.take_u64_be_opt();
    
    rec.counters.read_current = cursor.take_u64_be_opt();
    rec.counters.read_last_5min = cursor.take_u64_be_opt();
    rec.counters.read_total = cursor.take_u64_be_opt();

    rec.counters.write_current = cursor.take_u64_be_opt();
    rec.counters.write_last_5min = cursor.take_u64_be_opt();
    rec.counters.write_total = cursor.take_u64_be_opt();
    
    rec.counters.duration_current_service = cursor.take_u32_be_opt();
    rec.counters.duration_last_5min_service = cursor.take_u64_be_opt();
    rec.counters.duration_all_service = cursor.take_u32_be_opt();
    
    rec.current_service_name = cursor.take_str8_opt();

    rec.counters.cpu_time_current = cursor.take_u64_be_opt();
    rec.counters.cpu_time_last_5min = cursor.take_u64_be_opt();
    rec.counters.cpu_time_total = cursor.take_u64_be_opt();
    
    rec.data_separation = cursor.take_str8_opt();
    rec.client_ip = cursor.take_str8_opt();

    Ok(rec)
}

fn parse_licenses(cursor: &mut RecordCursor) -> Option<SessionLicense> {
    let count = cursor.take_u8();
    for _ in 0..count {
        println!("Licenses count: {}", count);

        let full_name = cursor.take_str8_opt();
        let full_presentation = cursor.take_str8_opt();

        let issued_by_server = cursor.take_bool_opt();
        let license_type = cursor.take_u32_be_opt();

        let max_users_all = cursor.take_u32_be_opt();
        let max_users_current = cursor.take_u32_be_opt();

        let network_key = cursor.take_bool();

        let server_address = cursor.take_str8_opt();
        let process_id = cursor.take_str8_opt();
        let server_port = cursor.take_u32_be_opt();

        let key_series = cursor.take_str8_opt();
        // let software_license = cursor.take_bool_opt();

        let brief_presentation = cursor.take_str8_opt();
        return Some(SessionLicense {
            license_type,
            server_address,
            process_id: process_id.filter(|s| !s.is_empty()),
            file_name: full_name.filter(|s| !s.is_empty()),
            brief_presentation,
            max_users_all,
            max_users_current,
            full_presentation,
            issued_by_server,
            server_port,
            software_license: Some(false),
            key_series,
            network_key: Some(network_key),
        });
    }
    None
}

fn is_probable_rfc4122_uuid(uuid: &Uuid16) -> bool {
    if uuid.iter().all(|&b| b == 0) {
        return false;
    }
    let version = uuid[6] >> 4;
    if !(1..=5).contains(&version) {
        return false;
    }
    (uuid[8] & 0b1100_0000) == 0b1000_0000
}

fn is_reasonable_app_id(value: &str) -> bool {
    if value.is_empty() || value.len() > 64 {
        return false;
    }
    value
        .bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-' | b'.'))
}

fn is_locale(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() == 2 {
        return bytes[0].is_ascii_lowercase() && bytes[1].is_ascii_lowercase();
    }
    if bytes.len() == 5 && bytes[2] == b'_' {
        return bytes[0].is_ascii_lowercase()
            && bytes[1].is_ascii_lowercase()
            && bytes[3].is_ascii_uppercase()
            && bytes[4].is_ascii_uppercase();
    }
    false
}

fn is_ipv4(value: &str) -> bool {
    let mut count = 0usize;
    for part in value.split('.') {
        count += 1;
        if part.is_empty() || part.len() > 3 || !part.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        if part.parse::<u8>().is_err() {
            return false;
        }
    }
    count == 4
}

fn looks_like_path(value: &str) -> bool {
    value.contains('/') || value.contains('\\') || value.contains("://")
}

fn looks_like_host(value: &str) -> bool {
    if value.len() < 2 || value.len() > 128 {
        return false;
    }
    if is_locale(value) || is_ipv4(value) || looks_like_path(value) {
        return false;
    }
    value
        .bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.'))
}

fn looks_like_user_name(value: &str) -> bool {
    if value.len() < 3 || value.len() > 128 {
        return false;
    }
    if !value.chars().any(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    value
        .bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.'))
}

fn scan_raw_uuids(data: &[u8]) -> Vec<Uuid16> {
    if data.len() < 16 {
        return Vec::new();
    }
    let mut out = Vec::new();
    for off in 0..=data.len() - 16 {
        if let Ok(uuid) = uuid_from_slice(&data[off..off + 16]) {
            out.push(uuid);
        }
    }
    out
}

fn find_process_after_locale(data: &[u8], locale: &str, rec: &SessionRecord) -> Option<Uuid16> {
    let locale_bytes = locale.as_bytes();
    let needed = locale_bytes.len() + 1 + 16;
    if data.len() < needed {
        return None;
    }
    for off in 0..=data.len() - needed {
        if data[off] as usize != locale_bytes.len() {
            continue;
        }
        if &data[off + 1..off + 1 + locale_bytes.len()] != locale_bytes {
            continue;
        }
        let uuid_off = off + 1 + locale_bytes.len();
        if uuid_off + 16 + 12 > data.len() {
            continue;
        }
        if let Ok(uuid) = uuid_from_slice(&data[uuid_off..uuid_off + 16]) {
            if uuid == rec.session
                || rec.connection.is_some_and(|v| v == uuid)
                || rec.infobase.is_some_and(|v| v == uuid)
            {
                continue;
            }
            let sid_off = uuid_off + 16;
            let session_id = u32::from_be_bytes([
                data[sid_off],
                data[sid_off + 1],
                data[sid_off + 2],
                data[sid_off + 3],
            ]);
            if session_id == 0 {
                continue;
            }
            let started_raw = u64::from_be_bytes([
                data[sid_off + 4],
                data[sid_off + 5],
                data[sid_off + 6],
                data[sid_off + 7],
                data[sid_off + 8],
                data[sid_off + 9],
                data[sid_off + 10],
                data[sid_off + 11],
            ]);
            if v8_datetime_to_iso(started_raw).is_none() {
                continue;
            }
            if is_probable_rfc4122_uuid(&uuid) {
                return Some(uuid);
            }
        }
    }
    None
}

fn find_session_triplet(data: &[u8]) -> Option<(u32, String, String)> {
    if data.len() < 13 {
        return None;
    }
    for off in 0..=data.len() - 13 {
        let session_id =
            u32::from_be_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]);
        if session_id == 0 {
            continue;
        }
        let started_raw = u64::from_be_bytes([
            data[off + 4],
            data[off + 5],
            data[off + 6],
            data[off + 7],
            data[off + 8],
            data[off + 9],
            data[off + 10],
            data[off + 11],
        ]);
        let started_at = match v8_datetime_to_iso(started_raw) {
            Some(value) => value,
            None => continue,
        };
        let (user_name, _) = match take_str8(data, off + 12) {
            Ok(value) => value,
            Err(_) => continue,
        };
        if user_name.len() >= 4
            && looks_like_user_name(&user_name)
            && !is_reasonable_app_id(&user_name)
            && !is_locale(&user_name)
            && !looks_like_host(&user_name)
            && !is_ipv4(&user_name)
        {
            return Some((session_id, started_at, user_name));
        }
    }
    None
}

fn apply_user_anchor_backfill(data: &[u8], rec: &mut SessionRecord) {
    for (off, user_name) in scan_len_prefixed_strings(data) {
        if !looks_like_user_name(&user_name)
            || is_reasonable_app_id(&user_name)
            || is_locale(&user_name)
            || looks_like_host(&user_name)
            || is_ipv4(&user_name)
        {
            continue;
        }
        if off < 12 || off + 1 + user_name.len() > data.len() {
            continue;
        }
        let sid_off = off - 12;
        let session_id = u32::from_be_bytes([
            data[sid_off],
            data[sid_off + 1],
            data[sid_off + 2],
            data[sid_off + 3],
        ]);
        if session_id == 0 {
            continue;
        }
        let started_raw = u64::from_be_bytes([
            data[sid_off + 4],
            data[sid_off + 5],
            data[sid_off + 6],
            data[sid_off + 7],
            data[sid_off + 8],
            data[sid_off + 9],
            data[sid_off + 10],
            data[sid_off + 11],
        ]);
        let started_at = match v8_datetime_to_iso(started_raw) {
            Some(value) => value,
            None => continue,
        };

        rec.session_id = Some(session_id);
        rec.started_at = Some(started_at);
        rec.user_name = Some(user_name);

        if sid_off >= 16 {
            let process_off = sid_off - 16;
            if let Ok(process) = uuid_from_slice(&data[process_off..process_off + 16]) {
                if is_probable_rfc4122_uuid(&process)
                    && process != rec.session
                    && rec.connection.is_none_or(|v| v != process)
                    && rec.infobase.is_none_or(|v| v != process)
                {
                    rec.process = Some(process);
                }
            }
        }
        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn parse_session_list_sessions_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/session_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let records = parse_session_list_records(body).expect("session list parse");
        let sessions: Vec<_> = records.iter().map(|r| r.session).collect();

        assert_eq!(sessions.len(), 3);
        assert_eq!(
            sessions[0],
            crate::rac_wire::parse_uuid("25510e27-f24a-4586-9ac9-9f7837c0dea1").unwrap()
        );
        assert_eq!(
            sessions[1],
            crate::rac_wire::parse_uuid("56bde8c0-d008-4d33-a6b9-8db9b6f82de5").unwrap()
        );
        assert_eq!(
            sessions[2],
            crate::rac_wire::parse_uuid("eb61231d-7bee-4a06-8869-41f70e2289de").unwrap()
        );

        assert_eq!(records[0].app_id.as_deref(), Some("1CV8C"));
        assert_eq!(records[1].app_id.as_deref(), Some("Designer"));
        assert_eq!(records[2].app_id.as_deref(), Some("SystemBackgroundJob"));
        assert_eq!(records[0].client_ip.as_deref(), Some("127.0.0.1"));
        assert_eq!(records[1].locale.as_deref(), Some("ru_RU"));
        assert_eq!(records[2].session_id, Some(5));
        assert_eq!(records[2].counters.dbms_bytes_all, Some(3088));
    }

    #[test]
    fn parse_session_info_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/session_info_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let record = parse_session_record_1cv8c(body).expect("session info parse");

        assert_eq!(
            record.session,
            crate::rac_wire::parse_uuid("56bde8c0-d008-4d33-a6b9-8db9b6f82de5").unwrap()
        );
        assert_eq!(record.app_id.as_deref(), Some("Designer"));
        assert_eq!(record.host.as_deref(), Some("alko-home"));
        assert_eq!(record.user_name.as_deref(), Some("DefUser"));
        assert_eq!(record.client_ip.as_deref(), Some("127.0.0.1"));
        assert_eq!(record.session_id, Some(1));
        assert_eq!(record.counters.bytes_all, Some(253146));
        assert_eq!(record.counters.dbms_bytes_all, Some(654414));
        assert_eq!(record.counters.cpu_time_total, Some(1357));
    }

    #[test]
    fn parse_session_info_1cv8c_from_capture() {
        let hex = include_str!("../../../../artifacts/session_info_response_1cv8c.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let requested =
            crate::rac_wire::parse_uuid("25510e27-f24a-4586-9ac9-9f7837c0dea1").expect("uuid");
        let record =
            parse_session_record_for_info(body, requested).expect("session info 1cv8c parse");

        assert_eq!(
            record.session,
            crate::rac_wire::parse_uuid("25510e27-f24a-4586-9ac9-9f7837c0dea1").unwrap()
        );
        assert_eq!(record.app_id.as_deref(), Some("1CV8C"));
        assert_eq!(record.host.as_deref(), Some("alko-home"));
        assert_eq!(record.locale.as_deref(), Some("ru"));
        assert_eq!(record.user_name.as_deref(), Some("DefUser"));
        assert_eq!(record.started_at.as_deref(), Some("2026-02-15T00:10:57"));
        assert_eq!(record.client_ip.as_deref(), Some("127.0.0.1"));
        assert_eq!(record.software_license, Some(true));
        assert_eq!(record.network_key, Some(false));
        assert_eq!(record.retrieved_by_server, Some(false));
        assert_eq!(record.session_id, Some(3));
        assert_eq!(record.counters.bytes_all, Some(7807077));
        assert_eq!(record.counters.dbms_bytes_all, Some(10914466));
        let lic = record.license.as_ref().expect("license");
        assert_eq!(
            lic.file_name.as_deref(),
            Some("file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic")
        );
        assert_eq!(lic.key_series.as_deref(), Some("500000025347"));
        assert_eq!(lic.max_users_all, Some(4));
        assert_eq!(lic.max_users_current, Some(4));
    }

    #[test]
    fn parse_session_info_1cv8c_dbproc_from_capture() {
        let hex = include_str!("../../../../artifacts/session_info_response_1cv8c_dbproc.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let requested =
            crate::rac_wire::parse_uuid("25510e27-f24a-4586-9ac9-9f7837c0dea1").expect("uuid");
        let record = parse_session_record_for_info(body, requested)
            .expect("session info 1cv8c dbproc parse");

        assert_eq!(
            record.connection,
            Some(crate::rac_wire::parse_uuid("e41e750e-56d7-40fb-b2e0-5e71b8e8f508").unwrap())
        );
        assert_eq!(
            record.process,
            Some(crate::rac_wire::parse_uuid("f77f2c1d-1e5b-4855-a0b9-94390ccd4ce5").unwrap())
        );
        assert_eq!(record.db_proc_info.as_deref(), Some("5719"));
        assert_eq!(record.counters.blocked_by_ls, Some(6));
        assert_eq!(record.counters.db_proc_took, Some(18982));
        assert_eq!(record.counters.duration_current, Some(20172));
        assert_eq!(record.counters.memory_current, Some(-47080));
        assert_eq!(record.counters.read_current, Some(16176));
        assert_eq!(record.counters.duration_current_service, Some(0));
        assert_eq!(record.counters.cpu_time_current, Some(1051));
        assert_eq!(record.software_license, Some(true));
        assert_eq!(record.network_key, Some(false));
        assert_eq!(record.retrieved_by_server, Some(false));
        assert_eq!(
            record.last_active_at.as_deref(),
            Some("2026-02-16T00:28:41")
        );
        assert_eq!(record.data_separation.as_deref(), Some("''"));
        assert_eq!(record.client_ip.as_deref(), Some("127.0.0.1"));
        let lic = record.license.as_ref().expect("license");
        assert_eq!(lic.process_id.as_deref(), Some("381094"));
        assert_eq!(
            lic.brief_presentation.as_deref(),
            Some("Клиент, 500000025347 4 4")
        );
    }
}
