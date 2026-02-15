mod frame;
mod format;
mod parse;
mod rpc;
mod types;
mod consts;

pub use frame::{detect_swp_init_len, parse_frames, read_frame, write_frame, Frame};
pub use format::{encode_rpc, encode_varuint, encode_with_len, encode_with_len_u8};
pub use parse::{
    scan_len_prefixed_strings, scan_prefixed_uuids, take_str8, take_u16_be, take_u32_be, take_u32_le,
    take_u64_be, take_u64_le, take_uuid16,
};
pub use rpc::{
    decode_rpc_method, encode_agent_version, encode_close, encode_cluster_context, encode_cluster_scoped,
    encode_cluster_scoped_object, encode_infobase_context, encode_service_negotiation, init_packet,
};
pub use types::{format_uuid, parse_uuid, uuid_from_slice, WireError};
pub use consts::*;

pub type Result<T> = std::result::Result<T, WireError>;
