use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::metadata::RpcMethodMeta;
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

pub const RPC_AGENT_AUTH_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 8,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_ADMIN_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 0,
    method_resp: Some(1),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_AGENT_VERSION_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 135,
    method_resp: Some(136),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

#[allow(dead_code)]
pub fn rpc_metadata(request: &crate::client::RacRequest) -> Option<RpcMethodMeta> {
    match request {
        crate::client::RacRequest::AgentAuth { .. } => Some(RPC_AGENT_AUTH_META),
        crate::client::RacRequest::AgentAdminList => Some(RPC_AGENT_ADMIN_LIST_META),
        crate::client::RacRequest::AgentVersion => Some(RPC_AGENT_VERSION_META),
        _ => None,
    }
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
