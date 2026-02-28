use serde::Serialize;

use crate::client::RacClient;
use crate::codec::RecordCursor;
use crate::error::RacError;
use crate::error::Result;
use crate::protocol::{ProtocolCodec, ProtocolVersion};
use crate::rpc::Response;
use crate::rpc::decode_utils::rpc_body;
use crate::Uuid16;

mod generated {
    include!("cluster_generated.rs");
    pub type ClusterListResp = super::ClusterListResp;
    pub type ClusterInfoResp = super::ClusterInfoResp;
}

pub use generated::{
    ClusterAdminRecord,
    ClusterAdminListResp,
    ClusterAdminListRpc,
    ClusterAdminRemoveRpc,
    ClusterAdminRegisterRpc,
    ClusterAuthRpc,
    ClusterInfoRpc,
    ClusterListRpc,
    ClusterRecord,
};

pub fn cluster_auth(
    client: &mut RacClient,
    cluster: Uuid16,
    user: &str,
    pwd: &str,
) -> Result<bool> {
    let resp = client.call_typed(ClusterAuthRpc {
        cluster,
        user: user.to_string(),
        pwd: pwd.to_string(),
    })?;
    Ok(resp.acknowledged)
}

pub fn cluster_admin_list(
    client: &mut RacClient,
    cluster: Uuid16,
) -> Result<Vec<ClusterAdminRecord>> {
    let resp = client.call_typed(ClusterAdminListRpc { cluster })?;
    Ok(resp.admins)
}

pub fn cluster_admin_register(
    client: &mut RacClient,
    cluster: Uuid16,
    name: String,
    descr: String,
    pwd: String,
    auth_flags: u8,
) -> Result<bool> {
    let resp = client.call_typed(ClusterAdminRegisterRpc {
        cluster,
        name,
        descr,
        pwd,
        auth_tag: 0x01,
        auth_flags,
        os_user: String::new(),
    })?;
    Ok(resp.acknowledged)
}

pub fn cluster_admin_remove(
    client: &mut RacClient,
    cluster: Uuid16,
    name: &str,
) -> Result<bool> {
    let resp = client.call_typed(ClusterAdminRemoveRpc {
        cluster,
        name: name.to_string(),
    })?;
    Ok(resp.acknowledged)
}

fn decode_cluster_record(
    cursor: &mut RecordCursor<'_>,
    protocol_version: ProtocolVersion,
) -> Result<ClusterRecord> {
    match protocol_version {
        ProtocolVersion::V11_0 => ClusterRecord::decode(cursor, protocol_version),
        ProtocolVersion::V16_0 => {
            let uuid = cursor.take_uuid()?;
            let expiration_timeout = cursor.take_u32_be()?;
            let host = cursor.take_str8()?;
            let lifetime_limit = cursor.take_u32_be()?;
            let port = cursor.take_u16_be()?;
            let max_memory_size = cursor.take_u32_be()?;
            let max_memory_time_limit = cursor.take_u32_be()?;
            let display_name = cursor.take_str8()?;
            let security_level = cursor.take_u32_be()?;
            let session_fault_tolerance_level = cursor.take_u32_be()?;
            let load_balancing_mode = cursor.take_u32_be()?;
            let errors_count_threshold = cursor.take_u32_be()?;
            let flags_raw = cursor.take_u32_be()?;
            let _tail_u32_5 = cursor.take_u32_be()?;
            let ping_period_raw = cursor.take_u32_be()?;
            let ping_timeout_raw = cursor.take_u32_be()?;
            let restart_schedule_len = (ping_timeout_raw & 0xff) as usize;
            let restart_schedule_bytes = cursor.take_bytes(restart_schedule_len)?;
            let flags_bytes = flags_raw.to_be_bytes();
            Ok(ClusterRecord {
                uuid,
                expiration_timeout,
                host,
                lifetime_limit,
                port,
                max_memory_size,
                max_memory_time_limit,
                display_name,
                security_level,
                session_fault_tolerance_level,
                load_balancing_mode,
                errors_count_threshold,
                kill_problem_processes: flags_bytes[0],
                kill_by_memory_with_dump: flags_bytes[1],
                allow_access_right_audit_events_recording: Some(flags_bytes[2]),
                ping_period: Some(ping_period_raw >> 8),
                ping_timeout: Some(ping_timeout_raw >> 8),
                restart_schedule_cron: Some(
                    String::from_utf8_lossy(&restart_schedule_bytes).to_string(),
                ),
                restart_interval: None,
            })
        }
    }
}

fn parse_cluster_list_body(
    body: &[u8],
    protocol_version: ProtocolVersion,
) -> Result<Vec<ClusterRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body);
    let count = cursor.take_u8()? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(decode_cluster_record(&mut cursor, protocol_version)?);
    }
    Ok(out)
}

fn parse_cluster_info_body(
    body: &[u8],
    protocol_version: ProtocolVersion,
) -> Result<ClusterRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("cluster info empty body"));
    }
    match protocol_version {
        ProtocolVersion::V11_0 => generated::parse_cluster_info_body(body, 0, protocol_version),
        ProtocolVersion::V16_0 => {
            let mut cursor = RecordCursor::new(body);
            decode_cluster_record(&mut cursor, protocol_version)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ClusterListResp {
    clusters: Vec<ClusterRecord>,
}

impl Response for ClusterListResp {
    fn decode(payload: &[u8], codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let clusters = parse_cluster_list_body(body, codec.protocol_version())?;
        Ok(Self { clusters })
    }
}

pub fn cluster_list(client: &mut RacClient) -> Result<Vec<ClusterRecord>> {
    let resp = client.call_typed(ClusterListRpc)?;
    Ok(resp.clusters)
}

#[derive(Debug, Serialize)]
pub struct ClusterInfoResp {
    cluster: ClusterRecord,
}

impl Response for ClusterInfoResp {
    fn decode(payload: &[u8], codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let cluster = parse_cluster_info_body(body, codec.protocol_version())?;
        Ok(Self { cluster })
    }
}

pub fn cluster_info(client: &mut RacClient, cluster: Uuid16) -> Result<ClusterRecord> {
    let resp = client.call_typed(ClusterInfoRpc { cluster })?;
    Ok(resp.cluster)
}

#[cfg(all(test, feature = "artifacts"))]
mod tests {
    use super::*;
    use crate::protocol::ProtocolVersion;
    use crate::rpc::Request;
    use crate::commands::rpc_body;
    use crate::commands::parse_list_u8;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn parse_cluster_admin_list_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/cluster_admin_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let admins = parse_list_u8(body, |cursor| {
            ClusterAdminRecord::decode(cursor, ProtocolVersion::V16_0)
        })
        .expect("parse list");

        assert_eq!(admins.len(), 1);
        assert_eq!(admins[0].name, "cadmin");
        assert_eq!(admins[0].unknown_tag, 0);
        assert_eq!(admins[0].unknown_flags, 0x03efbfbd);
        assert_eq!(admins[0].unknown_tail, [0x01, 0x00, 0x00]);
    }

    #[test]
    fn encode_cluster_admin_register_request() {
        let expected =
            decode_hex_str(include_str!("../../../../artifacts/rac/cluster_admin_register_request.hex"));
        let cluster = [
            0x16, 0x19, 0x82, 0x0a, 0xd3, 0x6f, 0x4d, 0x8a, 0xa7, 0x16, 0x15, 0x16, 0xb1,
            0xde, 0xa0, 0x77,
        ];
        let req = ClusterAdminRegisterRpc {
            cluster,
            name: "test_admin1".to_string(),
            descr: "test admin".to_string(),
            pwd: "test_pass1".to_string(),
            auth_tag: 0x01,
            auth_flags: 0x01,
            os_user: String::new(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
    }

    #[test]
    fn encode_cluster_admin_list_request() {
        let expected =
            decode_hex_str(include_str!("../../../../artifacts/rac/cluster_admin_list_request.hex"));
        let cluster = [
            0x16, 0x19, 0x82, 0x0a, 0xd3, 0x6f, 0x4d, 0x8a, 0xa7, 0x16, 0x15, 0x16, 0xb1,
            0xde, 0xa0, 0x77,
        ];
        let req = ClusterAdminListRpc { cluster };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
    }

    #[test]
    fn encode_cluster_auth_request() {
        let expected = decode_hex_str(
            "01000001091619820ad36f4d8aa7161516b1dea0770561646d696e0470617373",
        );
        let cluster = [
            0x16, 0x19, 0x82, 0x0a, 0xd3, 0x6f, 0x4d, 0x8a, 0xa7, 0x16, 0x15, 0x16, 0xb1,
            0xde, 0xa0, 0x77,
        ];
        let req = ClusterAuthRpc {
            cluster,
            user: "admin".to_string(),
            pwd: "pass".to_string(),
        };
        let protocol = ProtocolVersion::V16_0.boxed();
        let serialized = req.encode(protocol.as_ref()).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn parse_cluster_list_restart_schedule_v16() {
        let hex = include_str!(
            "../../../../artifacts/rac/v16/v16_20260226_cluster_list_restart_schedule_response.hex"
        );
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let clusters = parse_cluster_list_body(body, ProtocolVersion::V16_0).expect("parse body");
        assert_eq!(clusters.len(), 1);
        assert_eq!(
            clusters[0].allow_access_right_audit_events_recording,
            Some(0)
        );
        assert_eq!(clusters[0].ping_period, Some(59999));
        assert_eq!(clusters[0].ping_timeout, Some(65366));
        assert_eq!(
            clusters[0].restart_schedule_cron,
            Some("0 3 * * 6".to_string())
        );
    }

    #[test]
    fn parse_cluster_info_restart_schedule_v16() {
        let hex = include_str!(
            "../../../../artifacts/rac/v16/v16_20260226_cluster_info_restart_schedule_response.hex"
        );
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let cluster = parse_cluster_info_body(body, ProtocolVersion::V16_0).expect("parse body");
        assert_eq!(cluster.allow_access_right_audit_events_recording, Some(0));
        assert_eq!(cluster.ping_period, Some(59999));
        assert_eq!(cluster.ping_timeout, Some(65366));
        assert_eq!(
            cluster.restart_schedule_cron,
            Some("0 3 * * 6".to_string())
        );
    }

    // Additional cluster list/info capture assertions should be added when artifacts are present.
}
