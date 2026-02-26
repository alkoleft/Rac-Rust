use serde::Serialize;

use crate::client::RacClient;
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::protocol::ProtocolCodec;
use crate::rpc::{Meta, Request, Response};
use crate::rpc::decode_utils::rpc_body;
use crate::rac_wire::{METHOD_PROFILE_LIST_REQ, METHOD_PROFILE_LIST_RESP};
use crate::Uuid16;


#[derive(Debug, Serialize)]
pub struct ProfileListResp {
    pub profiles: Vec<Uuid16>,
}

struct ProfileListRpc {
    cluster: Uuid16,
}

impl Request for ProfileListRpc {
    type Response = ProfileListResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_PROFILE_LIST_REQ,
            method_resp: Some(METHOD_PROFILE_LIST_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        Ok(self.cluster.to_vec())
    }
}

impl Response for ProfileListResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let profiles = parse_profile_list(body)?;
        Ok(Self { profiles })
    }
}

pub fn profile_list(client: &mut RacClient, cluster: Uuid16) -> Result<ProfileListResp> {
    client.call_typed(ProfileListRpc { cluster })
}

fn parse_profile_list(body: &[u8]) -> Result<Vec<Uuid16>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body);
    let count = cursor.take_u8()? as usize;
    let mut profiles = Vec::with_capacity(count);
    for _ in 0..count {
        profiles.push(cursor.take_uuid()?);
    }
    Ok(profiles)
}
