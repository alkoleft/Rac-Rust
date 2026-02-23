use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ManagerRecord {
    pub manager: Uuid16,
    pub descr: String,
    pub host: String,
    pub using: u32,
    pub port: u16,
    pub pid: String,
}

impl ManagerRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let manager = cursor.take_uuid()?;
        let descr = cursor.take_str8()?;
        let host = cursor.take_str8()?;
        let using = cursor.take_u32_be()?;
        let port = cursor.take_u16_be()?;
        let pid = cursor.take_str8()?;
        Ok(Self {
            manager,
            descr,
            host,
            using,
            port,
            pid,
        })
    }
}

pub fn parse_manager_list_body(body: &[u8]) -> Result<Vec<ManagerRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(ManagerRecord::decode(&mut cursor)?);
    }
    Ok(out)
}

pub fn parse_manager_info_body(body: &[u8]) -> Result<ManagerRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("manager info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    ManagerRecord::decode(&mut cursor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::rpc_body;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn manager_list_response_hex() {
        let hex = include_str!("../../../../artifacts/rac/manager_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let items = parse_manager_list_body(body).expect("parse body");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].host, "alko-home");
        assert_eq!(items[0].using, 1);
        assert_eq!(items[0].port, 0x605);
        assert_eq!(items[0].pid, "314037");
    }

    #[test]
    fn manager_info_response_hex() {
        let hex = include_str!("../../../../artifacts/rac/manager_info_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let record = parse_manager_info_body(body).expect("parse body");
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.using, 1);
        assert_eq!(record.port, 0x605);
        assert_eq!(record.pid, "314037");
    }

}
