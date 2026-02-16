pub fn decode_rpc_method(payload: &[u8]) -> Option<u8> {
    if payload.len() >= 5 && payload[0..4] == [0x01, 0x00, 0x00, 0x01] {
        Some(payload[4])
    } else {
        None
    }
}
