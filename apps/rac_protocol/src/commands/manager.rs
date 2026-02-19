use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

mod generated {
    include!("manager_generated.rs");
}

pub use generated::ManagerRecord;

#[derive(Debug, Serialize)]
pub struct ManagerListResp {
    pub managers: Vec<ManagerRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ManagerInfoResp {
    pub manager: ManagerRecord,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn manager_list(client: &mut RacClient, cluster: Uuid16) -> Result<ManagerListResp> {
    let reply = client.call(RacRequest::ManagerList { cluster })?;
    Ok(ManagerListResp {
        managers: parse_manager_list_body(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn manager_info(
    client: &mut RacClient,
    cluster: Uuid16,
    manager: Uuid16,
) -> Result<ManagerInfoResp> {
    let reply = client.call(RacRequest::ManagerInfo { cluster, manager })?;
    Ok(ManagerInfoResp {
        manager: parse_manager_info_body(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

fn parse_manager_list_body(body: &[u8]) -> Result<Vec<ManagerRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut managers = Vec::with_capacity(count);
    for _ in 0..count {
        managers.push(parse_manager_record(&mut cursor)?);
    }
    Ok(managers)
}

fn parse_manager_info_body(body: &[u8]) -> Result<ManagerRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("manager info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    parse_manager_record(&mut cursor)
}

fn parse_manager_record(cursor: &mut RecordCursor<'_>) -> Result<ManagerRecord> {
    ManagerRecord::decode(cursor)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    fn expected_descr() -> String {
        let bytes = vec![
            0xd0, 0x93, 0xd0, 0xbb, 0xd0, 0xb0, 0xd0, 0xb2, 0xd0, 0xbd, 0xd1, 0x8b, 0xd0,
            0xb9, 0x20, 0xd0, 0xbc, 0xd0, 0xb5, 0xd0, 0xbd, 0xd0, 0xb5, 0xd0, 0xb4, 0xd0,
            0xb6, 0xd0, 0xb5, 0xd1, 0x80, 0x20, 0xd0, 0xba, 0xd0, 0xbb, 0xd0, 0xb0, 0xd1,
            0x81, 0xd1, 0x82, 0xd0, 0xb5, 0xd1, 0x80, 0xd0, 0xb0,
        ];
        String::from_utf8(bytes).expect("descr utf8")
    }

    #[test]
    fn parse_manager_list_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/manager_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let managers = parse_manager_list_body(body).expect("parse list");

        assert_eq!(managers.len(), 1);
        assert_eq!(
            managers[0].manager,
            [
                0x39, 0x85, 0xf9, 0x06, 0xba, 0x9d, 0x48, 0x4f, 0xae, 0xbc, 0x3e, 0x1c, 0x6f,
                0x1a, 0x8f, 0xe8,
            ]
        );
        assert_eq!(managers[0].descr, expected_descr());
        assert_eq!(managers[0].host, "alko-home");
        assert_eq!(managers[0].using, 1);
        assert_eq!(managers[0].port, 0x0605);
        assert_eq!(managers[0].pid, "314037");
    }

    #[test]
    fn parse_manager_info_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/manager_info_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let manager = parse_manager_info_body(body).expect("parse info");

        assert_eq!(
            manager.manager,
            [
                0x39, 0x85, 0xf9, 0x06, 0xba, 0x9d, 0x48, 0x4f, 0xae, 0xbc, 0x3e, 0x1c, 0x6f,
                0x1a, 0x8f, 0xe8,
            ]
        );
        assert_eq!(manager.descr, expected_descr());
        assert_eq!(manager.host, "alko-home");
        assert_eq!(manager.using, 1);
        assert_eq!(manager.port, 0x0605);
        assert_eq!(manager.pid, "314037");
    }
}
