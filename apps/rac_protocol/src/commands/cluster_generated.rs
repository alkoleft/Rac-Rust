use crate::Uuid16;
use crate::error::RacError;
use crate::protocol::ProtocolVersion;
use crate::codec::RecordCursor;
use crate::error::Result;
use serde::Serialize;
use crate::rac_wire::encode_with_len_u14;
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
    pub descr: String,
    pub record_marker: u32,
    pub auth_pwd: u8,
    pub auth_os: u8,
    pub os_user: String,
}

impl ClusterAdminRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>, _: ProtocolVersion) -> Result<Self> {
        let name = cursor.take_str8()?;
        let descr = {
            let b0 = cursor.take_u8()? as usize;
            let len = if (b0 & 0x40) != 0 {
                let b1 = cursor.take_u8()? as usize;
                (b0 & 0x3f) | (b1 << 6)
            } else {
                b0
            };
            let bytes = cursor.take_bytes(len)?;
            String::from_utf8_lossy(&bytes).to_string()
        };
        let record_marker = cursor.take_u32_be()?;
        let auth_pwd = cursor.take_u8()?;
        let auth_os = cursor.take_u8()?;
        let os_user = cursor.take_str8()?;
        Ok(Self {
            name,
            descr,
            record_marker,
            auth_pwd,
            auth_os,
            os_user,
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
    pub restart_schedule_len: Option<u8>,
    pub restart_schedule_cron: Option<String>,
    pub restart_interval: Option<u32>,
}

impl ClusterRecord {
    pub fn decode(cursor: &mut RecordCursor<'_>, protocol_version: ProtocolVersion) -> Result<Self> {
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
        if protocol_version >= ProtocolVersion::V16_0 {
            let _reserved_u32_5 = cursor.take_u32_be()?;
        }
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
        let restart_schedule_len = if protocol_version >= ProtocolVersion::V16_0 {
            Some(cursor.take_u8()?)
        } else {
            None
        };
        let restart_schedule_cron = if protocol_version >= ProtocolVersion::V16_0 {
            Some({
                let len = restart_schedule_len.unwrap_or_default() as usize;
                let bytes = cursor.take_bytes(len)?;
                String::from_utf8_lossy(&bytes).to_string()
            })
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
            restart_schedule_len,
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
    pub auth_pwd: u8,
    pub auth_os: u8,
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
        let mut out = Vec::with_capacity(if protocol_version >= ProtocolVersion::V11_0 { 16 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.name.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { if self.descr.len() < 0x40 { 1 + self.descr.len() } else { 2 + self.descr.len() } } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.pwd.len() } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 } else { 0 } + if protocol_version >= ProtocolVersion::V11_0 { 1 + self.os_user.len() } else { 0 });
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&self.cluster);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.name.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u14(self.descr.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.extend_from_slice(&encode_with_len_u8(self.pwd.as_bytes())?);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.auth_pwd);
        }
        if protocol_version >= ProtocolVersion::V11_0 {
            out.push(self.auth_os);
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

#[derive(Debug, Serialize)]
pub struct ClusterListResp {
    pub clusters: Vec<ClusterRecord>,
}

impl crate::rpc::Response for ClusterListResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        Ok(Self {
            clusters: crate::commands::parse_list_u8(body, |cursor| ClusterRecord::decode(cursor, protocol_version))?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ClusterInfoResp {
    pub cluster: ClusterRecord,
}

impl crate::rpc::Response for ClusterInfoResp {
    fn decode(payload: &[u8], _codec: &dyn crate::protocol::ProtocolCodec) -> Result<Self> {
        let body = crate::rpc::decode_utils::rpc_body(payload)?;
        let protocol_version = _codec.protocol_version();
        let record = parse_cluster_info_body(body, protocol_version)?;
        Ok(Self {
            cluster: record,
        })
    }
}


pub fn parse_cluster_info_body(body: &[u8], protocol_version: ProtocolVersion) -> Result<ClusterRecord> {
    if body.is_empty() {
        return Err(RacError::Decode("cluster info empty body"));
    }
    let mut cursor = RecordCursor::new(body);
    ClusterRecord::decode(&mut cursor, protocol_version)
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
    fn cluster_admin_list_response_20260226_hex() {
        let hex = include_str!("../../../../artifacts/rac/v16/v16_20260226_053425_cluster_admin_list_response_rpc.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let items = crate::commands::parse_list_u8(body, |cursor| ClusterAdminRecord::decode(cursor, protocol_version)).expect("parse body");
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].name, "cadmin");
        assert_eq!(items[0].descr, "");
        assert_eq!(items[0].record_marker, 0x3efbfbd);
        assert_eq!(items[0].auth_pwd, 1);
        assert_eq!(items[0].auth_os, 0);
        assert_eq!(items[0].os_user, "");
        assert_eq!(items[1].name, "codex_cadmin_pwd_20260226_053425");
        assert_eq!(items[1].descr, "Codex cluster admin pwd");
        assert_eq!(items[1].auth_pwd, 1);
        assert_eq!(items[1].auth_os, 0);
        assert_eq!(items[1].os_user, "");
        assert_eq!(items[2].name, "codex_cadmin_os_20260226_053425");
        assert_eq!(items[2].descr, "Codex cluster admin os");
        assert_eq!(items[2].auth_pwd, 1);
        assert_eq!(items[2].auth_os, 1);
        assert_eq!(items[2].os_user, "codex_os_user");
    }

    #[test]
    fn cluster_list_response_ping_hex() {
        let hex = include_str!("../../../../artifacts/rac/v16/v16_20260226_cluster_list_ping_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let items = crate::commands::parse_list_u8(body, |cursor| ClusterRecord::decode(cursor, protocol_version)).expect("parse body");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].kill_problem_processes, 1);
        assert_eq!(items[0].kill_by_memory_with_dump, 0);
        assert_eq!(items[0].allow_access_right_audit_events_recording.unwrap_or_default(), 0);
        assert_eq!(items[0].ping_period.unwrap_or_default(), 1);
        assert_eq!(items[0].ping_timeout.unwrap_or_default(), 2);
        assert_eq!(items[0].restart_schedule_cron.clone().unwrap_or_default(), "");
    }

    #[test]
    fn cluster_list_response_restart_schedule_hex() {
        let hex = include_str!("../../../../artifacts/rac/v16/v16_20260226_cluster_list_restart_schedule_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let items = crate::commands::parse_list_u8(body, |cursor| ClusterRecord::decode(cursor, protocol_version)).expect("parse body");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].kill_problem_processes, 0);
        assert_eq!(items[0].kill_by_memory_with_dump, 1);
        assert_eq!(items[0].allow_access_right_audit_events_recording.unwrap_or_default(), 0);
        assert_eq!(items[0].ping_period.unwrap_or_default(), 0xea5f);
        assert_eq!(items[0].ping_timeout.unwrap_or_default(), 0xff56);
        assert_eq!(items[0].restart_schedule_cron.clone().unwrap_or_default(), "0 3 * * 6");
    }

    #[test]
    fn cluster_info_response_ping_hex() {
        let hex = include_str!("../../../../artifacts/rac/v16/v16_20260226_cluster_info_ping_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let record = parse_cluster_info_body(body, protocol_version).expect("parse body");
        assert_eq!(record.ping_period.unwrap_or_default(), 1);
        assert_eq!(record.ping_timeout.unwrap_or_default(), 2);
        assert_eq!(record.restart_schedule_cron.clone().unwrap_or_default(), "");
    }

    #[test]
    fn cluster_info_response_restart_schedule_hex() {
        let hex = include_str!("../../../../artifacts/rac/v16/v16_20260226_cluster_info_restart_schedule_response.hex");
        let payload = decode_hex_str(hex);
        let body = rpc_body(&payload).expect("rpc body");
        let protocol_version = ProtocolVersion::V16_0;
        let record = parse_cluster_info_body(body, protocol_version).expect("parse body");
        assert_eq!(record.ping_period.unwrap_or_default(), 0xea5f);
        assert_eq!(record.ping_timeout.unwrap_or_default(), 0xff56);
        assert_eq!(record.restart_schedule_cron.clone().unwrap_or_default(), "0 3 * * 6");
    }

}
