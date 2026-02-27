use crate::client::RacClient;
use crate::error::Result;
use crate::Uuid16;

#[derive(Debug, serde::Serialize, Clone)]
pub struct LockDescr {
    pub descr: String,
    pub descr_flag: Option<u8>,
}

mod generated {
    use super::LockDescr;
    include!("lock_generated.rs");
}

pub use generated::{LockListResp, LockListRpc, LockRecordRaw};

pub fn lock_list(client: &mut RacClient, cluster: Uuid16) -> Result<LockListResp> {
    client.call_typed(LockListRpc { cluster })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::parse_list_u8;
    use crate::protocol::ProtocolVersion;

    fn push_uuid(out: &mut Vec<u8>, value: Uuid16) {
        out.extend_from_slice(&value);
    }

    fn push_u64_be(out: &mut Vec<u8>, value: u64) {
        out.extend_from_slice(&value.to_be_bytes());
    }

    fn append_record_no_flag(
        out: &mut Vec<u8>,
        connection: Uuid16,
        descr: &str,
        locked_raw: u64,
        session: Uuid16,
        object: Uuid16,
    ) {
        push_uuid(out, connection);
        out.push(descr.len() as u8);
        out.extend_from_slice(descr.as_bytes());
        push_u64_be(out, locked_raw);
        push_uuid(out, session);
        push_uuid(out, object);
    }

    fn append_record_with_flag(
        out: &mut Vec<u8>,
        connection: Uuid16,
        descr: &str,
        flag: u8,
        locked_raw: u64,
        session: Uuid16,
        object: Uuid16,
    ) {
        push_uuid(out, connection);
        out.push(descr.len() as u8);
        out.push(flag);
        out.extend_from_slice(descr.as_bytes());
        push_u64_be(out, locked_raw);
        push_uuid(out, session);
        push_uuid(out, object);
    }

    #[test]
    fn parse_lock_list_records_with_and_without_flag() {
        let connection_a = crate::rac_wire::parse_uuid("c030e65d-680a-41ed-a15a-6b859025f0b7")
            .unwrap();
        let session_a =
            crate::rac_wire::parse_uuid("717bdda7-2f60-4577-b262-f1fc8c0e472c").unwrap();
        let object_a =
            crate::rac_wire::parse_uuid("f77f2c1d-1e5b-4855-a0b9-94390ccd4ce5").unwrap();
        let connection_b =
            crate::rac_wire::parse_uuid("11111111-2222-3333-4444-555555555555").unwrap();
        let session_b =
            crate::rac_wire::parse_uuid("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee").unwrap();
        let object_b =
            crate::rac_wire::parse_uuid("99999999-8888-7777-6666-555555555555").unwrap();

        let locked_a = 621_355_968_010_000u64;
        let locked_b = 621_355_968_020_000u64;

        let mut body = Vec::new();
        body.push(2);
        append_record_no_flag(
            &mut body,
            connection_a,
            "Lock-A",
            locked_a,
            session_a,
            object_a,
        );
        append_record_with_flag(
            &mut body,
            connection_b,
            "B",
            0x01,
            locked_b,
            session_b,
            object_b,
        );

        let records = parse_list_u8(&body, |cursor| {
            LockRecordRaw::decode(cursor, ProtocolVersion::V16_0)
        })
        .unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].connection, connection_a);
        assert_eq!(records[0].descr.descr, "Lock-A");
        assert_eq!(records[0].descr.descr_flag, None);
        assert_eq!(records[0].locked_at, "1970-01-01T00:00:01");
        assert_eq!(records[0].session, session_a);
        assert_eq!(records[0].object, object_a);
        assert_eq!(records[1].connection, connection_b);
        assert_eq!(records[1].descr.descr, "B");
        assert_eq!(records[1].descr.descr_flag, Some(0x01));
        assert_eq!(records[1].locked_at, "1970-01-01T00:00:02");
        assert_eq!(records[1].session, session_b);
        assert_eq!(records[1].object, object_b);
    }
}
