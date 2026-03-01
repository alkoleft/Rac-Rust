use crate::client::RacClient;
use crate::error::Result;
use crate::Uuid16;

mod generated {
    include!("cluster_generated.rs");
}

pub use generated::{
    ClusterAdminRecord,
    ClusterAdminListResp,
    ClusterAdminListRpc,
    ClusterAdminRemoveRpc,
    ClusterAdminRegisterRpc,
    ClusterAuthRpc,
    ClusterInfoResp,
    ClusterInfoRpc,
    ClusterListResp,
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
    auth_pwd: u8,
    auth_os: u8,
) -> Result<bool> {
    let resp = client.call_typed(ClusterAdminRegisterRpc {
        cluster,
        name,
        descr,
        pwd,
        auth_pwd,
        auth_os,
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

pub fn cluster_list(client: &mut RacClient) -> Result<Vec<ClusterRecord>> {
    let resp = client.call_typed(ClusterListRpc)?;
    Ok(resp.clusters)
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
        let hex = include_str!(
            "../../../../artifacts/rac/v16/v16_20260226_053425_cluster_admin_list_response_rpc.hex"
        );
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let admins = parse_list_u8(body, |cursor| {
            ClusterAdminRecord::decode(cursor, ProtocolVersion::V16_0)
        })
        .expect("parse list");

        assert_eq!(admins.len(), 3);
        assert_eq!(admins[0].name, "cadmin");
        assert_eq!(admins[0].descr, "");
        assert_eq!(admins[0].record_marker, 0x03efbfbd);
        assert_eq!(admins[0].auth_pwd, 0x01);
        assert_eq!(admins[0].auth_os, 0x00);
        assert_eq!(admins[0].os_user, "");
        assert_eq!(admins[1].name, "codex_cadmin_pwd_20260226_053425");
        assert_eq!(admins[1].descr, "Codex cluster admin pwd");
        assert_eq!(admins[1].auth_pwd, 0x01);
        assert_eq!(admins[1].auth_os, 0x00);
        assert_eq!(admins[1].os_user, "");
        assert_eq!(admins[2].name, "codex_cadmin_os_20260226_053425");
        assert_eq!(admins[2].descr, "Codex cluster admin os");
        assert_eq!(admins[2].auth_pwd, 0x01);
        assert_eq!(admins[2].auth_os, 0x01);
        assert_eq!(admins[2].os_user, "codex_os_user");
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
            auth_pwd: 0x01,
            auth_os: 0x01,
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
