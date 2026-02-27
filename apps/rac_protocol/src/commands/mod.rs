use crate::codec::RecordCursor;
use crate::error::Result;

mod auth;
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
pub mod service_setting;

pub use self::agent::{
    agent_admin_list, agent_admin_register, agent_admin_remove, agent_version,
    AgentAdminListResp, AgentAdminRecord,
};
pub use self::auth::{agent_auth_optional, cluster_auth_optional, AuthPair};
pub use self::cluster::{
    cluster_admin_list, cluster_admin_register, cluster_auth, cluster_info, cluster_list,
    ClusterAdminRecord, ClusterRecord,
};
pub use self::connection::{
    connection_info, connection_list, ConnectionInfoResp, ConnectionListResp, ConnectionRecord,
};
pub use self::counter::{
    counter_accumulated_values, counter_clear, counter_info, counter_list, counter_remove,
    counter_update, counter_values, CounterAccumulatedValuesResp, CounterAccumulatedValuesRpc,
    CounterClearRpc, CounterInfoResp, CounterInfoRpc, CounterListResp, CounterListRpc,
    CounterRecord, CounterRemoveRpc, CounterUpdateRpc, CounterValuesRecord, CounterValuesResp,
    CounterValuesRpc,
};
pub use self::infobase::{
    infobase_info, infobase_summary_info, infobase_summary_list, InfobaseInfoResp,
    InfobaseSummary, InfobaseSummaryInfoResp, InfobaseSummaryListResp,
};
pub use self::lock::{lock_list, LockListResp, LockListRpc, LockRecordRaw};
pub use self::limit::{
    limit_info, limit_list, limit_remove, limit_update, LimitInfoResp, LimitInfoRpc, LimitListResp,
    LimitListRpc, LimitRecord, LimitRemoveRpc, LimitUpdateRpc,
};
pub use self::manager::{
    manager_info, manager_list, ManagerInfoResp, ManagerInfoRpc, ManagerListResp, ManagerListRpc,
    ManagerRecord,
};
pub use self::process::{
    process_info, process_list, ProcessInfoResp, ProcessLicense, ProcessListResp, ProcessRecord,
};
pub use self::profile::{
    profile_list, profile_update, ProfileListResp, ProfileRecord, ProfileUpdateRpc,
};
pub use self::rule::{
    rule_apply,
    rule_info,
    rule_insert,
    rule_list,
    rule_remove,
    rule_update,
    RuleApplyRpc,
    RuleIdRecord,
    RuleInfoResp,
    RuleInfoRpc,
    RuleInsertResp,
    RuleInsertRpc,
    RuleListResp,
    RuleListRpc,
    RuleRecord,
    RuleRemoveRpc,
    RuleUpdateResp,
    RuleUpdateRpc,
};
pub use self::server::{
    server_info,
    server_list,
    ServerInfoResp,
    ServerInfoRpc,
    ServerListResp,
    ServerListRpc,
    ServerRecord,
};
pub use self::session::{
    session_info,
    session_interrupt_current_server_call,
    session_list,
    session_terminate,
    SessionInfoResp,
    SessionInfoRpc,
    SessionLicense,
    SessionListResp,
    SessionListRpc,
    SessionRecord,
};
pub use self::service_setting::{
    service_setting_apply, service_setting_get_service_data_dirs_for_transfer,
    service_setting_info, service_setting_info_no_auth, service_setting_insert,
    service_setting_list, service_setting_remove, service_setting_update,
    service_setting_update_no_auth, ServiceSettingApplyRpc, ServiceSettingGetDataDirsResp,
    ServiceSettingGetDataDirsRpc, ServiceSettingIdRecord, ServiceSettingInfoResp,
    ServiceSettingInfoRpc, ServiceSettingInsertResp, ServiceSettingInsertRpc,
    ServiceSettingListResp, ServiceSettingListRpc, ServiceSettingRecord,
    ServiceSettingRemoveRpc, ServiceSettingTransferDataDirRecord, ServiceSettingUpdateResp,
    ServiceSettingUpdateRpc,
};

pub(crate) fn parse_list_u8<T, F>(body: &[u8], mut decode: F) -> Result<Vec<T>>
where
    F: FnMut(&mut RecordCursor<'_>) -> Result<T>,
{
    if body.is_empty() {
        return Ok(Vec::new());
    }
    let mut cursor = RecordCursor::new(body);
    let count = cursor.take_u8()? as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        out.push(decode(&mut cursor)?);
    }
    Ok(out)
}
