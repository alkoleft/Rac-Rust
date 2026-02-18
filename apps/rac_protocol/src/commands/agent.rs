use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::Result;

use super::rpc_body;

#[derive(Debug, Serialize, Clone)]
pub struct AgentAdminRecord {
    pub name: String,
    pub unknown_tag: u8,
    pub unknown_flags: u32,
    pub unknown_tail: [u8; 3],
}

#[derive(Debug, Serialize)]
pub struct AgentAdminListResp {
    pub admins: Vec<AgentAdminRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct AgentVersionResp {
    pub version: Option<String>,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn agent_admin_list(
    client: &mut RacClient,
    agent_user: &str,
    agent_pwd: &str,
) -> Result<AgentAdminListResp> {
    client.call(RacRequest::AgentAuth {
        user: agent_user.to_string(),
        pwd: agent_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::AgentAdminList)?;
    Ok(AgentAdminListResp {
        admins: parse_agent_admin_list_body(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn agent_version(client: &mut RacClient) -> Result<AgentVersionResp> {
    let reply = client.call(RacRequest::AgentVersion)?;
    let body = rpc_body(&reply)?;
    let mut cursor = RecordCursor::new(body, 0);
    let version = if cursor.remaining_len() == 0 {
        None
    } else {
        Some(cursor.take_str8()?)
    };
    Ok(AgentVersionResp {
        version,
        raw_payload: Some(reply),
    })
}

fn parse_agent_admin_list_body(body: &[u8]) -> Result<Vec<AgentAdminRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut admins = Vec::with_capacity(count);
    for _ in 0..count {
        let name = cursor.take_str8()?;
        let unknown_tag = cursor.take_u8()?;
        let unknown_flags = cursor.take_u32_be()?;
        let tail = cursor.take_bytes(3)?;
        let unknown_tail = [tail[0], tail[1], tail[2]];
        admins.push(AgentAdminRecord {
            name,
            unknown_tag,
            unknown_flags,
            unknown_tail,
        });
    }
    Ok(admins)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::RacProtocolVersion;
    use crate::rac_wire::{METHOD_AGENT_ADMIN_LIST_RESP, METHOD_AGENT_VERSION_RESP};

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    fn rpc_with_body(method: u8, body: &[u8]) -> Vec<u8> {
        let mut out = vec![0x01, 0x00, 0x00, 0x01, method];
        out.extend_from_slice(body);
        out
    }

    #[test]
    fn parse_agent_admin_list_from_capture() {
        let payload = decode_hex_str("0100000101010561646d696e0003efbfbd010000");
        let body = rpc_body(&payload).expect("rpc body");
        let admins = parse_agent_admin_list_body(body).expect("parse list");

        assert_eq!(admins.len(), 1);
        assert_eq!(admins[0].name, "admin");
        assert_eq!(admins[0].unknown_tag, 0);
        assert_eq!(admins[0].unknown_flags, 0x03efbfbd);
        assert_eq!(admins[0].unknown_tail, [0x01, 0x00, 0x00]);
    }

    #[test]
    fn encode_agent_auth_request() {
        let expected = decode_hex_str("01000001080561646d696e0470617373");
        let req = RacRequest::AgentAuth {
            user: "admin".to_string(),
            pwd: "pass".to_string(),
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_agent_admin_list_request() {
        let expected = decode_hex_str("0100000100");
        let req = RacRequest::AgentAdminList;
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(METHOD_AGENT_ADMIN_LIST_RESP));
    }

    #[test]
    fn parse_agent_version() {
        let mut body = Vec::new();
        body.push(0x05);
        body.extend_from_slice(b"1.2.3");
        let payload = rpc_with_body(METHOD_AGENT_VERSION_RESP, &body);
        let body = rpc_body(&payload).unwrap();
        let mut cursor = RecordCursor::new(body, 0);
        let version = cursor.take_str8().unwrap();
        assert_eq!(version, "1.2.3");
    }
}
