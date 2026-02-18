use crate::error::{RacError, Result};
use crate::rac_wire::{
    decode_rpc_method, encode_rpc, OPCODE_CLOSE, OPCODE_INIT_ACK, OPCODE_RPC, OPCODE_SERVICE_ACK,
    OPCODE_SERVICE_NEGOTIATION,
};
use crate::Uuid16;

#[derive(Debug, Clone)]
pub struct SerializedRpc {
    pub payload: Vec<u8>,
    pub expect_method: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RacRequest {
    AgentVersion,
    ClusterAuth {
        cluster: Uuid16,
        user: String,
        pwd: String,
    },
    ClusterAdminList {
        cluster: Uuid16,
    },
    ClusterAdminRegister {
        cluster: Uuid16,
        name: String,
        descr: String,
        pwd: String,
        auth_flags: u8,
    },
    ClusterList,
    ClusterInfo { cluster: Uuid16 },
    ManagerList { cluster: Uuid16 },
    ManagerInfo { cluster: Uuid16, manager: Uuid16 },
    ServerList { cluster: Uuid16 },
    ServerInfo { cluster: Uuid16, server: Uuid16 },
    ProcessList { cluster: Uuid16 },
    ProcessInfo { cluster: Uuid16, process: Uuid16 },
    InfobaseSummaryList { cluster: Uuid16 },
    InfobaseSummaryInfo { cluster: Uuid16, infobase: Uuid16 },
    InfobaseInfo { cluster: Uuid16, infobase: Uuid16 },
    ConnectionList { cluster: Uuid16 },
    ConnectionInfo { cluster: Uuid16, connection: Uuid16 },
    SessionList { cluster: Uuid16 },
    SessionInfo { cluster: Uuid16, session: Uuid16 },
    LockList { cluster: Uuid16 },
    ProfileList { cluster: Uuid16 },
    RuleList { cluster: Uuid16, server: Uuid16 },
    RuleInfo {
        cluster: Uuid16,
        server: Uuid16,
        rule: Uuid16,
    },
    RuleApply {
        cluster: Uuid16,
        apply_mode: u32,
    },
    RuleRemove {
        cluster: Uuid16,
        server: Uuid16,
        rule: Uuid16,
    },
    RuleInsert {
        cluster: Uuid16,
        server: Uuid16,
        position: u32,
        object_type: u32,
        infobase_name: String,
        rule_type: u8,
        application_ext: String,
        priority: u32,
    },
    RuleUpdate {
        cluster: Uuid16,
        server: Uuid16,
        rule: Uuid16,
        position: u32,
        object_type: u32,
        infobase_name: String,
        rule_type: u8,
        application_ext: String,
        priority: u32,
    },
    CounterList { cluster: Uuid16 },
    CounterInfo { cluster: Uuid16, counter: String },
    CounterUpdate {
        cluster: Uuid16,
        name: String,
        collection_time: u64,
        group: u8,
        filter_type: u8,
        filter: String,
        duration: u8,
        cpu_time: u8,
        duration_dbms: u8,
        service: u8,
        memory: u8,
        read: u8,
        write: u8,
        dbms_bytes: u8,
        call: u8,
        number_of_active_sessions: u8,
        number_of_sessions: u8,
        descr: String,
    },
    LimitList { cluster: Uuid16 },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RequiredContext {
    pub cluster: Option<Uuid16>,
    pub infobase_cluster: Option<Uuid16>,
}

pub trait RacProtocol: Send + Sync {
    fn name(&self) -> &'static str;

    fn init_packet(&self) -> &'static [u8];
    fn service_negotiation_payload(&self) -> &'static [u8];
    fn close_payload(&self) -> &'static [u8];

    fn opcode_init_ack(&self) -> u8;
    fn opcode_service_negotiation(&self) -> u8;
    fn opcode_service_ack(&self) -> u8;
    fn opcode_rpc(&self) -> u8;
    fn opcode_close(&self) -> u8;

    fn decode_rpc_method_id(&self, payload: &[u8]) -> Option<u8>;

    fn required_context(&self, request: &RacRequest) -> RequiredContext;
    fn serialize_set_cluster_context(&self, cluster: Uuid16) -> Result<SerializedRpc>;
    fn serialize_set_infobase_context(&self, cluster: Uuid16) -> Result<SerializedRpc>;

    fn serialize(&self, request: RacRequest) -> Result<SerializedRpc>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RacProtocolVersion {
    V16_0,
}

impl Default for RacProtocolVersion {
    fn default() -> Self {
        Self::V16_0
    }
}

impl RacProtocolVersion {
    pub fn boxed(self) -> Box<dyn RacProtocol> {
        match self {
            RacProtocolVersion::V16_0 => Box::new(V16Protocol),
        }
    }
}

#[derive(Debug)]
pub struct V16Protocol;

impl V16Protocol {
    const INIT_PACKET: &'static [u8] = &[
        0x1c, 0x53, 0x57, 0x50, 0x01, 0x00, 0x01, 0x00, 0x01, 0x16, 0x01, 0x0f, 0x63, 0x6f, 0x6e,
        0x6e, 0x65, 0x63, 0x74, 0x2e, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x04, 0x00, 0x00,
        0x07, 0xd0,
    ];

    const SERVICE_NEGOTIATION: &'static [u8] = &[
        0x18, 0x76, 0x38, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x41, 0x64, 0x6d,
        0x69, 0x6e, 0x2e, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x04, 0x31, 0x36, 0x2e, 0x30,
        0x80,
    ];

    const CLOSE: &'static [u8] = &[0x01];

    fn encode_cluster_context(cluster: Uuid16) -> Vec<u8> {
        let mut body = Vec::with_capacity(16 + 2);
        body.extend_from_slice(&cluster);
        body.extend_from_slice(&[0x00, 0x00]);
        encode_rpc(crate::rac_wire::METHOD_CLUSTER_CONTEXT, &body)
    }

    fn encode_infobase_context(cluster: Uuid16) -> Vec<u8> {
        let mut body = Vec::with_capacity(16 + 2);
        body.extend_from_slice(&cluster);
        body.extend_from_slice(&[0x00, 0x00]);
        encode_rpc(crate::rac_wire::METHOD_INFOBASE_CONTEXT, &body)
    }

    fn encode_cluster_scoped(method_id: u8, cluster: Uuid16) -> Vec<u8> {
        encode_rpc(method_id, &cluster)
    }

    fn encode_cluster_scoped_object(method_id: u8, cluster: Uuid16, object: Uuid16) -> Vec<u8> {
        let mut body = Vec::with_capacity(16 + 16);
        body.extend_from_slice(&cluster);
        body.extend_from_slice(&object);
        encode_rpc(method_id, &body)
    }
}

impl RacProtocol for V16Protocol {
    fn name(&self) -> &'static str {
        "v16.0"
    }

    fn init_packet(&self) -> &'static [u8] {
        Self::INIT_PACKET
    }

    fn service_negotiation_payload(&self) -> &'static [u8] {
        Self::SERVICE_NEGOTIATION
    }

    fn close_payload(&self) -> &'static [u8] {
        Self::CLOSE
    }

    fn opcode_init_ack(&self) -> u8 {
        OPCODE_INIT_ACK
    }

    fn opcode_service_negotiation(&self) -> u8 {
        OPCODE_SERVICE_NEGOTIATION
    }

    fn opcode_service_ack(&self) -> u8 {
        OPCODE_SERVICE_ACK
    }

    fn opcode_rpc(&self) -> u8 {
        OPCODE_RPC
    }

    fn opcode_close(&self) -> u8 {
        OPCODE_CLOSE
    }

    fn decode_rpc_method_id(&self, payload: &[u8]) -> Option<u8> {
        decode_rpc_method(payload)
    }

    fn required_context(&self, request: &RacRequest) -> RequiredContext {
        match request {
            RacRequest::AgentVersion
            | RacRequest::ClusterAuth { .. }
            | RacRequest::ClusterAdminList { .. }
            | RacRequest::ClusterAdminRegister { .. }
            | RacRequest::ClusterList
            | RacRequest::ClusterInfo { .. } => {
                RequiredContext::default()
            }
            RacRequest::InfobaseInfo { cluster, .. } => RequiredContext {
                cluster: Some(*cluster),
                infobase_cluster: Some(*cluster),
            },
            RacRequest::ManagerList { cluster }
            | RacRequest::ManagerInfo { cluster, .. }
            | RacRequest::ServerList { cluster }
            | RacRequest::ServerInfo { cluster, .. }
            | RacRequest::ProcessList { cluster }
            | RacRequest::ProcessInfo { cluster, .. }
            | RacRequest::InfobaseSummaryList { cluster }
            | RacRequest::InfobaseSummaryInfo { cluster, .. }
            | RacRequest::ConnectionList { cluster }
            | RacRequest::ConnectionInfo { cluster, .. }
            | RacRequest::SessionList { cluster }
            | RacRequest::SessionInfo { cluster, .. }
            | RacRequest::LockList { cluster }
            | RacRequest::ProfileList { cluster }
            | RacRequest::RuleList { cluster, .. }
            | RacRequest::RuleInfo { cluster, .. }
            | RacRequest::RuleApply { cluster, .. }
            | RacRequest::RuleRemove { cluster, .. }
            | RacRequest::RuleInsert { cluster, .. }
            | RacRequest::RuleUpdate { cluster, .. }
            | RacRequest::CounterList { cluster }
            | RacRequest::CounterInfo { cluster, .. }
            | RacRequest::CounterUpdate { cluster, .. }
            | RacRequest::LimitList { cluster } => RequiredContext {
                cluster: Some(*cluster),
                infobase_cluster: None,
            },
        }
    }

    fn serialize_set_cluster_context(&self, cluster: Uuid16) -> Result<SerializedRpc> {
        Ok(SerializedRpc {
            payload: Self::encode_cluster_context(cluster),
            expect_method: None,
        })
    }

    fn serialize_set_infobase_context(&self, cluster: Uuid16) -> Result<SerializedRpc> {
        Ok(SerializedRpc {
            payload: Self::encode_infobase_context(cluster),
            expect_method: None,
        })
    }

    fn serialize(&self, request: RacRequest) -> Result<SerializedRpc> {
        use crate::rac_wire::*;

        let (payload, expect_method) = match request {
            RacRequest::AgentVersion => (
                encode_rpc(METHOD_AGENT_VERSION_REQ, &[]),
                Some(METHOD_AGENT_VERSION_RESP),
            ),
            RacRequest::ClusterAuth { cluster, user, pwd } => {
                let mut body = Vec::with_capacity(16 + 2 + user.len() + pwd.len());
                body.extend_from_slice(&cluster);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(user.as_bytes())?);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(pwd.as_bytes())?);
                (encode_rpc(METHOD_CLUSTER_CONTEXT, &body), None)
            }
            RacRequest::ClusterAdminList { cluster } => (
                Self::encode_cluster_scoped(METHOD_CLUSTER_ADMIN_LIST_REQ, cluster),
                Some(METHOD_CLUSTER_ADMIN_LIST_RESP),
            ),
            RacRequest::ClusterAdminRegister {
                cluster,
                name,
                descr,
                pwd,
                auth_flags,
            } => {
                let mut body =
                    Vec::with_capacity(16 + 4 + name.len() + descr.len() + pwd.len());
                body.extend_from_slice(&cluster);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(name.as_bytes())?);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(descr.as_bytes())?);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(pwd.as_bytes())?);
                body.push(auth_flags);
                body.extend_from_slice(&[0x00, 0x00]);
                (
                    encode_rpc(METHOD_CLUSTER_ADMIN_REGISTER_REQ, &body),
                    None,
                )
            }
            RacRequest::ClusterList => (
                encode_rpc(METHOD_CLUSTER_LIST_REQ, &[]),
                Some(METHOD_CLUSTER_LIST_RESP),
            ),
            RacRequest::ClusterInfo { cluster } => (
                Self::encode_cluster_scoped(METHOD_CLUSTER_INFO_REQ, cluster),
                Some(METHOD_CLUSTER_INFO_RESP),
            ),
            RacRequest::ManagerList { cluster } => (
                Self::encode_cluster_scoped(METHOD_MANAGER_LIST_REQ, cluster),
                Some(METHOD_MANAGER_LIST_RESP),
            ),
            RacRequest::ManagerInfo { cluster, manager } => (
                Self::encode_cluster_scoped_object(METHOD_MANAGER_INFO_REQ, cluster, manager),
                Some(METHOD_MANAGER_INFO_RESP),
            ),
            RacRequest::ServerList { cluster } => (
                Self::encode_cluster_scoped(METHOD_SERVER_LIST_REQ, cluster),
                Some(METHOD_SERVER_LIST_RESP),
            ),
            RacRequest::ServerInfo { cluster, server } => (
                Self::encode_cluster_scoped_object(METHOD_SERVER_INFO_REQ, cluster, server),
                Some(METHOD_SERVER_INFO_RESP),
            ),
            RacRequest::ProcessList { cluster } => (
                Self::encode_cluster_scoped(METHOD_PROCESS_LIST_REQ, cluster),
                Some(METHOD_PROCESS_LIST_RESP),
            ),
            RacRequest::ProcessInfo { cluster, process } => (
                Self::encode_cluster_scoped_object(METHOD_PROCESS_INFO_REQ, cluster, process),
                Some(METHOD_PROCESS_INFO_RESP),
            ),
            RacRequest::InfobaseSummaryList { cluster } => (
                Self::encode_cluster_scoped(METHOD_INFOBASE_SUMMARY_LIST_REQ, cluster),
                Some(METHOD_INFOBASE_SUMMARY_LIST_RESP),
            ),
            RacRequest::InfobaseSummaryInfo { cluster, infobase } => (
                Self::encode_cluster_scoped_object(
                    METHOD_INFOBASE_SUMMARY_INFO_REQ,
                    cluster,
                    infobase,
                ),
                Some(METHOD_INFOBASE_SUMMARY_INFO_RESP),
            ),
            RacRequest::InfobaseInfo { cluster, infobase } => (
                Self::encode_cluster_scoped_object(METHOD_INFOBASE_INFO_REQ, cluster, infobase),
                Some(METHOD_INFOBASE_INFO_RESP),
            ),
            RacRequest::ConnectionList { cluster } => (
                Self::encode_cluster_scoped(METHOD_CONNECTION_LIST_REQ, cluster),
                Some(METHOD_CONNECTION_LIST_RESP),
            ),
            RacRequest::ConnectionInfo {
                cluster,
                connection,
            } => (
                Self::encode_cluster_scoped_object(METHOD_CONNECTION_INFO_REQ, cluster, connection),
                Some(METHOD_CONNECTION_INFO_RESP),
            ),
            RacRequest::SessionList { cluster } => (
                Self::encode_cluster_scoped(METHOD_SESSION_LIST_REQ, cluster),
                Some(METHOD_SESSION_LIST_RESP),
            ),
            RacRequest::SessionInfo { cluster, session } => (
                Self::encode_cluster_scoped_object(METHOD_SESSION_INFO_REQ, cluster, session),
                Some(METHOD_SESSION_INFO_RESP),
            ),
            RacRequest::LockList { cluster } => (
                Self::encode_cluster_scoped(METHOD_LOCK_LIST_REQ, cluster),
                Some(METHOD_LOCK_LIST_RESP),
            ),
            RacRequest::ProfileList { cluster } => (
                Self::encode_cluster_scoped(METHOD_PROFILE_LIST_REQ, cluster),
                Some(METHOD_PROFILE_LIST_RESP),
            ),
            RacRequest::RuleList { cluster, server } => (
                Self::encode_cluster_scoped_object(METHOD_RULE_LIST_REQ, cluster, server),
                Some(METHOD_RULE_LIST_RESP),
            ),
            RacRequest::RuleInfo {
                cluster,
                server,
                rule,
            } => {
                let mut body = Vec::with_capacity(16 + 16 + 16);
                body.extend_from_slice(&cluster);
                body.extend_from_slice(&server);
                body.extend_from_slice(&rule);
                (encode_rpc(METHOD_RULE_INFO_REQ, &body), Some(METHOD_RULE_INFO_RESP))
            }
            RacRequest::RuleApply {
                cluster,
                apply_mode,
            } => {
                let mut body = Vec::with_capacity(16 + 4);
                body.extend_from_slice(&cluster);
                body.extend_from_slice(&apply_mode.to_be_bytes());
                (encode_rpc(METHOD_RULE_APPLY_REQ, &body), None)
            }
            RacRequest::RuleRemove {
                cluster,
                server,
                rule,
            } => {
                let mut body = Vec::with_capacity(16 + 16 + 16);
                body.extend_from_slice(&cluster);
                body.extend_from_slice(&server);
                body.extend_from_slice(&rule);
                (encode_rpc(METHOD_RULE_REMOVE_REQ, &body), None)
            }
            RacRequest::RuleInsert {
                cluster,
                server,
                position,
                object_type,
                infobase_name,
                rule_type,
                application_ext,
                priority,
            } => {
                let mut body = Vec::with_capacity(
                    16 + 16 + 16 + 4 + 4 + infobase_name.len() + application_ext.len() + 4 + 3,
                );
                body.extend_from_slice(&cluster);
                body.extend_from_slice(&server);
                body.extend_from_slice(&[0u8; 16]);
                body.extend_from_slice(&position.to_be_bytes());
                body.extend_from_slice(&object_type.to_be_bytes());
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
                    infobase_name.as_bytes(),
                )?);
                body.push(rule_type);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
                    application_ext.as_bytes(),
                )?);
                body.extend_from_slice(&priority.to_be_bytes());
                (
                    encode_rpc(METHOD_RULE_INSERT_REQ, &body),
                    Some(METHOD_RULE_INSERT_RESP),
                )
            }
            RacRequest::RuleUpdate {
                cluster,
                server,
                rule,
                position,
                object_type,
                infobase_name,
                rule_type,
                application_ext,
                priority,
            } => {
                let mut body = Vec::with_capacity(
                    16 + 16 + 16 + 4 + 4 + infobase_name.len() + application_ext.len() + 4 + 3,
                );
                body.extend_from_slice(&cluster);
                body.extend_from_slice(&server);
                body.extend_from_slice(&rule);
                body.extend_from_slice(&position.to_be_bytes());
                body.extend_from_slice(&object_type.to_be_bytes());
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
                    infobase_name.as_bytes(),
                )?);
                body.push(rule_type);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
                    application_ext.as_bytes(),
                )?);
                body.extend_from_slice(&priority.to_be_bytes());
                (
                    encode_rpc(METHOD_RULE_INSERT_REQ, &body),
                    Some(METHOD_RULE_INSERT_RESP),
                )
            }
            RacRequest::CounterList { cluster } => (
                Self::encode_cluster_scoped(METHOD_COUNTER_LIST_REQ, cluster),
                Some(METHOD_COUNTER_LIST_RESP),
            ),
            RacRequest::CounterInfo { cluster, counter } => {
                let mut body = Vec::with_capacity(16 + 1 + counter.len());
                body.extend_from_slice(&cluster);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
                    counter.as_bytes(),
                )?);
                (
                    encode_rpc(METHOD_COUNTER_INFO_REQ, &body),
                    Some(METHOD_COUNTER_INFO_RESP),
                )
            }
            RacRequest::CounterUpdate {
                cluster,
                name,
                collection_time,
                group,
                filter_type,
                filter,
                duration,
                cpu_time,
                duration_dbms,
                service,
                memory,
                read,
                write,
                dbms_bytes,
                call,
                number_of_active_sessions,
                number_of_sessions,
                descr,
            } => {
                let mut body = Vec::with_capacity(
                    16 + 32 + name.len() + filter.len() + descr.len(),
                );
                body.extend_from_slice(&cluster);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
                    name.as_bytes(),
                )?);
                body.extend_from_slice(&collection_time.to_be_bytes());
                body.push(group);
                body.push(filter_type);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
                    filter.as_bytes(),
                )?);
                body.push(duration);
                body.push(cpu_time);
                body.push(duration_dbms);
                body.push(service);
                body.push(memory);
                body.push(read);
                body.push(write);
                body.push(dbms_bytes);
                body.push(call);
                body.push(number_of_active_sessions);
                body.push(number_of_sessions);
                body.extend_from_slice(&crate::rac_wire::encode_with_len_u8(
                    descr.as_bytes(),
                )?);
                (encode_rpc(METHOD_COUNTER_UPDATE_REQ, &body), None)
            }
            RacRequest::LimitList { cluster } => (
                Self::encode_cluster_scoped(METHOD_LIMIT_LIST_REQ, cluster),
                Some(METHOD_LIMIT_LIST_RESP),
            ),
        };

        if payload.is_empty() {
            return Err(RacError::Unsupported("empty payload"));
        }
        Ok(SerializedRpc {
            payload,
            expect_method,
        })
    }
}
