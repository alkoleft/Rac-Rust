use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::Uuid16;

use super::rpc_body;

#[derive(Debug, Serialize)]
pub struct ProfileListResp {
    pub profiles: Vec<Uuid16>,
}

pub fn profile_list(client: &mut RacClient, cluster: Uuid16) -> Result<ProfileListResp> {
    let reply = client.call(RacRequest::ProfileList { cluster })?;
    let body = rpc_body(&reply)?;
    let profiles = parse_profile_list(body)?;
    Ok(ProfileListResp {
        profiles,
    })
}

fn parse_profile_list(body: &[u8]) -> Result<Vec<Uuid16>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut profiles = Vec::with_capacity(count);
    for _ in 0..count {
        profiles.push(cursor.take_uuid()?);
    }
    Ok(profiles)
}
