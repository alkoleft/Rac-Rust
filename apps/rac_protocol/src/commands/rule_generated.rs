use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
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
        let mut out = Vec::with_capacity(16 + 4);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.mode.to_be_bytes());
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
        let mut out = Vec::with_capacity(16 + 16 + 16);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.server);
        out.extend_from_slice(&self.rule);
        Ok(out)
    }
}



pub fn parse_rule_info_body(body: &[u8]) -> Result<RuleRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("rule info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    RuleRecord::decode(&mut cursor)
}

pub fn parse_rule_insert_body(body: &[u8]) -> Result<RuleIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("rule insert empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    RuleIdRecord::decode(&mut cursor)
}

pub fn parse_rule_update_body(body: &[u8]) -> Result<RuleIdRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("rule update empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    RuleIdRecord::decode(&mut cursor)
}


pub const RPC_RULE_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x55,
    method_resp: Some(0x56),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x57,
    method_resp: Some(0x58),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_APPLY_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x51,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_REMOVE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x54,
    method_resp: None,
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_INSERT_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x52,
    method_resp: Some(0x53),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_RULE_UPDATE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: 0x52,
    method_resp: Some(0x53),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


