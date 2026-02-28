use crate::Uuid16;
use crate::error::RacError;
use crate::protocol::ProtocolVersion;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::rac_wire::encode_with_len_u8;

pub const METHOD_SERVICE_SETTING_INFO_REQ: u8 = 0x89;
pub const METHOD_SERVICE_SETTING_INFO_RESP: u8 = 0x8a;
pub const METHOD_SERVICE_SETTING_LIST_REQ: u8 = 0x8b;
pub const METHOD_SERVICE_SETTING_LIST_RESP: u8 = 0x8c;
pub const METHOD_SERVICE_SETTING_INSERT_REQ: u8 = 0x8d;
pub const METHOD_SERVICE_SETTING_INSERT_RESP: u8 = 0x8e;
pub const METHOD_SERVICE_SETTING_UPDATE_REQ: u8 = 0x8d;
pub const METHOD_SERVICE_SETTING_UPDATE_RESP: u8 = 0x8e;
pub const METHOD_SERVICE_SETTING_REMOVE_REQ: u8 = 0x8f;
pub const METHOD_SERVICE_SETTING_APPLY_REQ: u8 = 0x90;
pub const METHOD_SERVICE_SETTING_GET_DATA_DIRS_REQ: u8 = 0x91;
pub const METHOD_SERVICE_SETTING_GET_DATA_DIRS_RESP: u8 = 0x92;

#[derive(Debug, Serialize, Clone)]
pub struct ServiceSettingRecord {
    pub setting: Uuid16,
    pub service_name: String,
    pub infobase_name: String,
    pub service_data_dir: String,
    pub active: bool,
}

impl ServiceSettingRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>, _: ProtocolVersion) -> Result<Self> {
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
    pub fn decode(cursor: &mut RecordCursor<'_>, _: ProtocolVersion) -> Result<Self> {
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
    pub fn decode(cursor: &mut RecordCursor<'_>, _: ProtocolVersion) -> Result<Self> {
        let setting = cursor.take_uuid()?;
        Ok(Self {
            setting,
        })
    }
}

pub struct ServiceSettingInfoRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub setting: Uuid16,
}

impl crate::rpc::Request for ServiceSettingInfoRpc {
    type Response = ServiceSettingInfoResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_SERVICE_SETTING_INFO_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ServiceSettingInfo unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.setting);
        }
        Ok(out)
    }
}

pub struct ServiceSettingListRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
}

impl crate::rpc::Request for ServiceSettingListRpc {
    type Response = ServiceSettingListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_SERVICE_SETTING_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ServiceSettingList unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        Ok(out)
    }
}

pub struct ServiceSettingInsertRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub service_name: String,
    pub infobase_name: String,
    pub service_data_dir: String,
    pub active: u16,
}

impl crate::rpc::Request for ServiceSettingInsertRpc {
    type Response = ServiceSettingInsertResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_SERVICE_SETTING_INSERT_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ServiceSettingInsert unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.service_name.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.infobase_name.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.service_data_dir.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 2 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.service_name.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.infobase_name.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.service_data_dir.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.active.to_be_bytes());
        }
        Ok(out)
    }
}

pub struct ServiceSettingUpdateRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub setting: Uuid16,
    pub service_name: String,
    pub infobase_name: String,
    pub service_data_dir: String,
    pub active: u16,
}

impl crate::rpc::Request for ServiceSettingUpdateRpc {
    type Response = ServiceSettingUpdateResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_SERVICE_SETTING_UPDATE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ServiceSettingUpdate unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.service_name.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.infobase_name.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.service_data_dir.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 2 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.setting);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.service_name.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.infobase_name.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.service_data_dir.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.active.to_be_bytes());
        }
        Ok(out)
    }
}

pub struct ServiceSettingRemoveRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub setting: Uuid16,
}

impl crate::rpc::Request for ServiceSettingRemoveRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_SERVICE_SETTING_REMOVE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ServiceSettingRemove unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.setting);
        }
        Ok(out)
    }
}

pub struct ServiceSettingApplyRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
}

impl crate::rpc::Request for ServiceSettingApplyRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_SERVICE_SETTING_APPLY_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ServiceSettingApply unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        Ok(out)
    }
}

pub struct ServiceSettingGetDataDirsRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub service_name: String,
}

impl crate::rpc::Request for ServiceSettingGetDataDirsRpc {
    type Response = ServiceSettingGetDataDirsResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_SERVICE_SETTING_GET_DATA_DIRS_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ServiceSettingGetDataDirs unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.service_name.len() } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.service_name.as_bytes())?);
        }
        Ok(out)
    }
}


#[derive(Debug, Serialize)]
pub struct ServiceSettingListResp {
    pub records: Vec<ServiceSettingRecord>,
}

impl crate::rpc::Response for ServiceSettingListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        Ok(Self {
            records: crate::commands::parse_list_u8(body, |cursor| ServiceSettingRecord::decode(cursor, protocol_version))?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ServiceSettingInfoResp {
    pub record: ServiceSettingRecord,
}

impl crate::rpc::Response for ServiceSettingInfoResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        let record = parse_service_setting_info_body(body, protocol_version)?;
        Ok(Self {
            record: record,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ServiceSettingInsertResp {
    pub setting: Uuid16,
}

impl crate::rpc::Response for ServiceSettingInsertResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        let record = parse_service_setting_insert_body(body, protocol_version)?;
        Ok(Self {
            setting: record.setting,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ServiceSettingUpdateResp {
    pub setting: Uuid16,
}

impl crate::rpc::Response for ServiceSettingUpdateResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        let record = parse_service_setting_update_body(body, protocol_version)?;
        Ok(Self {
            setting: record.setting,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ServiceSettingGetDataDirsResp {
    pub records: Vec<ServiceSettingTransferDataDirRecord>,
}

impl crate::rpc::Response for ServiceSettingGetDataDirsResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        Ok(Self {
            records: crate::commands::parse_list_u8(body, |cursor| ServiceSettingTransferDataDirRecord::decode(cursor, protocol_version))?,
        })
    }
}


pub fn parse_service_setting_info_body(body: &[u8], protocol_version: ProtocolVersion) -> Result<ServiceSettingRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("service setting info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    ServiceSettingRecord::decode(&mut cursor, protocol_version)
}

pub fn parse_service_setting_insert_body(body: &[u8], protocol_version: ProtocolVersion) -> Result<ServiceSettingIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("service setting insert empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    ServiceSettingIdRecord::decode(&mut cursor, protocol_version)
}

pub fn parse_service_setting_update_body(body: &[u8], protocol_version: ProtocolVersion) -> Result<ServiceSettingIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("service setting update empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    ServiceSettingIdRecord::decode(&mut cursor, protocol_version)
}


pub const RPC_SERVICE_SETTING_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_SERVICE_SETTING_INFO_REQ,
    method_resp: Some(METHOD_SERVICE_SETTING_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_SERVICE_SETTING_LIST_REQ,
    method_resp: Some(METHOD_SERVICE_SETTING_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_INSERT_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_SERVICE_SETTING_INSERT_REQ,
    method_resp: Some(METHOD_SERVICE_SETTING_INSERT_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_UPDATE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_SERVICE_SETTING_UPDATE_REQ,
    method_resp: Some(METHOD_SERVICE_SETTING_UPDATE_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_REMOVE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_SERVICE_SETTING_REMOVE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_APPLY_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_SERVICE_SETTING_APPLY_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_SERVICE_SETTING_GET_DATA_DIRS_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_SERVICE_SETTING_GET_DATA_DIRS_REQ,
    method_resp: Some(METHOD_SERVICE_SETTING_GET_DATA_DIRS_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


