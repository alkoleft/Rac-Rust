use crate::rac_wire::types::{format_uuid, uuid_from_slice, WireError};

pub fn take_u32_le(data: &[u8], offset: usize) -> Result<(u32, usize), WireError> {
    let end = offset + 4;
    if end > data.len() {
        return Err(WireError::Truncated("u32"));
    }
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&data[offset..end]);
    Ok((u32::from_le_bytes(buf), end))
}

pub fn take_u16_be(data: &[u8], offset: usize) -> Result<(u16, usize), WireError> {
    let end = offset + 2;
    if end > data.len() {
        return Err(WireError::Truncated("u16"));
    }
    let mut buf = [0u8; 2];
    buf.copy_from_slice(&data[offset..end]);
    Ok((u16::from_be_bytes(buf), end))
}

pub fn take_u32_be(data: &[u8], offset: usize) -> Result<(u32, usize), WireError> {
    let end = offset + 4;
    if end > data.len() {
        return Err(WireError::Truncated("u32"));
    }
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&data[offset..end]);
    Ok((u32::from_be_bytes(buf), end))
}

pub fn take_u64_le(data: &[u8], offset: usize) -> Result<(u64, usize), WireError> {
    let end = offset + 8;
    if end > data.len() {
        return Err(WireError::Truncated("u64"));
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&data[offset..end]);
    Ok((u64::from_le_bytes(buf), end))
}

pub fn take_u64_be(data: &[u8], offset: usize) -> Result<(u64, usize), WireError> {
    let end = offset + 8;
    if end > data.len() {
        return Err(WireError::Truncated("u64"));
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&data[offset..end]);
    Ok((u64::from_be_bytes(buf), end))
}

pub fn take_uuid16(data: &[u8], offset: usize) -> Result<([u8; 16], usize), WireError> {
    let end = offset + 16;
    if end > data.len() {
        return Err(WireError::Truncated("uuid"));
    }
    let uuid = uuid_from_slice(&data[offset..end])?;
    Ok((uuid, end))
}

pub fn take_str8(data: &[u8], offset: usize) -> Result<(String, usize), WireError> {
    if offset >= data.len() {
        return Err(WireError::Truncated("str8 len"));
    }
    let len = data[offset] as usize;
    let start = offset + 1;
    let end = start + len;
    if end > data.len() {
        return Err(WireError::Truncated("str8 data"));
    }
    let s = std::str::from_utf8(&data[start..end])
        .map_err(|_| WireError::InvalidData("invalid utf-8"))?
        .to_string();
    if s.len() != len {
        println!("Несовпадение длин {} != {}: {}", s.len(), len, s)
    }
    Ok((s, end))
}

pub fn scan_len_prefixed_strings(data: &[u8]) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < data.len() {
        let len = data[i] as usize;
        let start = i + 1;
        let end = start + len;
        if len > 0 && end <= data.len() {
            if let Ok(s) = std::str::from_utf8(&data[start..end]) {
                if s.chars().all(|c| !c.is_control()) {
                    out.push((i, s.to_string()));
                }
            }
        }
        i += 1;
    }
    out
}

pub fn scan_prefixed_uuids(data: &[u8]) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for i in 0..data.len() {
        let marker = data[i];
        if marker != 0x16 && marker != 0x19 {
            continue;
        }
        let start = i + 1;
        let end = start + 16;
        if end <= data.len() {
            if let Ok(uuid) = uuid_from_slice(&data[start..end]) {
                out.push((i, format_uuid(&uuid)));
            }
        }
    }
    out
}
