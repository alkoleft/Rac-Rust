use std::fmt::{self, Display, Write as _};

use serde::Serialize;

use rac_protocol::commands::{
    AgentVersionResp, ClusterInfoResp, ClusterListResp, InfobaseSummary, SessionCounters,
    SessionLicense, SessionRecord,
};
use rac_protocol::rac_wire::format_uuid;
use rac_protocol::Uuid16;

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
        let mut out = String::new();
        let _ = writeln!(&mut out, "{}: {}", self.label, self.items.len());
        for uuid in self.items.iter().take(5) {
            let _ = writeln!(&mut out, "- {}", format_uuid(uuid));
        }
        if self.items.len() > 5 {
            let _ = writeln!(&mut out, "... and {} more", self.items.len() - 5);
        }
        write!(f, "{}", out.trim_end())
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
        let mut out = String::new();
        let _ = writeln!(&mut out, "{}: {}", self.label, format_uuid(self.uuid));
        for value in self.fields.iter().take(6) {
            let _ = writeln!(&mut out, "- {value}");
        }
        if self.fields.len() > 6 {
            let _ = writeln!(&mut out, "... and {} more", self.fields.len() - 6);
        }
        write!(f, "{}", out.trim_end())
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
        let mut out = String::new();
        let _ = writeln!(&mut out, "infobases: {}", self.items.len());
        for (idx, item) in self.items.iter().enumerate().take(5) {
            let _ = writeln!(&mut out, "infobase[{idx}]: {}", format_uuid(&item.infobase));
            let _ = writeln!(&mut out, "name[{idx}]: {}", item.name);
            let _ = writeln!(&mut out, "descr[{idx}]: \"{}\"", item.descr);
        }
        if self.items.len() > 5 {
            let _ = writeln!(&mut out, "infobase_more: {}", self.items.len() - 5);
        }
        write!(f, "{}", out.trim_end())
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
        let mut out = String::new();
        let _ = writeln!(&mut out, "sessions: {}", self.items.len());
        for (idx, item) in self.items.iter().enumerate().take(5) {
            let _ = writeln!(&mut out, "session[{idx}]: {}", format_uuid(&item.session));
            let _ = writeln!(&mut out, "{}", session_info(&item));
        }
        if self.items.len() > 5 {
            let _ = writeln!(&mut out, "... and {} more", self.items.len() - 5);
        }
        write!(f, "{}", out.trim_end())
    }
}

pub struct SessionInfoDisplay<'a> {
    item: &'a SessionRecord,
}

pub fn session_info(item: &SessionRecord) -> SessionInfoDisplay<'_> {
    SessionInfoDisplay { item }
}

impl Display for SessionInfoDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let item = self.item;
        let _ = writeln!(&mut out, "session: {}", format_uuid(&item.session));
        append_opt_str(&mut out, "app-id", item.app_id.as_deref());
        append_opt_uuid(&mut out, "connection", item.connection.as_ref());
        append_opt_uuid(&mut out, "infobase", item.infobase.as_ref());
        append_opt_uuid(&mut out, "process", item.process.as_ref());
        append_opt_str(&mut out, "host", item.host.as_deref());
        append_opt_bool(&mut out, "hibernate", item.hibernate);
        append_opt_str(&mut out, "locale", item.locale.as_deref());
        append_opt_str(&mut out, "user-name", item.user_name.as_deref());
        append_opt_str(&mut out, "started-at", item.started_at.as_deref());
        append_opt_str(&mut out, "last-active-at", item.last_active_at.as_deref());
        append_opt_str(&mut out, "client-ip", item.client_ip.as_deref());
        append_opt_bool(&mut out, "retrieved-by-server", item.retrieved_by_server);
        append_opt_bool(&mut out, "software-license", item.software_license);
        append_opt_bool(&mut out, "network-key", item.network_key);
        append_opt_str(&mut out, "db-proc-info", item.db_proc_info.as_deref());
        append_opt_str(&mut out, "db-proc-took-at", item.db_proc_took_at.as_deref());
        append_opt_str(
            &mut out,
            "current-service-name",
            item.current_service_name.as_deref(),
        );
        append_opt_str(&mut out, "data-separation", item.data_separation.as_deref());
        append_license_prefixed(&mut out, item.license.as_ref(), "license.");
        append_opt_u32(&mut out, "session-id", item.session_id);
        append_counters_prefixed(&mut out, &item.counters, "");
        write!(f, "{}", out.trim_end())
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
        let mut out = String::new();
        let _ = writeln!(&mut out, "clusters: {}", self.resp.clusters.len());
        for (idx, cluster) in self.resp.clusters.iter().enumerate().take(5) {
            let _ = writeln!(
                &mut out,
                "cluster_uuid[{idx}]: {}",
                format_uuid(&cluster.uuid)
            );
            if let Some(host) = &cluster.host {
                let _ = writeln!(&mut out, "cluster_host[{idx}]: {host}");
            }
            if let Some(port) = cluster.port {
                let _ = writeln!(&mut out, "cluster_port[{idx}]: {port}");
            }
            if let Some(name) = &cluster.display_name {
                let _ = writeln!(&mut out, "cluster_name[{idx}]: {name}");
            }
            if let Some(timeout) = cluster.expiration_timeout {
                let _ = writeln!(&mut out, "cluster_expiration_timeout[{idx}]: {timeout}");
            }
        }
        if self.resp.clusters.len() > 5 {
            let _ = writeln!(&mut out, "cluster_more: {}", self.resp.clusters.len() - 5);
        }
        write!(f, "{}", out.trim_end())
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
        let _ = writeln!(
            &mut out,
            "cluster_uuid: {}",
            format_uuid(&self.resp.cluster.uuid)
        );
        if let Some(host) = &self.resp.cluster.host {
            let _ = writeln!(&mut out, "host: {host}");
        }
        if let Some(port) = self.resp.cluster.port {
            let _ = writeln!(&mut out, "port: {port}");
        }
        if let Some(name) = &self.resp.cluster.display_name {
            let _ = writeln!(&mut out, "display_name: {name}");
        }
        if let Some(timeout) = self.resp.cluster.expiration_timeout {
            let _ = writeln!(&mut out, "expiration_timeout: {timeout}");
        }
        write!(f, "{}", out.trim_end())
    }
}

fn append_opt_str(out: &mut String, label: &str, value: Option<&str>) {
    if let Some(value) = value {
        let _ = writeln!(out, "{label}: {value}");
    }
}

fn append_opt_uuid(out: &mut String, label: &str, value: Option<&Uuid16>) {
    if let Some(value) = value {
        let _ = writeln!(out, "{label}: {}", format_uuid(value));
    }
}

fn append_opt_u32(out: &mut String, label: &str, value: Option<u32>) {
    if let Some(value) = value {
        let _ = writeln!(out, "{label}: {value}");
    }
}

fn append_opt_u64(out: &mut String, label: &str, value: Option<u64>) {
    if let Some(value) = value {
        let _ = writeln!(out, "{label}: {value}");
    }
}

fn append_opt_bool(out: &mut String, label: &str, value: Option<bool>) {
    if let Some(value) = value {
        let rendered = if value { "yes" } else { "no" };
        let _ = writeln!(out, "{label}: {rendered}");
    }
}

fn append_opt_i32(out: &mut String, label: &str, value: Option<i32>) {
    if let Some(value) = value {
        let _ = writeln!(out, "{label}: {value}");
    }
}

fn append_license_prefixed(out: &mut String, license: Option<&SessionLicense>, prefix: &str) {
    let Some(license) = license else {
        return;
    };
    let _ = writeln!(
        out,
        "{prefix}license-type: '{}'",
        license.license_type.unwrap_or(9999999)
    );
    let _ = writeln!(
        out,
        "{prefix}file-name: '{}'",
        license.file_name.as_deref().unwrap_or("---")
    );
    let _ = writeln!(
        out,
        "{prefix}server-address: '{}'",
        license.server_address.as_deref().unwrap_or("---")
    );
    let _ = writeln!(
        out,
        "{prefix}server-port: {}",
        license.server_port.unwrap_or(0)
    );

    let _ = writeln!(
        out,
        "{prefix}process-id: {}",
        license.process_id.as_deref().unwrap_or("---")
    );
    let _ = writeln!(
        out,
        "{prefix}brief-presentation: '{}'",
        license.brief_presentation.as_deref().unwrap_or("---")
    );
    let _ = writeln!(
        out,
        "{prefix}max-users: {}",
        license.max_users_all.unwrap_or(0)
    );
    let _ = writeln!(
        out,
        "{prefix}max-software-license-users: {}",
        license.max_users_current.unwrap_or(0)
    );

    let _ = writeln!(
        out,
        "{prefix}brief-presentation: '{}'",
        license.brief_presentation.as_deref().unwrap_or("---")
    );
    let _ = writeln!(
        out,
        "{prefix}detailed-presentation: '{}'",
        license.full_presentation.as_deref().unwrap_or("---")
    );
    let _ = writeln!(
        out,
        "{prefix}key-series: {}",
        license.key_series.as_deref().unwrap_or("---")
    );

    append_opt_bool(
        out,
        &format!("{prefix}software-license"),
        license.software_license,
    );
    append_opt_bool(
        out,
        &format!("{prefix}retrieved-by-server"),
        license.issued_by_server,
    );
    append_opt_bool(out, &format!("{prefix}network-key"), license.network_key);
}

fn append_counters_prefixed(out: &mut String, counters: &SessionCounters, prefix: &str) {
    append_opt_u32(
        out,
        &format!("{prefix}blocked-by-dbms"),
        counters.blocked_by_dbms,
    );
    append_opt_u32(
        out,
        &format!("{prefix}blocked-by-ls"),
        counters.blocked_by_ls,
    );
    append_opt_u64(out, &format!("{prefix}bytes-all"), counters.bytes_all);
    append_opt_u64(
        out,
        &format!("{prefix}bytes-last-5min"),
        counters.bytes_last_5min,
    );
    append_opt_u32(out, &format!("{prefix}calls-all"), counters.calls_all);
    append_opt_u64(
        out,
        &format!("{prefix}calls-last-5min"),
        counters.calls_last_5min,
    );
    append_opt_u64(
        out,
        &format!("{prefix}dbms-bytes-all"),
        counters.dbms_bytes_all,
    );
    append_opt_u64(
        out,
        &format!("{prefix}dbms-bytes-last-5min"),
        counters.dbms_bytes_last_5min,
    );
    append_opt_u32(out, &format!("{prefix}db-proc-took"), counters.db_proc_took);
    append_opt_u32(out, &format!("{prefix}duration-all"), counters.duration_all);
    append_opt_u32(
        out,
        &format!("{prefix}duration-all-dbms"),
        counters.duration_all_dbms,
    );
    append_opt_u32(
        out,
        &format!("{prefix}duration-current"),
        counters.duration_current,
    );
    append_opt_u32(
        out,
        &format!("{prefix}duration-current-dbms"),
        counters.duration_current_dbms,
    );
    append_opt_u64(
        out,
        &format!("{prefix}duration-last-5min"),
        counters.duration_last_5min,
    );
    append_opt_u64(
        out,
        &format!("{prefix}duration-last-5min-dbms"),
        counters.duration_last_5min_dbms,
    );
    append_opt_u32(
        out,
        &format!("{prefix}passive-session-hibernate-time"),
        counters.passive_session_hibernate_time,
    );
    append_opt_u32(
        out,
        &format!("{prefix}hibernate-session-terminate-time"),
        counters.hibernate_session_terminate_time,
    );
    append_opt_u64(
        out,
        &format!("{prefix}memory-current"),
        counters.memory_current,
    );
    append_opt_u64(
        out,
        &format!("{prefix}memory-last-5min"),
        counters.memory_last_5min,
    );
    append_opt_u64(out, &format!("{prefix}memory-total"), counters.memory_total);
    append_opt_u64(out, &format!("{prefix}read-current"), counters.read_current);
    append_opt_u64(
        out,
        &format!("{prefix}read-last-5min"),
        counters.read_last_5min,
    );
    append_opt_u64(out, &format!("{prefix}read-total"), counters.read_total);
    append_opt_u64(
        out,
        &format!("{prefix}write-current"),
        counters.write_current,
    );
    append_opt_u64(
        out,
        &format!("{prefix}write-last-5min"),
        counters.write_last_5min,
    );
    append_opt_u64(out, &format!("{prefix}write-total"), counters.write_total);
    append_opt_u32(
        out,
        &format!("{prefix}duration-current-service"),
        counters.duration_current_service,
    );
    append_opt_u64(
        out,
        &format!("{prefix}duration-last-5min-service"),
        counters.duration_last_5min_service,
    );
    append_opt_u32(
        out,
        &format!("{prefix}duration-all-service"),
        counters.duration_all_service,
    );
    append_opt_u64(
        out,
        &format!("{prefix}cpu-time-last-5min"),
        counters.cpu_time_last_5min,
    );
    append_opt_u64(
        out,
        &format!("{prefix}cpu-time-current"),
        counters.cpu_time_current,
    );
    append_opt_u64(
        out,
        &format!("{prefix}cpu-time-total"),
        counters.cpu_time_total,
    );
}
