use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::metadata::RpcMethodMeta;
use crate::protocol::ProtocolCodec;
use crate::rpc::{Meta, Request};
use crate::rpc::AckResponse;
use crate::rac_wire::encode_with_len_u8;

#[derive(Debug, Serialize, Clone)]
pub struct AgentAdminRecord {
    pub name: String,
    pub unknown_tag: u8,
    pub unknown_flags: u32,
    pub unknown_tail: [u8; 3],
}

impl AgentAdminRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let name = cursor.take_str8()?;
        let unknown_tag = cursor.take_u8()?;
        let unknown_flags = cursor.take_u32_be()?;
        let unknown_tail = {
            let bytes = cursor.take_bytes(3)?;
            let value: [u8; 3] = bytes.as_slice().try_into().map_err(|_| RacError::Decode("bytes_fixed"))?;
            value
        };
        Ok(Self {
            name,
            unknown_tag,
            unknown_flags,
            unknown_tail,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct AgentVersionRecord {
    pub version: String,
}

impl AgentVersionRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let version = cursor.take_str8()?;
        Ok(Self {
            version,
        })
    }
}

pub const RPC_AGENT_AUTH_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_AGENT_AUTH_REQ,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_ADMIN_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_AGENT_ADMIN_LIST_REQ,
    method_resp: Some(crate::rac_wire::METHOD_AGENT_ADMIN_LIST_RESP),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_VERSION_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_AGENT_VERSION_REQ,
    method_resp: Some(crate::rac_wire::METHOD_AGENT_VERSION_RESP),
    requires_cluster_context: false,
    requires_infobase_context: false,
};


pub fn parse_agent_version_body(body: &[u8]) -> Result<AgentVersionRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("agent version empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    AgentVersionRecord::decode(&mut cursor)
}

#[derive(Debug, Clone)]
pub struct AgentAuthRequest {
    pub user: String,
    pub pwd: String,
}

impl AgentAuthRequest {
    pub fn encoded_len(&self) -> usize {
        1 + self.user.len() + 1 + self.pwd.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&encode_with_len_u8(self.user.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AgentVersionRequest {
}

impl AgentVersionRequest {
    pub fn encoded_len(&self) -> usize {
        0
    }

    pub fn encode_body(&self, _out: &mut Vec<u8>) -> Result<()> {
        Ok(())
    }
}

pub struct AgentAuthRpc {
    pub user: String,
    pub pwd: String,
}

impl Request for AgentAuthRpc {
    type Response = AckResponse;

    fn meta(&self) -> Meta {
        RPC_AGENT_AUTH_META
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

pub struct AgentAdminListRpc;

impl Request for AgentAdminListRpc {
    type Response = super::AgentAdminListResp;

    fn meta(&self) -> Meta {
        RPC_AGENT_ADMIN_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

pub struct AgentVersionRpc;

impl Request for AgentVersionRpc {
    type Response = super::AgentVersionResp;

    fn meta(&self) -> Meta {
        RPC_AGENT_VERSION_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::rpc_body;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn agent_admin_list_response_hex() {
        let hex = include_str!("../../../../artifacts/rac/agent_admin_list_response_rpc.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let items = crate::commands::parse_list_u8(body, AgentAdminRecord::decode).expect("parse body");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "admin");
        assert_eq!(items[0].unknown_tag, 0);
        assert_eq!(items[0].unknown_flags, 0x3efbfbd);
        assert_eq!(items[0].unknown_tail, [1, 0, 0]);
    }

}
