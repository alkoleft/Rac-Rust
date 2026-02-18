use crate::codec::RecordCursor;
use crate::error::{RacError, Result};
use crate::rac_wire::decode_rpc_method;

pub mod agent;
pub mod cluster;
pub mod connection;
pub mod counter;
pub mod infobase;
pub mod lock;
pub mod limit;
pub mod manager;
pub mod process;
pub mod profile;
pub mod rule;
pub mod server;
pub mod session;

pub use self::agent::{agent_version, AgentVersionResp};
pub use self::cluster::{
    cluster_admin_list, cluster_admin_register, cluster_info, cluster_list, ClusterAdminListResp,
    ClusterAdminRecord, ClusterAdminRegisterReq, ClusterAdminRegisterResp, ClusterInfoResp,
    ClusterListResp, ClusterSummary,
};
pub use self::connection::{
    connection_info, connection_list, ConnectionInfoResp, ConnectionListResp, ConnectionRecord,
};
pub use self::counter::{counter_list, CounterListResp};
pub use self::infobase::{
    infobase_info, infobase_summary_info, infobase_summary_list, InfobaseInfoResp,
    InfobaseSummary, InfobaseSummaryInfoResp, InfobaseSummaryListResp,
};
pub use self::lock::{lock_list, LockListResp, LockRecord};
pub use self::limit::{limit_list, LimitListResp, LimitRecord};
pub use self::manager::{manager_info, manager_list, ManagerInfoResp, ManagerListResp, ManagerRecord};
pub use self::process::{
    process_info, process_list, ProcessInfoResp, ProcessLicense, ProcessListResp, ProcessRecord,
};
pub use self::profile::{profile_list, ProfileListResp};
pub use self::rule::{rule_apply, rule_list, RuleApplyMode, RuleApplyResp, RuleListResp, RuleRecord};
pub use self::server::{
    server_info, server_list, ServerInfoResp, ServerListResp, ServerRecord,
};
pub use self::session::{
    session_info, session_list, SessionCounters, SessionInfoResp, SessionLicense, SessionListResp,
    SessionRecord,
};

pub(crate) fn rpc_body(payload: &[u8]) -> Result<&[u8]> {
    let mut cursor = RecordCursor::new(payload, 0);
    if cursor.remaining_len() >= 5 {
        let head = cursor.take_bytes(4)?;
        if head == [0x01, 0x00, 0x00, 0x01] {
            let _ = cursor.take_u8()?;
            return Ok(cursor.remaining_slice());
        }
    }
    if decode_rpc_method(payload).is_none() {
        return Err(RacError::Decode("missing rpc header"));
    }
    Err(RacError::Decode("unexpected rpc header"))
}
