use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::Uuid16;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ClusterAdminRecord {
    pub name: String,
    pub unknown_tag: u8,
    pub unknown_flags: u32,
    pub unknown_tail: [u8; 3],
}

impl ClusterAdminRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let name = cursor.take_str8()?;
        let unknown_tag = cursor.take_u8()?;
        let unknown_flags = cursor.take_u32_be()?;
        let unknown_tail = {
            let bytes = cursor.take_bytes(3)?;
            let value: [u8; 3] = bytes.as_slice().try_into().map_err(|_| RacError::Decode("bytes_fixed"))?;
            value
        };
        Ok(Self {
            name,
            unknown_tag,
            unknown_flags,
            unknown_tail,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ClusterRecord {
    pub uuid: Uuid16,
    pub expiration_timeout: u32,
    pub host: String,
    pub port: u16,
    pub display_name: String,
}

impl ClusterRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let uuid = cursor.take_uuid()?;
        let expiration_timeout = cursor.take_u32_be()?;
        let host = cursor.take_str8()?;
        let __unknown_u32 = cursor.take_u32_be()?;
        let port = cursor.take_u16_be()?;
        let __unknown_u64 = cursor.take_u64_be()?;
        let display_name = cursor.take_str8()?;
        let __tail = cursor.take_bytes(32)?;
        Ok(Self {
            uuid,
            expiration_timeout,
            host,
            port,
            display_name,
        })
    }
}
