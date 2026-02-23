use serde::Serialize;

use crate::client::RacClient;
use crate::error::Result;
use crate::protocol::ProtocolCodec;
use crate::rpc::{Meta, Request, Response};
use crate::rpc::decode_utils::rpc_body;
use crate::rac_wire::{
    METHOD_MANAGER_INFO_REQ, METHOD_MANAGER_INFO_RESP, METHOD_MANAGER_LIST_REQ,
    METHOD_MANAGER_LIST_RESP,
};
use crate::Uuid16;

use super::parse_list_u8;

mod generated {
    include!("manager_generated.rs");
}

pub use generated::ManagerRecord;
use generated::parse_manager_info_body;

#[derive(Debug, Serialize)]
pub struct ManagerListResp {
    pub managers: Vec<ManagerRecord>,
}

#[derive(Debug, Serialize)]
pub struct ManagerInfoResp {
    pub manager: ManagerRecord,
}

struct ManagerListRpc {
    cluster: Uuid16,
}

impl Request for ManagerListRpc {
    type Response = ManagerListResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_MANAGER_LIST_REQ,
            method_resp: Some(METHOD_MANAGER_LIST_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        Ok(self.cluster.to_vec())
    }
}

impl Response for ManagerListResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        Ok(Self {
            managers: parse_list_u8(body, ManagerRecord::decode)?,
        })
    }
}

pub fn manager_list(client: &mut RacClient, cluster: Uuid16) -> Result<ManagerListResp> {
    client.call_typed(ManagerListRpc { cluster })
}

struct ManagerInfoRpc {
    cluster: Uuid16,
    manager: Uuid16,
}

impl Request for ManagerInfoRpc {
    type Response = ManagerInfoResp;

    fn meta(&self) -> Meta {
        Meta {
            method_req: METHOD_MANAGER_INFO_REQ,
            method_resp: Some(METHOD_MANAGER_INFO_RESP),
            requires_cluster_context: true,
            requires_infobase_context: false,
        }
    }

    fn cluster(&self) -> Option<Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(32);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.manager);
        Ok(out)
    }
}

impl Response for ManagerInfoResp {
    fn decode(payload: &[u8], _codec: &dyn ProtocolCodec) -> Result<Self> {
        let body = rpc_body(payload)?;
        Ok(Self {
            manager: parse_manager_info_body(body)?,
        })
    }
}

pub fn manager_info(
    client: &mut RacClient,
    cluster: Uuid16,
    manager: Uuid16,
) -> Result<ManagerInfoResp> {
    client.call_typed(ManagerInfoRpc { cluster, manager })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::rpc_body;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    fn expected_descr() -> String {
        let bytes = vec![
            0xd0, 0x93, 0xd0, 0xbb, 0xd0, 0xb0, 0xd0, 0xb2, 0xd0, 0xbd, 0xd1, 0x8b, 0xd0,
            0xb9, 0x20, 0xd0, 0xbc, 0xd0, 0xb5, 0xd0, 0xbd, 0xd0, 0xb5, 0xd0, 0xb4, 0xd0,
            0xb6, 0xd0, 0xb5, 0xd1, 0x80, 0x20, 0xd0, 0xba, 0xd0, 0xbb, 0xd0, 0xb0, 0xd1,
            0x81, 0xd1, 0x82, 0xd0, 0xb5, 0xd1, 0x80, 0xd0, 0xb0,
        ];
        String::from_utf8(bytes).expect("descr utf8")
    }

    #[test]
    fn parse_manager_list_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/manager_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let managers = parse_list_u8(body, ManagerRecord::decode).expect("parse list");

        assert_eq!(managers.len(), 1);
        assert_eq!(
            managers[0].manager,
            [
                0x39, 0x85, 0xf9, 0x06, 0xba, 0x9d, 0x48, 0x4f, 0xae, 0xbc, 0x3e, 0x1c, 0x6f,
                0x1a, 0x8f, 0xe8,
            ]
        );
        assert_eq!(managers[0].descr, expected_descr());
        assert_eq!(managers[0].host, "alko-home");
        assert_eq!(managers[0].using, 1);
        assert_eq!(managers[0].port, 0x0605);
        assert_eq!(managers[0].pid, "314037");
    }

    #[test]
    fn parse_manager_info_from_capture() {
        let hex = include_str!("../../../../artifacts/rac/manager_info_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let manager = parse_manager_info_body(body).expect("parse info");

        assert_eq!(
            manager.manager,
            [
                0x39, 0x85, 0xf9, 0x06, 0xba, 0x9d, 0x48, 0x4f, 0xae, 0xbc, 0x3e, 0x1c, 0x6f,
                0x1a, 0x8f, 0xe8,
            ]
        );
        assert_eq!(manager.descr, expected_descr());
        assert_eq!(manager.host, "alko-home");
        assert_eq!(manager.using, 1);
        assert_eq!(manager.port, 0x0605);
        assert_eq!(manager.pid, "314037");
    }
}
