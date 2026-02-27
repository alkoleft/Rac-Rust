use serde::Serialize;

use crate::client::RacClient;
use crate::error::Result;
use crate::protocol::ProtocolCodec;
use crate::rpc::Response;
use crate::rpc::decode_utils::rpc_body;
use crate::Uuid16;

mod generated {
    include!("profile_generated.rs");
    pub type ProfileListResp = super::ProfileListResp;
}

pub use generated::{ProfileListRpc, ProfileRecord};

#[derive(Debug, Serialize)]
pub struct ProfileListResp {
    pub profiles: Vec<Uuid16>,
}

impl Response for ProfileListResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let records = crate::commands::parse_list_u8(body, ProfileRecord::decode)?;
        let profiles = records.into_iter().map(|record| record.profile).collect();
        Ok(Self { profiles })
    }
}

pub fn profile_list(client: &mut RacClient, cluster: Uuid16) -> Result<ProfileListResp> {
    client.call_typed(ProfileListRpc { cluster })
}
