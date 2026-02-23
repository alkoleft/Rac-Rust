use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::{parse_ack_payload, parse_uuid_body, rpc_body};

mod generated {
    include!("service_setting_generated.rs");
}

pub use generated::{ServiceSettingRecord, ServiceSettingTransferDataDirRecord};

#[derive(Debug, Serialize)]
pub struct ServiceSettingListResp {
    pub records: Vec<ServiceSettingRecord>,
}

#[derive(Debug, Serialize)]
pub struct ServiceSettingInfoResp {
    pub record: ServiceSettingRecord,
}

#[derive(Debug, Serialize, Clone)]
pub struct ServiceSettingInsertReq {
    pub server: Uuid16,
    pub service_name: String,
    pub infobase_name: String,
    pub service_data_dir: String,
    pub active: bool,
}

#[derive(Debug, Serialize)]
pub struct ServiceSettingInsertResp {
    pub setting: Uuid16,
}

#[derive(Debug, Serialize, Clone)]
pub struct ServiceSettingUpdateReq {
    pub server: Uuid16,
    pub setting: Uuid16,
    pub service_name: String,
    pub infobase_name: String,
    pub service_data_dir: String,
    pub active: bool,
}

#[derive(Debug, Serialize)]
pub struct ServiceSettingUpdateResp {
    pub setting: Uuid16,
}

#[derive(Debug, Serialize)]
pub struct ServiceSettingRemoveResp {
    pub acknowledged: bool,
}

#[derive(Debug, Serialize)]
pub struct ServiceSettingApplyResp {
    pub acknowledged: bool,
}

#[derive(Debug, Serialize)]
pub struct ServiceSettingTransferDataDirsResp {
    pub records: Vec<ServiceSettingTransferDataDirRecord>,
}

pub fn service_setting_info(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    server: Uuid16,
    setting: Uuid16,
) -> Result<ServiceSettingInfoResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    service_setting_info_no_auth(client, cluster, server, setting)
}

pub fn service_setting_info_no_auth(
    client: &mut RacClient,
    cluster: Uuid16,
    server: Uuid16,
    setting: Uuid16,
) -> Result<ServiceSettingInfoResp> {
    let reply = client.call(RacRequest::ServiceSettingInfo {
        cluster,
        server,
        setting,
    })?;
    let body = rpc_body(&reply)?;
    let record = parse_service_setting_info(body)?;
    Ok(ServiceSettingInfoResp {
        record,
    })
}

pub fn service_setting_list(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    server: Uuid16,
) -> Result<ServiceSettingListResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::ServiceSettingList { cluster, server })?;
    let body = rpc_body(&reply)?;
    let records = parse_service_setting_list(body)?;
    Ok(ServiceSettingListResp {
        records,
    })
}

pub fn service_setting_insert(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    req: ServiceSettingInsertReq,
) -> Result<ServiceSettingInsertResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::ServiceSettingInsert {
        cluster,
        server: req.server,
        service_name: req.service_name,
        infobase_name: req.infobase_name,
        service_data_dir: req.service_data_dir,
        active: req.active,
    })?;
    let body = rpc_body(&reply)?;
    let setting = parse_service_setting_insert_body(body)?;
    Ok(ServiceSettingInsertResp {
        setting,
    })
}

pub fn service_setting_update(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    req: ServiceSettingUpdateReq,
) -> Result<ServiceSettingUpdateResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    service_setting_update_no_auth(client, cluster, req)
}

pub fn service_setting_update_no_auth(
    client: &mut RacClient,
    cluster: Uuid16,
    req: ServiceSettingUpdateReq,
) -> Result<ServiceSettingUpdateResp> {
    let reply = client.call(RacRequest::ServiceSettingUpdate {
        cluster,
        server: req.server,
        setting: req.setting,
        service_name: req.service_name,
        infobase_name: req.infobase_name,
        service_data_dir: req.service_data_dir,
        active: req.active,
    })?;
    let body = rpc_body(&reply)?;
    let setting = parse_service_setting_update_body(body)?;
    Ok(ServiceSettingUpdateResp {
        setting,
    })
}

pub fn service_setting_remove(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    server: Uuid16,
    setting: Uuid16,
) -> Result<ServiceSettingRemoveResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::ServiceSettingRemove {
        cluster,
        server,
        setting,
    })?;
    let acknowledged = parse_service_setting_remove_ack(&reply)?;
    if !acknowledged {
        return Err(RacError::Decode("service-setting remove expected ack"));
    }
    Ok(ServiceSettingRemoveResp {
        acknowledged,
    })
}

pub fn service_setting_apply(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    server: Uuid16,
) -> Result<ServiceSettingApplyResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::ServiceSettingApply { cluster, server })?;
    let acknowledged = parse_service_setting_apply_ack(&reply)?;
    if !acknowledged {
        return Err(RacError::Decode("service-setting apply expected ack"));
    }
    Ok(ServiceSettingApplyResp {
        acknowledged,
    })
}

pub fn service_setting_get_service_data_dirs_for_transfer(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    server: Uuid16,
    service_name: &str,
) -> Result<ServiceSettingTransferDataDirsResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::ServiceSettingGetServiceDataDirsForTransfer {
        cluster,
        server,
        service_name: service_name.to_string(),
    })?;
    let body = rpc_body(&reply)?;
    let records = parse_service_setting_transfer_data_dirs(body)?;
    Ok(ServiceSettingTransferDataDirsResp {
        records,
    })
}

fn parse_service_setting_list(body: &[u8]) -> Result<Vec<ServiceSettingRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(parse_service_setting_record(&mut cursor)?);
    }
    Ok(records)
}

fn parse_service_setting_info(body: &[u8]) -> Result<ServiceSettingRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("service-setting info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    parse_service_setting_record(&mut cursor)
}

fn parse_service_setting_insert_body(body: &[u8]) -> Result<Uuid16> {
    parse_uuid_body(body, "service-setting insert empty body")
}

fn parse_service_setting_update_body(body: &[u8]) -> Result<Uuid16> {
    parse_uuid_body(body, "service-setting update empty body")
}

fn parse_service_setting_remove_ack(payload: &[u8]) -> Result<bool> {
    parse_ack_payload(payload, "service-setting remove ack truncated")
}

fn parse_service_setting_apply_ack(payload: &[u8]) -> Result<bool> {
    parse_ack_payload(payload, "service-setting apply ack truncated")
}

fn parse_service_setting_transfer_data_dirs(
    body: &[u8],
) -> Result<Vec<ServiceSettingTransferDataDirRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        records.push(parse_service_setting_transfer_data_dir_record(&mut cursor)?);
    }
    Ok(records)
}

fn parse_service_setting_record(cursor: &mut RecordCursor<'_>) -> Result<ServiceSettingRecord> {
    if cursor.remaining_len() < 16 {
        return Err(RacError::Decode("service-setting record truncated"));
    }
    ServiceSettingRecord::decode(cursor)
}

fn parse_service_setting_transfer_data_dir_record(
    cursor: &mut RecordCursor<'_>,
) -> Result<ServiceSettingTransferDataDirRecord> {
    ServiceSettingTransferDataDirRecord::decode(cursor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::RacProtocolVersion;
    use crate::rac_wire::parse_frames;
    use crate::rac_wire::parse_uuid;

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
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let records = parse_service_setting_list(body).expect("parse list");

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
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let record = parse_service_setting_info(body).expect("parse info");

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
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let setting = parse_service_setting_insert_body(body).expect("parse insert response");

        let expected_setting =
            parse_uuid("1496c164-a9f5-446e-a021-16a127b06a11").expect("setting uuid");
        assert_eq!(setting, expected_setting);
    }

    #[test]
    fn parse_service_setting_update_response_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/service_setting_update_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 5);
        assert_eq!(frames[4].opcode, 0x0e);
        let body = rpc_body(&frames[4].payload).expect("rpc body");
        let setting = parse_service_setting_update_body(body).expect("parse update response");

        let expected_setting =
            parse_uuid("1496c164-a9f5-446e-a021-16a127b06a11").expect("setting uuid");
        assert_eq!(setting, expected_setting);
    }

    #[test]
    fn encode_service_setting_list_request() {
        let expected = decode_hex_str(
            "010000018b1619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e8",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let req = RacRequest::ServiceSettingList { cluster, server };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
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
        let req = RacRequest::ServiceSettingInfo {
            cluster,
            server,
            setting,
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
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
        let req = RacRequest::ServiceSettingInsert {
            cluster,
            server,
            service_name: "EventLogService".to_string(),
            infobase_name: "".to_string(),
            service_data_dir: "/tmp/codex_service_setting".to_string(),
            active: false,
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
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
        let req = RacRequest::ServiceSettingUpdate {
            cluster,
            server,
            setting,
            service_name: "EventLogService".to_string(),
            infobase_name: "".to_string(),
            service_data_dir: "/tmp/codex_service_setting_updated".to_string(),
            active: false,
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
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
        let ack_first =
            parse_service_setting_remove_ack(&frames[2].payload).expect("remove ack");
        let ack_second =
            parse_service_setting_remove_ack(&frames[3].payload).expect("remove ack");
        assert!(ack_first);
        assert!(ack_second);
    }

    #[test]
    fn parse_service_setting_apply_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/service_setting_apply_response.hex");
        let payload = decode_hex_str(hex);
        let frames = parse_frames(&payload).expect("frames");
        assert_eq!(frames.len(), 4);
        assert_eq!(frames[2].opcode, 0x0e);
        assert_eq!(frames[3].opcode, 0x0e);
        let ack_first = parse_service_setting_apply_ack(&frames[2].payload).expect("apply ack");
        let ack_second = parse_service_setting_apply_ack(&frames[3].payload).expect("apply ack");
        assert!(ack_first);
        assert!(ack_second);
    }

    #[test]
    fn encode_service_setting_remove_request() {
        let expected = decode_hex_str(
            "010000018f1619820ad36f4d8aa7161516b1dea0776aa3a88a934644998034a4a72d7ee8e81496c164a9f5446ea02116a127b06a11",
        );
        let cluster = parse_uuid("1619820a-d36f-4d8a-a716-1516b1dea077").expect("cluster uuid");
        let server = parse_uuid("6aa3a88a-9346-4499-8034-a4a72d7ee8e8").expect("server uuid");
        let setting = parse_uuid("1496c164-a9f5-446e-a021-16a127b06a11").expect("setting uuid");
        let req = RacRequest::ServiceSettingRemove {
            cluster,
            server,
            setting,
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
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
        let req = RacRequest::ServiceSettingApply { cluster, server };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
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
        let body = rpc_body(&frames[3].payload).expect("rpc body");
        let records = parse_service_setting_transfer_data_dirs(body).expect("parse list");

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
        let req = RacRequest::ServiceSettingGetServiceDataDirsForTransfer {
            cluster,
            server,
            service_name: "EventLogService".to_string(),
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, Some(0x92));
    }
}
