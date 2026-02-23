use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::metadata::RpcMethodMeta;
use crate::rac_wire::encode_with_len_u8;

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

#[derive(Debug, Serialize, Clone)]
pub struct RuleIdRecord {
    pub rule: Uuid16,
}

impl RuleIdRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let rule = cursor.take_uuid()?;
        Ok(Self {
            rule,
        })
    }
}

pub const RPC_RULE_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_RULE_LIST_REQ,
    method_resp: Some(crate::rac_wire::METHOD_RULE_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_INFO_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_RULE_INFO_REQ,
    method_resp: Some(crate::rac_wire::METHOD_RULE_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_APPLY_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_RULE_APPLY_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_REMOVE_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_RULE_REMOVE_REQ,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_INSERT_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_RULE_INSERT_REQ,
    method_resp: Some(crate::rac_wire::METHOD_RULE_INSERT_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_UPDATE_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_RULE_UPDATE_REQ,
    method_resp: Some(crate::rac_wire::METHOD_RULE_UPDATE_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


pub fn parse_rule_info_body(body: &[u8]) -> Result<RuleRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("rule info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    RuleRecord::decode(&mut cursor)
}

pub fn parse_rule_insert_body(body: &[u8]) -> Result<RuleIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("rule insert empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    RuleIdRecord::decode(&mut cursor)
}

pub fn parse_rule_update_body(body: &[u8]) -> Result<RuleIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("rule update empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    RuleIdRecord::decode(&mut cursor)
}

#[derive(Debug, Clone)]
pub struct RuleListRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
}

impl RuleListRequest {
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
pub struct RuleInfoRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub rule: Uuid16,
}

impl RuleInfoRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        out.extend_from_slice(&self.rule);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RuleApplyRequest {
    pub cluster: Uuid16,
    pub mode: u32,
}

impl RuleApplyRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 4
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.mode.to_be_bytes());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RuleRemoveRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub rule: Uuid16,
}

impl RuleRemoveRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        out.extend_from_slice(&self.rule);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RuleInsertRequest {
    pub cluster: Uuid16,
    pub server: Uuid16,
    pub position: u32,
    pub object_type: u32,
    pub infobase_name: String,
    pub rule_type: u8,
    pub application_ext: String,
    pub priority: u32,
}

impl RuleInsertRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16 + 16 + 4 + 4 + 1 + self.infobase_name.len() + 1 + 1 + self.application_ext.len() + 4
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        out.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        out.extend_from_slice(&self.position.to_be_bytes());
        out.extend_from_slice(&self.object_type.to_be_bytes());
        out.extend_from_slice(&encode_with_len_u8(self.infobase_name.as_bytes())?);
        out.push(self.rule_type);
        out.extend_from_slice(&encode_with_len_u8(self.application_ext.as_bytes())?);
        out.extend_from_slice(&self.priority.to_be_bytes());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RuleUpdateRequest {
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

impl RuleUpdateRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16 + 16 + 4 + 4 + 1 + self.infobase_name.len() + 1 + 1 + self.application_ext.len() + 4
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        out.extend_from_slice(&self.rule);
        out.extend_from_slice(&self.position.to_be_bytes());
        out.extend_from_slice(&self.object_type.to_be_bytes());
        out.extend_from_slice(&encode_with_len_u8(self.infobase_name.as_bytes())?);
        out.push(self.rule_type);
        out.extend_from_slice(&encode_with_len_u8(self.application_ext.as_bytes())?);
        out.extend_from_slice(&self.priority.to_be_bytes());
        Ok(())
    }
}


