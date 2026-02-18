use std::fmt::{self, Display, Write as _};

use serde::Serialize;

use rac_protocol::commands::{
    AgentVersionResp, ClusterAdminRecord, ClusterAdminRegisterResp, ClusterInfoResp,
    ClusterListResp, ConnectionRecord, InfobaseSummary, LimitRecord, LockRecord, ManagerRecord,
    ProcessLicense, ProcessRecord, RuleApplyResp, ServerRecord, SessionCounters, SessionLicense,
    SessionRecord,
};
use rac_protocol::rac_wire::format_uuid;
use rac_protocol::Uuid16;

use super::format::{
    append_opt_yes_no, info_display_to_string, list_to_string, write_trimmed,
};
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
        let out = list_to_string("sessions", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "session[{idx}]: {}", format_uuid(&item.session));
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
    items: &'a [LockRecord],
}

pub fn lock_list(items: &[LockRecord]) -> LockListDisplay<'_> {
    LockListDisplay { items }
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

pub struct ClusterAdminListDisplay<'a> {
    items: &'a [ClusterAdminRecord],
}

pub fn cluster_admin_list(items: &[ClusterAdminRecord]) -> ClusterAdminListDisplay<'_> {
    ClusterAdminListDisplay { items }
}

pub struct ClusterAdminRegisterDisplay<'a> {
    resp: &'a ClusterAdminRegisterResp,
}

pub fn cluster_admin_register(resp: &ClusterAdminRegisterResp) -> ClusterAdminRegisterDisplay<'_> {
    ClusterAdminRegisterDisplay { resp }
}

pub struct RuleApplyDisplay<'a> {
    resp: &'a RuleApplyResp,
}

pub fn rule_apply(resp: &RuleApplyResp) -> RuleApplyDisplay<'_> {
    RuleApplyDisplay { resp }
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

impl Display for ClusterAdminRegisterDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = if self.resp.acknowledged {
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
        let out = list_to_string("processes", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "process[{idx}]: {}", format_uuid(&item.process));
            outln!(out, "{}", process_info(item));
        });
        write_trimmed(f, &out)
    }
}

impl Display for LockListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("locks", self.items, 5, MoreLabel::Default, |out, idx, item| {
            outln!(out, "connection[{idx}]: {}", format_uuid(&item.connection));
            outln!(out, "descr[{idx}]: {}", display_str(&item.descr));
            if let Some(flag) = item.descr_flag {
                outln!(out, "descr-flag[{idx}]: {flag}");
            }
            outln!(out, "locked-at[{idx}]: {}", display_str(&item.locked_at));
            outln!(out, "session[{idx}]: {}", format_uuid(&item.session));
            outln!(out, "object[{idx}]: {}", format_uuid(&item.object));
        });
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
        let yes_no = |value: bool| if value { "yes" } else { "no" };
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
            "available-perfomance: {}",
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
        let yes_no = |value: bool| if value { "yes" } else { "no" };
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
        outln!(
            &mut out,
            "retrieved-by-server: {}",
            yes_no(item.retrieved_by_server)
        );
        outln!(
            &mut out,
            "software-license: {}",
            yes_no(item.software_license)
        );
        outln!(&mut out, "network-key: {}", yes_no(item.network_key));
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
        append_counters_prefixed(&mut out, &item.counters, "");
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
    resp: &'a AgentVersionResp,
}

pub fn agent_version(resp: &AgentVersionResp) -> AgentVersionDisplay<'_> {
    AgentVersionDisplay { resp }
}

impl Display for AgentVersionDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = self
            .resp
            .version
            .as_ref()
            .map(|v| format!("version: {v}"))
            .unwrap_or_else(|| "version: <not found>".to_string());
        write!(f, "{rendered}")
    }
}

pub struct ClusterListDisplay<'a> {
    resp: &'a ClusterListResp,
}

pub fn cluster_list(resp: &ClusterListResp) -> ClusterListDisplay<'_> {
    ClusterListDisplay { resp }
}

impl Display for ClusterListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("clusters", &self.resp.clusters, 5, MoreLabel::Default, |out, idx, cluster| {
            outln!(
                out,
                "cluster_uuid[{idx}]: {}",
                format_uuid(&cluster.uuid)
            );
            if let Some(host) = &cluster.host {
                outln!(out, "cluster_host[{idx}]: {host}");
            }
            if let Some(port) = cluster.port {
                outln!(out, "cluster_port[{idx}]: {port}");
            }
            if let Some(name) = &cluster.display_name {
                outln!(out, "cluster_name[{idx}]: {name}");
            }
            if let Some(timeout) = cluster.expiration_timeout {
                outln!(out, "cluster_expiration_timeout[{idx}]: {timeout}");
            }
        });
        write_trimmed(f, &out)
    }
}

pub struct ClusterInfoDisplay<'a> {
    resp: &'a ClusterInfoResp,
}

pub fn cluster_info(resp: &ClusterInfoResp) -> ClusterInfoDisplay<'_> {
    ClusterInfoDisplay { resp }
}

impl Display for ClusterInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        outln!(
            &mut out,
            "cluster_uuid: {}",
            format_uuid(&self.resp.cluster.uuid)
        );
        if let Some(host) = &self.resp.cluster.host {
            outln!(&mut out, "host: {host}");
        }
        if let Some(port) = self.resp.cluster.port {
            outln!(&mut out, "port: {port}");
        }
        if let Some(name) = &self.resp.cluster.display_name {
            outln!(&mut out, "display_name: {name}");
        }
        if let Some(timeout) = self.resp.cluster.expiration_timeout {
            outln!(&mut out, "expiration_timeout: {timeout}");
        }
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

    append_opt_yes_no(
        out,
        &format!("{prefix}software-license"),
        Some(license.software_license),
    );
    append_opt_yes_no(
        out,
        &format!("{prefix}retrieved-by-server"),
        Some(license.issued_by_server),
    );
    append_opt_yes_no(out, &format!("{prefix}network-key"), Some(license.network_key));
}

fn manager_using_label(value: u32) -> String {
    match value {
        1 => "main".to_string(),
        _ => value.to_string(),
    }
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

fn server_using_label(value: u32) -> String {
    match value {
        1 => "main".to_string(),
        _ => value.to_string(),
    }
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
    let yes_no = |value: bool| if value { "yes" } else { "no" };
    let label = |name: &str| match license_idx {
        Some(license_idx) => format!("{name}[{record_idx}.{license_idx}]"),
        None => format!("{name}[{record_idx}]"),
    };
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

fn append_process_license_plain(out: &mut String, license: &ProcessLicense) {
    let yes_no = |value: bool| if value { "yes" } else { "no" };
    outln!(out, "full-name: \"{}\"", display_str(&license.file_name));
    outln!(out, "series: \"{}\"", display_str(&license.key_series));
    outln!(
        out,
        "issued-by-server: {}",
        yes_no(license.issued_by_server)
    );
    outln!(
        out,
        "license-type: {}",
        process_license_type_label(license.license_type)
    );
    outln!(out, "net: {}", yes_no(license.network_key));
    outln!(out, "max-users-all: {}", license.max_users_all);
    outln!(out, "max-users-cur: {}", license.max_users_current);
    outln!(
        out,
        "rmngr-address: \"{}\"",
        display_str(&license.server_address)
    );
    outln!(out, "rmngr-port: {}", license.server_port);
    outln!(
        out,
        "rmngr-pid: {}",
        display_str(&license.process_id)
    );
    outln!(
        out,
        "short-presentation: \"{}\"",
        display_str(&license.brief_presentation)
    );
    outln!(
        out,
        "full-presentation: \"{}\"",
        display_str(&license.full_presentation)
    );
}

fn append_counters_prefixed(out: &mut String, counters: &SessionCounters, prefix: &str) {
    opt_field!(out, prefix, "blocked-by-dbms", counters.blocked_by_dbms);
    opt_field!(out, prefix, "blocked-by-ls", counters.blocked_by_ls);
    opt_field!(out, prefix, "bytes-all", counters.bytes_all);
    opt_field!(out, prefix, "bytes-last-5min", counters.bytes_last_5min);
    opt_field!(out, prefix, "calls-all", counters.calls_all);
    opt_field!(out, prefix, "calls-last-5min", counters.calls_last_5min);
    opt_field!(out, prefix, "dbms-bytes-all", counters.dbms_bytes_all);
    opt_field!(
        out,
        prefix,
        "dbms-bytes-last-5min",
        counters.dbms_bytes_last_5min
    );
    opt_field!(out, prefix, "db-proc-took", counters.db_proc_took);
    opt_field!(out, prefix, "duration-all", counters.duration_all);
    opt_field!(out, prefix, "duration-all-dbms", counters.duration_all_dbms);
    opt_field!(out, prefix, "duration-current", counters.duration_current);
    opt_field!(
        out,
        prefix,
        "duration-current-dbms",
        counters.duration_current_dbms
    );
    opt_field!(
        out,
        prefix,
        "duration-last-5min",
        counters.duration_last_5min
    );
    opt_field!(
        out,
        prefix,
        "duration-last-5min-dbms",
        counters.duration_last_5min_dbms
    );
    opt_field!(
        out,
        prefix,
        "passive-session-hibernate-time",
        counters.passive_session_hibernate_time
    );
    opt_field!(
        out,
        prefix,
        "hibernate-session-terminate-time",
        counters.hibernate_session_terminate_time
    );
    opt_field!(out, prefix, "memory-current", counters.memory_current);
    opt_field!(out, prefix, "memory-last-5min", counters.memory_last_5min);
    opt_field!(out, prefix, "memory-total", counters.memory_total);
    opt_field!(out, prefix, "read-current", counters.read_current);
    opt_field!(out, prefix, "read-last-5min", counters.read_last_5min);
    opt_field!(out, prefix, "read-total", counters.read_total);
    opt_field!(out, prefix, "write-current", counters.write_current);
    opt_field!(out, prefix, "write-last-5min", counters.write_last_5min);
    opt_field!(out, prefix, "write-total", counters.write_total);
    opt_field!(
        out,
        prefix,
        "duration-current-service",
        counters.duration_current_service
    );
    opt_field!(
        out,
        prefix,
        "duration-last-5min-service",
        counters.duration_last_5min_service
    );
    opt_field!(
        out,
        prefix,
        "duration-all-service",
        counters.duration_all_service
    );
    opt_field!(
        out,
        prefix,
        "cpu-time-last-5min",
        counters.cpu_time_last_5min
    );
    opt_field!(out, prefix, "cpu-time-current", counters.cpu_time_current);
    opt_field!(out, prefix, "cpu-time-total", counters.cpu_time_total);
}
