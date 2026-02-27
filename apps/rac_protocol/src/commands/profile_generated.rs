use crate::error::RacError;
use crate::protocol::ProtocolVersion;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::Uuid16;
use crate::rac_wire::encode_with_len_u8;

pub const METHOD_PROFILE_LIST_REQ: u8 = 0x59;
pub const METHOD_PROFILE_LIST_RESP: u8 = 0x5a;
pub const METHOD_PROFILE_UPDATE_REQ: u8 = 0x5b;

#[derive(Debug, Serialize, Clone)]
pub struct ProfileRecord {
    pub name: String,
    pub descr: String,
    pub directory_access: u8,
    pub com_access: u8,
    pub addin_access: u8,
    pub module_access: u8,
    pub app_access: u8,
    pub config: u8,
    pub privileged_mode: u8,
    pub inet_access: u8,
    pub crypto: u8,
    pub right_extension: u8,
    pub right_extension_definition_roles: String,
    pub all_modules_extension: u8,
    pub modules_available_for_extension: String,
    pub modules_not_available_for_extension: String,
    pub privileged_mode_roles: String,
}

impl ProfileRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>, protocol_version: ProtocolVersion) -> Result<Self> {
        let name = cursor.take_str8()?;
        let descr = cursor.take_str8()?;
        let directory_access = cursor.take_u8()?;
        let com_access = cursor.take_u8()?;
        let addin_access = cursor.take_u8()?;
        let module_access = cursor.take_u8()?;
        let app_access = cursor.take_u8()?;
        let config = cursor.take_u8()?;
        let privileged_mode = cursor.take_u8()?;
        let inet_access = cursor.take_u8()?;
        let crypto = cursor.take_u8()?;
        let right_extension = cursor.take_u8()?;
        let right_extension_definition_roles = cursor.take_str8()?;
        let all_modules_extension = cursor.take_u8()?;
        let modules_available_for_extension = cursor.take_str8()?;
        let modules_not_available_for_extension = cursor.take_str8()?;
        let privileged_mode_roles = cursor.take_str8()?;
        Ok(Self {
            name,
            descr,
            directory_access,
            com_access,
            addin_access,
            module_access,
            app_access,
            config,
            privileged_mode,
            inet_access,
            crypto,
            right_extension,
            right_extension_definition_roles,
            all_modules_extension,
            modules_available_for_extension,
            modules_not_available_for_extension,
            privileged_mode_roles,
        })
    }
}

pub struct ProfileListRpc {
    pub cluster: Uuid16,
}

impl crate::rpc::Request for ProfileListRpc {
    type Response = ProfileListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_PROFILE_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ProfileList unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        Ok(out)
    }
}

pub struct ProfileUpdateRpc {
    pub cluster: Uuid16,
    pub name: String,
    pub descr: String,
    pub directory_access: u8,
    pub com_access: u8,
    pub addin_access: u8,
    pub module_access: u8,
    pub app_access: u8,
    pub config: u8,
    pub privileged_mode: u8,
    pub inet_access: u8,
    pub crypto: u8,
    pub right_extension: u8,
    pub right_extension_definition_roles: String,
    pub all_modules_extension: u8,
    pub modules_available_for_extension: String,
    pub modules_not_available_for_extension: String,
    pub privileged_mode_roles: String,
}

impl crate::rpc::Request for ProfileUpdateRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_PROFILE_UPDATE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ProfileUpdate unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.name.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.descr.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.right_extension_definition_roles.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.modules_available_for_extension.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.modules_not_available_for_extension.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.privileged_mode_roles.len() } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.descr.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.directory_access);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.com_access);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.addin_access);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.module_access);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.app_access);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.config);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.privileged_mode);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.inet_access);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.crypto);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.right_extension);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.right_extension_definition_roles.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.all_modules_extension);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.modules_available_for_extension.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.modules_not_available_for_extension.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.privileged_mode_roles.as_bytes())?);
        }
        Ok(out)
    }
}


#[derive(Debug, Serialize)]
pub struct ProfileListResp {
    pub profiles: Vec<ProfileRecord>,
}

impl crate::rpc::Response for ProfileListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        Ok(Self {
            profiles: crate::commands::parse_list_u8(body, |cursor| ProfileRecord::decode(cursor, protocol_version))?,
        })
    }
}



pub const RPC_PROFILE_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_PROFILE_LIST_REQ,
    method_resp: Some(METHOD_PROFILE_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_PROFILE_UPDATE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_PROFILE_UPDATE_REQ,
    method_resp: None,
    requires_cluster_context: true,
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
    fn profile_list_response_empty_hex() {
        let hex = include_str!("../../../../artifacts/rac/profile_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let items = crate::commands::parse_list_u8(body, |cursor| ProfileRecord::decode(cursor, protocol_version)).expect("parse body");
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn profile_list_response_nonempty_hex() {
        let hex = include_str!("../../../../artifacts/rac/v11/v11_profile_list_nonempty2_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let items = crate::commands::parse_list_u8(body, |cursor| ProfileRecord::decode(cursor, protocol_version)).expect("parse body");
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].name, "codex_prof_all_yes");
        assert_eq!(items[0].config, 1);
        assert_eq!(items[0].privileged_mode, 1);
        assert_eq!(items[0].crypto, 1);
        assert_eq!(items[0].right_extension, 1);
        assert_eq!(items[0].right_extension_definition_roles, "role3;role4");
        assert_eq!(items[0].all_modules_extension, 1);
        assert_eq!(items[0].modules_available_for_extension, "mod1;mod2");
        assert_eq!(items[0].modules_not_available_for_extension, "mod3;mod4");
        assert_eq!(items[0].privileged_mode_roles, "role1;role2");
        assert_eq!(items[3].name, "codex_prof_cfg_no");
        assert_eq!(items[3].config, 0);
        assert_eq!(items[3].privileged_mode, 1);
        assert_eq!(items[3].crypto, 1);
        assert_eq!(items[3].right_extension, 0);
    }

}
