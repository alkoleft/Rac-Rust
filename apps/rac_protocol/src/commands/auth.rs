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
        (Some(user), None) => Ok(AuthPair { user, pwd: "" }),
        (None, None) => Ok(AuthPair { user: "", pwd: "" }),
        (None, Some(_)) => Err(RacError::Unsupported(missing_msg)),
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
        let detail =
            parse_error_message(body).or_else(|| Some(format!("rpc error method=0x{method:02x}")));
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
    if body.is_empty() {
        return None;
    }
    parse_error_envelope(body).or_else(|| parse_message_variants(body))
}

fn parse_error_envelope(body: &[u8]) -> Option<String> {
    let mut cursor = RecordCursor::new(body, 0);
    cursor.take_str8_opt().ok()??;
    parse_message_variants(cursor.remaining_slice())
}

fn parse_message_variants(body: &[u8]) -> Option<String> {
    parse_str8(body)
        .or_else(|| parse_str8_opt(body))
        .or_else(|| parse_str16_be(body))
        .or_else(|| parse_str16_le(body))
        .or_else(|| parse_str32_be(body))
        .or_else(|| parse_code_and_str8(body))
        .or_else(|| parse_code_and_str16_be(body))
        .or_else(|| parse_code_and_str16_le(body))
        .or_else(|| parse_code_and_str32_be(body))
        .or_else(|| parse_code_and_rest_utf8(body))
        .or_else(|| decode_message_from_bytes(body))
}

fn parse_str8(body: &[u8]) -> Option<String> {
    let mut cursor = RecordCursor::new(body, 0);
    let value = cursor.take_str8().ok()?;
    normalize_message(value)
}

fn parse_str8_opt(body: &[u8]) -> Option<String> {
    let mut cursor = RecordCursor::new(body, 0);
    let value = cursor.take_str8_opt().ok()??;
    normalize_message(value)
}

fn parse_str16_be(body: &[u8]) -> Option<String> {
    parse_len_prefixed(body, |cursor| {
        cursor.take_u16_be().ok().map(|value| value as usize)
    })
}

fn parse_str16_le(body: &[u8]) -> Option<String> {
    parse_len_prefixed(body, |cursor| {
        cursor.take_u16_le().ok().map(|value| value as usize)
    })
}

fn parse_str32_be(body: &[u8]) -> Option<String> {
    parse_len_prefixed(body, |cursor| {
        cursor
            .take_u32_be()
            .ok()
            .and_then(|value| usize::try_from(value).ok())
    })
}

fn parse_code_and_str8(body: &[u8]) -> Option<String> {
    let mut cursor = RecordCursor::new(body, 0);
    cursor.take_u32_be().ok()?;
    let value = cursor.take_str8().ok()?;
    normalize_message(value)
}

fn parse_code_and_str16_be(body: &[u8]) -> Option<String> {
    parse_code_and_len_prefixed(body, |cursor| {
        cursor.take_u16_be().ok().map(|value| value as usize)
    })
}

fn parse_code_and_str16_le(body: &[u8]) -> Option<String> {
    parse_code_and_len_prefixed(body, |cursor| {
        cursor.take_u16_le().ok().map(|value| value as usize)
    })
}

fn parse_code_and_str32_be(body: &[u8]) -> Option<String> {
    parse_code_and_len_prefixed(body, |cursor| {
        cursor
            .take_u32_be()
            .ok()
            .and_then(|value| usize::try_from(value).ok())
    })
}

fn parse_code_and_rest_utf8(body: &[u8]) -> Option<String> {
    let mut cursor = RecordCursor::new(body, 0);
    cursor.take_u32_be().ok()?;
    decode_message_from_bytes(cursor.remaining_slice())
}

fn parse_len_prefixed<F>(body: &[u8], read_len: F) -> Option<String>
where
    F: FnOnce(&mut RecordCursor<'_>) -> Option<usize>,
{
    let mut cursor = RecordCursor::new(body, 0);
    let len = read_len(&mut cursor)?;
    if cursor.remaining_len() < len {
        return None;
    }
    let bytes = cursor.take_bytes(len).ok()?;
    decode_message_from_bytes(&bytes)
}

fn parse_code_and_len_prefixed<F>(body: &[u8], read_len: F) -> Option<String>
where
    F: FnOnce(&mut RecordCursor<'_>) -> Option<usize>,
{
    let mut cursor = RecordCursor::new(body, 0);
    cursor.take_u32_be().ok()?;
    let len = read_len(&mut cursor)?;
    if cursor.remaining_len() < len {
        return None;
    }
    let bytes = cursor.take_bytes(len).ok()?;
    decode_message_from_bytes(&bytes)
}

fn decode_message_from_bytes(bytes: &[u8]) -> Option<String> {
    let value = std::str::from_utf8(bytes).ok()?;
    normalize_message(value.to_string())
}

fn normalize_message(value: String) -> Option<String> {
    let trimmed = value.trim_end_matches('\0');
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed.to_string())
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
