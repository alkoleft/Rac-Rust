use crate::client::RacClient;
use crate::commands::cluster_auth;
use crate::error::Result;
use crate::rpc::AckResponse;
use crate::Uuid16;

mod generated {
    include!("limit_generated.rs");
}

pub use generated::{
    LimitInfoResp,
    LimitInfoRpc,
    LimitListResp,
    LimitListRpc,
    LimitRecord,
    LimitRemoveRpc,
    LimitUpdateRpc,
};

pub fn limit_list(client: &mut RacClient, cluster: Uuid16) -> Result<LimitListResp> {
    client.call_typed(LimitListRpc { cluster })
}

pub fn limit_info(client: &mut RacClient, cluster: Uuid16, name: &str) -> Result<LimitInfoResp> {
    client.call_typed(LimitInfoRpc {
        cluster,
        name: name.to_string(),
    })
}

pub fn limit_update(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: LimitUpdateRpc,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn limit_remove(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: LimitRemoveRpc,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

#[cfg(all(test, feature = "artifacts"))]
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
    fn parse_limit_list_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/limit_list_nonempty_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp =
            LimitListResp::decode(&frames[3].payload, protocol.as_ref()).expect("parse response");

        let limits = resp.limits;
        assert_eq!(limits.len(), 2);
        assert_eq!(limits[0].name, "limit_codex_a");
        assert_eq!(limits[0].counter, "cpu");
        assert_eq!(limits[0].action, 2);
        assert_eq!(limits[0].cpu_time, 11);
        assert_eq!(limits[0].error_message, "limit_a");
        assert_eq!(limits[0].descr, "limit_a");

        assert_eq!(limits[1].name, "limit_codex_b");
        assert_eq!(limits[1].counter, "call");
        assert_eq!(limits[1].action, 3);
        assert_eq!(limits[1].call, 7);
        assert_eq!(limits[1].error_message, "limit_b");
        assert_eq!(limits[1].descr, "limit_b");
    }

    #[test]
    fn parse_limit_info_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/limit_info_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert!(!frames.is_empty());
        let frame = frames.last().expect("frame");
        assert_eq!(frame.opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp =
            LimitInfoResp::decode(&frame.payload, protocol.as_ref()).expect("parse response");

        let record = resp.record;
        assert_eq!(record.name, "limit_codex_tmp");
        assert_eq!(record.counter, "cpu");
        assert_eq!(record.action, 2);
        assert_eq!(record.cpu_time, 10);
        assert_eq!(record.error_message, "limit_tmp");
        assert_eq!(record.descr, "limit_tmp");
    }

    #[test]
    fn parse_limit_update_ack_payload() {
        let payload = decode_hex_str("01000000");
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn parse_limit_update_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/limit_update_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[2].opcode, 0x0e);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&frames[2].payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
        let resp = AckResponse::decode(&frames[3].payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn parse_limit_remove_ack_payload() {
        let payload = decode_hex_str("01000000");
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn parse_limit_remove_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/limit_remove_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[2].opcode, 0x0e);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&frames[2].payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
        let resp = AckResponse::decode(&frames[3].payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn encode_limit_update_request() {
        let expected = decode_hex_str(
            "01000001801619820ad36f4d8aa7161516b1dea0770f6c696d69745f636f6465785f746d7003637075020000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000096c696d69745f746d70096c696d69745f746d70",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = LimitUpdateRpc {
            cluster,
            name: "limit_codex_tmp".to_string(),
            counter: "cpu".to_string(),
            action: 2,
            duration: 0,
            cpu_time: 10,
            memory: 0,
            read: 0,
            write: 0,
            duration_dbms: 0,
            dbms_bytes: 0,
            service: 0,
            call: 0,
            number_of_active_sessions: 0,
            number_of_sessions: 0,
            error_message: "limit_tmp".to_string(),
            descr: "limit_tmp".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_limit_remove_request() {
        let expected = decode_hex_str(
            "01000001811619820ad36f4d8aa7161516b1dea0770f6c696d69745f636f6465785f746d70",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = LimitRemoveRpc {
            cluster,
            name: "limit_codex_tmp".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }
}
