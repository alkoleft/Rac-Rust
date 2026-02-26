use crate::client::RacClient;
use crate::commands::cluster_auth;
use crate::error::Result;
use crate::rpc::AckResponse;
use crate::Uuid16;

mod generated {
    include!("counter_generated.rs");
}

pub use generated::{
    CounterAccumulatedValuesResp,
    CounterAccumulatedValuesRpc,
    CounterClearRpc,
    CounterInfoResp,
    CounterInfoRpc,
    CounterListResp,
    CounterListRpc,
    CounterRecord,
    CounterRemoveRpc,
    CounterUpdateRpc,
    CounterValuesRecord,
    CounterValuesResp,
    CounterValuesRpc,
};

pub fn counter_list(client: &mut RacClient, cluster: Uuid16) -> Result<CounterListResp> {
    client.call_typed(CounterListRpc { cluster })
}

pub fn counter_info(
    client: &mut RacClient,
    cluster: Uuid16,
    counter: &str,
) -> Result<CounterInfoResp> {
    client.call_typed(CounterInfoRpc {
        cluster,
        counter: counter.to_string(),
    })
}

pub fn counter_update(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: CounterUpdateRpc,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn counter_clear(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: CounterClearRpc,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn counter_remove(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: CounterRemoveRpc,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn counter_values(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: CounterValuesRpc,
) -> Result<CounterValuesResp> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn counter_accumulated_values(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: CounterAccumulatedValuesRpc,
) -> Result<CounterAccumulatedValuesResp> {
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
    fn parse_counter_list_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_list_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp =
            CounterListResp::decode(&frames[3].payload, protocol.as_ref()).expect("parse response");

        let records = resp.records;
        assert_eq!(records.len(), 11);
        assert_eq!(records[0].name, "Вызовы");
        assert_eq!(records[0].collection_time, 5);
        assert_eq!(records[0].group, 0);
        assert_eq!(records[0].filter_type, 2);
        assert_eq!(records[0].filter, "2");
        assert_eq!(records[0].duration, 1);
        assert_eq!(records[0].descr, "");

        assert_eq!(records[1].name, "cpu");
        assert_eq!(records[1].collection_time, 6);
        assert_eq!(records[1].cpu_time, 1);
        assert_eq!(records[1].descr, "cpu desc");

        assert_eq!(records[8].name, "sessions");
        assert_eq!(records[8].collection_time, 2000);
        assert_eq!(records[8].number_of_sessions, 1);
        assert_eq!(records[8].descr, "sessions d");

        assert_eq!(records[10].name, "serv call");
        assert_eq!(records[10].call, 1);
    }

    #[test]
    fn parse_counter_info_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_info_codex_tmp_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp =
            CounterInfoResp::decode(&frames[3].payload, protocol.as_ref()).expect("parse response");

        let record = resp.record;
        assert_eq!(record.name, "codex_tmp");
        assert_eq!(record.collection_time, 12);
        assert_eq!(record.group, 0);
        assert_eq!(record.filter_type, 2);
        assert_eq!(record.filter, "1");
        assert_eq!(record.duration, 1);
        assert_eq!(record.cpu_time, 0);
        assert_eq!(record.duration_dbms, 0);
        assert_eq!(record.service, 0);
        assert_eq!(record.memory, 1);
        assert_eq!(record.read, 0);
        assert_eq!(record.write, 1);
        assert_eq!(record.dbms_bytes, 1);
        assert_eq!(record.call, 1);
        assert_eq!(record.number_of_active_sessions, 0);
        assert_eq!(record.number_of_sessions, 1);
        assert_eq!(record.descr, "codex_tmp");
    }

    #[test]
    fn parse_counter_update_ack_payload() {
        let payload = decode_hex_str("01000000");
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn parse_counter_update_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_update_codex_tmp_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&frames[3].payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn parse_counter_clear_ack_payload() {
        let payload = decode_hex_str("01000000");
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn parse_counter_clear_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_clear_codex_tmp_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&frames[3].payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn parse_counter_remove_ack_payload() {
        let payload = decode_hex_str("01000000");
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&payload, protocol.as_ref()).expect("parse ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn parse_counter_remove_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_remove_codex_tmp_response.hex");
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
    fn encode_counter_update_request() {
        let expected = decode_hex_str(
            "010000017a1619820ad36f4d8aa7161516b1dea07709636f6465785f746d70000000000000000c00020131010000000100010101000109636f6465785f746d70",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = CounterUpdateRpc {
            cluster,
            name: "codex_tmp".to_string(),
            collection_time: 12,
            group: 0,
            filter_type: 2,
            filter: "1".to_string(),
            duration: 1,
            cpu_time: 0,
            duration_dbms: 0,
            service: 0,
            memory: 1,
            read: 0,
            write: 1,
            dbms_bytes: 1,
            call: 1,
            number_of_active_sessions: 0,
            number_of_sessions: 1,
            descr: "codex_tmp".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_counter_remove_request() {
        let expected = decode_hex_str(
            "010000017b1619820ad36f4d8aa7161516b1dea07709636f6465785f746d70",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = CounterRemoveRpc {
            cluster,
            name: "codex_tmp".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_counter_clear_request() {
        let expected = decode_hex_str(
            "01000001841619820ad36f4d8aa7161516b1dea07709636f6465785f746d7000",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = CounterClearRpc {
            cluster,
            counter: "codex_tmp".to_string(),
            object: "".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn parse_counter_values_from_golden_capture() {
        let hex = include_str!("../../../../artifacts/rac/counter_values_codex_tmp_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = CounterValuesResp::decode(&frames[3].payload, protocol.as_ref())
            .expect("parse response");

        let records = resp.records;
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].object, "infobase=yaxunit;user=DefUser");
        assert_eq!(records[0].collection_time, 12);
        assert_eq!(records[0].duration, 1006);
        assert_eq!(records[0].cpu_time, 0);
        assert_eq!(records[0].memory, 0);
        assert_eq!(records[0].read, 0);
        assert_eq!(records[0].write, 0);
        assert_eq!(records[0].duration_dbms, 0);
        assert_eq!(records[0].dbms_bytes, 0);
        assert_eq!(records[0].service, 0);
        assert_eq!(records[0].call, 0);
        assert_eq!(records[0].number_of_active_sessions, 0);
        assert_eq!(records[0].number_of_sessions, 1);
        assert_eq!(records[0].time, "2026-02-17T19:42:41");
    }

    #[test]
    fn encode_counter_values_request() {
        let expected = decode_hex_str(
            "01000001821619820ad36f4d8aa7161516b1dea07709636f6465785f746d7000",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = CounterValuesRpc {
            cluster,
            counter: "codex_tmp".to_string(),
            object: "".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x83));
    }

    #[test]
    fn parse_counter_accumulated_values_from_golden_capture() {
        let hex = include_str!(
            "../../../../artifacts/rac/counter_accumulated_values_codex_tmp_response.hex"
        );
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = CounterAccumulatedValuesResp::decode(&frames[3].payload, protocol.as_ref())
            .expect("parse response");

        let records = resp.records;
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].object, "infobase=yaxunit;user=DefUser");
        assert_eq!(records[0].collection_time, 10000);
        assert_eq!(records[0].duration, 1001);
        assert_eq!(records[0].cpu_time, 0);
        assert_eq!(records[0].memory, 0);
        assert_eq!(records[0].read, 0);
        assert_eq!(records[0].write, 0);
        assert_eq!(records[0].duration_dbms, 0);
        assert_eq!(records[0].dbms_bytes, 0);
        assert_eq!(records[0].service, 0);
        assert_eq!(records[0].call, 0);
        assert_eq!(records[0].number_of_active_sessions, 0);
        assert_eq!(records[0].number_of_sessions, 0);
        assert_eq!(records[0].time, "2026-02-17T19:42:45");
    }

    #[test]
    fn encode_counter_accumulated_values_request() {
        let expected = decode_hex_str(
            "01000001851619820ad36f4d8aa7161516b1dea07709636f6465785f746d7000",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let rpc = CounterAccumulatedValuesRpc {
            cluster,
            counter: "codex_tmp".to_string(),
            object: "".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = rpc.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x86));
    }
}
