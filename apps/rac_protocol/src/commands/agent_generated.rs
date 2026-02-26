use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
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

pub struct AgentAuthRpc {
    pub user: String,
    pub pwd: String,
}

impl crate::rpc::Request for AgentAuthRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_AGENT_AUTH_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(1 + self.user.len() + 1 + self.pwd.len());
        out.extend_from_slice(&encode_with_len_u8(self.user.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        Ok(out)
    }
}

pub struct AgentAdminListRpc;

impl crate::rpc::Request for AgentAdminListRpc {
    type Response = AgentAdminListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_AGENT_ADMIN_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

pub struct AgentAdminRegisterRpc {
    pub name: String,
    pub descr: String,
    pub pwd: String,
    pub auth_tag: u8,
    pub auth_flags: u8,
    pub os_user: String,
}

impl crate::rpc::Request for AgentAdminRegisterRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_AGENT_ADMIN_REGISTER_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(1 + self.name.len() + 1 + self.descr.len() + 1 + self.pwd.len() + 1 + 1 + 1 + self.os_user.len());
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.descr.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        out.push(self.auth_tag);
        out.push(self.auth_flags);
        out.extend_from_slice(&encode_with_len_u8(self.os_user.as_bytes())?);
        Ok(out)
    }
}

pub struct AgentAdminRemoveRpc {
    pub name: String,
}

impl crate::rpc::Request for AgentAdminRemoveRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_AGENT_ADMIN_REMOVE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(1 + self.name.len());
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        Ok(out)
    }
}

pub struct AgentVersionRpc;

impl crate::rpc::Request for AgentVersionRpc {
    type Response = AgentVersionResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_AGENT_VERSION_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(0);
        let _ = &mut out;
        Ok(out)
    }
}


#[derive(Debug, Serialize)]
pub struct AgentAdminListResp {
    pub admins: Vec<AgentAdminRecord>,
}

impl crate::rpc::Response for AgentAdminListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        Ok(Self {
            admins: crate::commands::parse_list_u8(body, AgentAdminRecord::decode)?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct AgentVersionResp {
    pub version: String,
}

impl crate::rpc::Response for AgentVersionResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let record = parse_agent_version_body(body)?;
        Ok(Self {
            version: record.version,
        })
    }
}


pub fn parse_agent_version_body(body: &[u8]) -> Result<AgentVersionRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("agent version empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    AgentVersionRecord::decode(&mut cursor)
}


pub const RPC_AGENT_AUTH_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x08,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_ADMIN_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x00,
    method_resp: Some(0x01),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_ADMIN_REGISTER_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x04,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_ADMIN_REMOVE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x06,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_VERSION_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x87,
    method_resp: Some(0x88),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

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
