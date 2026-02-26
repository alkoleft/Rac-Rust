use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

pub fn rpc_body(payload: &[u8]) -> Result<&[u8]> {
    let mut cursor = RecordCursor::new(payload);
    if cursor.remaining_len() >= 5 {
        let head = cursor.take_bytes(4)?;
        if head == [0x01, 0x00, 0x00, 0x01] {
            let _ = cursor.take_u8()?;
            return Ok(cursor.remaining_slice());
        }
    }
    Err(RacError::Decode("missing rpc header"))
}

pub fn parse_ack_payload(payload: &[u8], context: &'static str) -> Result<bool> {
    let mut cursor = RecordCursor::new(payload);
    if cursor.remaining_len() < 4 {
        return Err(RacError::Decode(context));
    }
    let ack = cursor.take_u32_be()?;
    Ok(ack == 0x01000000)
}

pub fn expect_ack(payload: &[u8], context: &'static str) -> Result<()> {
    let acknowledged = parse_ack_payload(payload, context)?;
    if !acknowledged {
        return Err(RacError::Decode(context));
    }
    Ok(())
}

pub fn parse_uuid_body(body: &[u8], context: &'static str) -> Result<Uuid16> {
    if body.is_empty() {
        return Err(RacError::Decode(context));
    }
    let mut cursor = RecordCursor::new(body);
    Ok(cursor.take_uuid()?)
}
