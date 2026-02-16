use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::error::{RacError, Result};
use crate::rac_wire::{
    scan_len_prefixed_strings, take_str8, take_u32_be, take_u64_be, take_uuid16, uuid_from_slice
};
use crate::Uuid16;

use super::{first_uuid, rpc_body};

#[derive(Debug, Serialize, Default, Clone)]
pub struct SessionCounters {
    pub blocked_by_dbms: Option<u32>,
    pub blocked_by_ls: Option<u64>,
    pub bytes_all: Option<u64>,
    pub bytes_last_5min: Option<u32>,
    pub calls_all: Option<u64>,
    pub calls_last_5min: Option<u32>,
    pub dbms_bytes_all: Option<u32>,
    pub dbms_bytes_last_5min: Option<u32>,
    pub db_proc_took: Option<u32>,
    pub duration_all: Option<u32>,
    pub duration_all_dbms: Option<u32>,
    pub duration_current: Option<u32>,
    pub duration_current_dbms: Option<u32>,
    pub duration_last_5min: Option<u32>,
    pub duration_last_5min_dbms: Option<u32>,
    pub passive_session_hibernate_time: Option<u32>,
    pub hibernate_session_terminate_time: Option<u32>,
    pub memory_current: Option<i32>,
    pub memory_last_5min: Option<u32>,
    pub memory_total: Option<u32>,
    pub read_current: Option<u32>,
    pub read_last_5min: Option<u32>,
    pub read_total: Option<u32>,
    pub write_current: Option<u32>,
    pub write_last_5min: Option<u32>,
    pub write_total: Option<u32>,
    pub duration_current_service: Option<u32>,
    pub duration_last_5min_service: Option<u32>,
    pub duration_all_service: Option<u32>,
    pub cpu_time_current: Option<u32>,
    pub cpu_time_last_5min: Option<u32>,
    pub cpu_time_total: Option<u32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SessionLicense {
    pub server_address: Option<String>,
    pub process_id: Option<String>,
    pub file_name: Option<String>,
    pub brief_presentation: Option<String>,
    pub max_users: Option<u32>,
    pub max_software_license_users: Option<u32>,
    pub detailed_presentation: Option<String>,
    pub retrieved_by_server: Option<bool>,
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
    parse_session_record_legacy_1cv8c(data)
}

fn parse_session_record_legacy_1cv8c(data: &[u8]) -> Result<SessionRecord> {
    let rec = enrich_legacy_1cv8c_fields(data)?;
    Ok(rec)
}

fn enrich_legacy_1cv8c_fields(data: &[u8]) -> Result<SessionRecord> {
    println!("Data: {:?}", data);
    let mut cursor = RecordCursor::new(data, 0);
    let session = cursor.take_uuid()?;

    let mut rec = SessionRecord {
        session,
        app_id: None,
        connection: None,
        process: None,
        infobase: None,
        host: None,
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
        current_service_name: None,
        data_separation: None,
        session_id: None,
        counters: SessionCounters::default(),
    };

    rec.app_id = Some(cursor.take_str8()?);
    rec.counters.blocked_by_dbms = cursor.take_u32_be_opt();
    rec.counters.blocked_by_ls = cursor.take_u64_be_opt();
    rec.counters.bytes_all = cursor.take_u64_be_opt();
    rec.counters.bytes_last_5min = cursor.take_u32_be_opt();
    rec.counters.calls_all = cursor.take_u64_be_opt();
    rec.counters.calls_last_5min = cursor.take_u32_be_opt();
    rec.connection = Some(cursor.take_uuid()?);
    rec.counters.dbms_bytes_all = cursor_seq_u32(&mut cursor, 0x4e);
    rec.counters.dbms_bytes_last_5min = cursor_seq_u32(&mut cursor, 0x56);

    let mut probe = RecordCursor::new(data, 0);
    let db_proc_info_len = cursor_seq_u8(&mut probe, 0x5a).unwrap_or(0) as usize;
    let shift = if db_proc_info_len > 0 && db_proc_info_len <= 16 {
        db_proc_info_len
    } else {
        0
    };
    rec.db_proc_info = cursor_seq_str8(&mut cursor, 0x5a).filter(|s| !s.is_empty());
    rec.counters.db_proc_took = cursor_seq_u32(&mut cursor, 0x5f);

    rec.counters.duration_all = cursor_seq_u32(&mut cursor, 0x67 + shift);
    rec.counters.duration_all_dbms = cursor_seq_u32(&mut cursor, 0x6b + shift);
    rec.counters.duration_current = cursor_seq_u32(&mut cursor, 0x6f + shift);
    rec.counters.duration_current_dbms = cursor_seq_u32(&mut cursor, 0x73 + shift);
    rec.counters.duration_last_5min = cursor_seq_u32(&mut cursor, 0x7b + shift);
    rec.counters.duration_last_5min_dbms = cursor_seq_u32(&mut cursor, 0x83 + shift);
    rec.host = cursor_seq_str8(&mut cursor, 0x87 + shift);
    rec.infobase = cursor_seq_uuid(&mut cursor, 0x95);
    rec.last_active_at = cursor_seq_u64(&mut cursor, 0xa1 + shift).and_then(v8_datetime_to_iso);
    rec.counters.passive_session_hibernate_time = cursor_seq_u32(&mut cursor, 0xaa + shift);
    rec.counters.hibernate_session_terminate_time = cursor_seq_u32(&mut cursor, 0xae + shift);
    rec.software_license = cursor_seq_u8(&mut cursor, 0xb2 + shift).map(|v| v != 0);

    let file_name = cursor_seq_str8(&mut cursor, 0xb3 + shift);
    let detailed_presentation = cursor_seq_str8(&mut cursor, 0xec + shift);
    rec.network_key = cursor_seq_u8(&mut cursor, 0x168 + shift).map(|v| v != 0);
    rec.retrieved_by_server = cursor_seq_u8(&mut cursor, 0x169 + shift).map(|v| v != 0);
    let max_users = cursor_seq_u16_le(&mut cursor, 0x16e + shift).map(u32::from);
    let max_software_license_users = cursor_seq_u16_le(&mut cursor, 0x172 + shift).map(u32::from);
    let process_id = cursor_seq_str8(&mut cursor, 0x175 + shift);
    let key_series = cursor_seq_str8(&mut cursor, 0x180 + shift);
    let brief_presentation = cursor_seq_str8(&mut cursor, 0x18d + shift);

    rec.locale = cursor_seq_str8(&mut cursor, 0x1ac + shift);
    rec.process =
        cursor_seq_uuid(&mut cursor, 0x1af + shift).filter(|uuid| !uuid.iter().all(|&b| b == 0));
    rec.session_id = rec
        .session_id
        .or_else(|| cursor_seq_u32(&mut cursor, 0x1bf + shift));
    rec.started_at = cursor_seq_u64(&mut cursor, 0x1c3 + shift).and_then(v8_datetime_to_iso);
    rec.user_name = cursor_seq_str8(&mut cursor, 0x1cb + shift);
    rec.counters.memory_current = cursor_seq_i32(&mut cursor, 0x1d7 + shift);
    rec.counters.memory_last_5min = cursor_seq_u32(&mut cursor, 0x1df + shift);
    rec.counters.memory_total = cursor_seq_u32(&mut cursor, 0x1e7 + shift);
    rec.counters.read_current = cursor_seq_u32(&mut cursor, 0x1ef + shift);
    rec.counters.read_last_5min = cursor_seq_u32(&mut cursor, 0x1f7 + shift);
    rec.counters.read_total = cursor_seq_u32(&mut cursor, 0x1ff + shift);
    rec.counters.write_current = cursor_seq_u32(&mut cursor, 0x207 + shift);
    rec.counters.write_last_5min = cursor_seq_u32(&mut cursor, 0x20f + shift);
    rec.counters.write_total = cursor_seq_u32(&mut cursor, 0x217 + shift);
    rec.counters.duration_current_service = cursor_seq_u32(&mut cursor, 0x21f + shift);
    rec.counters.duration_last_5min_service = cursor_seq_u32(&mut cursor, 0x223 + shift);
    rec.counters.duration_all_service = cursor_seq_u32(&mut cursor, 0x227 + shift);
    rec.counters.cpu_time_current = cursor_seq_u32(&mut cursor, 0x230 + shift);
    rec.counters.cpu_time_last_5min = cursor_seq_u32(&mut cursor, 0x238 + shift);
    rec.counters.cpu_time_total = cursor_seq_u32(&mut cursor, 0x240 + shift);
    rec.data_separation = cursor_seq_str8(&mut cursor, 0x244 + shift);
    rec.client_ip = cursor_seq_str8(&mut cursor, 0x247 + shift);

    let has_any_license = file_name.as_ref().is_some_and(|s| !s.is_empty())
        || detailed_presentation
            .as_ref()
            .is_some_and(|s| !s.is_empty())
        || brief_presentation.as_ref().is_some_and(|s| !s.is_empty())
        || process_id.as_ref().is_some_and(|s| !s.is_empty())
        || key_series.as_ref().is_some_and(|s| !s.is_empty())
        || max_users.is_some();
    rec.license = has_any_license.then_some(SessionLicense {
        server_address: None,
        process_id: process_id.filter(|s| !s.is_empty()),
        file_name: file_name.filter(|s| !s.is_empty()),
        brief_presentation: brief_presentation.filter(|s| !s.is_empty()),
        max_users,
        max_software_license_users,
        detailed_presentation: detailed_presentation.filter(|s| !s.is_empty()),
        retrieved_by_server: rec.retrieved_by_server,
        server_port: None,
        software_license: rec.software_license,
        key_series: key_series.filter(|s| !s.is_empty()),
        network_key: rec.network_key,
    });
    Ok(rec)
}

fn cursor_seq_seek(cursor: &mut RecordCursor<'_>, target: usize) -> Option<()> {
    if target < cursor.off || target > cursor.data.len() {
        return None;
    }
    cursor.skip(target - cursor.off).ok()
}

fn cursor_seq_u8(cursor: &mut RecordCursor<'_>, target: usize) -> Option<u8> {
    cursor_seq_seek(cursor, target)?;
    cursor.data.get(cursor.off).copied().inspect(|_| {
        cursor.off += 1;
    })
}

fn cursor_seq_u16_le(cursor: &mut RecordCursor<'_>, target: usize) -> Option<u16> {
    cursor_seq_seek(cursor, target)?;
    if cursor.off + 2 > cursor.data.len() {
        return None;
    }
    let mut raw = [0u8; 2];
    raw.copy_from_slice(&cursor.data[cursor.off..cursor.off + 2]);
    cursor.off += 2;
    Some(u16::from_le_bytes(raw))
}

fn cursor_seq_u32(cursor: &mut RecordCursor<'_>, target: usize) -> Option<u32> {
    cursor_seq_seek(cursor, target)?;
    cursor.take_u32_be().ok()
}

fn cursor_seq_uuid(cursor: &mut RecordCursor<'_>, target: usize) -> Option<Uuid16> {
    cursor_seq_seek(cursor, target)?;
    cursor.take_uuid().ok()
}

fn cursor_seq_i32(cursor: &mut RecordCursor<'_>, target: usize) -> Option<i32> {
    cursor_seq_seek(cursor, target)?;
    if cursor.off + 4 > cursor.data.len() {
        return None;
    }
    let mut raw = [0u8; 4];
    raw.copy_from_slice(&cursor.data[cursor.off..cursor.off + 4]);
    cursor.off += 4;
    Some(i32::from_be_bytes(raw))
}

fn cursor_seq_u64(cursor: &mut RecordCursor<'_>, target: usize) -> Option<u64> {
    cursor_seq_seek(cursor, target)?;
    if cursor.off + 8 > cursor.data.len() {
        return None;
    }
    let mut raw = [0u8; 8];
    raw.copy_from_slice(&cursor.data[cursor.off..cursor.off + 8]);
    cursor.off += 8;
    Some(u64::from_be_bytes(raw))
}

fn cursor_seq_str8(cursor: &mut RecordCursor<'_>, target: usize) -> Option<String> {
    cursor_seq_seek(cursor, target)?;
    cursor.take_str8().ok()
}

fn parse_session_license_block(data: &[u8], shift: usize) -> Option<SessionLicense> {
    let file_name = read_str8_at(data, 0xb3 + shift);
    let detailed_presentation = read_str8_at(data, 0xec + shift);
    let process_id = read_str8_at(data, 0x175 + shift);
    let key_series = read_str8_at(data, 0x180 + shift);
    let brief_presentation = read_str8_at(data, 0x18d + shift);
    let server_address: Option<String> = None;
    let max_users = read_u16_le(data, 0x16e + shift).map(u32::from);
    let max_software_license_users = read_u16_le(data, 0x172 + shift).map(u32::from);
    let server_port = None;
    let software_license = read_u8(data, 0xb2 + shift).map(|v| v != 0);
    let network_key = read_u8(data, 0x168 + shift).map(|v| v != 0);
    let retrieved_by_server = read_u8(data, 0x169 + shift).map(|v| v != 0);

    let has_any = file_name.as_ref().is_some_and(|s| !s.is_empty())
        || detailed_presentation
            .as_ref()
            .is_some_and(|s| !s.is_empty())
        || brief_presentation.as_ref().is_some_and(|s| !s.is_empty())
        || process_id.as_ref().is_some_and(|s| !s.is_empty())
        || key_series.as_ref().is_some_and(|s| !s.is_empty())
        || max_users.is_some();
    if !has_any {
        return None;
    }

    Some(SessionLicense {
        server_address: server_address.filter(|s| !s.is_empty()),
        process_id: process_id.filter(|s| !s.is_empty()),
        file_name: file_name.filter(|s| !s.is_empty()),
        brief_presentation: brief_presentation.filter(|s| !s.is_empty()),
        max_users,
        max_software_license_users,
        detailed_presentation: detailed_presentation.filter(|s| !s.is_empty()),
        retrieved_by_server,
        server_port,
        software_license,
        key_series: key_series.filter(|s| !s.is_empty()),
        network_key,
    })
}

struct RecordCursor<'a> {
    data: &'a [u8],
    off: usize,
}

impl<'a> RecordCursor<'a> {
    fn new(data: &'a [u8], off: usize) -> Self {
        Self { data, off }
    }

    fn skip(&mut self, n: usize) -> Result<()> {
        let next = self
            .off
            .checked_add(n)
            .ok_or(RacError::Decode("session record: cursor overflow"))?;
        if next > self.data.len() {
            return Err(RacError::Decode("session record: truncated while skipping"));
        }
        self.off = next;
        Ok(())
    }

    fn take_uuid(&mut self) -> Result<Uuid16> {
        let (uuid, next) = take_uuid16(self.data, self.off)
            .map_err(|_| RacError::Decode("session record: truncated uuid field"))?;
        self.off = next;
        Ok(uuid)
    }

    fn take_str8(&mut self) -> Result<String> {
        let (value, next) = take_str8(self.data, self.off)
            .map_err(|_| RacError::Decode("session record: truncated str8 field"))?;
        self.off = next;
        Ok(value)
    }

    fn take_u32_be(&mut self) -> Result<u32> {
        let (value, next) = take_u32_be(self.data, self.off)
            .map_err(|_| RacError::Decode("session record: truncated u32 field"))?;
        self.off = next;
        Ok(value)
    }
    fn take_u32_be_opt(&mut self) -> Option<u32> {
        let (value, next) = take_u32_be(self.data, self.off)
            .map_err(|_| RacError::Decode("session record: truncated u32 field")).ok()?;
        self.off = next;
        Some(value)
    }
    fn take_u64_be_opt(&mut self) -> Option<u64> {
        let (value, next) = take_u64_be(self.data, self.off)
            .map_err(|_| RacError::Decode("session record: truncated u32 field")).ok()?;
        self.off = next;
        Some(value)
    }
}

fn read_u32(data: &[u8], off: usize) -> Option<u32> {
    if off + 4 > data.len() {
        return None;
    }
    let mut raw = [0u8; 4];
    raw.copy_from_slice(&data[off..off + 4]);
    Some(u32::from_be_bytes(raw))
}

fn read_i32(data: &[u8], off: usize) -> Option<i32> {
    if off + 4 > data.len() {
        return None;
    }
    let mut raw = [0u8; 4];
    raw.copy_from_slice(&data[off..off + 4]);
    Some(i32::from_be_bytes(raw))
}

fn read_u8(data: &[u8], off: usize) -> Option<u8> {
    data.get(off).copied()
}

fn read_u64_be(data: &[u8], off: usize) -> Option<u64> {
    if off + 8 > data.len() {
        return None;
    }
    let mut raw = [0u8; 8];
    raw.copy_from_slice(&data[off..off + 8]);
    Some(u64::from_be_bytes(raw))
}

fn read_u16_le(data: &[u8], off: usize) -> Option<u16> {
    if off + 2 > data.len() {
        return None;
    }
    let mut raw = [0u8; 2];
    raw.copy_from_slice(&data[off..off + 2]);
    Some(u16::from_le_bytes(raw))
}

fn v8_datetime_to_iso(raw: u64) -> Option<String> {
    // 1C timestamp observed in captures: 1 unit = 1/10000 second,
    // epoch offset equals Unix epoch at 621355968000000.
    const UNIX_EPOCH_OFFSET: i128 = 621_355_968_000_000;
    let raw_i = i128::from(raw);
    if raw_i < UNIX_EPOCH_OFFSET {
        return None;
    }
    let unix_secs = (raw_i - UNIX_EPOCH_OFFSET) / 10_000;
    let unix_secs = i64::try_from(unix_secs).ok()?;

    let days = unix_secs.div_euclid(86_400);
    let sod = unix_secs.rem_euclid(86_400);
    let hour = sod / 3_600;
    let minute = (sod % 3_600) / 60;
    let second = sod % 60;

    let (year, month, day) = civil_from_days(days);
    Some(format!(
        "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}"
    ))
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i64, i64, i64) {
    // Howard Hinnant, civil_from_days
    let z = i128::from(days_since_unix_epoch) + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (
        i64::try_from(year).unwrap_or(0),
        i64::try_from(m).unwrap_or(0),
        i64::try_from(d).unwrap_or(0),
    )
}

fn read_uuid_at(data: &[u8], off: usize) -> Option<Uuid16> {
    if off + 16 > data.len() {
        return None;
    }
    uuid_from_slice(&data[off..off + 16]).ok()
}

fn read_str8_at(data: &[u8], off: usize) -> Option<String> {
    take_str8(data, off).ok().map(|(value, _)| value)
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
    let mut parts = value.split('.');
    let mut count = 0usize;
    while let Some(part) = parts.next() {
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
    if value.chars().all(|ch| ch.is_ascii_digit()) {
        return false;
    }
    value
        .bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.'))
}

fn looks_like_user_name(value: &str) -> bool {
    if value.is_empty() || value.len() > 128 {
        return false;
    }
    if !value.chars().any(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    value
        .bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.'))
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
        assert_eq!(lic.max_users, Some(4));
        assert_eq!(lic.max_software_license_users, Some(4));
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
