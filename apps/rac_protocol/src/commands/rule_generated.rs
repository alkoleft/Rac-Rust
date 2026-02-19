use crate::codec::RecordCursor;
use crate::error::Result;
use crate::Uuid16;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct RuleRecord {
    pub rule: Uuid16,
    pub object_type: u32,
    pub infobase_name: String,
    pub rule_type: u8,
    pub application_ext: String,
    pub priority: u32,
}

impl RuleRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let rule = cursor.take_uuid()?;
        let object_type = cursor.take_u32_be()?;
        let infobase_name = cursor.take_str8()?;
        let rule_type = cursor.take_u8()?;
        let application_ext = cursor.take_str8()?;
        let priority = cursor.take_u32_be()?;
        Ok(Self {
            rule,
            object_type,
            infobase_name,
            rule_type,
            application_ext,
            priority,
        })
    }
}
