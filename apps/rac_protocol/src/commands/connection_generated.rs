use crate::Uuid16;
use crate::codec::v8_datetime_to_iso;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ConnectionRecord {
    pub connection: Uuid16,
    pub application: String,
    pub blocked_by_ls: u32,
    pub connected_at: String,
    pub conn_id: u32,
    pub host: String,
    pub infobase: Uuid16,
    pub process: Uuid16,
    pub session_number: u32,
}

impl ConnectionRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let connection = cursor.take_uuid()?;
        let application = cursor.take_str8()?;
        let blocked_by_ls = cursor.take_u32_be()?;
        let connected_at = v8_datetime_to_iso(cursor.take_u64_be()?).unwrap_or_default();
        let conn_id = cursor.take_u32_be()?;
        let host = cursor.take_str8()?;
        let infobase = cursor.take_uuid()?;
        let process = cursor.take_uuid()?;
        let session_number = cursor.take_u32_be()?;
        Ok(Self {
            connection,
            application,
            blocked_by_ls,
            connected_at,
            conn_id,
            host,
            infobase,
            process,
            session_number,
        })
    }
}
