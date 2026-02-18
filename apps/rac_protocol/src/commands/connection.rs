use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::{v8_datetime_to_iso, RecordCursor};
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

#[derive(Debug, Serialize, Clone)]
pub struct ConnectionRecord {
    pub connection: Uuid16,
    pub application: String,
    pub blocked_by_ls: u32,
    pub connected_at: String,
    pub conn_id: u32,
    pub host: String,
    pub infobase: Uuid16,
    pub process: Uuid16,
    pub session_number: u32,
}

#[derive(Debug, Serialize)]
pub struct ConnectionListResp {
    pub connections: Vec<Uuid16>,
    pub records: Vec<ConnectionRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ConnectionInfoResp {
    pub connection: Uuid16,
    pub record: ConnectionRecord,
    pub fields: Vec<String>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn connection_list(client: &mut RacClient, cluster: Uuid16) -> Result<ConnectionListResp> {
    let reply = client.call(RacRequest::ConnectionList { cluster })?;
    let body = rpc_body(&reply)?;
    let records = parse_connection_list_records(body)?;
    Ok(ConnectionListResp {
        connections: records.iter().map(|record| record.connection).collect(),
        records,
        raw_payload: Some(reply),
    })
}

pub fn connection_info(
    client: &mut RacClient,
    cluster: Uuid16,
    connection: Uuid16,
) -> Result<ConnectionInfoResp> {
    let reply = client.call(RacRequest::ConnectionInfo {
        cluster,
        connection,
    })?;
    let body = rpc_body(&reply)?;
    let record = parse_connection_record_1cv8c(body)?;
    let fields = collect_connection_fields(&record);
    Ok(ConnectionInfoResp {
        connection: record.connection,
        record,
        fields,
        raw_payload: Some(reply),
    })
}

fn parse_connection_list_records(body: &[u8]) -> Result<Vec<ConnectionRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(parse_connection_record(&mut cursor)?);
    }
    Ok(records)
}

fn parse_connection_record_1cv8c(body: &[u8]) -> Result<ConnectionRecord> {
    let mut cursor = RecordCursor::new(body, 0);
    parse_connection_record(&mut cursor)
}

fn parse_connection_record(cursor: &mut RecordCursor<'_>) -> Result<ConnectionRecord> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("connection record truncated"));
    }
    let connection = cursor.take_uuid()?;
    let application = cursor.take_str8()?;
    let blocked_by_ls = cursor.take_u32_be()?;
    let connected_raw = cursor.take_u64_be()?;
    let conn_id = cursor.take_u32_be()?;
    let host = cursor.take_str8()?;
    let infobase = cursor.take_uuid()?;
    let process = cursor.take_uuid()?;
    let session_number = cursor.take_u32_be()?;
    let connected_at = v8_datetime_to_iso(connected_raw).unwrap_or_default();
    Ok(ConnectionRecord {
        connection,
        application,
        blocked_by_ls,
        connected_at,
        conn_id,
        host,
        infobase,
        process,
        session_number,
    })
}

fn collect_connection_fields(record: &ConnectionRecord) -> Vec<String> {
    let mut out = Vec::new();
    push_if_nonempty(&mut out, &record.application);
    push_if_nonempty(&mut out, &record.host);
    push_if_nonempty(&mut out, &record.connected_at);
    out.push(record.conn_id.to_string());
    out.push(record.session_number.to_string());
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

    fn push_uuid(out: &mut Vec<u8>, value: Uuid16) {
        out.extend_from_slice(&value);
    }

    fn push_str8(out: &mut Vec<u8>, value: &str) {
        let bytes = value.as_bytes();
        out.push(bytes.len() as u8);
        out.extend_from_slice(bytes);
    }

    fn push_u32_be(out: &mut Vec<u8>, value: u32) {
        out.extend_from_slice(&value.to_be_bytes());
    }

    fn push_u64_be(out: &mut Vec<u8>, value: u64) {
        out.extend_from_slice(&value.to_be_bytes());
    }

    fn append_record(out: &mut Vec<u8>, record: &ConnectionRecord, raw_time: u64) {
        push_uuid(out, record.connection);
        push_str8(out, &record.application);
        push_u32_be(out, record.blocked_by_ls);
        push_u64_be(out, raw_time);
        push_u32_be(out, record.conn_id);
        push_str8(out, &record.host);
        push_uuid(out, record.infobase);
        push_uuid(out, record.process);
        push_u32_be(out, record.session_number);
    }

    #[test]
    fn parse_connection_list_records_two_entries() {
        let conn_a = crate::rac_wire::parse_uuid("c030e65d-680a-41ed-a15a-6b859025f0b7").unwrap();
        let info_a = crate::rac_wire::parse_uuid("717bdda7-2f60-4577-b262-f1fc8c0e472c").unwrap();
        let proc_a = crate::rac_wire::parse_uuid("f77f2c1d-1e5b-4855-a0b9-94390ccd4ce5").unwrap();
        let conn_b = crate::rac_wire::parse_uuid("11111111-2222-3333-4444-555555555555").unwrap();
        let proc_b = crate::rac_wire::parse_uuid("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee").unwrap();

        let record_a = ConnectionRecord {
            connection: conn_a,
            application: "RAS".to_string(),
            blocked_by_ls: 0,
            connected_at: String::new(),
            conn_id: 2347,
            host: "alko-home".to_string(),
            infobase: info_a,
            process: proc_a,
            session_number: 0,
        };
        let record_b = ConnectionRecord {
            connection: conn_b,
            application: "1CV8C".to_string(),
            blocked_by_ls: 7,
            connected_at: String::new(),
            conn_id: 42,
            host: "host-2".to_string(),
            infobase: Uuid16::default(),
            process: proc_b,
            session_number: 5,
        };

        let raw_a = 621_355_968_000_000u64;
        let raw_b = raw_a + 10_000;

        let mut body = Vec::new();
        body.push(2);
        append_record(&mut body, &record_a, raw_a);
        append_record(&mut body, &record_b, raw_b);

        let records = parse_connection_list_records(&body).expect("connection list parse");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].connection, conn_a);
        assert_eq!(records[0].application, "RAS");
        assert_eq!(records[0].blocked_by_ls, 0);
        assert_eq!(records[0].connected_at, "1970-01-01T00:00:00");
        assert_eq!(records[0].conn_id, 2347);
        assert_eq!(records[0].host, "alko-home");
        assert_eq!(records[0].infobase, info_a);
        assert_eq!(records[0].process, proc_a);
        assert_eq!(records[0].session_number, 0);
        assert_eq!(records[1].connection, conn_b);
        assert_eq!(records[1].application, "1CV8C");
        assert_eq!(records[1].blocked_by_ls, 7);
        assert_eq!(records[1].connected_at, "1970-01-01T00:00:01");
        assert_eq!(records[1].conn_id, 42);
        assert_eq!(records[1].host, "host-2");
        assert_eq!(records[1].infobase, Uuid16::default());
        assert_eq!(records[1].process, proc_b);
        assert_eq!(records[1].session_number, 5);
    }

    #[test]
    fn parse_connection_info_record_from_body() {
        let conn = crate::rac_wire::parse_uuid("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee").unwrap();
        let info = crate::rac_wire::parse_uuid("bbbbbbbb-cccc-dddd-eeee-ffffffffffff").unwrap();
        let proc = crate::rac_wire::parse_uuid("cccccccc-dddd-eeee-ffff-111111111111").unwrap();

        let record = ConnectionRecord {
            connection: conn,
            application: "AgentStandardCall".to_string(),
            blocked_by_ls: 12,
            connected_at: String::new(),
            conn_id: 777,
            host: "host-3".to_string(),
            infobase: info,
            process: proc,
            session_number: 9,
        };

        let raw_time = 621_355_968_010_000u64;
        let mut body = Vec::new();
        append_record(&mut body, &record, raw_time);

        let parsed = parse_connection_record_1cv8c(&body).expect("connection info parse");
        assert_eq!(parsed.connection, conn);
        assert_eq!(parsed.application, "AgentStandardCall");
        assert_eq!(parsed.blocked_by_ls, 12);
        assert_eq!(parsed.connected_at, "1970-01-01T00:00:01");
        assert_eq!(parsed.conn_id, 777);
        assert_eq!(parsed.host, "host-3");
        assert_eq!(parsed.infobase, info);
        assert_eq!(parsed.process, proc);
        assert_eq!(parsed.session_number, 9);
    }
}
