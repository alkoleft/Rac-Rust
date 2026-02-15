use crate::rac_wire::types::WireError;

pub fn encode_varuint(mut value: usize) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        let mut b = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            b |= 0x80;
        }
        out.push(b);
        if value == 0 {
            break;
        }
    }
    out
}

pub fn encode_with_len(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 + payload.len());
    out.extend_from_slice(&encode_varuint(payload.len()));
    out.extend_from_slice(payload);
    out
}

pub fn encode_with_len_u8(payload: &[u8]) -> Result<Vec<u8>, WireError> {
    if payload.len() > u8::MAX as usize {
        return Err(WireError::InvalidData("payload too long for u8 length"));
    }
    let mut out = Vec::with_capacity(1 + payload.len());
    out.push(payload.len() as u8);
    out.extend_from_slice(payload);
    Ok(out)
}

pub fn encode_rpc(method_id: u8, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(5 + body.len());
    out.extend_from_slice(&[0x01, 0x00, 0x00, 0x01, method_id]);
    out.extend_from_slice(body);
    out
}

