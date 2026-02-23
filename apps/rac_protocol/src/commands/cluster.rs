use serde::Serialize;

use crate::client::{RacClient, RacProtocolVersion, RacRequest};
use crate::error::{RacError, Result};
use crate::Uuid16;

use super::{parse_ack_payload, parse_list_u8, parse_list_u8_tail, rpc_body};

mod generated {
    include!("cluster_generated.rs");
}

pub use generated::{
    ClusterAdminRecord,
    ClusterAdminRegisterRequest,
    ClusterAuthRequest,
    ClusterIdRequest,
    ClusterRecord,
    parse_cluster_info_body,
    RpcMethodMeta,
    RPC_CLUSTER_ADMIN_LIST_META,
    RPC_CLUSTER_ADMIN_REGISTER_META,
    RPC_CLUSTER_AUTH_META,
    RPC_CLUSTER_INFO_META,
    RPC_CLUSTER_LIST_META,
};

#[derive(Debug, Serialize)]
pub struct ClusterAdminListResp {
    pub admins: Vec<ClusterAdminRecord>,
}

#[derive(Debug, Serialize)]
pub struct ClusterAuthResp {
    pub acknowledged: bool,
}

#[derive(Debug, Serialize)]
pub struct ClusterAdminRegisterResp {
    pub acknowledged: bool,
}

#[derive(Debug, Serialize)]
pub struct ClusterListResp {
    pub clusters: Vec<ClusterRecord>,
}

#[derive(Debug, Serialize)]
pub struct ClusterInfoResp {
    pub cluster: ClusterRecord,
}

pub fn cluster_auth(
    client: &mut RacClient,
    cluster: Uuid16,
    user: &str,
    pwd: &str,
) -> Result<ClusterAuthResp> {
    let reply = client.call(RacRequest::ClusterAuth {
        cluster,
        user: user.to_string(),
        pwd: pwd.to_string(),
    })?;
    let acknowledged = parse_ack_payload(&reply, "cluster auth expected ack")?;
    if !acknowledged {
        return Err(RacError::Decode("cluster auth expected ack"));
    }
    Ok(ClusterAuthResp {
        acknowledged,
    })
}

pub fn cluster_admin_list(
    client: &mut RacClient,
    cluster: Uuid16,
) -> Result<ClusterAdminListResp> {
    let reply = client.call(RacRequest::ClusterAdminList { cluster })?;
    Ok(ClusterAdminListResp {
        admins: parse_list_u8(rpc_body(&reply)?, ClusterAdminRecord::decode)?,
    })
}

pub fn cluster_admin_register(
    client: &mut RacClient,
    cluster: Uuid16,
    name: String,
    descr: String,
    pwd: String,
    auth_flags: u8,
) -> Result<ClusterAdminRegisterResp> {
    let reply = client.call(RacRequest::ClusterAdminRegister {
        cluster,
        name,
        descr,
        pwd,
        auth_flags,
    })?;
    let acknowledged = parse_ack_payload(&reply, "cluster admin register expected ack")?;
    if !acknowledged {
        return Err(RacError::Decode("cluster admin register expected ack"));
    }
    Ok(ClusterAdminRegisterResp {
        acknowledged,
    })
}

pub fn cluster_list(client: &mut RacClient) -> Result<ClusterListResp> {
    let reply = client.call(RacRequest::ClusterList)?;
    let body = rpc_body(&reply)?;
    let protocol_version = client.protocol_version();
    let tail_len = cluster_tail_len(protocol_version);
    let clusters = parse_list_u8_tail(body, tail_len, ClusterRecord::decode)?;
    Ok(ClusterListResp {
        clusters,
    })
}

pub fn cluster_info(client: &mut RacClient, cluster: Uuid16) -> Result<ClusterInfoResp> {
    let reply = client.call(RacRequest::ClusterInfo { cluster })?;
    let body = rpc_body(&reply)?;
    let protocol_version = client.protocol_version();
    let tail_len = cluster_tail_len(protocol_version);
    let summary = parse_cluster_info_body(body, tail_len)?;
    Ok(ClusterInfoResp {
        cluster: summary,
    })
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
        let admins = parse_list_u8(body, ClusterAdminRecord::decode).expect("parse list");

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

    // Additional cluster list/info capture assertions should be added when artifacts are present.
}
