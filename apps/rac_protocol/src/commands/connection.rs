use crate::client::RacClient;
use crate::error::Result;
use crate::Uuid16;

mod generated {
    include!("connection_generated.rs");
}

pub use generated::{
    ConnectionDisconnectRpc,
    ConnectionInfoResp,
    ConnectionInfoRpc,
    ConnectionListByInfobaseRpc,
    ConnectionListResp,
    ConnectionListRpc,
    ConnectionRecord,
};

pub fn connection_list(client: &mut RacClient, cluster: Uuid16) -> Result<ConnectionListResp> {
    client.call_typed(ConnectionListRpc { cluster })
}

pub fn connection_list_by_infobase(
    client: &mut RacClient,
    cluster: Uuid16,
    infobase: Uuid16,
) -> Result<ConnectionListResp> {
    client.call_typed(ConnectionListByInfobaseRpc { cluster, infobase })
}

pub fn connection_info(
    client: &mut RacClient,
    cluster: Uuid16,
    connection: Uuid16,
) -> Result<ConnectionInfoResp> {
    client.call_typed(ConnectionInfoRpc { cluster, connection })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::parse_list_u8;
    use crate::protocol::ProtocolVersion;

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

        let records = parse_list_u8(&body, |cursor| {
            ConnectionRecord::decode(cursor, ProtocolVersion::V16_0)
        })
        .expect("connection list parse");
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

        let parsed =
            generated::parse_connection_info_body(&body, ProtocolVersion::V16_0)
                .expect("connection info parse");
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
    fn parse_connection_list_body_second_entry() {
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

        let parsed = parse_list_u8(&body, |cursor| {
            ConnectionRecord::decode(cursor, ProtocolVersion::V16_0)
        })
        .expect("connection list parse");
        assert_eq!(parsed[1].connection, conn_b);
        assert_eq!(parsed[1].application, "1CV8C");
        assert_eq!(parsed[1].blocked_by_ls, 3);
        assert_eq!(parsed[1].connected_at, "1970-01-01T00:00:10");
        assert_eq!(parsed[1].conn_id, 77);
        assert_eq!(parsed[1].host, "host-b");
        assert_eq!(parsed[1].infobase, info_b);
        assert_eq!(parsed[1].process, proc_b);
        assert_eq!(parsed[1].session_number, 9);
    }
}
