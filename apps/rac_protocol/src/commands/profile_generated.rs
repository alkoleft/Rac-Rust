use crate::Uuid16;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;

pub const METHOD_PROFILE_LIST_REQ: u8 = 0x59;
pub const METHOD_PROFILE_LIST_RESP: u8 = 0x5a;

#[derive(Debug, Serialize, Clone)]
pub struct ProfileRecord {
    pub profile: Uuid16,
}

impl ProfileRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let profile = cursor.take_uuid()?;
        Ok(Self {
            profile,
        })
    }
}

pub struct ProfileListRpc {
    pub cluster: Uuid16,
}

impl crate::rpc::Request for ProfileListRpc {
    type Response = ProfileListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_PROFILE_LIST_META
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




pub const RPC_PROFILE_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_PROFILE_LIST_REQ,
    method_resp: Some(METHOD_PROFILE_LIST_RESP),
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
    fn profile_list_response_empty_hex() {
        let hex = include_str!("../../../../artifacts/rac/profile_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let items = crate::commands::parse_list_u8(body, ProfileRecord::decode).expect("parse body");
        assert_eq!(items.len(), 0);
    }

}
