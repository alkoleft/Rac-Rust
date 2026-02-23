use serde::Serialize;

use crate::client::RacClient;
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::protocol::ProtocolCodec;
use crate::rpc::{AckResponse, Meta, Request, Response};
use crate::rpc::decode_utils::rpc_body;
use crate::rac_wire::{
    METHOD_AGENT_ADMIN_LIST_REQ, METHOD_AGENT_ADMIN_LIST_RESP, METHOD_AGENT_AUTH_REQ,
    METHOD_AGENT_VERSION_REQ, METHOD_AGENT_VERSION_RESP,
};

use super::parse_list_u8;

mod generated {
    include!("agent_generated.rs");
}

pub use generated::{AgentAdminRecord, AgentAuthRequest};

#[derive(Debug, Serialize)]
pub struct AgentAdminListResp {
    pub admins: Vec<AgentAdminRecord>,
}

#[derive(Debug, Serialize)]
pub struct AgentVersionResp {
    pub version: Option<String>,
}

struct AgentAuthRpc {
    user: String,
    pwd: String,
}

impl Request for AgentAuthRpc {
    type Response = AckResponse;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_AGENT_AUTH_REQ,
            method_resp: None,
            requires_cluster_context: false,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let req = AgentAuthRequest {
            user: self.user.clone(),
            pwd: self.pwd.clone(),
        };
        let mut out = Vec::with_capacity(req.encoded_len());
        req.encode_body(&mut out)?;
        Ok(out)
    }
}

struct AgentAdminListRpc;

impl Request for AgentAdminListRpc {
    type Response = AgentAdminListResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_AGENT_ADMIN_LIST_REQ,
            method_resp: Some(METHOD_AGENT_ADMIN_LIST_RESP),
            requires_cluster_context: false,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

impl Response for AgentAdminListResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        Ok(Self {
            admins: parse_agent_admin_list_body(body)?,
        })
    }
}

struct AgentVersionRpc;

impl Request for AgentVersionRpc {
    type Response = AgentVersionResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_AGENT_VERSION_REQ,
            method_resp: Some(METHOD_AGENT_VERSION_RESP),
            requires_cluster_context: false,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

impl Response for AgentVersionResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        if body.is_empty() {
            return Ok(Self { version: None });
        }
        let mut cursor = RecordCursor::new(body, 0);
        Ok(Self {
            version: Some(cursor.take_str8()?),
        })
    }
}

pub fn agent_admin_list(
    client: &mut RacClient,
    agent_user: &str,
    agent_pwd: &str,
) -> Result<AgentAdminListResp> {
    let _ = client.call_typed(AgentAuthRpc {
        user: agent_user.to_string(),
        pwd: agent_pwd.to_string(),
    })?;
    client.call_typed(AgentAdminListRpc)
}

pub fn agent_version(client: &mut RacClient) -> Result<Option<String>> {
    let resp = client.call_typed(AgentVersionRpc)?;
    Ok(resp.version)
}

fn parse_agent_admin_list_body(body: &[u8]) -> Result<Vec<AgentAdminRecord>> {
    parse_list_u8(body, AgentAdminRecord::decode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ProtocolVersion;
    use crate::rpc::Request;
    use crate::commands::rpc_body;

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
        let req = AgentAuthRpc {
            user: "admin".to_string(),
            pwd: "pass".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_agent_admin_list_request() {
        let expected = decode_hex_str("0100000100");
        let req = AgentAdminListRpc;
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
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
