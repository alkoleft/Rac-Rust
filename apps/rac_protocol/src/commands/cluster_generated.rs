use crate::Uuid16;
use crate::error::RacError;
use crate::protocol::ProtocolVersion;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::rac_wire::encode_with_len_u8;

pub const METHOD_CLUSTER_AUTH_REQ: u8 = 0x09;
pub const METHOD_CLUSTER_ADMIN_LIST_REQ: u8 = 0x02;
pub const METHOD_CLUSTER_ADMIN_LIST_RESP: u8 = 0x03;
pub const METHOD_CLUSTER_ADMIN_REGISTER_REQ: u8 = 0x05;
pub const METHOD_CLUSTER_ADMIN_REMOVE_REQ: u8 = 0x07;
pub const METHOD_CLUSTER_LIST_REQ: u8 = 0x0b;
pub const METHOD_CLUSTER_LIST_RESP: u8 = 0x0c;
pub const METHOD_CLUSTER_INFO_REQ: u8 = 0x0d;
pub const METHOD_CLUSTER_INFO_RESP: u8 = 0x0e;

#[derive(Debug, Serialize, Clone)]
pub struct ClusterAdminRecord {
    pub name: String,
    pub unknown_tag: u8,
    pub unknown_flags: u32,
    pub unknown_tail: [u8; 3],
}

impl ClusterAdminRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>, protocol_version: ProtocolVersion) -> Result<Self> {
        let _ = protocol_version;
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
    pub allow_access_right_audit_events_recording: Option<u8>,
    pub ping_period: Option<u32>,
    pub ping_timeout: Option<u32>,
    pub restart_schedule_cron: Option<String>,
    pub restart_interval: Option<u32>,
}

impl ClusterRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>, protocol_version: ProtocolVersion) -> Result<Self> {
        let _ = protocol_version;
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
        let allow_access_right_audit_events_recording = if protocol_version >= ProtocolVersion::V16_0 {
            Some(cursor.take_u8()?)
        } else {
            None
        };
        let ping_period = if protocol_version >= ProtocolVersion::V16_0 {
            Some(cursor.take_u32_be()?)
        } else {
            None
        };
        let ping_timeout = if protocol_version >= ProtocolVersion::V16_0 {
            Some(cursor.take_u32_be()?)
        } else {
            None
        };
        let restart_schedule_cron = if protocol_version >= ProtocolVersion::V16_0 {
            Some(cursor.take_str8()?)
        } else {
            None
        };
        let restart_interval = if protocol_version >= ProtocolVersion::V11_0 && protocol_version < ProtocolVersion::V16_0 {
            Some(cursor.take_u32_be()?)
        } else {
            None
        };
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
            allow_access_right_audit_events_recording,
            ping_period,
            ping_timeout,
            restart_schedule_cron,
            restart_interval,
        })
    }
}

pub struct ClusterAuthRpc {
    pub cluster: Uuid16,
    pub user: String,
    pub pwd: String,
}

impl crate::rpc::Request for ClusterAuthRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_CLUSTER_AUTH_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ClusterAuth unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.user.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.pwd.len() } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.user.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        }
        Ok(out)
    }
}

pub struct ClusterAdminListRpc {
    pub cluster: Uuid16,
}

impl crate::rpc::Request for ClusterAdminListRpc {
    type Response = ClusterAdminListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_CLUSTER_ADMIN_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ClusterAdminList unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        Ok(out)
    }
}

pub struct ClusterAdminRegisterRpc {
    pub cluster: Uuid16,
    pub name: String,
    pub descr: String,
    pub pwd: String,
    pub auth_tag: u8,
    pub auth_flags: u8,
    pub os_user: String,
}

impl crate::rpc::Request for ClusterAdminRegisterRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_CLUSTER_ADMIN_REGISTER_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ClusterAdminRegister unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.name.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.descr.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.pwd.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.os_user.len() } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.descr.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.auth_tag);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.auth_flags);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.os_user.as_bytes())?);
        }
        Ok(out)
    }
}

pub struct ClusterAdminRemoveRpc {
    pub cluster: Uuid16,
    pub name: String,
}

impl crate::rpc::Request for ClusterAdminRemoveRpc {
    type Response = crate::rpc::AckResponse;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_CLUSTER_ADMIN_REMOVE_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ClusterAdminRemove unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.name.len() } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        }
        Ok(out)
    }
}

pub struct ClusterListRpc;

impl crate::rpc::Request for ClusterListRpc {
    type Response = ClusterListResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_CLUSTER_LIST_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ClusterList unsupported for protocol"));
        }
        Ok(Vec::new())
    }
}

pub struct ClusterInfoRpc {
    pub cluster: Uuid16,
}

impl crate::rpc::Request for ClusterInfoRpc {
    type Response = ClusterInfoResp;

    fn meta(&self) -> crate::rpc::Meta {
        RPC_CLUSTER_INFO_META
    }

    fn cluster(&self) -> Option<crate::Uuid16> {
        None
    }

    fn encode_body(&self, _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Vec<u8>> {
        let protocol_version = _codec.protocol_version();
        if !(protocol_version >= ProtocolVersion::V11_0) {
            return Err(RacError::Unsupported("rpc ClusterInfo unsupported for protocol"));
        }
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        Ok(out)
    }
}


#[derive(Debug, Serialize)]
pub struct ClusterAdminListResp {
    pub admins: Vec<ClusterAdminRecord>,
}

impl crate::rpc::Response for ClusterAdminListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        Ok(Self {
            admins: crate::commands::parse_list_u8(body, |cursor| ClusterAdminRecord::decode(cursor, protocol_version))?,
        })
    }
}


pub fn parse_cluster_info_body(body: &[u8], tail_len: usize, protocol_version: ProtocolVersion) -> Result<ClusterRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("cluster info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    let record = ClusterRecord::decode(&mut cursor, protocol_version)?;
    if tail_len != 0 {
        let _tail = cursor.take_bytes(tail_len)?;
    }
    Ok(record)
}


pub const RPC_CLUSTER_AUTH_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_CLUSTER_AUTH_REQ,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_CLUSTER_ADMIN_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_CLUSTER_ADMIN_LIST_REQ,
    method_resp: Some(METHOD_CLUSTER_ADMIN_LIST_RESP),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_CLUSTER_ADMIN_REGISTER_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_CLUSTER_ADMIN_REGISTER_REQ,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_CLUSTER_ADMIN_REMOVE_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_CLUSTER_ADMIN_REMOVE_REQ,
    method_resp: None,
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_CLUSTER_LIST_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_CLUSTER_LIST_REQ,
    method_resp: Some(METHOD_CLUSTER_LIST_RESP),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

pub const RPC_CLUSTER_INFO_META: crate::rpc::Meta = crate::rpc::Meta {
    method_req: METHOD_CLUSTER_INFO_REQ,
    method_resp: Some(METHOD_CLUSTER_INFO_RESP),
    requires_cluster_context: false,
    requires_infobase_context: false,
};

#[cfg(all(test, feature = "artifacts"))]
mod tests {
    use super::*;
    use crate::commands::rpc_body;
    use crate::protocol::ProtocolVersion;

    fn decode_hex_str(input: &str) -> Vec<u8> {
        hex::decode(input.trim()).expect("hex decode")
    }

    #[test]
    fn cluster_admin_list_response_hex() {
        let hex = include_str!("../../../../artifacts/rac/cluster_admin_list_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let items = crate::commands::parse_list_u8(body, |cursor| ClusterAdminRecord::decode(cursor, protocol_version)).expect("parse body");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "cadmin");
        assert_eq!(items[0].unknown_tag, 0);
        assert_eq!(items[0].unknown_flags, 0x3efbfbd);
        assert_eq!(items[0].unknown_tail, [1, 0, 0]);
    }

    #[test]
    fn cluster_list_response_custom_hex() {
        let hex = include_str!("../../../../artifacts/rac/cluster_list_response_custom.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let items = crate::commands::parse_list_u8_tail(body, 0, |cursor| ClusterRecord::decode(cursor, protocol_version)).expect("parse body");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].lifetime_limit, 0x457);
        assert_eq!(items[0].security_level, 3);
        assert_eq!(items[0].session_fault_tolerance_level, 4);
        assert_eq!(items[0].load_balancing_mode, 1);
        assert_eq!(items[0].errors_count_threshold, 0);
        assert_eq!(items[0].kill_problem_processes, 0);
        assert_eq!(items[0].kill_by_memory_with_dump, 1);
    }

    #[test]
    fn cluster_list_response_flags_hex() {
        let hex = include_str!("../../../../artifacts/rac/cluster_list_response_flags.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let items = crate::commands::parse_list_u8_tail(body, 0, |cursor| ClusterRecord::decode(cursor, protocol_version)).expect("parse body");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].kill_problem_processes, 1);
        assert_eq!(items[0].kill_by_memory_with_dump, 0);
    }

}
