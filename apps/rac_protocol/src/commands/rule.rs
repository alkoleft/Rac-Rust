use crate::client::RacClient;
use crate::commands::cluster_auth;
use crate::error::Result;
use crate::rpc::AckResponse;

mod generated {
    include!("rule_generated.rs");
}

pub use generated::{
    RuleApplyRpc,
    RuleIdRecord,
    RuleInfoResp,
    RuleInfoRpc,
    RuleInsertResp,
    RuleInsertRpc,
    RuleListResp,
    RuleListRpc,
    RuleRecord,
    RuleRemoveRpc,
    RuleUpdateResp,
    RuleUpdateRpc,
};

pub fn rule_list(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: RuleListRpc,
) -> Result<RuleListResp> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn rule_info(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: RuleInfoRpc,
) -> Result<RuleInfoResp> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn rule_apply(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: RuleApplyRpc,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn rule_remove(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: RuleRemoveRpc,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn rule_insert(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: RuleInsertRpc,
) -> Result<RuleInsertResp> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn rule_update(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: RuleUpdateRpc,
) -> Result<RuleUpdateResp> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ProtocolVersion;
    use crate::rac_wire::parse_frames;
    use crate::rac_wire::parse_uuid;
    use crate::rpc::Request;
    use crate::rpc::Response;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn parse_rule_apply_ack_payload() {
        let payload = decode_hex_str("01000000");
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn encode_rule_apply_request_full() {
        let expected =
            decode_hex_str("01000001511619820ad36f4d8aa7161516b1dea07700000001");
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let req = RuleApplyRpc { cluster, mode: 1 };
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
        let req = RuleInsertRpc {
            cluster,
            server,
            rule: [0u8; 16],
            position: 1,
            object_type: 0,
            infobase_name: String::new(),
            rule_type: 0,
            application_ext: String::new(),
            priority: 0,
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x53));
    }

    #[test]
    fn encode_rule_update_request() {
        let expected = decode_hex_str("01000001521619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e8bec00d861c934eb881828f26dce197d6000000000001000000000000000000");
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let rule = parse_uuid("bec00d86-1c93-4eb8-8182-8f26dce197d6").expect("rule uuid");
        let req = RuleUpdateRpc {
            cluster,
            server,
            rule,
            position: 0,
            object_type: 65536,
            infobase_name: String::new(),
            rule_type: 0,
            application_ext: String::new(),
            priority: 0,
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
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
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = RuleListResp::decode(&frames[3].payload, protocol.as_ref())
            .expect("parse response");

        let records = resp.records;
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
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = RuleInfoResp::decode(&frames[3].payload, protocol.as_ref())
            .expect("parse response");

        let record = resp.record;
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
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = RuleInsertResp::decode(&frames[3].payload, protocol.as_ref())
            .expect("parse response");

        assert_eq!(
            resp.rule,
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
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = RuleUpdateResp::decode(&frames[4].payload, protocol.as_ref())
            .expect("parse response");

        assert_eq!(
            resp.rule,
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
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&frames[2].payload, protocol.as_ref())
            .expect("parse ack");
        assert!(resp.acknowledged);
        let resp = AckResponse::decode(&frames[3].payload, protocol.as_ref())
            .expect("parse ack");
        assert!(resp.acknowledged);
    }
}
