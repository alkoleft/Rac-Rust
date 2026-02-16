mod consts;
mod format;
mod frame;
mod parse;
mod rpc_header;
mod types;

pub use consts::*;
pub use format::{encode_rpc, encode_varuint, encode_with_len, encode_with_len_u8};
pub use frame::{detect_swp_init_len, parse_frames, read_frame, write_frame, Frame};
pub use parse::{
    scan_len_prefixed_strings, scan_prefixed_uuids, take_str8, take_u16_be, take_u32_be,
    take_u32_le, take_u64_be, take_u64_le, take_uuid16,
};
pub use rpc_header::decode_rpc_method;
pub use types::{format_uuid, parse_uuid, uuid_from_slice, WireError};

pub type Result<T> = std::result::Result<T, WireError>;
