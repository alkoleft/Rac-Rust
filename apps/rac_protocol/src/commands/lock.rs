use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

#[derive(Debug, Serialize)]
pub struct LockListResp {
    pub locks: Vec<Uuid16>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn lock_list(client: &mut RacClient, cluster: Uuid16) -> Result<LockListResp> {
    let reply = client.call(RacRequest::LockList { cluster })?;
    let body = rpc_body(&reply)?;
    let locks = parse_lock_list(body)?;
    Ok(LockListResp {
        locks,
        raw_payload: Some(reply),
    })
}

fn parse_lock_list(body: &[u8]) -> Result<Vec<Uuid16>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut locks = Vec::with_capacity(count);
    for _ in 0..count {
        let record = parse_lock_record(&mut cursor)?;
        locks.push(record.object);
    }
    Ok(locks)
}

struct LockRecordParts {
    object: Uuid16,
}

fn parse_lock_record(cursor: &mut RecordCursor<'_>) -> Result<LockRecordParts> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("lock record truncated"));
    }
    let _connection = cursor.take_uuid()?;
    let descr_len = cursor.take_u8()? as usize;
    if descr_len > 0 {
        let next = cursor.take_u8()?;
        let remaining = cursor.remaining_len();
        let needed_no_flag = descr_len.saturating_sub(1) + 40;
        let needed_flag = descr_len + 40;
        let use_flag = next == 0x01 && remaining >= needed_flag && remaining < needed_no_flag;
        if use_flag {
            let _ = cursor.take_bytes(descr_len)?;
        } else {
            let rest_len = descr_len.saturating_sub(1);
            let _ = cursor.take_bytes(rest_len)?;
        }
    }
    let _locked = cursor.take_u64_be()?;
    let _session = cursor.take_uuid()?;
    let object = cursor.take_uuid()?;
    Ok(LockRecordParts { object })
}
