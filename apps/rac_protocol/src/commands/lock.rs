use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::{v8_datetime_to_iso, RecordCursor};
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

#[derive(Debug, Serialize, Clone)]
pub struct LockRecord {
    pub connection: Uuid16,
    pub descr: String,
    pub descr_flag: Option<u8>,
    pub locked_at: String,
    pub session: Uuid16,
    pub object: Uuid16,
}

#[derive(Debug, Serialize)]
pub struct LockListResp {
    pub locks: Vec<Uuid16>,
    pub records: Vec<LockRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn lock_list(client: &mut RacClient, cluster: Uuid16) -> Result<LockListResp> {
    let reply = client.call(RacRequest::LockList { cluster })?;
    let body = rpc_body(&reply)?;
    let records = parse_lock_list_records(body)?;
    Ok(LockListResp {
        locks: records.iter().map(|record| record.object).collect(),
        records,
        raw_payload: Some(reply),
    })
}

fn parse_lock_list_records(body: &[u8]) -> Result<Vec<LockRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(parse_lock_record(&mut cursor)?);
    }
    Ok(records)
}

fn parse_lock_record(cursor: &mut RecordCursor<'_>) -> Result<LockRecord> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("lock record truncated"));
    }
    let connection = cursor.take_uuid()?;
    let (descr, descr_flag) = parse_lock_descr(cursor)?;
    let locked_raw = cursor.take_u64_be()?;
    let session = cursor.take_uuid()?;
    let object = cursor.take_uuid()?;
    let locked_at = v8_datetime_to_iso(locked_raw).unwrap_or_default();
    Ok(LockRecord {
        connection,
        descr,
        descr_flag,
        locked_at,
        session,
        object,
    })
}

fn parse_lock_descr(cursor: &mut RecordCursor<'_>) -> Result<(String, Option<u8>)> {
    let descr_len = cursor.take_u8()? as usize;
    if descr_len == 0 {
        return Ok((String::new(), None));
    }
    let first = cursor.take_u8()?;
    let remaining = cursor.remaining_len();
    let needed_no_flag = descr_len.saturating_sub(1) + 40;
    let needed_flag = descr_len + 40;
    let use_flag = if first == 0x01 {
        if remaining == needed_flag {
            true
        } else if remaining == needed_no_flag {
            false
        } else if remaining >= needed_flag && remaining < needed_no_flag {
            true
        } else if remaining >= needed_no_flag {
            false
        } else {
            remaining >= needed_flag
        }
    } else {
        false
    };
    if use_flag {
        let descr_bytes = cursor.take_bytes(descr_len)?;
        let descr = String::from_utf8(descr_bytes)
            .map_err(|_| RacError::Decode("lock descr invalid utf-8"))?;
        return Ok((descr, Some(first)));
    }
    let mut descr_bytes = Vec::with_capacity(descr_len);
    descr_bytes.push(first);
    if descr_len > 1 {
        descr_bytes.extend_from_slice(&cursor.take_bytes(descr_len - 1)?);
    }
    let descr =
        String::from_utf8(descr_bytes).map_err(|_| RacError::Decode("lock descr invalid utf-8"))?;
    Ok((descr, None))
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let records = parse_lock_list_records(&body).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].connection, connection_a);
        assert_eq!(records[0].descr, "Lock-A");
        assert_eq!(records[0].descr_flag, None);
        assert_eq!(records[0].locked_at, "1970-01-01T00:00:01");
        assert_eq!(records[0].session, session_a);
        assert_eq!(records[0].object, object_a);
        assert_eq!(records[1].connection, connection_b);
        assert_eq!(records[1].descr, "B");
        assert_eq!(records[1].descr_flag, Some(0x01));
        assert_eq!(records[1].locked_at, "1970-01-01T00:00:02");
        assert_eq!(records[1].session, session_b);
        assert_eq!(records[1].object, object_b);
    }
}
