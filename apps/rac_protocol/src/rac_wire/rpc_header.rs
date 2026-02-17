use crate::codec::RecordCursor;

pub fn decode_rpc_method(payload: &[u8]) -> Option<u8> {
    let mut cursor = RecordCursor::new(payload, 0);
    if cursor.remaining_len() < 5 {
        return None;
    }
    let head = cursor.take_bytes(4).ok()?;
    if head != [0x01, 0x00, 0x00, 0x01] {
        return None;
    }
    cursor.take_u8().ok()
}
