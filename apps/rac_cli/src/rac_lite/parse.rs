use rac_protocol::commands::RuleApplyMode;
use rac_protocol::error::{RacError, Result};
use rac_protocol::rac_wire::parse_uuid;
use rac_protocol::Uuid16;

pub fn parse_uuid_arg(input: &str) -> Result<Uuid16> {
    Ok(parse_uuid(input)?)
}

pub fn parse_auth_flags(input: &str) -> Result<u8> {
    let mut flags = 0u8;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(RacError::Unsupported("auth flags are empty"));
    }
    for item in trimmed.split(',') {
        let token = item.trim();
        if token.is_empty() {
            continue;
        }
        match token {
            "pwd" => flags |= 0x01,
            "os" => flags |= 0x02,
            _ => return Err(RacError::Unsupported("unknown auth flag")),
        }
    }
    Ok(flags)
}

pub fn parse_rule_apply_mode(input: &str) -> Result<RuleApplyMode> {
    match input.trim() {
        "full" => Ok(RuleApplyMode::Full),
        "partial" => Ok(RuleApplyMode::Partial),
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
