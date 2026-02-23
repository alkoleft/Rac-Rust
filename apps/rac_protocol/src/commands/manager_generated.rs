use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::metadata::RpcMethodMeta;

#[derive(Debug, Serialize, Clone)]
pub struct ManagerRecord {
    pub manager: Uuid16,
    pub descr: String,
    pub host: String,
    pub using: u32,
    pub port: u16,
    pub pid: String,
}

impl ManagerRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let manager = cursor.take_uuid()?;
        let descr = cursor.take_str8()?;
        let host = cursor.take_str8()?;
        let using = cursor.take_u32_be()?;
        let port = cursor.take_u16_be()?;
        let pid = cursor.take_str8()?;
        Ok(Self {
            manager,
            descr,
            host,
            using,
            port,
            pid,
        })
    }
}

pub const RPC_MANAGER_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_MANAGER_LIST_REQ,
    method_resp: Some(crate::rac_wire::METHOD_MANAGER_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_MANAGER_INFO_META: RpcMethodMeta = RpcMethodMeta {
    method_req: crate::rac_wire::METHOD_MANAGER_INFO_REQ,
    method_resp: Some(crate::rac_wire::METHOD_MANAGER_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};


pub fn parse_manager_info_body(body: &[u8]) -> Result<ManagerRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("manager info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    ManagerRecord::decode(&mut cursor)
}

#[derive(Debug, Clone)]
pub struct ManagerListRequest {
    pub cluster: Uuid16,
}

impl ManagerListRequest {
    pub fn encoded_len(&self) -> usize {
        16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ManagerInfoRequest {
    pub cluster: Uuid16,
    pub manager: Uuid16,
}

impl ManagerInfoRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.manager);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::rpc_body;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn manager_list_response_hex() {
        let hex = include_str!("../../../../artifacts/rac/manager_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let items = crate::commands::parse_list_u8(body, ManagerRecord::decode).expect("parse body");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].host, "alko-home");
        assert_eq!(items[0].using, 1);
        assert_eq!(items[0].port, 0x605);
        assert_eq!(items[0].pid, "314037");
    }

    #[test]
    fn manager_info_response_hex() {
        let hex = include_str!("../../../../artifacts/rac/manager_info_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let record = parse_manager_info_body(body).expect("parse body");
        assert_eq!(record.host, "alko-home");
        assert_eq!(record.using, 1);
        assert_eq!(record.port, 0x605);
        assert_eq!(record.pid, "314037");
    }

}
