use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;

pub const METHOD_MANAGER_LIST_REQ: u8 = 0x12;
pub const METHOD_MANAGER_LIST_RESP: u8 = 0x13;
pub const METHOD_MANAGER_INFO_REQ: u8 = 0x14;
pub const METHOD_MANAGER_INFO_RESP: u8 = 0x15;

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

pub struct ManagerListRpc {
    pub cluster: Uuid16,
}

impl crate::rpc::Request for ManagerListRpc {
    type Response = ManagerListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_MANAGER_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16);
        out.extend_from_slice(&self.cluster);
        Ok(out)
    }
}

pub struct ManagerInfoRpc {
    pub cluster: Uuid16,
    pub manager: Uuid16,
}

impl crate::rpc::Request for ManagerInfoRpc {
    type Response = ManagerInfoResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_MANAGER_INFO_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        Some(self.cluster)
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(16 + 16);
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&self.manager);
        Ok(out)
    }
}


#[derive(Debug, Serialize)]
pub struct ManagerListResp {
    pub managers: Vec<ManagerRecord>,
}

impl crate::rpc::Response for ManagerListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        Ok(Self {
            managers: crate::commands::parse_list_u8(body, ManagerRecord::decode)?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ManagerInfoResp {
    pub record: ManagerRecord,
}

impl crate::rpc::Response for ManagerInfoResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let record = parse_manager_info_body(body)?;
        Ok(Self {
            record: record,
        })
    }
}


pub fn parse_manager_info_body(body: &[u8]) -> Result<ManagerRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("manager info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    ManagerRecord::decode(&mut cursor)
}


pub const RPC_MANAGER_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_MANAGER_LIST_REQ,
    method_resp: Some(METHOD_MANAGER_LIST_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

pub const RPC_MANAGER_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_MANAGER_INFO_REQ,
    method_resp: Some(METHOD_MANAGER_INFO_RESP),
    requires_cluster_context: true,
    requires_infobase_context: false,
};

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
