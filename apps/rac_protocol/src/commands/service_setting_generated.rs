use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::rac_wire::encode_with_len_u8;

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

#[derive(Debug, Serialize, Clone)]
pub struct ServiceSettingIdRecord {
    pub setting: Uuid16,
}

impl ServiceSettingIdRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let setting = cursor.take_uuid()?;
        Ok(Self {
            setting,
        })
    }
}



pub fn parse_service_setting_info_body(body: &[u8]) -> Result<ServiceSettingRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("service setting info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    ServiceSettingRecord::decode(&mut cursor)
}

pub fn parse_service_setting_insert_body(body: &[u8]) -> Result<ServiceSettingIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("service setting insert empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    ServiceSettingIdRecord::decode(&mut cursor)
}

pub fn parse_service_setting_update_body(body: &[u8]) -> Result<ServiceSettingIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("service setting update empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    ServiceSettingIdRecord::decode(&mut cursor)
}


pub const RPC_SERVICE_SETTING_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x89,
    method_resp: Some(0x8a),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x8b,
    method_resp: Some(0x8c),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_INSERT_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x8d,
    method_resp: Some(0x8e),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_UPDATE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x8d,
    method_resp: Some(0x8e),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_REMOVE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x8f,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_APPLY_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x90,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_GET_DATA_DIRS_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x91,
    method_resp: Some(0x92),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


