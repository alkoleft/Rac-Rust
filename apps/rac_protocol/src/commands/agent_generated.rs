use crate::error::RacError;
use crate::protocol::ProtocolVersion;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::rac_wire::encode_with_len_u8;

pub const METHOD_AGENT_AUTH_REQ: u8 = 0x08;
pub const METHOD_AGENT_ADMIN_LIST_REQ: u8 = 0x00;
pub const METHOD_AGENT_ADMIN_LIST_RESP: u8 = 0x01;
pub const METHOD_AGENT_ADMIN_REGISTER_REQ: u8 = 0x04;
pub const METHOD_AGENT_ADMIN_REMOVE_REQ: u8 = 0x06;
pub const METHOD_AGENT_VERSION_REQ: u8 = 0x87;
pub const METHOD_AGENT_VERSION_RESP: u8 = 0x88;

#[derive(Debug, Serialize, Clone)]
pub struct AgentAdminRecord {
    pub name: String,
    pub descr: String,
    pub record_marker: u32,
    pub auth_pwd: u8,
    pub auth_os: u8,
    pub os_user: String,
}

impl AgentAdminRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>, _: ProtocolVersion) -> Result<Self> {
        let name = cursor.take_str8()?;
        let descr = {
            let b0 = cursor.take_u8()? as usize;
            let b1 = cursor.take_u8()? as usize;
            let len = (b0 & 0x3f) | (b1 << 6);
            let bytes = cursor.take_bytes(len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        let record_marker = cursor.take_u32_be()?;
        let auth_pwd = cursor.take_u8()?;
        let auth_os = cursor.take_u8()?;
        let os_user = cursor.take_str8()?;
        Ok(Self {
            name,
            descr,
            record_marker,
            auth_pwd,
            auth_os,
            os_user,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct AgentVersionRecord {
    pub version: String,
}

impl AgentVersionRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>, _: ProtocolVersion) -> Result<Self> {
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
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc AgentAuth unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 1 + self.user.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.pwd.len() } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.user.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        }
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
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc AgentAdminList unsupported for protocol"));
        }
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
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc AgentAdminRegister unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 1 + self.name.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.descr.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.pwd.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.os_user.len() } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.descr.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.auth_tag);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.auth_flags);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.os_user.as_bytes())?);
        }
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
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc AgentAdminRemove unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 1 + self.name.len() } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        }
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
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc AgentVersion unsupported for protocol"));
        }
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
        let protocol_version = _codec.protocol_version();
        Ok(Self {
            admins: crate::commands::parse_list_u8(body, |cursor| AgentAdminRecord::decode(cursor, protocol_version))?,
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
        let protocol_version = _codec.protocol_version();
        let record = parse_agent_version_body(body, protocol_version)?;
        Ok(Self {
            version: record.version,
        })
    }
}


pub fn parse_agent_version_body(body: &[u8], protocol_version: ProtocolVersion) -> Result<AgentVersionRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("agent version empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    AgentVersionRecord::decode(&mut cursor, protocol_version)
}


pub const RPC_AGENT_AUTH_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_AGENT_AUTH_REQ,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_ADMIN_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_AGENT_ADMIN_LIST_REQ,
    method_resp: Some(METHOD_AGENT_ADMIN_LIST_RESP),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_ADMIN_REGISTER_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_AGENT_ADMIN_REGISTER_REQ,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_ADMIN_REMOVE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_AGENT_ADMIN_REMOVE_REQ,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_VERSION_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_AGENT_VERSION_REQ,
    method_resp: Some(METHOD_AGENT_VERSION_RESP),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

#[cfg(all(test, feature = "artifacts"))]
mod tests {
    use super::*;
    use crate::commands::rpc_body;
    use crate::protocol::ProtocolVersion;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn agent_admin_list_response_hex() {
        let hex = include_str!("../../../../artifacts/rac/agent_admin_list_response_rpc.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let items = crate::commands::parse_list_u8(body, |cursor| AgentAdminRecord::decode(cursor, protocol_version)).expect("parse body");
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].name, "admin");
        assert_eq!(items[0].descr, "");
        assert_eq!(items[0].record_marker, 0x3efbfbd);
        assert_eq!(items[0].auth_pwd, 1);
        assert_eq!(items[0].auth_os, 0);
        assert_eq!(items[0].os_user, "");
        assert_eq!(items[1].name, "codex_agent_pwd_20260226_053425");
        assert_eq!(items[1].descr, "Codex agent pwd");
        assert_eq!(items[1].auth_os, 0);
        assert_eq!(items[1].os_user, "");
        assert_eq!(items[2].name, "codex_agent_os_20260226_053425");
        assert_eq!(items[2].descr, "Codex agent os");
        assert_eq!(items[2].auth_os, 1);
        assert_eq!(items[2].os_user, "codex_os_user");
    }

    #[test]
    fn agent_admin_list_response_20260301_hex() {
        let hex = include_str!("../../../../artifacts/rac/v16/v16_20260301_agent_admin_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let items = crate::commands::parse_list_u8(body, |cursor| AgentAdminRecord::decode(cursor, protocol_version)).expect("parse body");
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].name, "admin");
        assert_eq!(items[0].descr, "Описание");
        assert_eq!(items[0].auth_pwd, 1);
        assert_eq!(items[0].auth_os, 0);
        assert_eq!(items[1].name, "admin2");
        assert_eq!(items[1].descr, "Описание аутентификация ОС");
        assert_eq!(items[1].auth_pwd, 0);
        assert_eq!(items[1].auth_os, 1);
        assert_eq!(items[1].os_user, "alko");
        assert_eq!(items[2].name, "admin3");
        assert_eq!(items[2].descr, "Описание аутентификация ОС и паролья");
        assert_eq!(items[2].auth_pwd, 1);
        assert_eq!(items[2].auth_os, 1);
        assert_eq!(items[2].os_user, "alko2");
    }

}
