use serde::Serialize;

use crate::client::RacClient;
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::protocol::ProtocolCodec;
use crate::rpc::{Meta, Request, Response};
use crate::rpc::decode_utils::{parse_ack_payload, rpc_body};
use crate::rac_wire::{
    METHOD_RULE_APPLY_REQ, METHOD_RULE_INFO_REQ, METHOD_RULE_INFO_RESP, METHOD_RULE_INSERT_REQ,
    METHOD_RULE_INSERT_RESP, METHOD_RULE_LIST_REQ, METHOD_RULE_LIST_RESP, METHOD_RULE_REMOVE_REQ,
};
use crate::Uuid16;

use super::parse_uuid_body;
use crate::commands::cluster_auth;

mod generated {
    include!("rule_generated.rs");
}

pub use generated::RuleRecord;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum RuleApplyMode {
    Full,
    Partial,
}

impl RuleApplyMode {
    pub fn as_u32(self) -> u32 {
        match self {
            // Captures show apply_mode = 1 for "full"; 0 is assumed for "partial".
            RuleApplyMode::Full => 1,
            RuleApplyMode::Partial => 0,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RuleApplyResp {
    pub acknowledged: bool,
}

#[derive(Debug, Serialize)]
pub struct RuleRemoveResp {
    pub acknowledged: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct RuleInsertReq {
    pub server: Uuid16,
    pub position: u32,
    pub object_type: u32,
    pub infobase_name: String,
    pub rule_type: u8,
    pub application_ext: String,
    pub priority: u32,
}

#[derive(Debug, Serialize)]
pub struct RuleInsertResp {
    pub rule: Uuid16,
}

#[derive(Debug, Serialize, Clone)]
pub struct RuleUpdateReq {
    pub server: Uuid16,
    pub rule: Uuid16,
    pub position: u32,
    pub object_type: u32,
    pub infobase_name: String,
    pub rule_type: u8,
    pub application_ext: String,
    pub priority: u32,
}

#[derive(Debug, Serialize)]
pub struct RuleUpdateResp {
    pub rule: Uuid16,
}

#[derive(Debug, Serialize)]
pub struct RuleListResp {
    pub records: Vec<RuleRecord>,
}

#[derive(Debug, Serialize)]
pub struct RuleInfoResp {
    pub record: RuleRecord,
}

struct RuleListRpc {
    cluster: Uuid16,
    server: Uuid16,
}

impl Request for RuleListRpc {
    type Response = RuleListResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_RULE_LIST_REQ,
            method_resp: Some(METHOD_RULE_LIST_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut body = Vec::with_capacity(32);
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&self.server);
        Ok(body)
    }
}

impl Response for RuleListResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let records = parse_rule_list_records(body)?;
        Ok(Self { records })
    }
}

struct RuleInfoRpc {
    cluster: Uuid16,
    server: Uuid16,
    rule: Uuid16,
}

impl Request for RuleInfoRpc {
    type Response = RuleInfoResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_RULE_INFO_REQ,
            method_resp: Some(METHOD_RULE_INFO_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut body = Vec::with_capacity(48);
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&self.server);
        body.extend_from_slice(&self.rule);
        Ok(body)
    }
}

impl Response for RuleInfoResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let record = parse_rule_info_body(body)?;
        Ok(Self { record })
    }
}

struct RuleApplyRpc {
    cluster: Uuid16,
    mode: RuleApplyMode,
}

impl Request for RuleApplyRpc {
    type Response = Vec<u8>;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_RULE_APPLY_REQ,
            method_resp: None,
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut body = Vec::with_capacity(20);
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&self.mode.as_u32().to_be_bytes());
        Ok(body)
    }
}

struct RuleRemoveRpc {
    cluster: Uuid16,
    server: Uuid16,
    rule: Uuid16,
}

impl Request for RuleRemoveRpc {
    type Response = Vec<u8>;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_RULE_REMOVE_REQ,
            method_resp: None,
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut body = Vec::with_capacity(48);
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&self.server);
        body.extend_from_slice(&self.rule);
        Ok(body)
    }
}

struct RuleInsertRpc {
    cluster: Uuid16,
    req: RuleInsertReq,
}

impl Request for RuleInsertRpc {
    type Response = RuleInsertResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_RULE_INSERT_REQ,
            method_resp: Some(METHOD_RULE_INSERT_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let req = &self.req;
        let mut body = Vec::with_capacity(
            16 + 16 + 16 + 4 + 4 + req.infobase_name.len() + req.application_ext.len() + 4 + 3,
        );
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&req.server);
        body.extend_from_slice(&[0u8; 16]);
        body.extend_from_slice(&req.position.to_be_bytes());
        body.extend_from_slice(&req.object_type.to_be_bytes());
        body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
            req.infobase_name.as_bytes(),
        )?);
        body.push(req.rule_type);
        body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
            req.application_ext.as_bytes(),
        )?);
        body.extend_from_slice(&req.priority.to_be_bytes());
        Ok(body)
    }
}

impl Response for RuleInsertResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let rule = parse_rule_insert_body(body)?;
        Ok(Self { rule })
    }
}

struct RuleUpdateRpc {
    cluster: Uuid16,
    req: RuleUpdateReq,
}

impl Request for RuleUpdateRpc {
    type Response = RuleUpdateResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_RULE_INSERT_REQ,
            method_resp: Some(METHOD_RULE_INSERT_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let req = &self.req;
        let mut body = Vec::with_capacity(
            16 + 16 + 16 + 4 + 4 + req.infobase_name.len() + req.application_ext.len() + 4 + 3,
        );
        body.extend_from_slice(&self.cluster);
        body.extend_from_slice(&req.server);
        body.extend_from_slice(&req.rule);
        body.extend_from_slice(&req.position.to_be_bytes());
        body.extend_from_slice(&req.object_type.to_be_bytes());
        body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
            req.infobase_name.as_bytes(),
        )?);
        body.push(req.rule_type);
        body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
            req.application_ext.as_bytes(),
        )?);
        body.extend_from_slice(&req.priority.to_be_bytes());
        Ok(body)
    }
}

impl Response for RuleUpdateResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let rule = parse_rule_update_body(body)?;
        Ok(Self { rule })
    }
}

pub fn rule_list(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    server: Uuid16,
) -> Result<RuleListResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    client.call_typed(RuleListRpc { cluster, server })
}

pub fn rule_info(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    server: Uuid16,
    rule: Uuid16,
) -> Result<RuleInfoResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    client.call_typed(RuleInfoRpc {
        cluster,
        server,
        rule,
    })
}

pub fn rule_apply(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    mode: RuleApplyMode,
) -> Result<RuleApplyResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    let reply = client.call(RuleApplyRpc { cluster, mode })?;
    let acknowledged = parse_ack_payload(&reply, "rule apply expected ack")?;
    Ok(RuleApplyResp { acknowledged })
}

pub fn rule_remove(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    server: Uuid16,
    rule: Uuid16,
) -> Result<RuleRemoveResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    let reply = client.call(RuleRemoveRpc {
        cluster,
        server,
        rule,
    })?;
    let acknowledged = parse_ack_payload(&reply, "rule remove expected ack")?;
    Ok(RuleRemoveResp { acknowledged })
}

pub fn rule_insert(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    req: RuleInsertReq,
) -> Result<RuleInsertResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    client.call_typed(RuleInsertRpc { cluster, req })
}

pub fn rule_update(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    req: RuleUpdateReq,
) -> Result<RuleUpdateResp> {
    let _ = cluster_auth(client, cluster, cluster_user, cluster_pwd)?;
    client.call_typed(RuleUpdateRpc { cluster, req })
}

fn parse_rule_list_records(body: &[u8]) -> Result<Vec<RuleRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(parse_rule_record(&mut cursor)?);
    }
    Ok(records)
}

fn parse_rule_info_body(body: &[u8]) -> Result<RuleRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("rule info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    parse_rule_record(&mut cursor)
}

fn parse_rule_record(cursor: &mut RecordCursor<'_>) -> Result<RuleRecord> {
    RuleRecord::decode(cursor).map_err(|_| RacError::Decode("rule record truncated"))
}

fn parse_rule_insert_body(body: &[u8]) -> Result<Uuid16> {
    parse_uuid_body(body, "rule insert empty body")
}

fn parse_rule_update_body(body: &[u8]) -> Result<Uuid16> {
    parse_uuid_body(body, "rule update empty body")
}

#[cfg(test)]
fn parse_rule_apply_ack(payload: &[u8]) -> Result<bool> {
    crate::rpc::decode_utils::parse_ack_payload(payload, "rule apply ack truncated")
}

#[cfg(test)]
fn parse_rule_remove_ack(payload: &[u8]) -> Result<bool> {
    crate::rpc::decode_utils::parse_ack_payload(payload, "rule remove ack truncated")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ProtocolVersion;
    use crate::rpc::Request;
    use crate::rac_wire::parse_frames;
    use crate::rac_wire::parse_uuid;
    use crate::commands::rpc_body;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn parse_rule_apply_ack_payload() {
        let payload = decode_hex_str("01000000");
        let acknowledged = parse_rule_apply_ack(&payload).expect("parse ack");
        assert!(acknowledged);
    }

    #[test]
    fn encode_rule_apply_request_full() {
        let expected = decode_hex_str("01000001511619820ad36f4d8aa7161516b1dea07700000001");
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let req = RuleApplyRpc {
            cluster,
            mode: RuleApplyMode::Full,
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_rule_remove_request() {
        let expected = decode_hex_str("01000001541619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e8bec00d861c934eb881828f26dce197d6");
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let rule = parse_uuid("bec00d86-1c93-4eb8-8182-8f26dce197d6").expect("rule uuid");
        let req = RuleRemoveRpc {
            cluster,
            server,
            rule,
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_rule_list_request() {
        let expected = decode_hex_str(
            "01000001551619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e8",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let req = RuleListRpc { cluster, server };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x56));
    }

    #[test]
    fn encode_rule_info_request() {
        let expected = decode_hex_str("01000001571619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e8bec00d861c934eb881828f26dce197d6");
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let rule = parse_uuid("bec00d86-1c93-4eb8-8182-8f26dce197d6").expect("rule uuid");
        let req = RuleInfoRpc {
            cluster,
            server,
            rule,
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x58));
    }

    #[test]
    fn encode_rule_insert_request() {
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let mut expected = Vec::new();
        expected.extend_from_slice(&[0x01, 0x00, 0x00, 0x01, 0x52]);
        expected.extend_from_slice(&cluster);
        expected.extend_from_slice(&server);
        expected.extend_from_slice(&[0u8; 16]);
        expected.extend_from_slice(&1u32.to_be_bytes());
        expected.extend_from_slice(&0u32.to_be_bytes());
        expected.push(0);
        expected.push(0);
        expected.push(0);
        expected.extend_from_slice(&0u32.to_be_bytes());
        let req = RuleInsertReq {
            server,
            position: 1,
            object_type: 0,
            infobase_name: String::new(),
            rule_type: 0,
            application_ext: String::new(),
            priority: 0,
        };
        let rpc = RuleInsertRpc { cluster, req };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x53));
    }

    #[test]
    fn encode_rule_update_request() {
        let expected = decode_hex_str("01000001521619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e8bec00d861c934eb881828f26dce197d6000000000001000000000000000000");
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let rule = parse_uuid("bec00d86-1c93-4eb8-8182-8f26dce197d6").expect("rule uuid");
        let req = RuleUpdateReq {
            server,
            rule,
            position: 0,
            object_type: 65536,
            infobase_name: String::new(),
            rule_type: 0,
            application_ext: String::new(),
            priority: 0,
        };
        let rpc = RuleUpdateRpc { cluster, req };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x53));
    }

    #[test]
    fn parse_rule_list_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/rule_list_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let records = parse_rule_list_records(body).expect("rule list");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].rule,
            parse_uuid("bec00d86-1c93-4eb8-8182-8f26dce197d6").unwrap()
        );
        assert_eq!(records[0].object_type, 0);
        assert_eq!(records[0].infobase_name, "");
        assert_eq!(records[0].rule_type, 1);
        assert_eq!(records[0].application_ext, "");
        assert_eq!(records[0].priority, 0);
    }

    #[test]
    fn parse_rule_info_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/rule_info_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let record = parse_rule_info_body(body).expect("rule info");

        assert_eq!(
            record.rule,
            parse_uuid("bec00d86-1c93-4eb8-8182-8f26dce197d6").unwrap()
        );
        assert_eq!(record.object_type, 0);
        assert_eq!(record.infobase_name, "");
        assert_eq!(record.rule_type, 1);
        assert_eq!(record.application_ext, "");
        assert_eq!(record.priority, 0);
    }

    #[test]
    fn parse_rule_insert_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/rule_insert_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let rule = parse_rule_insert_body(body).expect("rule insert");

        assert_eq!(
            rule,
            parse_uuid("bec00d86-1c93-4eb8-8182-8f26dce197d6").unwrap()
        );
    }

    #[test]
    fn parse_rule_update_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/rule_update_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 5);
        assert_eq!(frames[4].opcode, 0x0e);
        let body = rpc_body(&frames[4].payload).expect("rpc body");
        let rule = parse_rule_update_body(body).expect("rule update");

        assert_eq!(
            rule,
            parse_uuid("bec00d86-1c93-4eb8-8182-8f26dce197d6").unwrap()
        );
    }

    #[test]
    fn parse_rule_remove_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/rule_remove_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[2].opcode, 0x0e);
        assert_eq!(frames[3].opcode, 0x0e);
        let ack_first = parse_rule_remove_ack(&frames[2].payload).expect("rule remove ack");
        let ack_second = parse_rule_remove_ack(&frames[3].payload).expect("rule remove ack");
        assert!(ack_first);
        assert!(ack_second);
    }
}
