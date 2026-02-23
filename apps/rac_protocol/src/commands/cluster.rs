use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::rpc_body;

mod generated {
    include!("cluster_generated.rs");
}

pub use generated::ClusterAdminRecord;

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
pub struct ClusterSummary {
    pub uuid: Uuid16,
    pub host: Option<String>,
    pub display_name: Option<String>,
    pub port: Option<u16>,
    pub expiration_timeout: Option<u32>,
    pub lifetime_limit: Option<u32>,
    pub security_level: Option<u32>,
    pub session_fault_tolerance_level: Option<u32>,
    pub load_balancing_mode: Option<u32>,
    pub errors_count_threshold: Option<u32>,
    pub kill_problem_processes: Option<u8>,
    pub kill_by_memory_with_dump: Option<u8>,
}

#[derive(Debug, Serialize)]
pub struct ClusterListResp {
    pub clusters: Vec<ClusterSummary>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ClusterInfoResp {
    pub cluster: ClusterSummary,
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
    let tail_len = cluster_tail_len(client.protocol_name());
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
    let tail_len = cluster_tail_len(client.protocol_name());
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

fn parse_cluster_list_body(body: &[u8], tail_len: usize) -> Result<Vec<ClusterSummary>> {
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

fn parse_cluster_record(cursor: &mut RecordCursor<'_>, tail_len: usize) -> Result<ClusterSummary> {
    let uuid = cursor.take_uuid()?;
    let expiration_timeout = cursor.take_u32_be()?;
    let host = cursor.take_str8()?;
    let lifetime_limit = cursor.take_u32_be()?;
    let port = cursor.take_u16_be()?;
    let _unknown_u64 = cursor.take_u64_be()?;
    let display_name = cursor.take_str8()?;
    let (security_level, session_fault_tolerance_level, load_balancing_mode, errors_count_threshold, kill_problem_processes, kill_by_memory_with_dump) =
        if tail_len == 18 {
            let security_level = cursor.take_u32_be()?;
            let session_fault_tolerance_level = cursor.take_u32_be()?;
            let load_balancing_mode = cursor.take_u32_be()?;
            let errors_count_threshold = cursor.take_u32_be()?;
            let kill_problem_processes = cursor.take_u8()?;
            let kill_by_memory_with_dump = cursor.take_u8()?;
            (
                Some(security_level),
                Some(session_fault_tolerance_level),
                Some(load_balancing_mode),
                Some(errors_count_threshold),
                Some(kill_problem_processes),
                Some(kill_by_memory_with_dump),
            )
        } else {
            let _tail = cursor.take_bytes(tail_len)?;
            (None, None, None, None, None, None)
        };
    Ok(ClusterSummary {
        uuid,
        host: Some(host),
        display_name: Some(display_name),
        port: Some(port),
        expiration_timeout: Some(expiration_timeout),
        lifetime_limit: Some(lifetime_limit),
        security_level,
        session_fault_tolerance_level,
        load_balancing_mode,
        errors_count_threshold,
        kill_problem_processes,
        kill_by_memory_with_dump,
    })
}

fn cluster_tail_len(protocol_name: &str) -> usize {
    match protocol_name {
        "v11.0" => 18,
        _ => 32,
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
    fn parse_cluster_list_custom_capture() {
        let hex = include_str!("../../../../artifacts/rac/cluster_list_response_custom.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let clusters = parse_cluster_list_body(body, 18).expect("parse list");

        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].lifetime_limit, Some(1111));
        assert_eq!(clusters[0].security_level, Some(3));
        assert_eq!(clusters[0].session_fault_tolerance_level, Some(4));
        assert_eq!(clusters[0].load_balancing_mode, Some(1));
        assert_eq!(clusters[0].errors_count_threshold, Some(0));
        assert_eq!(clusters[0].kill_problem_processes, Some(0));
        assert_eq!(clusters[0].kill_by_memory_with_dump, Some(1));
    }

    #[test]
    fn parse_cluster_list_flags_capture() {
        let hex = include_str!("../../../../artifacts/rac/cluster_list_response_flags.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let clusters = parse_cluster_list_body(body, 18).expect("parse list");

        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].kill_problem_processes, Some(1));
        assert_eq!(clusters[0].kill_by_memory_with_dump, Some(0));
    }

    // Additional cluster list/info capture assertions should be added when artifacts are present.
}
