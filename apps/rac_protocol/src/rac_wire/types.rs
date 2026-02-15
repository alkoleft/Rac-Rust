use std::fmt;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum WireError {
    InvalidData(&'static str),
    InvalidHex(String),
    Truncated(&'static str),
}

impl fmt::Display for WireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WireError::InvalidData(msg) => write!(f, "{msg}"),
            WireError::InvalidHex(msg) => write!(f, "{msg}"),
            WireError::Truncated(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for WireError {}

pub fn format_uuid(bytes: &[u8; 16]) -> String {
    Uuid::from_bytes(*bytes).to_string()
}

pub fn uuid_from_slice(slice: &[u8]) -> Result<[u8; 16], WireError> {
    if slice.len() < 16 {
        return Err(WireError::Truncated("uuid slice too short"));
    }
    let mut out = [0u8; 16];
    out.copy_from_slice(&slice[..16]);
    Ok(out)
}

pub fn parse_uuid(input: &str) -> Result<[u8; 16], WireError> {
    let uuid = Uuid::parse_str(input)
        .map_err(|_| WireError::InvalidHex(format!("invalid uuid: {input}")))?;
    Ok(*uuid.as_bytes())
}
