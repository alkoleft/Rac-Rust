use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::metadata::RpcMethodMeta;
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

pub const RPC_SERVICE_SETTING_INFO_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SERVICE_SETTING_INFO_REQ,
    method_resp: Some(crate::rac_wire::METHOD_SERVICE_SETTING_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SERVICE_SETTING_LIST_REQ,
    method_resp: Some(crate::rac_wire::METHOD_SERVICE_SETTING_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_INSERT_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SERVICE_SETTING_INSERT_REQ,
    method_resp: Some(crate::rac_wire::METHOD_SERVICE_SETTING_INSERT_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_UPDATE_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SERVICE_SETTING_UPDATE_REQ,
    method_resp: Some(crate::rac_wire::METHOD_SERVICE_SETTING_UPDATE_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_REMOVE_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SERVICE_SETTING_REMOVE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_APPLY_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SERVICE_SETTING_APPLY_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_GET_DATA_DIRS_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_SERVICE_SETTING_GET_DATA_DIRS_REQ,
    method_resp: Some(crate::rac_wire::METHOD_SERVICE_SETTING_GET_DATA_DIRS_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


pub fn parse_service_setting_info_body(body: &[u8]) -> Result<ServiceSettingRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("service setting info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    ServiceSettingRecord::decode(&mut cursor)
}

pub fn parse_service_setting_insert_body(body: &[u8]) -> Result<ServiceSettingIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("service setting insert empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    ServiceSettingIdRecord::decode(&mut cursor)
}

pub fn parse_service_setting_update_body(body: &[u8]) -> Result<ServiceSettingIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("service setting update empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    ServiceSettingIdRecord::decode(&mut cursor)
}

#[derive(Debug, Clone)]
pub struct ServiceSettingInfoRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub setting: Uuid16,
}

impl ServiceSettingInfoRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        out.extend_from_slice(&self.setting);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServiceSettingListRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
}

impl ServiceSettingListRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServiceSettingInsertRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub service_name: String,
    pub infobase_name: String,
    pub service_data_dir: String,
    pub active: u16,
}

impl ServiceSettingInsertRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16 + 16 + 1 + self.service_name.len() + 1 + self.infobase_name.len() + 1 + self.service_data_dir.len() + 2
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        out.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        out.extend_from_slice(&encode_with_len_u8(self.service_name.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.infobase_name.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.service_data_dir.as_bytes())?);
        out.extend_from_slice(&self.active.to_be_bytes());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServiceSettingUpdateRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub setting: Uuid16,
    pub service_name: String,
    pub infobase_name: String,
    pub service_data_dir: String,
    pub active: u16,
}

impl ServiceSettingUpdateRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16 + 16 + 1 + self.service_name.len() + 1 + self.infobase_name.len() + 1 + self.service_data_dir.len() + 2
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        out.extend_from_slice(&self.setting);
        out.extend_from_slice(&encode_with_len_u8(self.service_name.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.infobase_name.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.service_data_dir.as_bytes())?);
        out.extend_from_slice(&self.active.to_be_bytes());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServiceSettingRemoveRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub setting: Uuid16,
}

impl ServiceSettingRemoveRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        out.extend_from_slice(&self.setting);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServiceSettingApplyRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
}

impl ServiceSettingApplyRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ServiceSettingGetDataDirsRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub service_name: String,
}

impl ServiceSettingGetDataDirsRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16 + 1 + self.service_name.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        out.extend_from_slice(&encode_with_len_u8(self.service_name.as_bytes())?);
        Ok(())
    }
}




