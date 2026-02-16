use std::fmt;
use std::io;

use crate::rac_wire::WireError;

#[derive(Debug)]
pub enum RacError {
    Io(io::Error),
    Wire(WireError),
    Protocol(&'static str),
    ProtocolMessage(String),
    Unsupported(&'static str),
    Decode(&'static str),
    DecodeMessage(String),
    UnexpectedMethod { got: u8, expected: u8 },
}

impl fmt::Display for RacError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RacError::Io(err) => write!(f, "io error: {err}"),
            RacError::Wire(err) => write!(f, "wire error: {err}"),
            RacError::Protocol(msg) => write!(f, "protocol error: {msg}"),
            RacError::ProtocolMessage(msg) => write!(f, "protocol error: {msg}"),
            RacError::Unsupported(msg) => write!(f, "unsupported: {msg}"),
            RacError::Decode(msg) => write!(f, "decode error: {msg}"),
            RacError::DecodeMessage(msg) => write!(f, "decode error: {msg}"),
            RacError::UnexpectedMethod { got, expected } => {
                write!(
                    f,
                    "unexpected rpc method 0x{got:02x}, expected 0x{expected:02x}"
                )
            }
        }
    }
}

impl std::error::Error for RacError {}

impl From<io::Error> for RacError {
    fn from(err: io::Error) -> Self {
        RacError::Io(err)
    }
}

impl From<WireError> for RacError {
    fn from(err: WireError) -> Self {
        RacError::Wire(err)
    }
}

pub type Result<T> = std::result::Result<T, RacError>;
