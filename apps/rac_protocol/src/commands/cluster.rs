use serde::Serialize;

use crate::client::{RacClient, RacRequest};
use crate::codec::RecordCursor;
use crate::error::Result;
use crate::Uuid16;

use super::rpc_body;

const CLUSTER_TAIL_SIZE: usize = 32;

#[derive(Debug, Serialize, Clone)]
pub struct ClusterAdminRecord {
    pub name: String,
    pub unknown_tag: u8,
    pub unknown_flags: u32,
    pub unknown_tail: [u8; 3],
}

#[derive(Debug, Serialize)]
pub struct ClusterAdminListResp {
    pub admins: Vec<ClusterAdminRecord>,
    pub raw_payload: Option<Vec<u8>>,
}

#[derive(Debug, Serialize)]
pub struct ClusterSummary {
    pub uuid: Uuid16,
    pub host: Option<String>,
    pub display_name: Option<String>,
    pub port: Option<u16>,
    pub expiration_timeout: Option<u32>,
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

pub fn cluster_list(client: &mut RacClient) -> Result<ClusterListResp> {
    let reply = client.call(RacRequest::ClusterList)?;
    let body = rpc_body(&reply)?;
    let clusters = parse_cluster_list_body(body)?;
    Ok(ClusterListResp {
        clusters,
        raw_payload: Some(reply),
    })
}

pub fn cluster_info(client: &mut RacClient, cluster: Uuid16) -> Result<ClusterInfoResp> {
    let reply = client.call(RacRequest::ClusterInfo { cluster })?;
    let body = rpc_body(&reply)?;
    let mut cursor = RecordCursor::new(body, 0);
    let summary = parse_cluster_record(&mut cursor)?;
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
        let name = cursor.take_str8()?;
        let unknown_tag = cursor.take_u8()?;
        let unknown_flags = cursor.take_u32_be()?;
        let tail = cursor.take_bytes(3)?;
        let unknown_tail = [tail[0], tail[1], tail[2]];
        admins.push(ClusterAdminRecord {
            name,
            unknown_tag,
            unknown_flags,
            unknown_tail,
        });
    }
    Ok(admins)
}

fn parse_cluster_list_body(body: &[u8]) -> Result<Vec<ClusterSummary>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut clusters = Vec::with_capacity(count);
    for _ in 0..count {
        clusters.push(parse_cluster_record(&mut cursor)?);
    }
    Ok(clusters)
}

fn parse_cluster_record(cursor: &mut RecordCursor<'_>) -> Result<ClusterSummary> {
    let uuid = cursor.take_uuid()?;
    let expiration_timeout = cursor.take_u32_be()?;
    let host = cursor.take_str8()?;
    let _unknown_u32 = cursor.take_u32_be()?;
    let port = cursor.take_u16_be()?;
    let _unknown_u64 = cursor.take_u64_be()?;
    let display_name = cursor.take_str8()?;
    let _tail = cursor.take_bytes(CLUSTER_TAIL_SIZE)?;
    Ok(ClusterSummary {
        uuid,
        host: Some(host),
        display_name: Some(display_name),
        port: Some(port),
        expiration_timeout: Some(expiration_timeout),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        let s = input.trim();
        assert!(s.len() % 2 == 0, "hex length must be even");
        let mut out = Vec::with_capacity(s.len() / 2);
        let bytes = s.as_bytes();
        for i in (0..bytes.len()).step_by(2) {
            let hi = (bytes[i] as char).to_digit(16).expect("hex hi");
            let lo = (bytes[i + 1] as char).to_digit(16).expect("hex lo");
            out.push(((hi << 4) | lo) as u8);
        }
        out
    }

    #[test]
    fn parse_cluster_admin_list_from_capture() {
        let hex = include_str!("../../../../artifacts/cluster_admin_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let admins = parse_cluster_admin_list_body(body).expect("parse list");

        assert_eq!(admins.len(), 1);
        assert_eq!(admins[0].name, "cadmin");
        assert_eq!(admins[0].unknown_tag, 0);
        assert_eq!(admins[0].unknown_flags, 0x03efbfbd);
        assert_eq!(admins[0].unknown_tail, [0x01, 0x00, 0x00]);
    }

    // Additional cluster list/info capture assertions should be added when artifacts are present.
}
