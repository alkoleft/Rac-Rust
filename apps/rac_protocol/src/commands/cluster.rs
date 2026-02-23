use crate::client::{RacClient, RacProtocolVersion, RacRequest};
use crate::error::Result;
use crate::Uuid16;

use super::{call_body, expect_ack, parse_list_u8, parse_list_u8_tail};

mod generated {
    include!("cluster_generated.rs");
}

pub use generated::{
    parse_cluster_info_body,
    ClusterAdminRecord,
    ClusterAdminRegisterRequest,
    ClusterAuthRequest,
    ClusterIdRequest,
    ClusterRecord,
    RPC_CLUSTER_ADMIN_LIST_META,
    RPC_CLUSTER_ADMIN_REGISTER_META,
    RPC_CLUSTER_AUTH_META,
    RPC_CLUSTER_INFO_META,
    RPC_CLUSTER_LIST_META,
    rpc_metadata,
};

pub fn cluster_auth(
    client: &mut RacClient,
    cluster: Uuid16,
    user: &str,
    pwd: &str,
) -> Result<bool> {
    let reply = client.call(RacRequest::ClusterAuth {
        cluster,
        user: user.to_string(),
        pwd: pwd.to_string(),
    })?;
    expect_ack(&reply, "cluster auth expected ack")?;
    Ok(true)
}

pub fn cluster_admin_list(
    client: &mut RacClient,
    cluster: Uuid16,
) -> Result<Vec<ClusterAdminRecord>> {
    let body = call_body(client, RacRequest::ClusterAdminList { cluster })?;
    parse_list_u8(&body, ClusterAdminRecord::decode)
}

pub fn cluster_admin_register(
    client: &mut RacClient,
    cluster: Uuid16,
    name: String,
    descr: String,
    pwd: String,
    auth_flags: u8,
) -> Result<bool> {
    let reply = client.call(RacRequest::ClusterAdminRegister {
        cluster,
        name,
        descr,
        pwd,
        auth_flags,
    })?;
    expect_ack(&reply, "cluster admin register expected ack")?;
    Ok(true)
}

pub fn cluster_list(client: &mut RacClient) -> Result<Vec<ClusterRecord>> {
    let body = call_body(client, RacRequest::ClusterList)?;
    let protocol_version = client.protocol_version();
    let tail_len = cluster_tail_len(protocol_version);
    parse_list_u8_tail(&body, tail_len, ClusterRecord::decode)
}

pub fn cluster_info(client: &mut RacClient, cluster: Uuid16) -> Result<ClusterRecord> {
    let body = call_body(client, RacRequest::ClusterInfo { cluster })?;
    let protocol_version = client.protocol_version();
    let tail_len = cluster_tail_len(protocol_version);
    parse_cluster_info_body(&body, tail_len)
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
    use crate::commands::rpc_body;

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
