pub mod client;
pub mod codec;
pub mod commands;
pub mod error;
pub mod rac_wire;
#[cfg(feature = "rest")]
pub mod rest;

pub type Uuid16 = [u8; 16];
