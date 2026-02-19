use crate::codec::RecordCursor;
use crate::error::Result;
use crate::Uuid16;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ManagerRecord {
    pub manager: Uuid16,
    pub descr: String,
    pub host: String,
    pub using: u32,
    pub port: u16,
    pub pid: String,
}

impl ManagerRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let manager = cursor.take_uuid()?;
        let descr = cursor.take_str8()?;
        let host = cursor.take_str8()?;
        let using = cursor.take_u32_be()?;
        let port = cursor.take_u16_be()?;
        let pid = cursor.take_str8()?;
        Ok(Self {
            manager,
            descr,
            host,
            using,
            port,
            pid,
        })
    }
}
