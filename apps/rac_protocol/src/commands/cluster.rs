use serde::Serialize;

use crate::client::{RacClient, RacProtocolVersion, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

mod generated {
    include!("cluster_generated.rs");
}

pub use generated::{ClusterAdminRecord, ClusterRecord};

#[derive(Debug, Serialize)]
pub struct ClusterAdminListResp {
    pub admins: Vec<ClusterAdminRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClusterAdminRegisterReq {
    pub name: String,
    pub descr: String,
    pub pwd: String,
    pub auth_flags: u8,
}

#[derive(Debug, Serialize)]
pub struct ClusterAdminRegisterResp {
    pub acknowledged: bool,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ClusterListResp {
    pub clusters: Vec<ClusterRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ClusterInfoResp {
    pub cluster: ClusterRecord,
    pub raw_payload: Option<Vec<u8>>,
}

pub fn cluster_admin_list(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
) -> Result<ClusterAdminListResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::ClusterAdminList { cluster })?;
    Ok(ClusterAdminListResp {
        admins: parse_cluster_admin_list_body(rpc_body(&reply)?)?,
        raw_payload: Some(reply),
    })
}

pub fn cluster_admin_register(
    client: &mut RacClient,
    cluster: Uuid16,
    cluster_user: &str,
    cluster_pwd: &str,
    req: ClusterAdminRegisterReq,
) -> Result<ClusterAdminRegisterResp> {
    client.call(RacRequest::ClusterAuth {
        cluster,
        user: cluster_user.to_string(),
        pwd: cluster_pwd.to_string(),
    })?;
    let reply = client.call(RacRequest::ClusterAdminRegister {
        cluster,
        name: req.name,
        descr: req.descr,
        pwd: req.pwd,
        auth_flags: req.auth_flags,
    })?;
    let acknowledged = is_ack(&reply);
    if !acknowledged {
        return Err(RacError::Decode("cluster admin register expected ack"));
    }
    Ok(ClusterAdminRegisterResp {
        acknowledged,
        raw_payload: Some(reply),
    })
}

pub fn cluster_list(client: &mut RacClient) -> Result<ClusterListResp> {
    let reply = client.call(RacRequest::ClusterList)?;
    let body = rpc_body(&reply)?;
    let protocol_version = client.protocol_version();
    let tail_len = cluster_tail_len(protocol_version);
    let clusters = parse_cluster_list_body(body, tail_len)?;
    Ok(ClusterListResp {
        clusters,
        raw_payload: Some(reply),
    })
}

pub fn cluster_info(client: &mut RacClient, cluster: Uuid16) -> Result<ClusterInfoResp> {
    let reply = client.call(RacRequest::ClusterInfo { cluster })?;
    let body = rpc_body(&reply)?;
    let mut cursor = RecordCursor::new(body, 0);
    let protocol_version = client.protocol_version();
    let tail_len = cluster_tail_len(protocol_version);
    let summary = parse_cluster_record(&mut cursor, tail_len)?;
    Ok(ClusterInfoResp {
        cluster: summary,
        raw_payload: Some(reply),
    })
}

fn parse_cluster_admin_list_body(body: &[u8]) -> Result<Vec<ClusterAdminRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut admins = Vec::with_capacity(count);
    for _ in 0..count {
        admins.push(ClusterAdminRecord::decode(&mut cursor)?);
    }
    Ok(admins)
}

fn is_ack(payload: &[u8]) -> bool {
    payload == [0x01, 0x00, 0x00, 0x00]
}

fn parse_cluster_list_body(
    body: &[u8],
    tail_len: usize,
) -> Result<Vec<ClusterRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut clusters = Vec::with_capacity(count);
    for _ in 0..count {
        clusters.push(parse_cluster_record(&mut cursor, tail_len)?);
    }
    Ok(clusters)
}

fn parse_cluster_record(cursor: &mut RecordCursor<'_>, tail_len: usize) -> Result<ClusterRecord> {
    let record = ClusterRecord::decode(cursor)?;
    if tail_len != 0 {
        let _tail = cursor.take_bytes(tail_len)?;
    }
    Ok(record)
}

fn cluster_tail_len(protocol_version: RacProtocolVersion) -> usize {
    match protocol_version {
        RacProtocolVersion::Auto => 32,
        RacProtocolVersion::V11_0 => 0,
        RacProtocolVersion::V16_0 => 32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::RacProtocolVersion;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn parse_cluster_admin_list_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/cluster_admin_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let admins = parse_cluster_admin_list_body(body).expect("parse list");

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
        let req = RacRequest::ClusterAdminRegister {
            cluster,
            name: "test_admin1".to_string(),
            descr: "test admin".to_string(),
            pwd: "test_pass1".to_string(),
            auth_flags: 0x01,
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
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
        let req = RacRequest::ClusterAdminList { cluster };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
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
        let req = RacRequest::ClusterAuth {
            cluster,
            user: "admin".to_string(),
            pwd: "pass".to_string(),
        };
        let protocol = RacProtocolVersion::V16_0.boxed();
        let serialized = protocol.serialize(req).expect("serialize");
        assert_eq!(serialized.payload, expected);
        assert_eq!(serialized.expect_method, None);
    }

    #[test]
    fn parse_cluster_list_custom_capture() {
        let hex = include_str!("../../../../artifacts/rac/cluster_list_response_custom.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let clusters = parse_cluster_list_body(body, 0).expect("parse list");

        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].lifetime_limit, 1111);
        assert_eq!(clusters[0].security_level, 3);
        assert_eq!(clusters[0].session_fault_tolerance_level, 4);
        assert_eq!(clusters[0].load_balancing_mode, 1);
        assert_eq!(clusters[0].errors_count_threshold, 0);
        assert_eq!(clusters[0].kill_problem_processes, 0);
        assert_eq!(clusters[0].kill_by_memory_with_dump, 1);
    }

    #[test]
    fn parse_cluster_list_flags_capture() {
        let hex = include_str!("../../../../artifacts/rac/cluster_list_response_flags.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let clusters = parse_cluster_list_body(body, 0).expect("parse list");

        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].kill_problem_processes, 1);
        assert_eq!(clusters[0].kill_by_memory_with_dump, 0);
    }

    // Additional cluster list/info capture assertions should be added when artifacts are present.
}
