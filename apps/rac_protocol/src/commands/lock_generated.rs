use crate::error::RacError;
use crate::codec::v8_datetime_to_iso;
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::Uuid16;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct LockRecordRaw {
    pub connection: Uuid16,
    pub descr: LockDescr,
    pub locked_at: String,
    pub session: Uuid16,
    pub object: Uuid16,
}

impl LockRecordRaw {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let connection = cursor.take_uuid()?;
        let descr = {
            let descr_len = cursor.take_u8()? as usize;
            if descr_len == 0 {
                LockDescr { descr: String::new(), descr_flag: None }
            } else {
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
                    LockDescr { descr, descr_flag: Some(first) }
                } else {
                    let mut descr_bytes = Vec::with_capacity(descr_len);
                    descr_bytes.push(first);
                    if descr_len > 1 {
                        descr_bytes.extend_from_slice(&cursor.take_bytes(descr_len - 1)?);
                    }
                    let descr = String::from_utf8(descr_bytes)
                        .map_err(|_| RacError::Decode("lock descr invalid utf-8"))?;
                    LockDescr { descr, descr_flag: None }
                }
            }
        };
        let locked_at = v8_datetime_to_iso(cursor.take_u64_be()?).unwrap_or_default();
        let session = cursor.take_uuid()?;
        let object = cursor.take_uuid()?;
        Ok(Self {
            connection,
            descr,
            locked_at,
            session,
            object,
        })
    }
}
