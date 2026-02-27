use crate::Uuid16;
use crate::error::RacError;
use crate::protocol::ProtocolVersion;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::rac_wire::encode_with_len_u8;

pub const METHOD_RULE_LIST_REQ: u8 = 0x55;
pub const METHOD_RULE_LIST_RESP: u8 = 0x56;
pub const METHOD_RULE_INFO_REQ: u8 = 0x57;
pub const METHOD_RULE_INFO_RESP: u8 = 0x58;
pub const METHOD_RULE_APPLY_REQ: u8 = 0x51;
pub const METHOD_RULE_REMOVE_REQ: u8 = 0x54;
pub const METHOD_RULE_INSERT_REQ: u8 = 0x52;
pub const METHOD_RULE_INSERT_RESP: u8 = 0x53;
pub const METHOD_RULE_UPDATE_REQ: u8 = 0x52;
pub const METHOD_RULE_UPDATE_RESP: u8 = 0x53;

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
    pub fn decode(cursor: &mut RecordCursor<'_>, protocol_version: ProtocolVersion) -> Result<Self> {
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

#[derive(Debug, Serialize, Clone)]
pub struct RuleIdRecord {
    pub rule: Uuid16,
}

impl RuleIdRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>, protocol_version: ProtocolVersion) -> Result<Self> {
        let rule = cursor.take_uuid()?;
        Ok(Self {
            rule,
        })
    }
}

pub struct RuleListRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
}

impl crate::rpc::Request for RuleListRpc {
    type Response = RuleListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_RULE_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !protocol_version >= ProtocolVersion::V11_0 {
            return Err(RacError::Unsupported("rpc RuleList unsupported for protocol"));
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

pub struct RuleInfoRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub rule: Uuid16,
}

impl crate::rpc::Request for RuleInfoRpc {
    type Response = RuleInfoResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_RULE_INFO_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !protocol_version >= ProtocolVersion::V11_0 {
            return Err(RacError::Unsupported("rpc RuleInfo unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.rule);
        }
        Ok(out)
    }
}

pub struct RuleApplyRpc {
    pub cluster: Uuid16,
    pub mode: u32,
}

impl crate::rpc::Request for RuleApplyRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_RULE_APPLY_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !protocol_version >= ProtocolVersion::V11_0 {
            return Err(RacError::Unsupported("rpc RuleApply unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 4 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.mode.to_be_bytes());
        }
        Ok(out)
    }
}

pub struct RuleRemoveRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub rule: Uuid16,
}

impl crate::rpc::Request for RuleRemoveRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_RULE_REMOVE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !protocol_version >= ProtocolVersion::V11_0 {
            return Err(RacError::Unsupported("rpc RuleRemove unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.rule);
        }
        Ok(out)
    }
}

pub struct RuleInsertRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub rule: Uuid16,
    pub position: u32,
    pub object_type: u32,
    pub infobase_name: String,
    pub rule_type: u8,
    pub application_ext: String,
    pub priority: u32,
}

impl crate::rpc::Request for RuleInsertRpc {
    type Response = RuleInsertResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_RULE_INSERT_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !protocol_version >= ProtocolVersion::V11_0 {
            return Err(RacError::Unsupported("rpc RuleInsert unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 4 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 4 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.infobase_name.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.application_ext.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 4 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.rule);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.position.to_be_bytes());
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.object_type.to_be_bytes());
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.infobase_name.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.rule_type);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.application_ext.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.priority.to_be_bytes());
        }
        Ok(out)
    }
}

pub struct RuleUpdateRpc {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub rule: Uuid16,
    pub position: u32,
    pub object_type: u32,
    pub infobase_name: String,
    pub rule_type: u8,
    pub application_ext: String,
    pub priority: u32,
}

impl crate::rpc::Request for RuleUpdateRpc {
    type Response = RuleUpdateResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_RULE_UPDATE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !protocol_version >= ProtocolVersion::V11_0 {
            return Err(RacError::Unsupported("rpc RuleUpdate unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 4 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 4 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.infobase_name.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.application_ext.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 4 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.server);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.rule);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.position.to_be_bytes());
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.object_type.to_be_bytes());
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.infobase_name.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.rule_type);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.application_ext.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.priority.to_be_bytes());
        }
        Ok(out)
    }
}


#[derive(Debug, Serialize)]
pub struct RuleListResp {
    pub records: Vec<RuleRecord>,
}

impl crate::rpc::Response for RuleListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        Ok(Self {
            records: crate::commands::parse_list_u8(body, |cursor| RuleRecord::decode(cursor, protocol_version))?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct RuleInfoResp {
    pub record: RuleRecord,
}

impl crate::rpc::Response for RuleInfoResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        let record = parse_rule_info_body(body, protocol_version)?;
        Ok(Self {
            record: record,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct RuleInsertResp {
    pub rule: Uuid16,
}

impl crate::rpc::Response for RuleInsertResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        let record = parse_rule_insert_body(body, protocol_version)?;
        Ok(Self {
            rule: record.rule,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct RuleUpdateResp {
    pub rule: Uuid16,
}

impl crate::rpc::Response for RuleUpdateResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        let record = parse_rule_update_body(body, protocol_version)?;
        Ok(Self {
            rule: record.rule,
        })
    }
}


pub fn parse_rule_info_body(body: &[u8], protocol_version: ProtocolVersion) -> Result<RuleRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("rule info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    RuleRecord::decode(&mut cursor, protocol_version)
}

pub fn parse_rule_insert_body(body: &[u8], protocol_version: ProtocolVersion) -> Result<RuleIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("rule insert empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    RuleIdRecord::decode(&mut cursor, protocol_version)
}

pub fn parse_rule_update_body(body: &[u8], protocol_version: ProtocolVersion) -> Result<RuleIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("rule update empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    RuleIdRecord::decode(&mut cursor, protocol_version)
}


pub const RPC_RULE_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_RULE_LIST_REQ,
    method_resp: Some(METHOD_RULE_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_RULE_INFO_REQ,
    method_resp: Some(METHOD_RULE_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_APPLY_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_RULE_APPLY_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_REMOVE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_RULE_REMOVE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_INSERT_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_RULE_INSERT_REQ,
    method_resp: Some(METHOD_RULE_INSERT_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_UPDATE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_RULE_UPDATE_REQ,
    method_resp: Some(METHOD_RULE_UPDATE_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


