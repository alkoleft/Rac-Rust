use rac_protocol::client::RacClient;
use rac_protocol::commands::{cluster_auth, AgentAuthRpc};
use rac_protocol::error::{RacError, Result};
use rac_protocol::Uuid16;

pub struct AuthPair<'a> {
    pub user: &'a str,
    pub pwd: &'a str,
}

fn resolve_user_pwd<'a>(
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
    let creds = resolve_user_pwd(
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
    let creds = resolve_user_pwd(
        user,
        pwd,
        "cluster-user and cluster-pwd must be provided together",
    )?;
    let ok = cluster_auth(client, cluster, creds.user, creds.pwd)?;
    if !ok {
        return Err(RacError::Unsupported(
            "cluster auth rejected (provide --cluster-user/--cluster-pwd)",
        ));
    }
    Ok(creds)
}
