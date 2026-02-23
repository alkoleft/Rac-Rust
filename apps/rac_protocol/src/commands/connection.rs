use serde::Serialize;

use crate::client::RacClient;
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::protocol::ProtocolCodec;
use crate::rpc::{Meta, Request, Response};
use crate::rpc::decode_utils::rpc_body;
use crate::rac_wire::{
    METHOD_CONNECTION_INFO_REQ, METHOD_CONNECTION_INFO_RESP, METHOD_CONNECTION_LIST_REQ,
    METHOD_CONNECTION_LIST_RESP,
};
use crate::Uuid16;

use super::call_body;

mod generated {
    include!("connection_generated.rs");
}

pub use generated::ConnectionRecord;

#[derive(Debug, Serialize)]
pub struct ConnectionListResp {
    pub connections: Vec<Uuid16>,
    pub records: Vec<ConnectionRecord>,
}

#[derive(Debug, Serialize)]
pub struct ConnectionInfoResp {
    pub connection: Uuid16,
    pub record: ConnectionRecord,
    pub fields: Vec<String>,
}

struct ConnectionListRpc {
    cluster: Uuid16,
}

impl Request for ConnectionListRpc {
    type Response = ConnectionListResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_CONNECTION_LIST_REQ,
            method_resp: Some(METHOD_CONNECTION_LIST_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        Ok(self.cluster.to_vec())
    }
}

impl Response for ConnectionListResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let records = parse_connection_list_records(body)?;
        Ok(Self {
            connections: records.iter().map(|record| record.connection).collect(),
            records,
        })
    }
}

pub fn connection_list(client: &mut RacClient, cluster: Uuid16) -> Result<ConnectionListResp> {
    client.call_typed(ConnectionListRpc { cluster })
}

struct ConnectionInfoRpc {
    cluster: Uuid16,
    connection: Uuid16,
}

impl Request for ConnectionInfoRpc {
    type Response = Vec<u8>;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_CONNECTION_INFO_REQ,
            method_resp: Some(METHOD_CONNECTION_INFO_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(32);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.connection);
        Ok(out)
    }
}

pub fn connection_info(
    client: &mut RacClient,
    cluster: Uuid16,
    connection: Uuid16,
) -> Result<ConnectionInfoResp> {
    let body = call_body(client, ConnectionInfoRpc { cluster, connection })?;
    let record = parse_connection_info_body(&body, connection)?;
    let fields = collect_connection_fields(&record);
    Ok(ConnectionInfoResp {
        connection: record.connection,
        record,
        fields,
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

fn parse_connection_info_body(body: &[u8], requested: Uuid16) -> Result<ConnectionRecord> {
    match parse_connection_info_from_list(body, requested) {
        Ok(record) => Ok(record),
        Err(_) => parse_connection_record_for_info(body, requested),
    }
}

fn parse_connection_record_for_info(body: &[u8], requested: Uuid16) -> Result<ConnectionRecord> {
    let record = parse_connection_record_1cv8c(body)?;
    if record.connection != requested {
        return Err(RacError::Decode(
            "connection info record does not match requested connection",
        ));
    }
    Ok(record)
}

fn parse_connection_info_from_list(body: &[u8], requested: Uuid16) -> Result<ConnectionRecord> {
    let records = parse_connection_list_records(body)?;
    for record in records {
        if record.connection == requested {
            return Ok(record);
        }
    }
    Err(RacError::Decode("connection info record not found"))
}

fn parse_connection_record(cursor: &mut RecordCursor<'_>) -> Result<ConnectionRecord> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("connection record truncated"));
    }
    ConnectionRecord::decode(cursor)
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

    #[test]
    fn parse_connection_info_from_list_body() {
        let conn_a = crate::rac_wire::parse_uuid("10101010-2020-3030-4040-505050505050").unwrap();
        let conn_b = crate::rac_wire::parse_uuid("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee").unwrap();
        let info_b = crate::rac_wire::parse_uuid("bbbbbbbb-cccc-dddd-eeee-ffffffffffff").unwrap();
        let proc_a = crate::rac_wire::parse_uuid("11111111-2222-3333-4444-555555555555").unwrap();
        let proc_b = crate::rac_wire::parse_uuid("cccccccc-dddd-eeee-ffff-111111111111").unwrap();

        let record_a = ConnectionRecord {
            connection: conn_a,
            application: "RAS".to_string(),
            blocked_by_ls: 0,
            connected_at: String::new(),
            conn_id: 12,
            host: "host-a".to_string(),
            infobase: Uuid16::default(),
            process: proc_a,
            session_number: 1,
        };
        let record_b = ConnectionRecord {
            connection: conn_b,
            application: "1CV8C".to_string(),
            blocked_by_ls: 3,
            connected_at: String::new(),
            conn_id: 77,
            host: "host-b".to_string(),
            infobase: info_b,
            process: proc_b,
            session_number: 9,
        };

        let raw_a = 621_355_968_000_000u64;
        let raw_b = raw_a + 100_000;

        let mut body = Vec::new();
        body.push(2);
        append_record(&mut body, &record_a, raw_a);
        append_record(&mut body, &record_b, raw_b);

        let parsed = parse_connection_info_body(&body, conn_b).expect("connection info parse");
        assert_eq!(parsed.connection, conn_b);
        assert_eq!(parsed.application, "1CV8C");
        assert_eq!(parsed.blocked_by_ls, 3);
        assert_eq!(parsed.connected_at, "1970-01-01T00:00:10");
        assert_eq!(parsed.conn_id, 77);
        assert_eq!(parsed.host, "host-b");
        assert_eq!(parsed.infobase, info_b);
        assert_eq!(parsed.process, proc_b);
        assert_eq!(parsed.session_number, 9);
    }
}
