use std::fs;
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt;

use crate::client::{ClientConfig, RacClient};
use crate::commands::{
    agent_version, cluster_info, cluster_list, connection_info, connection_list, counter_info,
    counter_list, infobase_info, infobase_summary_info, infobase_summary_list, limit_info,
    limit_list, lock_list, manager_info, manager_list, process_info, process_list, profile_list,
    server_info, server_list, session_info, session_list,
};
use crate::error::RacError;
use crate::rac_wire::{format_uuid, parse_uuid};
use crate::Uuid16;

const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:8080";
const DEFAULT_RAC_ADDR: &str = "127.0.0.1:1545";
const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_READ_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_WRITE_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_POOL_MAX: usize = 4;
const DEFAULT_IDLE_TTL_SECS: u64 = 60;
const DEFAULT_INCLUDE_RAW_PAYLOAD: bool = false;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_addr: String,
    pub rac_addr: String,
    pub connect_timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub write_timeout_ms: u64,
    pub pool_max: usize,
    pub idle_ttl_secs: u64,
    pub include_raw_payload: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_addr: DEFAULT_LISTEN_ADDR.to_string(),
            rac_addr: DEFAULT_RAC_ADDR.to_string(),
            connect_timeout_ms: DEFAULT_CONNECT_TIMEOUT_MS,
            read_timeout_ms: DEFAULT_READ_TIMEOUT_MS,
            write_timeout_ms: DEFAULT_WRITE_TIMEOUT_MS,
            pool_max: DEFAULT_POOL_MAX,
            idle_ttl_secs: DEFAULT_IDLE_TTL_SECS,
            include_raw_payload: DEFAULT_INCLUDE_RAW_PAYLOAD,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    listen_addr: Option<String>,
    rac_addr: Option<String>,
    connect_timeout_ms: Option<u64>,
    read_timeout_ms: Option<u64>,
    write_timeout_ms: Option<u64>,
    pool_max: Option<usize>,
    idle_ttl_secs: Option<u64>,
    include_raw_payload: Option<bool>,
}

pub fn load_config(path: &str) -> Result<Config, RpcError> {
    let raw = fs::read_to_string(path)
        .map_err(|err| RpcError::new("config_error", format!("config read failed: {err}")))?;
    let file: ConfigFile = toml::from_str(&raw)
        .map_err(|err| RpcError::new("config_error", format!("config parse failed: {err}")))?;
    let mut cfg = Config::default();
    if let Some(value) = file.listen_addr {
        cfg.listen_addr = value;
    }
    if let Some(value) = file.rac_addr {
        cfg.rac_addr = value;
    }
    if let Some(value) = file.connect_timeout_ms {
        cfg.connect_timeout_ms = value;
    }
    if let Some(value) = file.read_timeout_ms {
        cfg.read_timeout_ms = value;
    }
    if let Some(value) = file.write_timeout_ms {
        cfg.write_timeout_ms = value;
    }
    if let Some(value) = file.pool_max {
        cfg.pool_max = value;
    }
    if let Some(value) = file.idle_ttl_secs {
        cfg.idle_ttl_secs = value;
    }
    if let Some(value) = file.include_raw_payload {
        cfg.include_raw_payload = value;
    }
    Ok(cfg)
}

#[derive(Debug, Deserialize)]
pub struct RpcRequest {
    pub command: String,
    pub args: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct RpcMeta {
    pub command: String,
}

#[derive(Debug, Serialize)]
pub struct RpcResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RpcError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<RpcMeta>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RpcError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

impl RpcError {
    pub fn new(code: &str, message: String) -> Self {
        Self {
            code: code.to_string(),
            message,
            details: None,
        }
    }

    pub fn with_details(code: &str, message: String, details: Value) -> Self {
        Self {
            code: code.to_string(),
            message,
            details: Some(details),
        }
    }
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for RpcError {}

#[derive(Debug)]
pub enum Command {
    AgentVersion,
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
    CounterList { cluster: Uuid16 },
    CounterInfo { cluster: Uuid16, counter: String },
    LimitList { cluster: Uuid16 },
    LimitInfo { cluster: Uuid16, limit: String },
}

pub fn parse_command(req: RpcRequest) -> Result<Command, RpcError> {
    match req.command.as_str() {
        "agent.version" => ensure_no_args(req.args).map(|_| Command::AgentVersion),
        "cluster.list" => ensure_no_args(req.args).map(|_| Command::ClusterList),
        "cluster.info" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::ClusterInfo {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "manager.list" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::ManagerList {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "manager.info" => {
            let args = parse_args::<ClusterManagerArg>(req.args)?;
            Ok(Command::ManagerInfo {
                cluster: parse_uuid_arg(&args.cluster)?,
                manager: parse_uuid_arg(&args.manager)?,
            })
        }
        "server.list" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::ServerList {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "server.info" => {
            let args = parse_args::<ClusterServerArg>(req.args)?;
            Ok(Command::ServerInfo {
                cluster: parse_uuid_arg(&args.cluster)?,
                server: parse_uuid_arg(&args.server)?,
            })
        }
        "process.list" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::ProcessList {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "process.info" => {
            let args = parse_args::<ClusterProcessArg>(req.args)?;
            Ok(Command::ProcessInfo {
                cluster: parse_uuid_arg(&args.cluster)?,
                process: parse_uuid_arg(&args.process)?,
            })
        }
        "infobase.summary_list" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::InfobaseSummaryList {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "infobase.summary_info" => {
            let args = parse_args::<ClusterInfobaseArg>(req.args)?;
            Ok(Command::InfobaseSummaryInfo {
                cluster: parse_uuid_arg(&args.cluster)?,
                infobase: parse_uuid_arg(&args.infobase)?,
            })
        }
        "infobase.info" => {
            let args = parse_args::<ClusterInfobaseArg>(req.args)?;
            Ok(Command::InfobaseInfo {
                cluster: parse_uuid_arg(&args.cluster)?,
                infobase: parse_uuid_arg(&args.infobase)?,
            })
        }
        "connection.list" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::ConnectionList {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "connection.info" => {
            let args = parse_args::<ClusterConnectionArg>(req.args)?;
            Ok(Command::ConnectionInfo {
                cluster: parse_uuid_arg(&args.cluster)?,
                connection: parse_uuid_arg(&args.connection)?,
            })
        }
        "session.list" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::SessionList {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "session.info" => {
            let args = parse_args::<ClusterSessionArg>(req.args)?;
            Ok(Command::SessionInfo {
                cluster: parse_uuid_arg(&args.cluster)?,
                session: parse_uuid_arg(&args.session)?,
            })
        }
        "lock.list" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::LockList {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "profile.list" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::ProfileList {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "counter.list" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::CounterList {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "counter.info" => {
            let args = parse_args::<ClusterCounterArg>(req.args)?;
            Ok(Command::CounterInfo {
                cluster: parse_uuid_arg(&args.cluster)?,
                counter: args.counter,
            })
        }
        "limit.list" => {
            let args = parse_args::<ClusterArg>(req.args)?;
            Ok(Command::LimitList {
                cluster: parse_uuid_arg(&args.cluster)?,
            })
        }
        "limit.info" => {
            let args = parse_args::<ClusterLimitArg>(req.args)?;
            Ok(Command::LimitInfo {
                cluster: parse_uuid_arg(&args.cluster)?,
                limit: args.limit,
            })
        }
        other => Err(RpcError::new(
            "bad_request",
            format!("unknown command: {other}"),
        )),
    }
}

pub fn dispatch_command(
    client: &mut RacClient,
    cmd: Command,
    include_raw_payload: bool,
) -> Result<Value, RpcError> {
    let value = match cmd {
        Command::AgentVersion => {
            let resp = agent_version(client).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ClusterList => {
            let resp = cluster_list(client).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ClusterInfo { cluster } => {
            let resp = cluster_info(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ManagerList { cluster } => {
            let resp = manager_list(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ManagerInfo { cluster, manager } => {
            let resp = manager_info(client, cluster, manager).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ServerList { cluster } => {
            let resp = server_list(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ServerInfo { cluster, server } => {
            let resp = server_info(client, cluster, server).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ProcessList { cluster } => {
            let resp = process_list(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ProcessInfo { cluster, process } => {
            let resp = process_info(client, cluster, process).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::InfobaseSummaryList { cluster } => {
            let resp = infobase_summary_list(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::InfobaseSummaryInfo { cluster, infobase } => {
            let resp = infobase_summary_info(client, cluster, infobase).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::InfobaseInfo { cluster, infobase } => {
            let resp = infobase_info(client, cluster, infobase).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ConnectionList { cluster } => {
            let resp = connection_list(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ConnectionInfo { cluster, connection } => {
            let resp = connection_info(client, cluster, connection).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::SessionList { cluster } => {
            let resp = session_list(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::SessionInfo { cluster, session } => {
            let resp = session_info(client, cluster, session).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::LockList { cluster } => {
            let resp = lock_list(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::ProfileList { cluster } => {
            let resp = profile_list(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::CounterList { cluster } => {
            let resp = counter_list(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::CounterInfo { cluster, counter } => {
            let resp = counter_info(client, cluster, &counter).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::LimitList { cluster } => {
            let resp = limit_list(client, cluster).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
        Command::LimitInfo { cluster, limit } => {
            let resp = limit_info(client, cluster, &limit).map_err(map_rac_error)?;
            response_value(resp, include_raw_payload)?
        }
    };
    Ok(value)
}

fn response_value<T: Serialize>(resp: T, include_raw_payload: bool) -> Result<Value, RpcError> {
    let mut value =
        serde_json::to_value(resp).map_err(|err| RpcError::new("internal", err.to_string()))?;
    if !include_raw_payload {
        if let Value::Object(map) = &mut value {
            map.remove("raw_payload");
        }
    }
    Ok(normalize_uuid_value(value, false))
}

fn map_rac_error(err: RacError) -> RpcError {
    RpcError::with_details(
        "rac_error",
        err.to_string(),
        json!({ "kind": format!("{err:?}") }),
    )
}

fn ensure_no_args(args: Option<Value>) -> Result<(), RpcError> {
    match args {
        None => Ok(()),
        Some(Value::Object(map)) if map.is_empty() => Ok(()),
        Some(_) => Err(RpcError::new(
            "bad_request",
            "command does not accept args".to_string(),
        )),
    }
}

fn parse_args<T: for<'de> Deserialize<'de>>(args: Option<Value>) -> Result<T, RpcError> {
    let value = args.ok_or_else(|| {
        RpcError::new("bad_request", "command requires args".to_string())
    })?;
    serde_json::from_value(value)
        .map_err(|err| RpcError::new("bad_request", format!("invalid args: {err}")))
}

fn parse_uuid_arg(input: &str) -> Result<Uuid16, RpcError> {
    parse_uuid(input).map_err(|err| RpcError::new("bad_request", err.to_string()))
}

fn normalize_uuid_value(value: Value, in_raw_payload: bool) -> Value {
    if in_raw_payload {
        return value;
    }
    match value {
        Value::Array(items) => {
            if let Some(uuid) = uuid_from_json_array(&items) {
                Value::String(format_uuid(&uuid))
            } else {
                Value::Array(
                    items
                        .into_iter()
                        .map(|item| normalize_uuid_value(item, false))
                        .collect(),
                )
            }
        }
        Value::Object(map) => {
            let mut out = serde_json::Map::with_capacity(map.len());
            for (key, value) in map {
                if key == "raw_payload" {
                    out.insert(key, value);
                } else {
                    out.insert(key, normalize_uuid_value(value, false));
                }
            }
            Value::Object(out)
        }
        other => other,
    }
}

fn uuid_from_json_array(items: &[Value]) -> Option<Uuid16> {
    if items.len() != 16 {
        return None;
    }
    let mut bytes = [0u8; 16];
    for (idx, item) in items.iter().enumerate() {
        let value = item.as_u64()?;
        if value > u8::MAX as u64 {
            return None;
        }
        bytes[idx] = value as u8;
    }
    Some(bytes)
}

#[derive(Debug, Deserialize)]
struct ClusterArg {
    cluster: String,
}

#[derive(Debug, Deserialize)]
struct ClusterManagerArg {
    cluster: String,
    manager: String,
}

#[derive(Debug, Deserialize)]
struct ClusterServerArg {
    cluster: String,
    server: String,
}

#[derive(Debug, Deserialize)]
struct ClusterProcessArg {
    cluster: String,
    process: String,
}

#[derive(Debug, Deserialize)]
struct ClusterInfobaseArg {
    cluster: String,
    infobase: String,
}

#[derive(Debug, Deserialize)]
struct ClusterConnectionArg {
    cluster: String,
    connection: String,
}

#[derive(Debug, Deserialize)]
struct ClusterSessionArg {
    cluster: String,
    session: String,
}

#[derive(Debug, Deserialize)]
struct ClusterCounterArg {
    cluster: String,
    counter: String,
}

#[derive(Debug, Deserialize)]
struct ClusterLimitArg {
    cluster: String,
    limit: String,
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub addr: String,
    pub client_cfg: ClientConfig,
    pub max: usize,
    pub idle_ttl: Duration,
}

pub trait Clock: Send + Sync + 'static {
    fn now(&self) -> Instant;
}

#[derive(Debug, Clone)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

#[derive(Debug)]
struct IdleClient<T> {
    client: T,
    last_used: Instant,
}

#[derive(Debug)]
struct PoolState<T> {
    idle: Vec<IdleClient<T>>,
    in_use: usize,
}

pub struct Pool<C: Clock> {
    cfg: PoolConfig,
    clock: C,
    inner: Mutex<PoolState<RacClient>>,
    condvar: Condvar,
}

impl<C: Clock> Pool<C> {
    pub fn new(cfg: PoolConfig, clock: C) -> Self {
        Self {
            cfg,
            clock,
            inner: Mutex::new(PoolState {
                idle: Vec::new(),
                in_use: 0,
            }),
            condvar: Condvar::new(),
        }
    }

    pub fn checkout(&self) -> Result<RacClient, RpcError> {
        loop {
            let now = self.clock.now();
            let mut state = self
                .inner
                .lock()
                .map_err(|_| RpcError::new("service_unavailable", "pool lock poisoned".into()))?;
            let expired = prune_idle(&mut state.idle, self.cfg.idle_ttl, now);
            drop(state);
            close_idle(expired);

            let mut state = self
                .inner
                .lock()
                .map_err(|_| RpcError::new("service_unavailable", "pool lock poisoned".into()))?;
            if let Some(client) = state.idle.pop() {
                state.in_use += 1;
                return Ok(client.client);
            }

            let total = state.in_use + state.idle.len();
            if total < self.cfg.max {
                state.in_use += 1;
                drop(state);
                match RacClient::connect(&self.cfg.addr, self.cfg.client_cfg.clone()) {
                    Ok(client) => return Ok(client),
                    Err(err) => {
                        let mut state = self.inner.lock().map_err(|_| {
                            RpcError::new("service_unavailable", "pool lock poisoned".into())
                        })?;
                        state.in_use = state.in_use.saturating_sub(1);
                        self.condvar.notify_one();
                        return Err(map_rac_error(err));
                    }
                }
            }

            let _guard = self
                .condvar
                .wait(state)
                .map_err(|_| RpcError::new("service_unavailable", "pool lock poisoned".into()))?;
        }
    }

    pub fn release(&self, client: RacClient, ok: bool) -> Result<(), RpcError> {
        let now = self.clock.now();
        let mut state = self
            .inner
            .lock()
            .map_err(|_| RpcError::new("service_unavailable", "pool lock poisoned".into()))?;
        state.in_use = state.in_use.saturating_sub(1);
        if ok {
            state.idle.push(IdleClient {
                client,
                last_used: now,
            });
        } else {
            drop(state);
            let _ = client.close();
            let mut state = self
                .inner
                .lock()
                .map_err(|_| RpcError::new("service_unavailable", "pool lock poisoned".into()))?;
            let expired = prune_idle(&mut state.idle, self.cfg.idle_ttl, now);
            drop(state);
            close_idle(expired);
            self.condvar.notify_one();
            return Ok(());
        }
        self.condvar.notify_one();
        Ok(())
    }
}

fn prune_idle<T>(idle: &mut Vec<IdleClient<T>>, ttl: Duration, now: Instant) -> Vec<T> {
    let mut expired = Vec::new();
    let mut kept = Vec::with_capacity(idle.len());
    while let Some(item) = idle.pop() {
        if now.duration_since(item.last_used) > ttl {
            expired.push(item.client);
        } else {
            kept.push(item);
        }
    }
    kept.reverse();
    *idle = kept;
    expired
}

fn close_idle(clients: Vec<RacClient>) {
    for client in clients {
        let _ = client.close();
    }
}

impl From<RpcError> for RpcResponse {
    fn from(err: RpcError) -> Self {
        Self {
            result: None,
            error: Some(err),
            meta: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_cluster_info() {
        let req = RpcRequest {
            command: "cluster.info".to_string(),
            args: Some(json!({ "cluster": "550e8400-e29b-41d4-a716-446655440000" })),
        };
        let cmd = parse_command(req).expect("command parse");
        let expected = parse_uuid("550e8400-e29b-41d4-a716-446655440000").expect("uuid");
        let formatted = format!("{cmd:?}");
        let expected_formatted = format!("ClusterInfo {{ cluster: {expected:?} }}");
        assert_eq!(formatted, expected_formatted);
    }

    #[test]
    fn parse_command_unknown() {
        let req = RpcRequest {
            command: "cluster.delete".to_string(),
            args: None,
        };
        let err = parse_command(req).expect_err("unknown command");
        assert_eq!(err.code, "bad_request");
    }

    #[test]
    fn parse_command_missing_args() {
        let req = RpcRequest {
            command: "cluster.info".to_string(),
            args: None,
        };
        let err = parse_command(req).expect_err("missing args");
        assert_eq!(err.code, "bad_request");
    }

    #[test]
    fn response_value_strips_raw_payload() {
        let resp = crate::commands::ClusterListResp {
            clusters: Vec::new(),
            raw_payload: Some(vec![1, 2, 3]),
        };
        let value = response_value(resp, false).expect("value");
        assert!(value.get("raw_payload").is_none());
    }

    #[test]
    fn response_value_formats_uuid_as_string() {
        let uuid = parse_uuid("550e8400-e29b-41d4-a716-446655440000").expect("uuid");
        let resp = crate::commands::ClusterListResp {
            clusters: vec![crate::commands::ClusterSummary {
                uuid,
                host: Some("127.0.0.1".to_string()),
                display_name: Some("cluster".to_string()),
                port: Some(1545),
                expiration_timeout: Some(600),
            }],
            raw_payload: None,
        };
        let value = response_value(resp, false).expect("value");
        let clusters = value.get("clusters").expect("clusters");
        let first = clusters.get(0).expect("cluster");
        let uuid_value = first.get("uuid").expect("uuid");
        let expected = Value::String("550e8400-e29b-41d4-a716-446655440000".to_string());
        assert_eq!(uuid_value, &expected);
    }

    #[derive(Debug)]
    struct TestClock {
        now: Mutex<Instant>,
    }

    impl TestClock {
        fn new(now: Instant) -> Self {
            Self {
                now: Mutex::new(now),
            }
        }

        fn set(&self, now: Instant) {
            let mut guard = self.now.lock().expect("lock");
            *guard = now;
        }
    }

    impl Clock for TestClock {
        fn now(&self) -> Instant {
            *self.now.lock().expect("lock")
        }
    }

    #[test]
    fn prune_idle_respects_ttl() {
        let base = Instant::now();
        let clock = TestClock::new(base);
        let mut idle = Vec::new();
        idle.push(IdleClient {
            client: 1u8,
            last_used: base - Duration::from_secs(120),
        });
        idle.push(IdleClient {
            client: 2u8,
            last_used: base - Duration::from_secs(10),
        });
        clock.set(base);
        let expired = prune_idle(&mut idle, Duration::from_secs(60), clock.now());
        assert_eq!(expired, vec![1u8]);
        assert_eq!(idle.len(), 1);
        assert_eq!(idle[0].client, 2u8);
    }
}
