use crate::Uuid16;
use crate::error::RacError;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::rac_wire::encode_with_len_u8;

#[derive(Debug, Serialize, Clone)]
pub struct ClusterAdminRecord {
    pub name: String,
    pub unknown_tag: u8,
    pub unknown_flags: u32,
    pub unknown_tail: [u8; 3],
}

impl ClusterAdminRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let name = cursor.take_str8()?;
        let unknown_tag = cursor.take_u8()?;
        let unknown_flags = cursor.take_u32_be()?;
        let unknown_tail = {
            let bytes = cursor.take_bytes(3)?;
            let value: [u8; 3] = bytes.as_slice().try_into().map_err(|_| RacError::Decode("bytes_fixed"))?;
            value
        };
        Ok(Self {
            name,
            unknown_tag,
            unknown_flags,
            unknown_tail,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ClusterRecord {
    pub uuid: Uuid16,
    pub expiration_timeout: u32,
    pub host: String,
    pub lifetime_limit: u32,
    pub port: u16,
    pub max_memory_size: u32,
    pub max_memory_time_limit: u32,
    pub display_name: String,
    pub security_level: u32,
    pub session_fault_tolerance_level: u32,
    pub load_balancing_mode: u32,
    pub errors_count_threshold: u32,
    pub kill_problem_processes: u8,
    pub kill_by_memory_with_dump: u8,
}

impl ClusterRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>) -> Result<Self> {
        let uuid = cursor.take_uuid()?;
        let expiration_timeout = cursor.take_u32_be()?;
        let host = cursor.take_str8()?;
        let lifetime_limit = cursor.take_u32_be()?;
        let port = cursor.take_u16_be()?;
        let max_memory_size = cursor.take_u32_be()?;
        let max_memory_time_limit = cursor.take_u32_be()?;
        let display_name = cursor.take_str8()?;
        let security_level = cursor.take_u32_be()?;
        let session_fault_tolerance_level = cursor.take_u32_be()?;
        let load_balancing_mode = cursor.take_u32_be()?;
        let errors_count_threshold = cursor.take_u32_be()?;
        let kill_problem_processes = cursor.take_u8()?;
        let kill_by_memory_with_dump = cursor.take_u8()?;
        Ok(Self {
            uuid,
            expiration_timeout,
            host,
            lifetime_limit,
            port,
            max_memory_size,
            max_memory_time_limit,
            display_name,
            security_level,
            session_fault_tolerance_level,
            load_balancing_mode,
            errors_count_threshold,
            kill_problem_processes,
            kill_by_memory_with_dump,
        })
    }
}

pub fn parse_cluster_admin_list_body(body: &[u8]) -> Result<Vec<ClusterAdminRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(ClusterAdminRecord::decode(&mut cursor)?);
    }
    Ok(out)
}

pub fn parse_cluster_list_body(body: &[u8], tail_len: usize) -> Result<Vec<ClusterRecord>> {
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body, 0);
    let count = cursor.take_u8()? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let record = ClusterRecord::decode(&mut cursor)?;
        if tail_len != 0 {
            let _tail = cursor.take_bytes(tail_len)?;
        }
        out.push(record);
    }
    Ok(out)
}

pub fn parse_cluster_info_body(body: &[u8], tail_len: usize) -> Result<ClusterRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("cluster info empty body"));
    }
    let mut cursor = RecordCursor::new(body, 0);
    let record = ClusterRecord::decode(&mut cursor)?;
    if tail_len != 0 {
        let _tail = cursor.take_bytes(tail_len)?;
    }
    Ok(record)
}


#[derive(Debug, Clone, Copy)]
pub struct RpcMethodMeta {
    pub method_req: u8,
    pub method_resp: Option<u8>,
    pub requires_cluster_context: bool,
    pub requires_infobase_context: bool,
}

pub const RPC_CLUSTER_AUTH_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 9,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_CLUSTER_ADMIN_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 2,
    method_resp: Some(3),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_CLUSTER_ADMIN_REGISTER_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 5,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_CLUSTER_LIST_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 11,
    method_resp: Some(12),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_CLUSTER_INFO_META: RpcMethodMeta = RpcMethodMeta {
    method_req: 13,
    method_resp: Some(14),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

#[allow(dead_code)]
pub fn rpc_metadata(request: &crate::client::RacRequest) -> Option<RpcMethodMeta> {
    match request {
        crate::client::RacRequest::ClusterAuth { .. } => Some(RPC_CLUSTER_AUTH_META),
        crate::client::RacRequest::ClusterAdminList { .. } => Some(RPC_CLUSTER_ADMIN_LIST_META),
        crate::client::RacRequest::ClusterAdminRegister { .. } => Some(RPC_CLUSTER_ADMIN_REGISTER_META),
        crate::client::RacRequest::ClusterList => Some(RPC_CLUSTER_LIST_META),
        crate::client::RacRequest::ClusterInfo { .. } => Some(RPC_CLUSTER_INFO_META),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct ClusterIdRequest {
    pub cluster: Uuid16,
}

impl ClusterIdRequest {
    pub fn encoded_len(&self) -> usize {
        16
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ClusterAuthRequest {
    pub cluster: Uuid16,
    pub user: String,
    pub pwd: String,
}

impl ClusterAuthRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.user.len() + 1 + self.pwd.len()
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.user.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ClusterAdminRegisterRequest {
    pub cluster: Uuid16,
    pub name: String,
    pub descr: String,
    pub pwd: String,
    pub auth_flags: u8,
}

impl ClusterAdminRegisterRequest {
    pub fn encoded_len(&self) -> usize {
        16 + 1 + self.name.len() + 1 + self.descr.len() + 1 + self.pwd.len() + 1 + 2
    }

    pub fn encode_body(&self, out: &mut Vec<u8>) -> Result<()> {
        out.extend_from_slice(&self.cluster);
        out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.descr.as_bytes())?);
        out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        out.push(self.auth_flags);
        out.extend_from_slice(&[0, 0]);
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
    fn cluster_admin_list_response_hex() {
        let hex = include_str!("../../../../artifacts/rac/cluster_admin_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let items = parse_cluster_admin_list_body(body).expect("parse body");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "cadmin");
        assert_eq!(items[0].unknown_tag, 0);
        assert_eq!(items[0].unknown_flags, 0x3efbfbd);
        assert_eq!(items[0].unknown_tail, [1, 0, 0]);
    }

}
