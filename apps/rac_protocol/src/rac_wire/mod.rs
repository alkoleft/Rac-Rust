mod consts;
mod format;
mod frame;
mod rpc_header;
mod swp;
mod types;

pub use consts::*;
pub use format::{encode_rpc, encode_varuint, encode_with_len, encode_with_len_u8};
pub use frame::{parse_frames, read_frame, write_frame, Frame};
pub use rpc_header::decode_rpc_method;
pub use swp::{parse_swp_init, SwpInit, SwpParam, SwpValue};
pub use types::{format_uuid, parse_uuid, uuid_from_slice, WireError};

pub type Result<T> = std::result::Result<T, WireError>;
