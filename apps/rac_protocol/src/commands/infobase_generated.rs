use crate::Uuid16;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct InfobaseSummary {
    pub infobase: Uuid16,
    pub descr: String,
    pub name: String,
}

impl InfobaseSummary {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let infobase = cursor.take_uuid()?;
        let descr = {
            let first = cursor.take_u8()? as usize;
            let len = if first == 0x2c { cursor.take_u8()? as usize } else { first };
            let bytes = cursor.take_bytes(len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        let name = {
            let len = cursor.take_u8()? as usize;
            let bytes = cursor.take_bytes(len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        Ok(Self {
            infobase,
            descr,
            name,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct InfobaseFieldsRecord {
    pub infobase: Uuid16,
    pub fields: Vec<String>,
}

impl InfobaseFieldsRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let infobase = cursor.take_uuid()?;
        let fields = {
            let mut out = Vec::new();
            while cursor.remaining_len() > 0 {
                out.push(cursor.take_str8()?);
            }
            out
        };
        Ok(Self {
            infobase,
            fields,
        })
    }
}
