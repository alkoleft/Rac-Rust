use std::fmt::{self, Display, Write as _};

use serde::Serialize;

use rac_protocol::commands::{
    AgentAdminRecord, ClusterAdminRecord, ClusterRecord, ConnectionRecord, CounterRecord,
    CounterValuesRecord, InfobaseSummary, LimitRecord, LockRecordRaw, ManagerRecord,
    ProcessLicense, ProcessRecord, ProfileRecord, RuleInsertResp, RuleRecord, RuleUpdateResp,
    ServerRecord, ServiceSettingInsertResp, ServiceSettingRecord,
    ServiceSettingTransferDataDirRecord, ServiceSettingUpdateResp, SessionLicense,
    SessionRecord,
};
use rac_protocol::rpc::AckResponse;
use rac_protocol::rac_wire::format_uuid;
use rac_protocol::Uuid16;

use super::format::{info_display_to_string, list_to_string, write_trimmed, MoreLabel};

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

fn render_ack(f: &mut fmt::Formatter<'_>, label: &str, acknowledged: bool) -> fmt::Result {
    if acknowledged {
        write!(f, "{label}: ok")
    } else {
        write!(f, "{label}: failed")
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

pub struct ProcessListLicensesDisplay<'a> {
    items: &'a [ProcessRecord],
}

pub fn process_list_licenses(items: &[ProcessRecord]) -> ProcessListLicensesDisplay<'_> {
    ProcessListLicensesDisplay { items }
}

pub struct ProcessInfoLicensesDisplay<'a> {
    item: &'a ProcessRecord,
}

pub fn process_info_licenses(item: &ProcessRecord) -> ProcessInfoLicensesDisplay<'_> {
    ProcessInfoLicensesDisplay { item }
}

pub struct ProfileListDisplay<'a> {
    items: &'a [ProfileRecord],
}

pub fn profile_list(items: &[ProfileRecord]) -> ProfileListDisplay<'_> {
    ProfileListDisplay { items }
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
    resp: &'a AckResponse,
}

pub fn service_setting_remove(resp: &AckResponse) -> ServiceSettingRemoveDisplay<'_> {
    ServiceSettingRemoveDisplay { resp }
}

pub struct ServiceSettingApplyDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn service_setting_apply(resp: &AckResponse) -> ServiceSettingApplyDisplay<'_> {
    ServiceSettingApplyDisplay { resp }
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

pub struct ClusterAdminRegisterDisplay {
    acknowledged: bool,
}

pub fn cluster_admin_register(acknowledged: bool) -> ClusterAdminRegisterDisplay {
    ClusterAdminRegisterDisplay { acknowledged }
}

pub struct AgentAdminRegisterDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn agent_admin_register(resp: &AckResponse) -> AgentAdminRegisterDisplay<'_> {
    AgentAdminRegisterDisplay { resp }
}

pub struct AgentAdminRemoveDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn agent_admin_remove(resp: &AckResponse) -> AgentAdminRemoveDisplay<'_> {
    AgentAdminRemoveDisplay { resp }
}

pub struct SessionTerminateDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn session_terminate(resp: &AckResponse) -> SessionTerminateDisplay<'_> {
    SessionTerminateDisplay { resp }
}

pub struct SessionInterruptCurrentServerCallDisplay<'a> {
    resp: &'a AckResponse,
}

pub fn session_interrupt_current_server_call(
    resp: &AckResponse,
) -> SessionInterruptCurrentServerCallDisplay<'_> {
    SessionInterruptCurrentServerCallDisplay { resp }
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

impl Display for ClusterAdminRegisterDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "cluster-admin-register", self.acknowledged)
    }
}

impl Display for AgentAdminRegisterDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "agent-admin-register", self.resp.acknowledged)
    }
}

impl Display for AgentAdminRemoveDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "agent-admin-remove", self.resp.acknowledged)
    }
}

impl Display for SessionTerminateDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "session-terminate", self.resp.acknowledged)
    }
}

impl Display for SessionInterruptCurrentServerCallDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "session-interrupt-current-server-call", self.resp.acknowledged)
    }
}

impl Display for RuleApplyDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "rule-apply", self.resp.acknowledged)
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
        render_ack(f, "rule-remove", self.resp.acknowledged)
    }
}

impl Display for CounterUpdateDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "counter-update", self.resp.acknowledged)
    }
}

impl Display for CounterClearDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "counter-clear", self.resp.acknowledged)
    }
}

impl Display for CounterRemoveDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "counter-remove", self.resp.acknowledged)
    }
}

impl Display for LimitUpdateDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "limit-update", self.resp.acknowledged)
    }
}

impl Display for LimitRemoveDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "limit-remove", self.resp.acknowledged)
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
        render_ack(f, "service-setting-remove", self.resp.acknowledged)
    }
}

impl Display for ServiceSettingApplyDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        render_ack(f, "service-setting-apply", self.resp.acknowledged)
    }
}

impl Display for ProcessListLicensesDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("processes", self.items, 5, MoreLabel::Default, |out, _idx, item| {
            outln!(out, "{}", process_info_licenses(item));
        });
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

impl Display for ProfileListDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = list_to_string("profiles", self.items, 5, MoreLabel::Default, |out, _idx, item| {
            outln!(
                out,
                "- {} ({}), config: {}, priv: {}, crypto: {}, right-extension: {}",
                display_str(&item.name),
                display_str(&item.descr),
                yes_no_u8(item.config),
                yes_no_u8(item.privileged_mode),
                yes_no_u8(item.crypto),
                yes_no_u8(item.right_extension),
            );
        });
        write_trimmed(f, &out)
    }
}

fn load_balancing_mode_name(value: u32) -> &'static str {
    match value {
        1 => "memory",
        _ => "performance",
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

fn yes_no_u8(value: u8) -> &'static str {
    if value == 0 { "no" } else { "yes" }
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

// Generated console output helpers.
include!("console_output_generated.rs");
