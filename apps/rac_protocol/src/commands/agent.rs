use crate::client::RacClient;
use crate::error::Result;
use crate::rpc::AckResponse;

mod generated {
    include!("agent_generated.rs");
}

pub use generated::{
    AgentAdminListRpc,
    AgentAdminListResp,
    AgentAdminRegisterRpc,
    AgentAdminRemoveRpc,
    AgentAdminRecord,
    AgentAuthRpc,
    AgentVersionResp,
    AgentVersionRpc,
};

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

pub fn agent_admin_register(
    client: &mut RacClient,
    agent_user: &str,
    agent_pwd: &str,
    name: String,
    descr: String,
    pwd: String,
    auth_flags: u8,
    os_user: String,
) -> Result<AckResponse> {
    let _ = client.call_typed(AgentAuthRpc {
        user: agent_user.to_string(),
        pwd: agent_pwd.to_string(),
    })?;
    client.call_typed(AgentAdminRegisterRpc {
        name,
        descr,
        pwd,
        auth_tag: 0x01,
        auth_flags,
        os_user,
    })
}

pub fn agent_admin_remove(
    client: &mut RacClient,
    agent_user: &str,
    agent_pwd: &str,
    name: &str,
) -> Result<AckResponse> {
    let _ = client.call_typed(AgentAuthRpc {
        user: agent_user.to_string(),
        pwd: agent_pwd.to_string(),
    })?;
    client.call_typed(AgentAdminRemoveRpc {
        name: name.to_string(),
    })
}

pub fn agent_version(client: &mut RacClient) -> Result<AgentVersionResp> {
    client.call_typed(AgentVersionRpc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ProtocolVersion;
    use crate::rpc::Request;
    use crate::rpc::Response;
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
        let resp = AgentAdminListResp::decode(&payload, ProtocolVersion::V16_0.boxed().as_ref())
            .expect("parse response");
        let admins = resp.admins;

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
        let resp = AgentVersionResp::decode(&payload, ProtocolVersion::V16_0.boxed().as_ref())
            .expect("parse response");
        assert_eq!(resp.version, "1.2.3");
    }

    #[test]
    fn encode_agent_admin_register_pwd_request() {
        let expected = decode_hex_str(
            "01000001041f636f6465785f6167656e745f7077645f32303236303232365f3035333432350f436f646578206167656e74207077640770617373313233010000",
        );
        let req = AgentAdminRegisterRpc {
            name: "codex_agent_pwd_20260226_053425".to_string(),
            descr: "Codex agent pwd".to_string(),
            pwd: "pass123".to_string(),
            auth_tag: 0x01,
            auth_flags: 0x00,
            os_user: String::new(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_agent_admin_remove_request() {
        let expected = decode_hex_str(
            "01000001061e636f6465785f6167656e745f6f735f32303236303232365f303533343235",
        );
        let req = AgentAdminRemoveRpc {
            name: "codex_agent_os_20260226_053425".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }
}
