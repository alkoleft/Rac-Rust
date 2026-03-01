use rac_protocol::error::{RacError, Result};
use rac_protocol::rac_wire::parse_uuid;
use rac_protocol::Uuid16;

pub fn parse_uuid_arg(input: &str) -> Result<Uuid16> {
    Ok(parse_uuid(input)?)
}

pub fn parse_auth_flags(input: &str) -> Result<(u8, u8)> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(RacError::Unsupported("auth flags are empty"));
    }
    let mut has_pwd = false;
    let mut has_os = false;
    for item in trimmed.split(',') {
        let token = item.trim();
        if token.is_empty() {
            continue;
        }
        match token {
            "pwd" => has_pwd = true,
            "os" => has_os = true,
            _ => return Err(RacError::Unsupported("unknown auth flag")),
        }
    }
    if !has_pwd && !has_os {
        return Err(RacError::Unsupported("auth flags are empty"));
    }
    let auth_pwd = if has_pwd { 0x01 } else { 0x00 };
    let auth_os = if has_os { 0x01 } else { 0x00 };
    Ok((auth_pwd, auth_os))
}

pub fn parse_rule_apply_mode(input: &str) -> Result<u32> {
    match input.trim() {
        "full" => Ok(1),
        "partial" => Ok(0),
        _ => Err(RacError::Unsupported("unknown rule apply mode")),
    }
}

pub fn parse_counter_group(input: &str) -> Result<u8> {
    match input.trim() {
        "users" | "0" => Ok(0),
        "data-separation" | "1" => Ok(1),
        _ => Err(RacError::Unsupported("unknown counter group")),
    }
}

pub fn parse_counter_filter_type(input: &str) -> Result<u8> {
    match input.trim() {
        "all-selected" | "0" => Ok(0),
        "all-but-selected" | "1" => Ok(1),
        "all" | "2" => Ok(2),
        _ => Err(RacError::Unsupported("unknown counter filter-type")),
    }
}

pub fn parse_counter_analyze_flag(label: &'static str, input: &str) -> Result<u8> {
    match input.trim() {
        "analyze" | "1" => Ok(1),
        "not-analyze" | "0" => Ok(0),
        _ => Err(RacError::Unsupported(label)),
    }
}

pub fn parse_limit_action(input: &str) -> Result<u8> {
    match input.trim() {
        "none" | "0" => Ok(0),
        "set-low-priority-thread" | "1" => Ok(1),
        "interrupt-current-call" | "2" => Ok(2),
        "interrupt-session" | "3" => Ok(3),
        _ => Err(RacError::Unsupported("unknown limit action")),
    }
}
