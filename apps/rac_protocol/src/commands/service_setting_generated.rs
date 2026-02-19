use crate::Uuid16;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ServiceSettingRecord {
    pub setting: Uuid16,
    pub service_name: String,
    pub infobase_name: String,
    pub service_data_dir: String,
    pub active: bool,
}

impl ServiceSettingRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let setting = cursor.take_uuid()?;
        let service_name = cursor.take_str8()?;
        let infobase_name = cursor.take_str8()?;
        let service_data_dir = cursor.take_str8()?;
        let active = cursor.take_u16_be()? != 0;
        Ok(Self {
            setting,
            service_name,
            infobase_name,
            service_data_dir,
            active,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ServiceSettingTransferDataDirRecord {
    pub service_name: String,
    pub user: String,
    pub source_dir_flag: u8,
    pub source_dir: String,
    pub target_dir_flag: u8,
    pub target_dir: String,
}

impl ServiceSettingTransferDataDirRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let service_name = cursor.take_str8()?;
        let user = cursor.take_str8()?;
        let _source_dir_len = cursor.take_u8()?;
        let source_dir_flag = cursor.take_u8()?;
        let source_dir = {
            let len = _source_dir_len as usize;
            let bytes = cursor.take_bytes(len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        let _target_dir_len = cursor.take_u8()?;
        let target_dir_flag = cursor.take_u8()?;
        let target_dir = {
            let len = _target_dir_len as usize;
            let bytes = cursor.take_bytes(len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        Ok(Self {
            service_name,
            user,
            source_dir_flag,
            source_dir,
            target_dir_flag,
            target_dir,
        })
    }
}
