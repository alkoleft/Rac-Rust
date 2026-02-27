use crate::client::RacClient;
use crate::error::Result;
use crate::Uuid16;

mod generated {
    include!("profile_generated.rs");
}

pub use generated::{ProfileListResp, ProfileListRpc, ProfileRecord, ProfileUpdateRpc};

use crate::rpc::AckResponse;

pub fn profile_list(client: &mut RacClient, cluster: Uuid16) -> Result<ProfileListResp> {
    client.call_typed(ProfileListRpc { cluster })
}

pub fn profile_update(client: &mut RacClient, req: ProfileUpdateRpc) -> Result<AckResponse> {
    client.call_typed(req)
}
