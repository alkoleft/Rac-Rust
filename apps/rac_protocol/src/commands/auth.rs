use crate::client::RacClient;
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::agent::AgentAuthRpc;
use super::cluster::ClusterAuthRpc;

pub struct AuthPair<'a> {
    pub user: &'a str,
    pub pwd: &'a str,
}

struct AuthReply {
    acknowledged: bool,
    detail: Option<String>,
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
    let reply = client.call(AgentAuthRpc {
        user: creds.user.to_string(),
        pwd: creds.pwd.to_string(),
    })?;
    let auth_reply = decode_auth_reply(&reply);
    if !auth_reply.acknowledged {
        let detail = auth_reply
            .detail
            .unwrap_or_else(|| format!("payload_hex={}", payload_hex(&reply, 96)));
        return Err(RacError::ProtocolMessage(format!(
            "agent auth rejected: {detail}"
        )));
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
    let reply = client.call(ClusterAuthRpc {
        cluster,
        user: creds.user.to_string(),
        pwd: creds.pwd.to_string(),
    })?;
    let auth_reply = decode_auth_reply(&reply);
    if !auth_reply.acknowledged {
        let detail = auth_reply
            .detail
            .unwrap_or_else(|| format!("payload_hex={}", payload_hex(&reply, 96)));
        return Err(RacError::ProtocolMessage(format!(
            "cluster auth rejected: {detail}"
        )));
    }
    Ok(creds)
}

fn decode_auth_reply(payload: &[u8]) -> AuthReply {
    let mut cursor = RecordCursor::new(payload, 0);
    if cursor.remaining_len() < 4 {
        return AuthReply {
            acknowledged: false,
            detail: Some("auth reply truncated".to_string()),
        };
    }

    let head = match cursor.take_bytes(4) {
        Ok(bytes) => bytes,
        Err(_) => {
            return AuthReply {
                acknowledged: false,
                detail: Some("auth reply truncated".to_string()),
            }
        }
    };

    if head == [0x01, 0x00, 0x00, 0x01] {
        let method = match cursor.take_u8() {
            Ok(value) => value,
            Err(_) => {
                return AuthReply {
                    acknowledged: false,
                    detail: Some("auth reply missing rpc method".to_string()),
                }
            }
        };
        let body = cursor.remaining_slice();
        let detail = parse_error_message(body)
            .map(|msg| format!("rpc error method=0x{method:02x}: {msg}"))
            .or_else(|| Some(format!("rpc error method=0x{method:02x}")));
        return AuthReply {
            acknowledged: false,
            detail,
        };
    }

    let acknowledged = head == [0x01, 0x00, 0x00, 0x00];
    let detail = if acknowledged {
        None
    } else {
        parse_error_message(cursor.remaining_slice())
    };
    AuthReply {
        acknowledged,
        detail,
    }
}

fn parse_error_message(body: &[u8]) -> Option<String> {
    let mut cursor = RecordCursor::new(body, 0);
    let len = cursor.take_u8().ok()? as usize;
    if cursor.remaining_len() < len {
        return None;
    }
    let bytes = cursor.take_bytes(len).ok()?;
    String::from_utf8(bytes.to_vec()).ok()
}

fn payload_hex(payload: &[u8], max_len: usize) -> String {
    let take = payload.len().min(max_len);
    let mut out = String::with_capacity(take * 2 + 4);
    for (idx, byte) in payload.iter().take(take).enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(&format!("{byte:02x}"));
    }
    if payload.len() > take {
        out.push_str(" ...");
    }
    out
}
