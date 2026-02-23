use serde::Serialize;

use crate::client::RacClient;
use crate::error::Result;
use crate::protocol::{ProtocolCodec, ProtocolVersion};
use crate::rpc::{AckResponse, Meta, Request, Response};
use crate::rpc::decode_utils::rpc_body;
use crate::rac_wire::{
    METHOD_CLUSTER_ADMIN_LIST_REQ, METHOD_CLUSTER_ADMIN_LIST_RESP,
    METHOD_CLUSTER_ADMIN_REGISTER_REQ, METHOD_CLUSTER_AUTH, METHOD_CLUSTER_INFO_REQ,
    METHOD_CLUSTER_INFO_RESP, METHOD_CLUSTER_LIST_REQ, METHOD_CLUSTER_LIST_RESP,
};
use crate::Uuid16;

use super::{parse_list_u8, parse_list_u8_tail};

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
};

struct ClusterAuthRpc {
    user: String,
    pwd: String,
    cluster: Uuid16,
}

impl Request for ClusterAuthRpc {
    type Response = AckResponse;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_CLUSTER_AUTH,
            method_resp: None,
            requires_cluster_context: false,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let req = ClusterAuthRequest {
            cluster: self.cluster,
            user: self.user.clone(),
            pwd: self.pwd.clone(),
        };
        let mut out = Vec::with_capacity(req.encoded_len());
        req.encode_body(&mut out)?;
        Ok(out)
    }
}

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

struct ClusterAdminListRpc {
    cluster: Uuid16,
}

impl Request for ClusterAdminListRpc {
    type Response = Vec<ClusterAdminRecord>;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_CLUSTER_ADMIN_LIST_REQ,
            method_resp: Some(METHOD_CLUSTER_ADMIN_LIST_RESP),
            requires_cluster_context: false,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let req = ClusterIdRequest { cluster: self.cluster };
        let mut out = Vec::with_capacity(req.encoded_len());
        req.encode_body(&mut out)?;
        Ok(out)
    }
}

impl Response for Vec<ClusterAdminRecord> {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        parse_list_u8(body, ClusterAdminRecord::decode)
    }
}

pub fn cluster_admin_list(
    client: &mut RacClient,
    cluster: Uuid16,
) -> Result<Vec<ClusterAdminRecord>> {
    client.call_typed(ClusterAdminListRpc { cluster })
}

struct ClusterAdminRegisterRpc {
    cluster: Uuid16,
    name: String,
    descr: String,
    pwd: String,
    auth_flags: u8,
}

impl Request for ClusterAdminRegisterRpc {
    type Response = AckResponse;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_CLUSTER_ADMIN_REGISTER_REQ,
            method_resp: None,
            requires_cluster_context: false,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let req = ClusterAdminRegisterRequest {
            cluster: self.cluster,
            name: self.name.clone(),
            descr: self.descr.clone(),
            pwd: self.pwd.clone(),
            auth_flags: self.auth_flags,
        };
        let mut out = Vec::with_capacity(req.encoded_len());
        req.encode_body(&mut out)?;
        Ok(out)
    }
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
        auth_flags,
    })?;
    Ok(resp.acknowledged)
}

struct ClusterListRpc;

#[derive(Debug, Serialize)]
struct ClusterListResp {
    clusters: Vec<ClusterRecord>,
}

impl Request for ClusterListRpc {
    type Response = ClusterListResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_CLUSTER_LIST_REQ,
            method_resp: Some(METHOD_CLUSTER_LIST_RESP),
            requires_cluster_context: false,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

impl Response for ClusterListResp {
    fn decode(payload: &[u8], codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let tail_len = cluster_tail_len(codec.protocol_version());
        let clusters = parse_list_u8_tail(body, tail_len, ClusterRecord::decode)?;
        Ok(Self { clusters })
    }
}

pub fn cluster_list(client: &mut RacClient) -> Result<Vec<ClusterRecord>> {
    let resp = client.call_typed(ClusterListRpc)?;
    Ok(resp.clusters)
}

struct ClusterInfoRpc {
    cluster: Uuid16,
}

#[derive(Debug, Serialize)]
struct ClusterInfoResp {
    cluster: ClusterRecord,
}

impl Request for ClusterInfoRpc {
    type Response = ClusterInfoResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_CLUSTER_INFO_REQ,
            method_resp: Some(METHOD_CLUSTER_INFO_RESP),
            requires_cluster_context: false,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let req = ClusterIdRequest { cluster: self.cluster };
        let mut out = Vec::with_capacity(req.encoded_len());
        req.encode_body(&mut out)?;
        Ok(out)
    }
}

impl Response for ClusterInfoResp {
    fn decode(payload: &[u8], codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        let tail_len = cluster_tail_len(codec.protocol_version());
        let cluster = parse_cluster_info_body(body, tail_len)?;
        Ok(Self { cluster })
    }
}

pub fn cluster_info(client: &mut RacClient, cluster: Uuid16) -> Result<ClusterRecord> {
    let resp = client.call_typed(ClusterInfoRpc { cluster })?;
    Ok(resp.cluster)
}

fn cluster_tail_len(protocol_version: ProtocolVersion) -> usize {
    match protocol_version {
        ProtocolVersion::V11_0 => 0,
        ProtocolVersion::V16_0 => 32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ProtocolVersion;
    use crate::rpc::Request;
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
        let req = ClusterAdminRegisterRpc {
            cluster,
            name: "test_admin1".to_string(),
            descr: "test admin".to_string(),
            pwd: "test_pass1".to_string(),
            auth_flags: 0x01,
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

    // Additional cluster list/info capture assertions should be added when artifacts are present.
}
