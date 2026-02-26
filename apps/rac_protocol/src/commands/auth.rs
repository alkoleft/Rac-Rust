use crate::client::RacClient;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::agent::AgentAuthRpc;
use super::cluster::ClusterAuthRpc;

pub struct AuthPair<'a> {
    pub user: &'a str,
    pub pwd: &'a str,
}

fn resolve_auth_pair<'a>(
    user: Option<&'a str>,
    pwd: Option<&'a str>,
    missing_msg: &'static str,
) -> Result<AuthPair<'a>> {
    match (user, pwd) {
        (Some(user), Some(pwd)) => Ok(AuthPair { user, pwd }),
        (None, None) => Ok(AuthPair { user: "", pwd: "" }),
        _ => Err(RacError::Unsupported(missing_msg)),
    }
}

pub fn agent_auth_optional<'a>(
    client: &mut RacClient,
    user: Option<&'a str>,
    pwd: Option<&'a str>,
) -> Result<AuthPair<'a>> {
    let creds = resolve_auth_pair(
        user,
        pwd,
        "agent-user and agent-pwd must be provided together",
    )?;
    let resp = client.call_typed(AgentAuthRpc {
        user: creds.user.to_string(),
        pwd: creds.pwd.to_string(),
    })?;
    if !resp.acknowledged {
        return Err(RacError::Unsupported(
            "agent auth rejected (provide --agent-user/--agent-pwd)",
        ));
    }
    Ok(creds)
}

pub fn cluster_auth_optional<'a>(
    client: &mut RacClient,
    cluster: Uuid16,
    user: Option<&'a str>,
    pwd: Option<&'a str>,
) -> Result<AuthPair<'a>> {
    let creds = resolve_auth_pair(
        user,
        pwd,
        "cluster-user and cluster-pwd must be provided together",
    )?;
    let resp = client.call_typed(ClusterAuthRpc {
        cluster,
        user: creds.user.to_string(),
        pwd: creds.pwd.to_string(),
    })?;
    if !resp.acknowledged {
        return Err(RacError::Unsupported(
            "cluster auth rejected (provide --cluster-user/--cluster-pwd)",
        ));
    }
    Ok(creds)
}
