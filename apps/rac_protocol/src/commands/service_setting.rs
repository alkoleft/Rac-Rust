use crate::client::RacClient;
use crate::commands::cluster_auth;
use crate::error::Result;
use crate::rpc::AckResponse;

mod generated {
    include!("service_setting_generated.rs");
}

pub use generated::{
    ServiceSettingApplyRpc,
    ServiceSettingGetDataDirsResp,
    ServiceSettingGetDataDirsRpc,
    ServiceSettingIdRecord,
    ServiceSettingInfoResp,
    ServiceSettingInfoRpc,
    ServiceSettingInsertResp,
    ServiceSettingInsertRpc,
    ServiceSettingListResp,
    ServiceSettingListRpc,
    ServiceSettingRecord,
    ServiceSettingRemoveRpc,
    ServiceSettingTransferDataDirRecord,
    ServiceSettingUpdateResp,
    ServiceSettingUpdateRpc,
};

pub fn service_setting_info(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: ServiceSettingInfoRpc,
) -> Result<ServiceSettingInfoResp> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn service_setting_info_no_auth(
    client: &mut RacClient,
    req: ServiceSettingInfoRpc,
) -> Result<ServiceSettingInfoResp> {
    client.call_typed(req)
}

pub fn service_setting_list(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: ServiceSettingListRpc,
) -> Result<ServiceSettingListResp> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn service_setting_insert(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: ServiceSettingInsertRpc,
) -> Result<ServiceSettingInsertResp> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn service_setting_update(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: ServiceSettingUpdateRpc,
) -> Result<ServiceSettingUpdateResp> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn service_setting_update_no_auth(
    client: &mut RacClient,
    req: ServiceSettingUpdateRpc,
) -> Result<ServiceSettingUpdateResp> {
    client.call_typed(req)
}

pub fn service_setting_remove(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: ServiceSettingRemoveRpc,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn service_setting_apply(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: ServiceSettingApplyRpc,
) -> Result<AckResponse> {
    let _ = cluster_auth(client, req.cluster, cluster_user, cluster_pwd)?;
    client.call_typed(req)
}

pub fn service_setting_get_service_data_dirs_for_transfer(
    client: &mut RacClient,
    cluster_user: &str,
    cluster_pwd: &str,
    req: ServiceSettingGetDataDirsRpc,
) -> Result<ServiceSettingGetDataDirsResp> {
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
    fn parse_service_setting_list_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/service_setting_list_nonempty_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = ServiceSettingListResp::decode(&frames[3].payload, protocol.as_ref())
            .expect("parse list");

        let records = resp.records;
        assert_eq!(records.len(), 1);
        let record = &records[0];
        let expected_setting =
            parse_uuid("1496c164-a9f5-446e-a021-16a127b06a11").expect("setting uuid");
        assert_eq!(record.setting, expected_setting);
        assert_eq!(record.service_name, "EventLogService");
        assert_eq!(record.infobase_name, "");
        assert_eq!(record.service_data_dir, "/tmp/codex_service_setting/");
        assert_eq!(record.active, false);
    }

    #[test]
    fn parse_service_setting_info_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/service_setting_info_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = ServiceSettingInfoResp::decode(&frames[3].payload, protocol.as_ref())
            .expect("parse info");

        let record = resp.record;
        let expected_setting =
            parse_uuid("1496c164-a9f5-446e-a021-16a127b06a11").expect("setting uuid");
        assert_eq!(record.setting, expected_setting);
        assert_eq!(record.service_name, "EventLogService");
        assert_eq!(record.infobase_name, "");
        assert_eq!(record.service_data_dir, "/tmp/codex_service_setting/");
        assert_eq!(record.active, false);
    }

    #[test]
    fn parse_service_setting_insert_response_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/service_setting_insert_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = ServiceSettingInsertResp::decode(&frames[3].payload, protocol.as_ref())
            .expect("parse insert response");

        let expected_setting =
            parse_uuid("1496c164-a9f5-446e-a021-16a127b06a11").expect("setting uuid");
        assert_eq!(resp.setting, expected_setting);
    }

    #[test]
    fn parse_service_setting_update_response_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/service_setting_update_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 5);
        assert_eq!(frames[4].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = ServiceSettingUpdateResp::decode(&frames[4].payload, protocol.as_ref())
            .expect("parse update response");

        let expected_setting =
            parse_uuid("1496c164-a9f5-446e-a021-16a127b06a11").expect("setting uuid");
        assert_eq!(resp.setting, expected_setting);
    }

    #[test]
    fn encode_service_setting_list_request() {
        let expected = decode_hex_str(
            "010000018b1619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e8",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let req = ServiceSettingListRpc { cluster, server };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x8c));
    }

    #[test]
    fn encode_service_setting_info_request() {
        let expected = decode_hex_str(
            "01000001891619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e81496c164a9f5446ea02116a127b06a11",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let setting = parse_uuid("1496c164-a9f5-446e-a021-16a127b06a11").expect("setting uuid");
        let req = ServiceSettingInfoRpc {
            cluster,
            server,
            setting,
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x8a));
    }

    #[test]
    fn encode_service_setting_insert_request() {
        let expected = decode_hex_str(
            "010000018d1619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e8000000000000000000000000000000000f4576656e744c6f6753657276696365001a2f746d702f636f6465785f736572766963655f73657474696e670000",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let req = ServiceSettingInsertRpc {
            cluster,
            server,
            service_name: "EventLogService".to_string(),
            infobase_name: "".to_string(),
            service_data_dir: "/tmp/codex_service_setting".to_string(),
            active: 0,
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x8e));
    }

    #[test]
    fn encode_service_setting_update_request() {
        let expected = decode_hex_str(
            "010000018d1619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e81496c164a9f5446ea02116a127b06a110f4576656e744c6f675365727669636500222f746d702f636f6465785f736572766963655f73657474696e675f757064617465640000",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let setting = parse_uuid("1496c164-a9f5-446e-a021-16a127b06a11").expect("setting uuid");
        let req = ServiceSettingUpdateRpc {
            cluster,
            server,
            setting,
            service_name: "EventLogService".to_string(),
            infobase_name: "".to_string(),
            service_data_dir: "/tmp/codex_service_setting_updated".to_string(),
            active: 0,
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x8e));
    }

    #[test]
    fn parse_service_setting_remove_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/service_setting_remove_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[2].opcode, 0x0e);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&frames[2].payload, protocol.as_ref())
            .expect("remove ack");
        assert!(resp.acknowledged);
        let resp = AckResponse::decode(&frames[3].payload, protocol.as_ref())
            .expect("remove ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn parse_service_setting_apply_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/service_setting_apply_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[2].opcode, 0x0e);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = AckResponse::decode(&frames[2].payload, protocol.as_ref())
            .expect("apply ack");
        assert!(resp.acknowledged);
        let resp = AckResponse::decode(&frames[3].payload, protocol.as_ref())
            .expect("apply ack");
        assert!(resp.acknowledged);
    }

    #[test]
    fn encode_service_setting_remove_request() {
        let expected = decode_hex_str(
            "010000018f1619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e81496c164a9f5446ea02116a127b06a11",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let setting = parse_uuid("1496c164-a9f5-446e-a021-16a127b06a11").expect("setting uuid");
        let req = ServiceSettingRemoveRpc {
            cluster,
            server,
            setting,
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn encode_service_setting_apply_request() {
        let expected = decode_hex_str(
            "01000001901619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e8",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let req = ServiceSettingApplyRpc { cluster, server };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn parse_service_setting_get_data_dirs_for_transfer_response_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/service_setting_get_data_dirs_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[3].opcode, 0x0e);
        let protocol = ProtocolVersion::V16_0.boxed();
        let resp = ServiceSettingGetDataDirsResp::decode(&frames[3].payload, protocol.as_ref())
            .expect("parse list");

        let records = resp.records;
        assert_eq!(records.len(), 1);
        let record = &records[0];
        assert_eq!(record.service_name, "EventLogService");
        assert_eq!(record.user, "yaxunit");
        assert_eq!(
            record.source_dir,
            "/tmp/1cv8-agent/reg_1541/717bdda7-2f60-4577-b262-f1fc8c0e472c/1Cv8Log"
        );
        assert_eq!(
            record.target_dir,
            "/tmp/codex_service_setting/reg_1541/717bdda7-2f60-4577-b262-f1fc8c0e472c/1Cv8Log"
        );
        assert_eq!(record.source_dir_flag, 0x01);
        assert_eq!(record.target_dir_flag, 0x01);
    }

    #[test]
    fn encode_service_setting_get_data_dirs_for_transfer_request() {
        let expected = decode_hex_str(
            "01000001911619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e80f4576656e744c6f6753657276696365",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let req = ServiceSettingGetDataDirsRpc {
            cluster,
            server,
            service_name: "EventLogService".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x92));
    }
}
