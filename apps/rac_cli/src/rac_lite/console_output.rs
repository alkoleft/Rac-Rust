use std::fmt::{self, Display, Write as _};

use serde::Serialize;

use rac_protocol::commands::{
    AgentAdminRecord, ClusterAdminRecord, ClusterRecord, ConnectionRecord, CounterRecord,
    CounterValuesRecord, InfobaseSummary, LimitRecord, LockRecordRaw, ManagerRecord, ProcessLicense,
    ProcessRecord, RuleInsertResp, RuleRecord, RuleUpdateResp,
    ServerRecord, ServiceSettingApplyResp, ServiceSettingInsertResp, ServiceSettingRecord,
    ServiceSettingRemoveResp, ServiceSettingTransferDataDirRecord, ServiceSettingUpdateResp,
    SessionLicense, SessionRecord,
};
use rac_protocol::rpc::AckResponse;
use rac_protocol::rac_wire::format_uuid;
use rac_protocol::Uuid16;

use super::format::{info_display_to_string, list_to_string, write_trimmed};
use super::format::MoreLabel;

macro_rules! outln {
    ($out:expr, $($arg:tt)*) => {
        let _ = writeln!($out, $($arg)*);
    };
}

macro_rules! opt_field {
    ($out:expr, $prefix:expr, $name:expr, $value:expr) => {
        let _ = writeln!($out, "{}{}: {}", $prefix, $name, $value);
    };
}

fn display_str(value: &str) -> &str {
    if value.is_empty() {
        "---"
    } else {
        value
    }
}

pub fn output<T, D>(json: bool, resp: &T, text: D)
where
    T: Serialize,
    D: Display,
{
    if json {
        match serde_json::to_string_pretty(resp) {
            Ok(payload) => println!("{payload}"),
            Err(err) => eprintln!("json error: {err}"),
        }
    } else {
        println!("{text}");
    }
}

pub struct UuidListDisplay<'a> {
    label: &'a str,
    items: &'a [Uuid16],
}

pub fn uuid_list<'a>(label: &'a str, items: &'a [Uuid16]) -> UuidListDisplay<'a> {
    UuidListDisplay { label, items }
}

impl Display for UuidListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string(self.label, self.items, 5, MoreLabel::Default, |out, _idx, uuid| {
            let _ = writeln!(out, "- {}", format_uuid(uuid));
        });
        write_trimmed(f, &out)
    }
}

pub struct StringListDisplay<'a> {
    label: &'a str,
    items: &'a [String],
}

pub fn string_list<'a>(label: &'a str, items: &'a [String]) -> StringListDisplay<'a> {
    StringListDisplay { label, items }
}

impl Display for StringListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string(self.label, self.items, 5, MoreLabel::Default, |out, _idx, value| {
            let _ = writeln!(out, "- {value}");
        });
        write_trimmed(f, &out)
    }
}

pub struct InfoDisplay<'a> {
    label: &'a str,
    uuid: &'a Uuid16,
    fields: &'a [String],
}

pub fn info<'a>(label: &'a str, uuid: &'a Uuid16, fields: &'a [String]) -> InfoDisplay<'a> {
    InfoDisplay {
        label,
        uuid,
        fields,
    }
}

impl Display for InfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = info_display_to_string(self.label, self.uuid, self.fields, 6, format_uuid);
        write_trimmed(f, &out)
    }
}

pub struct InfobaseSummaryListDisplay<'a> {
    items: &'a [InfobaseSummary],
}

pub fn infobase_summary_list(items: &[InfobaseSummary]) -> InfobaseSummaryListDisplay<'_> {
    InfobaseSummaryListDisplay { items }
}

impl Display for InfobaseSummaryListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("infobases", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "infobase[{idx}]: {}", format_uuid(&item.infobase));
            outln!(out, "name[{idx}]: {}", item.name);
            outln!(out, "descr[{idx}]: \"{}\"", item.descr);
        });
        write_trimmed(f, &out)
    }
}

pub struct SessionListDisplay<'a> {
    items: &'a [SessionRecord],
}

pub fn session_list(items: &[SessionRecord]) -> SessionListDisplay<'_> {
    SessionListDisplay { items }
}

impl Display for SessionListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out =
            list_to_string("sessions", self.items, 5, MoreLabel::Default, |out, _idx, item| {
            outln!(out, "{}", session_info(&item));
        });
        write_trimmed(f, &out)
    }
}

pub struct SessionInfoDisplay<'a> {
    item: &'a SessionRecord,
}

pub fn session_info(item: &SessionRecord) -> SessionInfoDisplay<'_> {
    SessionInfoDisplay { item }
}

pub struct ConnectionListDisplay<'a> {
    items: &'a [ConnectionRecord],
}

pub fn connection_list(items: &[ConnectionRecord]) -> ConnectionListDisplay<'_> {
    ConnectionListDisplay { items }
}

pub struct ConnectionInfoDisplay<'a> {
    item: &'a ConnectionRecord,
}

pub fn connection_info(item: &ConnectionRecord) -> ConnectionInfoDisplay<'_> {
    ConnectionInfoDisplay { item }
}

pub struct ManagerListDisplay<'a> {
    items: &'a [ManagerRecord],
}

pub fn manager_list(items: &[ManagerRecord]) -> ManagerListDisplay<'_> {
    ManagerListDisplay { items }
}

pub struct ManagerInfoDisplay<'a> {
    item: &'a ManagerRecord,
}

pub fn manager_info(item: &ManagerRecord) -> ManagerInfoDisplay<'_> {
    ManagerInfoDisplay { item }
}

pub struct ServerListDisplay<'a> {
    items: &'a [ServerRecord],
}

pub fn server_list(items: &[ServerRecord]) -> ServerListDisplay<'_> {
    ServerListDisplay { items }
}

pub struct ServerInfoDisplay<'a> {
    item: &'a ServerRecord,
}

pub fn server_info(item: &ServerRecord) -> ServerInfoDisplay<'_> {
    ServerInfoDisplay { item }
}

pub struct ProcessListDisplay<'a> {
    items: &'a [ProcessRecord],
}

pub fn process_list(items: &[ProcessRecord]) -> ProcessListDisplay<'_> {
    ProcessListDisplay { items }
}

pub struct LockListDisplay<'a> {
    items: &'a [LockRecordRaw],
}

pub fn lock_list(items: &[LockRecordRaw]) -> LockListDisplay<'_> {
    LockListDisplay { items }
}

pub struct RuleListDisplay<'a> {
    items: &'a [RuleRecord],
}

pub fn rule_list(items: &[RuleRecord]) -> RuleListDisplay<'_> {
    RuleListDisplay { items }
}

pub struct RuleInfoDisplay<'a> {
    item: &'a RuleRecord,
}

pub fn rule_info(item: &RuleRecord) -> RuleInfoDisplay<'_> {
    RuleInfoDisplay { item }
}

pub struct ProcessListLicensesDisplay<'a> {
    items: &'a [ProcessRecord],
}

pub fn process_list_licenses(items: &[ProcessRecord]) -> ProcessListLicensesDisplay<'_> {
    ProcessListLicensesDisplay { items }
}

pub struct ProcessInfoDisplay<'a> {
    item: &'a ProcessRecord,
}

pub fn process_info(item: &ProcessRecord) -> ProcessInfoDisplay<'_> {
    ProcessInfoDisplay { item }
}

pub struct ProcessInfoLicensesDisplay<'a> {
    item: &'a ProcessRecord,
}

pub fn process_info_licenses(item: &ProcessRecord) -> ProcessInfoLicensesDisplay<'_> {
    ProcessInfoLicensesDisplay { item }
}

pub struct LimitListDisplay<'a> {
    items: &'a [LimitRecord],
}

pub fn limit_list(items: &[LimitRecord]) -> LimitListDisplay<'_> {
    LimitListDisplay { items }
}

pub struct ServiceSettingListDisplay<'a> {
    items: &'a [ServiceSettingRecord],
}

pub fn service_setting_list(items: &[ServiceSettingRecord]) -> ServiceSettingListDisplay<'_> {
    ServiceSettingListDisplay { items }
}

pub struct ServiceSettingTransferDataDirsDisplay<'a> {
    items: &'a [ServiceSettingTransferDataDirRecord],
}

pub fn service_setting_get_data_dirs_for_transfer(
    items: &[ServiceSettingTransferDataDirRecord],
) -> ServiceSettingTransferDataDirsDisplay<'_> {
    ServiceSettingTransferDataDirsDisplay { items }
}

pub struct ServiceSettingInfoDisplay<'a> {
    item: &'a ServiceSettingRecord,
}

pub fn service_setting_info(item: &ServiceSettingRecord) -> ServiceSettingInfoDisplay<'_> {
    ServiceSettingInfoDisplay { item }
}

pub struct ServiceSettingInsertDisplay<'a> {
    resp: &'a ServiceSettingInsertResp,
}

pub fn service_setting_insert(resp: &ServiceSettingInsertResp) -> ServiceSettingInsertDisplay<'_> {
    ServiceSettingInsertDisplay { resp }
}

pub struct ServiceSettingUpdateDisplay<'a> {
    resp: &'a ServiceSettingUpdateResp,
}

pub fn service_setting_update(resp: &ServiceSettingUpdateResp) -> ServiceSettingUpdateDisplay<'_> {
    ServiceSettingUpdateDisplay { resp }
}

pub struct ServiceSettingRemoveDisplay<'a> {
    resp: &'a ServiceSettingRemoveResp,
}

pub fn service_setting_remove(resp: &ServiceSettingRemoveResp) -> ServiceSettingRemoveDisplay<'_> {
    ServiceSettingRemoveDisplay { resp }
}

pub struct ServiceSettingApplyDisplay<'a> {
    resp: &'a ServiceSettingApplyResp,
}

pub fn service_setting_apply(resp: &ServiceSettingApplyResp) -> ServiceSettingApplyDisplay<'_> {
    ServiceSettingApplyDisplay { resp }
}

pub struct LimitInfoDisplay<'a> {
    item: &'a LimitRecord,
}

pub fn limit_info(item: &LimitRecord) -> LimitInfoDisplay<'_> {
    LimitInfoDisplay { item }
}

pub struct LimitUpdateDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn limit_update(resp: &AckResponse) -> LimitUpdateDisplay<'_> {
    LimitUpdateDisplay { resp }
}

pub struct LimitRemoveDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn limit_remove(resp: &AckResponse) -> LimitRemoveDisplay<'_> {
    LimitRemoveDisplay { resp }
}

pub struct CounterListDisplay<'a> {
    items: &'a [CounterRecord],
}

pub fn counter_list(items: &[CounterRecord]) -> CounterListDisplay<'_> {
    CounterListDisplay { items }
}

pub struct CounterInfoDisplay<'a> {
    item: &'a CounterRecord,
}

pub fn counter_info(item: &CounterRecord) -> CounterInfoDisplay<'_> {
    CounterInfoDisplay { item }
}

pub struct CounterUpdateDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn counter_update(resp: &AckResponse) -> CounterUpdateDisplay<'_> {
    CounterUpdateDisplay { resp }
}

pub struct CounterClearDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn counter_clear(resp: &AckResponse) -> CounterClearDisplay<'_> {
    CounterClearDisplay { resp }
}

pub struct CounterRemoveDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn counter_remove(resp: &AckResponse) -> CounterRemoveDisplay<'_> {
    CounterRemoveDisplay { resp }
}

pub struct CounterValuesDisplay<'a> {
    items: &'a [CounterValuesRecord],
}

pub fn counter_values(items: &[CounterValuesRecord]) -> CounterValuesDisplay<'_> {
    CounterValuesDisplay { items }
}

pub struct CounterAccumulatedValuesDisplay<'a> {
    items: &'a [CounterValuesRecord],
}

pub fn counter_accumulated_values(
    items: &[CounterValuesRecord],
) -> CounterAccumulatedValuesDisplay<'_> {
    CounterAccumulatedValuesDisplay { items }
}

pub struct AgentAdminListDisplay<'a> {
    items: &'a [AgentAdminRecord],
}

pub fn agent_admin_list(items: &[AgentAdminRecord]) -> AgentAdminListDisplay<'_> {
    AgentAdminListDisplay { items }
}

pub struct ClusterAdminListDisplay<'a> {
    items: &'a [ClusterAdminRecord],
}

pub fn cluster_admin_list(items: &[ClusterAdminRecord]) -> ClusterAdminListDisplay<'_> {
    ClusterAdminListDisplay { items }
}

pub struct ClusterAdminRegisterDisplay {
    acknowledged: bool,
}

pub fn cluster_admin_register(acknowledged: bool) -> ClusterAdminRegisterDisplay {
    ClusterAdminRegisterDisplay { acknowledged }
}

pub struct RuleApplyDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn rule_apply(resp: &AckResponse) -> RuleApplyDisplay<'_> {
    RuleApplyDisplay { resp }
}

pub struct RuleInsertDisplay<'a> {
    resp: &'a RuleInsertResp,
}

pub fn rule_insert(resp: &RuleInsertResp) -> RuleInsertDisplay<'_> {
    RuleInsertDisplay { resp }
}

pub struct RuleUpdateDisplay<'a> {
    resp: &'a RuleUpdateResp,
}

pub fn rule_update(resp: &RuleUpdateResp) -> RuleUpdateDisplay<'_> {
    RuleUpdateDisplay { resp }
}

pub struct RuleRemoveDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn rule_remove(resp: &AckResponse) -> RuleRemoveDisplay<'_> {
    RuleRemoveDisplay { resp }
}

impl Display for ClusterAdminListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("cluster-admins", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "name[{idx}]: {}", display_str(&item.name));
            outln!(out, "unknown-tag[{idx}]: {}", item.unknown_tag);
            outln!(out, "unknown-flags[{idx}]: 0x{:08x}", item.unknown_flags);
            outln!(
                out,
                "unknown-tail[{idx}]: {:02x} {:02x} {:02x}",
                item.unknown_tail[0],
                item.unknown_tail[1],
                item.unknown_tail[2]
            );
        });
        write_trimmed(f, &out)
    }
}

impl Display for AgentAdminListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("agent-admins", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "name[{idx}]: {}", display_str(&item.name));
            outln!(out, "unknown-tag[{idx}]: {}", item.unknown_tag);
            outln!(out, "unknown-flags[{idx}]: 0x{:08x}", item.unknown_flags);
            outln!(
                out,
                "unknown-tail[{idx}]: {:02x} {:02x} {:02x}",
                item.unknown_tail[0],
                item.unknown_tail[1],
                item.unknown_tail[2]
            );
        });
        write_trimmed(f, &out)
    }
}

impl Display for ClusterAdminRegisterDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.acknowledged {
            "cluster-admin-register: ok"
        } else {
            "cluster-admin-register: failed"
        };
        write!(f, "{rendered}")
    }
}

impl Display for RuleApplyDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.resp.acknowledged {
            "rule-apply: ok"
        } else {
            "rule-apply: failed"
        };
        write!(f, "{rendered}")
    }
}

impl Display for RuleInsertDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        outln!(&mut out, "rule: {}", format_uuid(&self.resp.rule));
        write_trimmed(f, &out)
    }
}

impl Display for RuleUpdateDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        outln!(&mut out, "rule: {}", format_uuid(&self.resp.rule));
        write_trimmed(f, &out)
    }
}

impl Display for RuleRemoveDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.resp.acknowledged {
            "rule-remove: ok"
        } else {
            "rule-remove: failed"
        };
        write!(f, "{rendered}")
    }
}

impl Display for CounterUpdateDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.resp.acknowledged {
            "counter-update: ok"
        } else {
            "counter-update: failed"
        };
        write!(f, "{rendered}")
    }
}

impl Display for CounterClearDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.resp.acknowledged {
            "counter-clear: ok"
        } else {
            "counter-clear: failed"
        };
        write!(f, "{rendered}")
    }
}

impl Display for CounterRemoveDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.resp.acknowledged {
            "counter-remove: ok"
        } else {
            "counter-remove: failed"
        };
        write!(f, "{rendered}")
    }
}

impl Display for LimitUpdateDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.resp.acknowledged {
            "limit-update: ok"
        } else {
            "limit-update: failed"
        };
        write!(f, "{rendered}")
    }
}

impl Display for LimitRemoveDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.resp.acknowledged {
            "limit-remove: ok"
        } else {
            "limit-remove: failed"
        };
        write!(f, "{rendered}")
    }
}

impl Display for LimitListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("limits", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "limit[{idx}]: {}", display_str(&item.name));
            outln!(out, "counter[{idx}]: {}", display_str(&item.counter));
            outln!(out, "action[{idx}]: {}", item.action);
            outln!(out, "duration[{idx}]: {}", item.duration);
            outln!(out, "cpu-time[{idx}]: {}", item.cpu_time);
            outln!(out, "memory[{idx}]: {}", item.memory);
            outln!(out, "read[{idx}]: {}", item.read);
            outln!(out, "write[{idx}]: {}", item.write);
            outln!(out, "duration-dbms[{idx}]: {}", item.duration_dbms);
            outln!(out, "dbms-bytes[{idx}]: {}", item.dbms_bytes);
            outln!(out, "service[{idx}]: {}", item.service);
            outln!(out, "call[{idx}]: {}", item.call);
            outln!(
                out,
                "number-of-active-sessions[{idx}]: {}",
                item.number_of_active_sessions
            );
            outln!(
                out,
                "number-of-sessions[{idx}]: {}",
                item.number_of_sessions
            );
            outln!(
                out,
                "error-message[{idx}]: {}",
                display_str(&item.error_message)
            );
            outln!(out, "descr[{idx}]: {}", display_str(&item.descr));
        });
        write_trimmed(f, &out)
    }
}

impl Display for ServiceSettingListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string(
            "service-settings",
            self.items,
            5,
            MoreLabel::Default,
            |out, idx, item| {
                outln!(out, "setting[{idx}]: {}", format_uuid(&item.setting));
                outln!(
                    out,
                    "service-name[{idx}]: {}",
                    display_str(&item.service_name)
                );
                outln!(
                    out,
                    "infobase-name[{idx}]: {}",
                    display_str(&item.infobase_name)
                );
                outln!(
                    out,
                    "service-data-dir[{idx}]: {}",
                    display_str(&item.service_data_dir)
                );
                outln!(
                    out,
                    "active[{idx}]: {}",
                    if item.active { "yes" } else { "no" }
                );
            },
        );
        write_trimmed(f, &out)
    }
}

impl Display for ServiceSettingTransferDataDirsDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string(
            "service-data-dirs",
            self.items,
            5,
            MoreLabel::Default,
            |out, idx, item| {
                outln!(
                    out,
                    "service-name[{idx}]: {}",
                    display_str(&item.service_name)
                );
                outln!(out, "user[{idx}]: {}", display_str(&item.user));
                outln!(
                    out,
                    "source-dir[{idx}]: {}",
                    display_str(&item.source_dir)
                );
                outln!(
                    out,
                    "target-dir[{idx}]: {}",
                    display_str(&item.target_dir)
                );
            },
        );
        write_trimmed(f, &out)
    }
}

impl Display for ServiceSettingInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        outln!(&mut out, "setting: {}", format_uuid(&item.setting));
        outln!(
            &mut out,
            "service-name: {}",
            display_str(&item.service_name)
        );
        outln!(
            &mut out,
            "infobase-name: {}",
            display_str(&item.infobase_name)
        );
        outln!(
            &mut out,
            "service-data-dir: {}",
            display_str(&item.service_data_dir)
        );
        outln!(
            &mut out,
            "active: {}",
            if item.active { "yes" } else { "no" }
        );
        write_trimmed(f, &out)
    }
}

impl Display for ServiceSettingInsertDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        outln!(&mut out, "setting: {}", format_uuid(&self.resp.setting));
        write_trimmed(f, &out)
    }
}

impl Display for ServiceSettingUpdateDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        outln!(&mut out, "setting: {}", format_uuid(&self.resp.setting));
        write_trimmed(f, &out)
    }
}

impl Display for ServiceSettingRemoveDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.resp.acknowledged {
            "service-setting-remove: ok"
        } else {
            "service-setting-remove: failed"
        };
        write!(f, "{rendered}")
    }
}

impl Display for ServiceSettingApplyDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.resp.acknowledged {
            "service-setting-apply: ok"
        } else {
            "service-setting-apply: failed"
        };
        write!(f, "{rendered}")
    }
}

impl Display for LimitInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        outln!(&mut out, "limit: {}", display_str(&item.name));
        outln!(&mut out, "counter: {}", display_str(&item.counter));
        outln!(&mut out, "action: {}", item.action);
        outln!(&mut out, "duration: {}", item.duration);
        outln!(&mut out, "cpu-time: {}", item.cpu_time);
        outln!(&mut out, "memory: {}", item.memory);
        outln!(&mut out, "read: {}", item.read);
        outln!(&mut out, "write: {}", item.write);
        outln!(&mut out, "duration-dbms: {}", item.duration_dbms);
        outln!(&mut out, "dbms-bytes: {}", item.dbms_bytes);
        outln!(&mut out, "service: {}", item.service);
        outln!(&mut out, "call: {}", item.call);
        outln!(
            &mut out,
            "number-of-active-sessions: {}",
            item.number_of_active_sessions
        );
        outln!(&mut out, "number-of-sessions: {}", item.number_of_sessions);
        outln!(
            &mut out,
            "error-message: {}",
            display_str(&item.error_message)
        );
        outln!(&mut out, "descr: {}", display_str(&item.descr));
        write_trimmed(f, &out)
    }
}

impl Display for CounterListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("counters", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "name[{idx}]: {}", display_str(&item.name));
            outln!(out, "collection-time[{idx}]: {}", item.collection_time);
            outln!(out, "group[{idx}]: {}", item.group);
            outln!(out, "filter-type[{idx}]: {}", item.filter_type);
            outln!(out, "filter[{idx}]: {}", display_str(&item.filter));
            outln!(out, "duration[{idx}]: {}", item.duration);
            outln!(out, "cpu-time[{idx}]: {}", item.cpu_time);
            outln!(out, "duration-dbms[{idx}]: {}", item.duration_dbms);
            outln!(out, "service[{idx}]: {}", item.service);
            outln!(out, "memory[{idx}]: {}", item.memory);
            outln!(out, "read[{idx}]: {}", item.read);
            outln!(out, "write[{idx}]: {}", item.write);
            outln!(out, "dbms-bytes[{idx}]: {}", item.dbms_bytes);
            outln!(out, "call[{idx}]: {}", item.call);
            outln!(
                out,
                "number-of-active-sessions[{idx}]: {}",
                item.number_of_active_sessions
            );
            outln!(
                out,
                "number-of-sessions[{idx}]: {}",
                item.number_of_sessions
            );
            outln!(out, "descr[{idx}]: {}", display_str(&item.descr));
        });
        write_trimmed(f, &out)
    }
}

impl Display for CounterInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        outln!(&mut out, "name: {}", display_str(&item.name));
        outln!(&mut out, "collection-time: {}", item.collection_time);
        outln!(&mut out, "group: {}", item.group);
        outln!(&mut out, "filter-type: {}", item.filter_type);
        outln!(&mut out, "filter: {}", display_str(&item.filter));
        outln!(&mut out, "duration: {}", item.duration);
        outln!(&mut out, "cpu-time: {}", item.cpu_time);
        outln!(&mut out, "duration-dbms: {}", item.duration_dbms);
        outln!(&mut out, "service: {}", item.service);
        outln!(&mut out, "memory: {}", item.memory);
        outln!(&mut out, "read: {}", item.read);
        outln!(&mut out, "write: {}", item.write);
        outln!(&mut out, "dbms-bytes: {}", item.dbms_bytes);
        outln!(&mut out, "call: {}", item.call);
        outln!(
            &mut out,
            "number-of-active-sessions: {}",
            item.number_of_active_sessions
        );
        outln!(&mut out, "number-of-sessions: {}", item.number_of_sessions);
        outln!(&mut out, "descr: {}", display_str(&item.descr));
        write_trimmed(f, &out)
    }
}

impl Display for CounterValuesDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_trimmed(f, &render_counter_values("counter-values", self.items))
    }
}

impl Display for CounterAccumulatedValuesDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_trimmed(
            f,
            &render_counter_values("counter-accumulated-values", self.items),
        )
    }
}

fn render_counter_values(label: &str, items: &[CounterValuesRecord]) -> String {
    list_to_string(label, items, 5, MoreLabel::Default, |out, idx, item| {
        outln!(out, "object[{idx}]: {}", display_str(&item.object));
        outln!(out, "collection-time[{idx}]: {}", item.collection_time);
        outln!(out, "duration[{idx}]: {}", item.duration);
        outln!(out, "cpu-time[{idx}]: {}", item.cpu_time);
        outln!(out, "memory[{idx}]: {}", item.memory);
        outln!(out, "read[{idx}]: {}", item.read);
        outln!(out, "write[{idx}]: {}", item.write);
        outln!(out, "duration-dbms[{idx}]: {}", item.duration_dbms);
        outln!(out, "dbms-bytes[{idx}]: {}", item.dbms_bytes);
        outln!(out, "service[{idx}]: {}", item.service);
        outln!(out, "call[{idx}]: {}", item.call);
        outln!(
            out,
            "number-of-active-sessions[{idx}]: {}",
            item.number_of_active_sessions
        );
        outln!(
            out,
            "number-of-sessions[{idx}]: {}",
            item.number_of_sessions
        );
        outln!(out, "time[{idx}]: {}", display_str(&item.time));
    })
}

impl Display for ManagerListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("managers", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "manager[{idx}]: {}", format_uuid(&item.manager));
            outln!(out, "pid[{idx}]: {}", display_str(&item.pid));
            outln!(out, "using[{idx}]: {}", manager_using_label(item.using));
            outln!(out, "host[{idx}]: {}", display_str(&item.host));
            outln!(out, "port[{idx}]: {}", item.port);
            outln!(out, "descr[{idx}]: \"{}\"", display_str(&item.descr));
        });
        write_trimmed(f, &out)
    }
}

impl Display for ManagerInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        outln!(&mut out, "manager: {}", format_uuid(&item.manager));
        outln!(&mut out, "pid: {}", display_str(&item.pid));
        outln!(&mut out, "using: {}", manager_using_label(item.using));
        outln!(&mut out, "host: {}", display_str(&item.host));
        outln!(&mut out, "port: {}", item.port);
        outln!(&mut out, "descr: \"{}\"", display_str(&item.descr));
        write_trimmed(f, &out)
    }
}

impl Display for ServerListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("servers", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "server[{idx}]: {}", format_uuid(&item.server));
            outln!(out, "agent-host[{idx}]: {}", display_str(&item.agent_host));
            outln!(out, "agent-port[{idx}]: {}", item.agent_port);
            outln!(out, "name[{idx}]: \"{}\"", display_str(&item.name));
            outln!(out, "using[{idx}]: {}", server_using_label(item.using));
            outln!(
                out,
                "dedicate-managers[{idx}]: {}",
                dedicate_managers_label(item.dedicate_managers)
            );
            outln!(out, "infobases-limit[{idx}]: {}", item.infobases_limit);
            outln!(
                out,
                "safe-call-memory-limit[{idx}]: {}",
                item.safe_call_memory_limit
            );
            outln!(out, "connections-limit[{idx}]: {}", item.connections_limit);
            outln!(out, "cluster-port[{idx}]: {}", item.cluster_port);
            outln!(
                out,
                "port-range[{idx}]: {}:{}",
                item.port_range_start,
                item.port_range_end
            );
            outln!(
                out,
                "critical-total-memory[{idx}]: {}",
                item.critical_total_memory
            );
            outln!(
                out,
                "temporary-allowed-total-memory[{idx}]: {}",
                item.temporary_allowed_total_memory
            );
            outln!(
                out,
                "temporary-allowed-total-memory-time-limit[{idx}]: {}",
                item.temporary_allowed_total_memory_time_limit
            );
            outln!(
                out,
                "service-principal-name[{idx}]: \"{}\"",
                display_str(&item.service_principal_name)
            );
            outln!(
                out,
                "restart-schedule[{idx}]: \"{}\"",
                display_str(&item.restart_schedule)
            );
        });
        write_trimmed(f, &out)
    }
}

impl Display for ServerInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        outln!(&mut out, "server: {}", format_uuid(&item.server));
        outln!(&mut out, "agent-host: {}", display_str(&item.agent_host));
        outln!(&mut out, "agent-port: {}", item.agent_port);
        outln!(&mut out, "name: \"{}\"", display_str(&item.name));
        outln!(&mut out, "using: {}", server_using_label(item.using));
        outln!(
            &mut out,
            "dedicate-managers: {}",
            dedicate_managers_label(item.dedicate_managers)
        );
        outln!(&mut out, "infobases-limit: {}", item.infobases_limit);
        outln!(
            &mut out,
            "safe-call-memory-limit: {}",
            item.safe_call_memory_limit
        );
        outln!(&mut out, "connections-limit: {}", item.connections_limit);
        outln!(&mut out, "cluster-port: {}", item.cluster_port);
        outln!(
            &mut out,
            "port-range: {}:{}",
            item.port_range_start,
            item.port_range_end
        );
        outln!(
            &mut out,
            "critical-total-memory: {}",
            item.critical_total_memory
        );
        outln!(
            &mut out,
            "temporary-allowed-total-memory: {}",
            item.temporary_allowed_total_memory
        );
        outln!(
            &mut out,
            "temporary-allowed-total-memory-time-limit: {}",
            item.temporary_allowed_total_memory_time_limit
        );
        outln!(
            &mut out,
            "service-principal-name: \"{}\"",
            display_str(&item.service_principal_name)
        );
        outln!(
            &mut out,
            "restart-schedule: \"{}\"",
            display_str(&item.restart_schedule)
        );
        write_trimmed(f, &out)
    }
}

impl Display for ProcessListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out =
            list_to_string("processes", self.items, 5, MoreLabel::Default, |out, _idx, item| {
            outln!(out, "{}", process_info(item));
        });
        write_trimmed(f, &out)
    }
}

impl Display for LockListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("locks", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "connection[{idx}]: {}", format_uuid(&item.connection));
            outln!(out, "descr[{idx}]: {}", display_str(&item.descr.descr));
            if let Some(flag) = item.descr.descr_flag {
                outln!(out, "descr-flag[{idx}]: {flag}");
            }
            outln!(out, "locked-at[{idx}]: {}", display_str(&item.locked_at));
            outln!(out, "session[{idx}]: {}", format_uuid(&item.session));
            outln!(out, "object[{idx}]: {}", format_uuid(&item.object));
        });
        write_trimmed(f, &out)
    }
}

impl Display for RuleListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("rules", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "rule[{idx}]: {}", format_uuid(&item.rule));
            outln!(out, "object-type[{idx}]: {}", item.object_type);
            outln!(out, "infobase-name[{idx}]: {}", display_str(&item.infobase_name));
            outln!(
                out,
                "rule-type[{idx}]: {}",
                rule_type_label(item.rule_type)
            );
            outln!(
                out,
                "application-ext[{idx}]: {}",
                display_str(&item.application_ext)
            );
            outln!(out, "priority[{idx}]: {}", item.priority);
        });
        write_trimmed(f, &out)
    }
}

impl Display for RuleInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        outln!(&mut out, "rule: {}", format_uuid(&item.rule));
        outln!(&mut out, "object-type: {}", item.object_type);
        outln!(&mut out, "infobase-name: {}", display_str(&item.infobase_name));
        outln!(&mut out, "rule-type: {}", rule_type_label(item.rule_type));
        outln!(
            &mut out,
            "application-ext: {}",
            display_str(&item.application_ext)
        );
        outln!(&mut out, "priority: {}", item.priority);
        write_trimmed(f, &out)
    }
}

impl Display for ProcessListLicensesDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("processes", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "process[{idx}]: {}", format_uuid(&item.process));
            outln!(out, "host[{idx}]: {}", display_str(&item.host));
            outln!(out, "port[{idx}]: {}", item.port);
            outln!(out, "pid[{idx}]: {}", display_str(&item.pid));

            let license_count = item.licenses.len();
            if license_count == 0 {
                return;
            }

            for (license_idx, license) in item.licenses.iter().enumerate() {
                let suffix = if license_count > 1 {
                    Some(license_idx)
                } else {
                    None
                };
                append_process_license(out, idx, suffix, license);
            }
        });
        write_trimmed(f, &out)
    }
}

impl Display for ProcessInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        outln!(&mut out, "process: {}", format_uuid(&item.process));
        outln!(&mut out, "host: {}", display_str(&item.host));
        outln!(&mut out, "port: {}", item.port);
        outln!(&mut out, "pid: {}", display_str(&item.pid));
        outln!(&mut out, "turned-on: {}", yes_no(item.turned_on));
        outln!(&mut out, "running: {}", yes_no(item.running));
        outln!(&mut out, "started-at: {}", display_str(&item.started_at));
        outln!(&mut out, "use: {}", process_use_label(item.use_status));
        outln!(
            &mut out,
            "available-performance: {}",
            item.available_performance
        );
        outln!(&mut out, "capacity: {}", item.capacity);
        outln!(&mut out, "connections: {}", item.connections);
        outln!(&mut out, "memory-size: {}", item.memory_size);
        outln!(
            &mut out,
            "memory-excess-time: {}",
            item.memory_excess_time
        );
        outln!(&mut out, "selection-size: {}", item.selection_size);
        outln!(&mut out, "avg-call-time: {:.3}", item.avg_call_time);
        outln!(&mut out, "avg-db-call-time: {:.3}", item.avg_db_call_time);
        outln!(
            &mut out,
            "avg-lock-call-time: {:.3}",
            item.avg_lock_call_time
        );
        outln!(
            &mut out,
            "avg-server-call-time: {:.3}",
            item.avg_server_call_time
        );
        outln!(&mut out, "avg-threads: {:.3}", item.avg_threads);
        outln!(&mut out, "reserve: {}", yes_no(item.reserve));
        write_trimmed(f, &out)
    }
}

impl Display for ProcessInfoLicensesDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        outln!(&mut out, "process: {}", format_uuid(&item.process));
        outln!(&mut out, "host: {}", display_str(&item.host));
        outln!(&mut out, "port: {}", item.port);
        outln!(&mut out, "pid: {}", display_str(&item.pid));
        for license in &item.licenses {
            append_process_license_plain(&mut out, license);
        }
        write_trimmed(f, &out)
    }
}

impl Display for SessionInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        outln!(&mut out, "session: {}", format_uuid(&item.session));
        outln!(&mut out, "app-id: {}", display_str(&item.app_id));
        outln!(&mut out, "connection: {}", format_uuid(&item.connection));
        outln!(&mut out, "infobase: {}", format_uuid(&item.infobase));
        outln!(&mut out, "process: {}", format_uuid(&item.process));
        outln!(&mut out, "host: {}", display_str(&item.host));
        outln!(&mut out, "hibernate: {}", yes_no(item.hibernate));
        outln!(&mut out, "locale: {}", display_str(&item.locale));
        outln!(&mut out, "user-name: {}", display_str(&item.user_name));
        outln!(&mut out, "started-at: {}", display_str(&item.started_at));
        outln!(
            &mut out,
            "last-active-at: {}",
            display_str(&item.last_active_at)
        );
        outln!(&mut out, "client-ip: {}", display_str(&item.client_ip));
        outln!(&mut out, "db-proc-info: {}", display_str(&item.db_proc_info));
        outln!(
            &mut out,
            "db-proc-took-at: {}",
            display_str(&item.db_proc_took_at)
        );
        outln!(
            &mut out,
            "current-service-name: {}",
            display_str(&item.current_service_name)
        );
        outln!(
            &mut out,
            "data-separation: {}",
            display_str(&item.data_separation)
        );
        append_license_prefixed(&mut out, &item.license, "license.");
        outln!(&mut out, "session-id: {}", item.session_id);
        append_counters_prefixed(&mut out, item, "");
        write_trimmed(f, &out)
    }
}

impl Display for ConnectionListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("connections", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "connection[{idx}]: {}", format_uuid(&item.connection));
            outln!(out, "application[{idx}]: {}", display_str(&item.application));
            outln!(out, "connected-at[{idx}]: {}", display_str(&item.connected_at));
            outln!(out, "conn-id[{idx}]: {}", item.conn_id);
            outln!(out, "host[{idx}]: {}", display_str(&item.host));
            outln!(out, "infobase[{idx}]: {}", format_uuid(&item.infobase));
            outln!(out, "process[{idx}]: {}", format_uuid(&item.process));
            outln!(out, "session-number[{idx}]: {}", item.session_number);
            outln!(out, "blocked-by-ls[{idx}]: {}", item.blocked_by_ls);
        });
        write_trimmed(f, &out)
    }
}

impl Display for ConnectionInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        outln!(&mut out, "connection: {}", format_uuid(&item.connection));
        outln!(&mut out, "application: {}", display_str(&item.application));
        outln!(&mut out, "connected-at: {}", display_str(&item.connected_at));
        outln!(&mut out, "conn-id: {}", item.conn_id);
        outln!(&mut out, "host: {}", display_str(&item.host));
        outln!(&mut out, "infobase: {}", format_uuid(&item.infobase));
        outln!(&mut out, "process: {}", format_uuid(&item.process));
        outln!(&mut out, "session-number: {}", item.session_number);
        outln!(&mut out, "blocked-by-ls: {}", item.blocked_by_ls);
        write_trimmed(f, &out)
    }
}

pub struct AgentVersionDisplay<'a> {
    version: &'a str,
}

pub fn agent_version(version: &str) -> AgentVersionDisplay<'_> {
    AgentVersionDisplay { version }
}

impl Display for AgentVersionDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "version: {}", self.version)
    }
}

pub struct ClusterListDisplay<'a> {
    clusters: &'a [ClusterRecord],
}

pub fn cluster_list(clusters: &[ClusterRecord]) -> ClusterListDisplay<'_> {
    ClusterListDisplay { clusters }
}

fn load_balancing_mode_name(value: u32) -> &'static str {
    match value {
        1 => "memory",
        _ => "performance",
    }
}

impl Display for ClusterListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for (idx, cluster) in self.clusters.iter().enumerate() {
            if idx > 0 {
                out.push('\n');
            }
            outln!(&mut out, "cluster                                   : {}", format_uuid(&cluster.uuid));
            outln!(&mut out, "host                                      : {}", display_str(&cluster.host));
            outln!(&mut out, "port                                      : {}", cluster.port);
            outln!(
                &mut out,
                "name                                      : \"{}\"",
                display_str(&cluster.display_name)
            );
            outln!(
                &mut out,
                "expiration-timeout                        : {}",
                cluster.expiration_timeout
            );
            outln!(
                &mut out,
                "lifetime-limit                            : {}",
                cluster.lifetime_limit
            );
            outln!(
                &mut out,
                "max-memory-size                           : {}",
                cluster.max_memory_size
            );
            outln!(
                &mut out,
                "max-memory-time-limit                     : {}",
                cluster.max_memory_time_limit
            );
            outln!(
                &mut out,
                "security-level                            : {}",
                cluster.security_level
            );
            outln!(
                &mut out,
                "session-fault-tolerance-level             : {}",
                cluster.session_fault_tolerance_level
            );
            outln!(
                &mut out,
                "load-balancing-mode                       : {}",
                load_balancing_mode_name(cluster.load_balancing_mode)
            );
            outln!(
                &mut out,
                "errors-count-threshold                    : {}",
                cluster.errors_count_threshold
            );
            outln!(
                &mut out,
                "kill-problem-processes                    : {}",
                cluster.kill_problem_processes
            );
            outln!(
                &mut out,
                "kill-by-memory-with-dump                  : {}",
                cluster.kill_by_memory_with_dump
            );
            outln!(
                &mut out,
                "allow-access-right-audit-events-recording : {}",
                cluster.allow_access_right_audit_events_recording
            );
            outln!(
                &mut out,
                "ping-period                               : {}",
                cluster.ping_period
            );
            outln!(
                &mut out,
                "ping-timeout                              : {}",
                cluster.ping_timeout
            );
            outln!(
                &mut out,
                "restart-schedule                          : \"{}\"",
                display_str(&cluster.restart_schedule_cron)
            );
        }
        write_trimmed(f, &out)
    }
}

pub struct ClusterInfoDisplay<'a> {
    cluster: &'a ClusterRecord,
}

pub fn cluster_info(cluster: &ClusterRecord) -> ClusterInfoDisplay<'_> {
    ClusterInfoDisplay { cluster }
}

impl Display for ClusterInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        outln!(
            &mut out,
            "cluster                                   : {}",
            format_uuid(&self.cluster.uuid)
        );
        outln!(&mut out, "host                                      : {}", display_str(&self.cluster.host));
        outln!(&mut out, "port                                      : {}", self.cluster.port);
        outln!(
            &mut out,
            "name                                      : \"{}\"",
            display_str(&self.cluster.display_name)
        );
        outln!(
            &mut out,
            "expiration-timeout                        : {}",
            self.cluster.expiration_timeout
        );
        outln!(
            &mut out,
            "lifetime-limit                            : {}",
            self.cluster.lifetime_limit
        );
        outln!(
            &mut out,
            "max-memory-size                           : {}",
            self.cluster.max_memory_size
        );
        outln!(
            &mut out,
            "max-memory-time-limit                     : {}",
            self.cluster.max_memory_time_limit
        );
        outln!(
            &mut out,
            "security-level                            : {}",
            self.cluster.security_level
        );
        outln!(
            &mut out,
            "session-fault-tolerance-level             : {}",
            self.cluster.session_fault_tolerance_level
        );
        outln!(
            &mut out,
            "load-balancing-mode                       : {}",
            load_balancing_mode_name(self.cluster.load_balancing_mode)
        );
        outln!(
            &mut out,
            "errors-count-threshold                    : {}",
            self.cluster.errors_count_threshold
        );
        outln!(
            &mut out,
            "kill-problem-processes                    : {}",
            self.cluster.kill_problem_processes
        );
        outln!(
            &mut out,
            "kill-by-memory-with-dump                  : {}",
            self.cluster.kill_by_memory_with_dump
        );
        outln!(
            &mut out,
            "allow-access-right-audit-events-recording : {}",
            self.cluster.allow_access_right_audit_events_recording
        );
        outln!(
            &mut out,
            "ping-period                               : {}",
            self.cluster.ping_period
        );
        outln!(
            &mut out,
            "ping-timeout                              : {}",
            self.cluster.ping_timeout
        );
        outln!(
            &mut out,
            "restart-schedule                          : \"{}\"",
            display_str(&self.cluster.restart_schedule_cron)
        );
        write_trimmed(f, &out)
    }
}

fn append_license_prefixed(out: &mut String, license: &SessionLicense, prefix: &str) {
    outln!(
        out,
        "{prefix}license-type: '{}'",
        license.license_type
    );
    outln!(
        out,
        "{prefix}file-name: '{}'",
        display_str(&license.file_name)
    );
    outln!(
        out,
        "{prefix}server-address: '{}'",
        display_str(&license.server_address)
    );
    outln!(
        out,
        "{prefix}server-port: {}",
        license.server_port
    );

    outln!(
        out,
        "{prefix}process-id: {}",
        display_str(&license.process_id)
    );
    outln!(
        out,
        "{prefix}max-users: {}",
        license.max_users_all
    );
    outln!(
        out,
        "{prefix}max-software-license-users: {}",
        license.max_users_current
    );

    outln!(
        out,
        "{prefix}brief-presentation: '{}'",
        display_str(&license.brief_presentation)
    );
    outln!(
        out,
        "{prefix}detailed-presentation: '{}'",
        display_str(&license.full_presentation)
    );
    outln!(
        out,
        "{prefix}key-series: {}",
        display_str(&license.key_series)
    );
    outln!(
        out,
        "{prefix}retrieved-by-server: {}",
        yes_no(license.issued_by_server)
    );
    outln!(out, "{prefix}network-key: {}", yes_no(license.network_key));
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn using_label(value: u32) -> String {
    match value {
        1 => "main".to_string(),
        _ => value.to_string(),
    }
}

fn manager_using_label(value: u32) -> String {
    using_label(value)
}

fn process_use_label(value: u32) -> String {
    match value {
        1 => "used".to_string(),
        _ => value.to_string(),
    }
}

fn process_license_type_label(value: u32) -> String {
    match value {
        0 => "soft".to_string(),
        _ => value.to_string(),
    }
}

fn rule_type_label(value: u8) -> String {
    match value {
        1 => "auto".to_string(),
        2 => "always".to_string(),
        3 => "never".to_string(),
        _ => value.to_string(),
    }
}

fn server_using_label(value: u32) -> String {
    using_label(value)
}

fn dedicate_managers_label(value: u32) -> String {
    match value {
        0 => "none".to_string(),
        _ => value.to_string(),
    }
}

fn append_process_license(
    out: &mut String,
    record_idx: usize,
    license_idx: Option<usize>,
    license: &ProcessLicense,
) {
    let label = |name: &str| match license_idx {
        Some(license_idx) => format!("{name}[{record_idx}.{license_idx}]"),
        None => format!("{name}[{record_idx}]"),
    };
    append_process_license_fields(out, label, license);
}

fn append_process_license_plain(out: &mut String, license: &ProcessLicense) {
    append_process_license_fields(out, |name| name.to_string(), license);
}

fn append_process_license_fields<F>(out: &mut String, label: F, license: &ProcessLicense)
where
    F: Fn(&str) -> String,
{
    outln!(out, "{}: \"{}\"", label("full-name"), display_str(&license.file_name));
    outln!(out, "{}: \"{}\"", label("series"), display_str(&license.key_series));
    outln!(
        out,
        "{}: {}",
        label("issued-by-server"),
        yes_no(license.issued_by_server)
    );
    outln!(
        out,
        "{}: {}",
        label("license-type"),
        process_license_type_label(license.license_type)
    );
    outln!(out, "{}: {}", label("net"), yes_no(license.network_key));
    outln!(out, "{}: {}", label("max-users-all"), license.max_users_all);
    outln!(
        out,
        "{}: {}",
        label("max-users-cur"),
        license.max_users_current
    );
    outln!(
        out,
        "{}: \"{}\"",
        label("rmngr-address"),
        display_str(&license.server_address)
    );
    outln!(out, "{}: {}", label("rmngr-port"), license.server_port);
    outln!(
        out,
        "{}: {}",
        label("rmngr-pid"),
        display_str(&license.process_id)
    );
    outln!(
        out,
        "{}: \"{}\"",
        label("short-presentation"),
        display_str(&license.brief_presentation)
    );
    outln!(
        out,
        "{}: \"{}\"",
        label("full-presentation"),
        display_str(&license.full_presentation)
    );
}

fn append_counters_prefixed(out: &mut String, record: &SessionRecord, prefix: &str) {
    opt_field!(out, prefix, "blocked-by-dbms", record.blocked_by_dbms);
    opt_field!(out, prefix, "blocked-by-ls", record.blocked_by_ls);
    opt_field!(out, prefix, "bytes-all", record.bytes_all);
    opt_field!(out, prefix, "bytes-last-5min", record.bytes_last_5min);
    opt_field!(out, prefix, "calls-all", record.calls_all);
    opt_field!(out, prefix, "calls-last-5min", record.calls_last_5min);
    opt_field!(out, prefix, "dbms-bytes-all", record.dbms_bytes_all);
    opt_field!(
        out,
        prefix,
        "dbms-bytes-last-5min",
        record.dbms_bytes_last_5min
    );
    opt_field!(out, prefix, "db-proc-took", record.db_proc_took);
    opt_field!(out, prefix, "duration-all", record.duration_all);
    opt_field!(out, prefix, "duration-all-dbms", record.duration_all_dbms);
    opt_field!(out, prefix, "duration-current", record.duration_current);
    opt_field!(
        out,
        prefix,
        "duration-current-dbms",
        record.duration_current_dbms
    );
    opt_field!(
        out,
        prefix,
        "duration-last-5min",
        record.duration_last_5min
    );
    opt_field!(
        out,
        prefix,
        "duration-last-5min-dbms",
        record.duration_last_5min_dbms
    );
    opt_field!(
        out,
        prefix,
        "passive-session-hibernate-time",
        record.passive_session_hibernate_time
    );
    opt_field!(
        out,
        prefix,
        "hibernate-session-terminate-time",
        record.hibernate_session_terminate_time
    );
    opt_field!(out, prefix, "memory-current", record.memory_current);
    opt_field!(out, prefix, "memory-last-5min", record.memory_last_5min);
    opt_field!(out, prefix, "memory-total", record.memory_total);
    opt_field!(out, prefix, "read-current", record.read_current);
    opt_field!(out, prefix, "read-last-5min", record.read_last_5min);
    opt_field!(out, prefix, "read-total", record.read_total);
    opt_field!(out, prefix, "write-current", record.write_current);
    opt_field!(out, prefix, "write-last-5min", record.write_last_5min);
    opt_field!(out, prefix, "write-total", record.write_total);
    opt_field!(
        out,
        prefix,
        "duration-current-service",
        record.duration_current_service
    );
    opt_field!(
        out,
        prefix,
        "duration-last-5min-service",
        record.duration_last_5min_service
    );
    opt_field!(
        out,
        prefix,
        "duration-all-service",
        record.duration_all_service
    );
    opt_field!(
        out,
        prefix,
        "cpu-time-last-5min",
        record.cpu_time_last_5min
    );
    opt_field!(out, prefix, "cpu-time-current", record.cpu_time_current);
    opt_field!(out, prefix, "cpu-time-total", record.cpu_time_total);
}
