use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

mod generated {
    include!("session_generated.rs");
}

use generated::{SessionLicenseRecord, SessionRecordRaw};

#[derive(Debug, Serialize, Default, Clone)]
pub struct SessionCounters {
    pub blocked_by_dbms: u32,
    pub blocked_by_ls: u32,
    pub bytes_all: u64,
    pub bytes_last_5min: u64,
    pub calls_all: u32,
    pub calls_last_5min: u64,
    pub dbms_bytes_all: u64,
    pub dbms_bytes_last_5min: u64,
    pub db_proc_took: u32,
    pub duration_all: u32,
    pub duration_all_dbms: u32,
    pub duration_current: u32,
    pub duration_current_dbms: u32,
    pub duration_last_5min: u64,
    pub duration_last_5min_dbms: u64,
    pub passive_session_hibernate_time: u32,
    pub hibernate_session_terminate_time: u32,
    pub memory_current: u64,
    pub memory_last_5min: u64,
    pub memory_total: u64,
    pub read_current: u64,
    pub read_last_5min: u64,
    pub read_total: u64,
    pub write_current: u64,
    pub write_last_5min: u64,
    pub write_total: u64,
    pub duration_current_service: u32,
    pub duration_last_5min_service: u64,
    pub duration_all_service: u32,
    pub cpu_time_current: u64,
    pub cpu_time_last_5min: u64,
    pub cpu_time_total: u64,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct SessionLicense {
    pub license_type: u32,
    pub server_address: String,
    pub process_id: String,
    pub file_name: String,
    pub brief_presentation: String,
    pub max_users_all: u32,
    pub max_users_current: u32,
    pub full_presentation: String,
    pub issued_by_server: bool,
    pub server_port: u32,
    pub software_license: bool,
    pub key_series: String,
    pub network_key: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct SessionRecord {
    pub session: Uuid16,
    pub app_id: String,
    pub connection: Uuid16,
    pub process: Uuid16,
    pub infobase: Uuid16,
    pub host: String,
    pub hibernate: bool,
    pub locale: String,
    pub user_name: String,
    pub started_at: String,
    pub last_active_at: String,
    pub client_ip: String,
    pub retrieved_by_server: bool,
    pub software_license: bool,
    pub network_key: bool,
    pub license: SessionLicense,
    pub db_proc_info: String,
    pub db_proc_took_at: String,
    pub current_service_name: String,
    pub data_separation: String,
    pub session_id: u32,
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
        Err(_) => fallback_session_record(body)?,
    };
    let fields = collect_session_fields(&record);
    Ok(SessionInfoResp {
        session: record.session,
        record,
        fields,
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

    let mut cursor = RecordCursor::new(body, 0);
    let expected = cursor.take_u8()? as usize;
    if expected == 0 {
        return Ok(Vec::new());
    }

    let mut records = Vec::with_capacity(expected);
    for _ in 0..expected {
        records.push(parse_session_record(&mut cursor)?);
    }
    Ok(records)
}

fn parse_session_record_1cv8c(data: &[u8]) -> Result<SessionRecord> {
    let mut cursor = RecordCursor::new(data, 0);
    parse_session_record(&mut cursor)
}

fn parse_session_record(cursor: &mut RecordCursor<'_>) -> Result<SessionRecord> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("session record: truncated uuid"));
    }
    #[cfg(feature = "debug-parse")]
    log::debug!("Binary data: {:?}", cursor.remaining_len());
    let raw = SessionRecordRaw::decode(cursor)?;
    Ok(map_session_record(raw))
}

fn fallback_session_record(body: &[u8]) -> Result<SessionRecord> {
    let mut cursor = RecordCursor::new(body, 0);
    let session = cursor.take_uuid()?;
    Ok(SessionRecord {
        session,
        app_id: String::new(),
        connection: Uuid16::default(),
        process: Uuid16::default(),
        infobase: Uuid16::default(),
        host: String::new(),
        hibernate: false,
        locale: String::new(),
        user_name: String::new(),
        started_at: String::new(),
        last_active_at: String::new(),
        client_ip: String::new(),
        retrieved_by_server: false,
        software_license: false,
        network_key: false,
        license: SessionLicense::default(),
        db_proc_info: String::new(),
        db_proc_took_at: String::new(),
        current_service_name: String::new(),
        data_separation: String::new(),
        session_id: 0,
        counters: SessionCounters::default(),
    })
}

fn map_session_record(raw: SessionRecordRaw) -> SessionRecord {
    let counters = SessionCounters {
        blocked_by_dbms: raw.blocked_by_dbms,
        blocked_by_ls: raw.blocked_by_ls,
        bytes_all: raw.bytes_all,
        bytes_last_5min: raw.bytes_last_5min,
        calls_all: raw.calls_all,
        calls_last_5min: raw.calls_last_5min,
        dbms_bytes_all: raw.dbms_bytes_all,
        dbms_bytes_last_5min: raw.dbms_bytes_last_5min,
        db_proc_took: raw.db_proc_took,
        duration_all: raw.duration_all,
        duration_all_dbms: raw.duration_all_dbms,
        duration_current: raw.duration_current,
        duration_current_dbms: raw.duration_current_dbms,
        duration_last_5min: raw.duration_last_5min,
        duration_last_5min_dbms: raw.duration_last_5min_dbms,
        passive_session_hibernate_time: raw.passive_session_hibernate_time,
        hibernate_session_terminate_time: raw.hibernate_session_terminate_time,
        memory_current: raw.memory_current,
        memory_last_5min: raw.memory_last_5min,
        memory_total: raw.memory_total,
        read_current: raw.read_current,
        read_last_5min: raw.read_last_5min,
        read_total: raw.read_total,
        write_current: raw.write_current,
        write_last_5min: raw.write_last_5min,
        write_total: raw.write_total,
        duration_current_service: raw.duration_current_service,
        duration_last_5min_service: raw.duration_last_5min_service,
        duration_all_service: raw.duration_all_service,
        cpu_time_current: raw.cpu_time_current,
        cpu_time_last_5min: raw.cpu_time_last_5min,
        cpu_time_total: raw.cpu_time_total,
    };
    SessionRecord {
        session: raw.session,
        app_id: raw.app_id,
        connection: raw.connection,
        process: raw.process,
        infobase: raw.infobase,
        host: raw.host,
        hibernate: raw.hibernate,
        locale: raw.locale,
        user_name: raw.user_name,
        started_at: raw.started_at,
        last_active_at: raw.last_active_at,
        client_ip: raw.client_ip,
        retrieved_by_server: false,
        software_license: false,
        network_key: false,
        license: map_session_license(raw.license),
        db_proc_info: raw.db_proc_info,
        db_proc_took_at: raw.db_proc_took_at,
        current_service_name: raw.current_service_name,
        data_separation: raw.data_separation,
        session_id: raw.session_id,
        counters,
    }
}

fn map_session_license(raw: SessionLicenseRecord) -> SessionLicense {
    SessionLicense {
        license_type: raw.license_type,
        server_address: raw.server_address,
        process_id: raw.process_id,
        file_name: raw.file_name,
        brief_presentation: raw.brief_presentation,
        max_users_all: raw.max_users_all,
        max_users_current: raw.max_users_current,
        full_presentation: raw.full_presentation,
        issued_by_server: raw.issued_by_server,
        server_port: raw.server_port,
        software_license: false,
        key_series: raw.key_series,
        network_key: raw.network_key,
    }
}

fn collect_session_fields(record: &SessionRecord) -> Vec<String> {
    let mut out = Vec::new();
    push_if_nonempty(&mut out, &record.app_id);
    push_if_nonempty(&mut out, &record.db_proc_info);
    push_if_nonempty(&mut out, &record.db_proc_took_at);
    push_if_nonempty(&mut out, &record.host);
    push_if_nonempty(&mut out, &record.locale);
    push_if_nonempty(&mut out, &record.user_name);
    push_if_nonempty(&mut out, &record.started_at);
    push_if_nonempty(&mut out, &record.last_active_at);
    push_if_nonempty(&mut out, &record.client_ip);
    push_if_nonempty(&mut out, &record.current_service_name);
    push_if_nonempty(&mut out, &record.data_separation);
    push_if_nonempty(&mut out, &record.license.file_name);
    push_if_nonempty(&mut out, &record.license.brief_presentation);
    push_if_nonempty(&mut out, &record.license.full_presentation);
    push_if_nonempty(&mut out, &record.license.server_address);
    push_if_nonempty(&mut out, &record.license.process_id);
    push_if_nonempty(&mut out, &record.license.key_series);
    out
}

fn push_if_nonempty(out: &mut Vec<String>, value: &str) {
    if !value.is_empty() {
        out.push(value.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
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

        assert_eq!(records[0].app_id, "1CV8C");
        assert_eq!(records[1].app_id, "Designer");
        assert_eq!(records[2].app_id, "SystemBackgroundJob");
        assert_eq!(records[0].client_ip, "127.0.0.1");
        assert_eq!(records[1].locale, "ru_RU");
        assert_eq!(records[2].session_id, 5);
        assert_eq!(records[2].counters.dbms_bytes_all, 3088);
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
        assert_eq!(record.app_id, "Designer");
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.user_name, "DefUser");
        assert_eq!(record.client_ip, "127.0.0.1");
        assert_eq!(record.session_id, 1);
        assert_eq!(record.counters.bytes_all, 253146);
        assert_eq!(record.counters.dbms_bytes_all, 654414);
        assert_eq!(record.counters.cpu_time_total, 1357);
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
        assert_eq!(record.app_id, "1CV8C");
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.locale, "ru");
        assert_eq!(record.user_name, "DefUser");
        assert_eq!(record.started_at, "2026-02-15T00:10:57");
        assert_eq!(record.client_ip, "127.0.0.1");
        assert!(!record.software_license);
        assert!(!record.network_key);
        assert!(!record.retrieved_by_server);
        assert_eq!(record.session_id, 3);
        assert_eq!(record.counters.bytes_all, 7807077);
        assert_eq!(record.counters.dbms_bytes_all, 10914466);
        let lic = &record.license;
        assert_eq!(
            lic.file_name,
            "file:///home/alko/.1cv8/1C/1cv8/conf/20260213011049.lic"
        );
        assert_eq!(lic.key_series, "500000025347");
        assert_eq!(lic.max_users_all, 4);
        assert_eq!(lic.max_users_current, 4);
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
            crate::rac_wire::parse_uuid("e41e750e-56d7-40fb-b2e0-5e71b8e8f508").unwrap()
        );
        assert_eq!(
            record.process,
            crate::rac_wire::parse_uuid("f77f2c1d-1e5b-4855-a0b9-94390ccd4ce5").unwrap()
        );
        assert_eq!(record.db_proc_info, "5719");
        assert_eq!(record.counters.blocked_by_ls, 6);
        assert_eq!(record.counters.db_proc_took, 18982);
        assert_eq!(record.counters.duration_current, 20172);
        assert_eq!(record.counters.memory_current, 18446744073709504536);
        assert_eq!(record.counters.read_current, 16176);
        assert_eq!(record.counters.duration_current_service, 0);
        assert_eq!(record.counters.cpu_time_current, 1051);
        assert!(!record.software_license);
        assert!(!record.network_key);
        assert!(!record.retrieved_by_server);
        assert_eq!(record.last_active_at, "2026-02-16T00:28:41");
        assert_eq!(record.data_separation, "''");
        assert_eq!(record.client_ip, "127.0.0.1");
        let lic = &record.license;
        assert_eq!(lic.process_id, "381094");
        assert_eq!(lic.brief_presentation, "Клиент, 500000025347 4 4");
    }
}
