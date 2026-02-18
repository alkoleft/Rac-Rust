use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use clap::Parser;
use serde_json::{json, Value};
use tokio::task;

use rac_protocol::client::ClientConfig;
use rac_protocol::rest::{
    dispatch_command, load_config, parse_command, Command, Pool, PoolConfig, RpcError, RpcMeta,
    RpcRequest, RpcResponse, SystemClock,
};
use rac_protocol::rac_wire::parse_uuid;
use rac_protocol::Uuid16;

#[derive(Parser, Debug)]
#[command(name = "rac_rest", version, about = "RAC REST gateway with cached connections")]
struct Cli {
    #[arg(long, default_value = "rac_rest.toml")]
    config: String,
}

#[derive(Clone)]
struct AppState {
    pool: Arc<Pool<SystemClock>>,
    include_raw_payload: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let cfg = load_config(&cli.config)?;

    let mut client_cfg = ClientConfig::default();
    client_cfg.connect_timeout = Duration::from_millis(cfg.connect_timeout_ms);
    client_cfg.read_timeout = Duration::from_millis(cfg.read_timeout_ms);
    client_cfg.write_timeout = Duration::from_millis(cfg.write_timeout_ms);

    let pool_cfg = PoolConfig {
        addr: cfg.rac_addr.clone(),
        client_cfg,
        max: cfg.pool_max,
        idle_ttl: Duration::from_secs(cfg.idle_ttl_secs),
    };
    let pool = Arc::new(Pool::new(pool_cfg, SystemClock));

    let state = AppState {
        pool,
        include_raw_payload: cfg.include_raw_payload,
    };

    let app = Router::new()
        .route("/rpc", post(rpc_handler))
        .route("/agent/version", get(agent_version))
        .route("/clusters", get(clusters_list))
        .route("/clusters/:cluster", get(clusters_info))
        .route("/clusters/:cluster/managers", get(managers_list))
        .route("/clusters/:cluster/managers/:manager", get(managers_info))
        .route("/clusters/:cluster/servers", get(servers_list))
        .route("/clusters/:cluster/servers/:server", get(servers_info))
        .route("/clusters/:cluster/processes", get(processes_list))
        .route("/clusters/:cluster/processes/:process", get(processes_info))
        .route(
            "/clusters/:cluster/infobases/summary",
            get(infobase_summary_list),
        )
        .route(
            "/clusters/:cluster/infobases/summary/:infobase",
            get(infobase_summary_info),
        )
        .route("/clusters/:cluster/infobases/:infobase", get(infobase_info))
        .route("/clusters/:cluster/connections", get(connections_list))
        .route(
            "/clusters/:cluster/connections/:connection",
            get(connections_info),
        )
        .route("/clusters/:cluster/sessions", get(sessions_list))
        .route("/clusters/:cluster/sessions/:session", get(sessions_info))
        .route("/clusters/:cluster/locks", get(locks_list))
        .route("/clusters/:cluster/profiles", get(profiles_list))
        .route("/clusters/:cluster/counters", get(counters_list))
        .route("/clusters/:cluster/counters/:counter", get(counters_info))
        .route("/clusters/:cluster/limits", get(limits_list))
        .route("/clusters/:cluster/limits/:limit", get(limits_info))
        .with_state(state);

    let addr: SocketAddr = cfg.listen_addr.parse()?;
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

async fn rpc_handler(
    State(state): State<AppState>,
    Json(req): Json<RpcRequest>,
) -> Result<Json<RpcResponse>, (StatusCode, Json<RpcResponse>)> {
    let command = match parse_command(req) {
        Ok(cmd) => cmd,
        Err(err) => return Err(error_response(err)),
    };
    let command_label = command_name(&command).to_string();

    let pool = state.pool.clone();
    let include_raw = state.include_raw_payload;
    let result = task::spawn_blocking(move || exec_command(pool, command, include_raw)).await;

    match result {
        Ok(Ok(payload)) => Ok(Json(RpcResponse {
            result: Some(payload),
            error: None,
            meta: Some(RpcMeta {
                command: command_label,
            }),
        })),
        Ok(Err(err)) => Err(error_response(err)),
        Err(err) => Err(error_response(RpcError::new(
            "internal",
            format!("worker failed: {err}"),
        ))),
    }
}

async fn agent_version(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    exec_command_json(state, Command::AgentVersion).await
}

async fn clusters_list(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    exec_command_json(state, Command::ClusterList).await
}

async fn clusters_info(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::ClusterInfo { cluster }).await
}

async fn managers_list(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::ManagerList { cluster }).await
}

async fn managers_info(
    State(state): State<AppState>,
    Path((cluster, manager)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    let manager = parse_uuid_param(&manager)?;
    exec_command_json(state, Command::ManagerInfo { cluster, manager }).await
}

async fn servers_list(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::ServerList { cluster }).await
}

async fn servers_info(
    State(state): State<AppState>,
    Path((cluster, server)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    let server = parse_uuid_param(&server)?;
    exec_command_json(state, Command::ServerInfo { cluster, server }).await
}

async fn processes_list(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::ProcessList { cluster }).await
}

async fn processes_info(
    State(state): State<AppState>,
    Path((cluster, process)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    let process = parse_uuid_param(&process)?;
    exec_command_json(state, Command::ProcessInfo { cluster, process }).await
}

async fn infobase_summary_list(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::InfobaseSummaryList { cluster }).await
}

async fn infobase_summary_info(
    State(state): State<AppState>,
    Path((cluster, infobase)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    let infobase = parse_uuid_param(&infobase)?;
    exec_command_json(state, Command::InfobaseSummaryInfo { cluster, infobase }).await
}

async fn infobase_info(
    State(state): State<AppState>,
    Path((cluster, infobase)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    let infobase = parse_uuid_param(&infobase)?;
    exec_command_json(state, Command::InfobaseInfo { cluster, infobase }).await
}

async fn connections_list(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::ConnectionList { cluster }).await
}

async fn connections_info(
    State(state): State<AppState>,
    Path((cluster, connection)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    let connection = parse_uuid_param(&connection)?;
    exec_command_json(state, Command::ConnectionInfo { cluster, connection }).await
}

async fn sessions_list(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::SessionList { cluster }).await
}

async fn sessions_info(
    State(state): State<AppState>,
    Path((cluster, session)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    let session = parse_uuid_param(&session)?;
    exec_command_json(state, Command::SessionInfo { cluster, session }).await
}

async fn locks_list(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::LockList { cluster }).await
}

async fn profiles_list(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::ProfileList { cluster }).await
}

async fn counters_list(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::CounterList { cluster }).await
}

async fn counters_info(
    State(state): State<AppState>,
    Path((cluster, counter)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::CounterInfo { cluster, counter }).await
}

async fn limits_list(
    State(state): State<AppState>,
    Path(cluster): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::LimitList { cluster }).await
}

async fn limits_info(
    State(state): State<AppState>,
    Path((cluster, limit)): Path<(String, String)>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let cluster = parse_uuid_param(&cluster)?;
    exec_command_json(state, Command::LimitInfo { cluster, limit }).await
}

async fn exec_command_json(
    state: AppState,
    command: Command,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let pool = state.pool.clone();
    let include_raw = state.include_raw_payload;
    let result = task::spawn_blocking(move || exec_command(pool, command, include_raw)).await;

    match result {
        Ok(Ok(payload)) => Ok(Json(payload)),
        Ok(Err(err)) => Err(error_value(err)),
        Err(err) => Err(error_value(RpcError::new(
            "internal",
            format!("worker failed: {err}"),
        ))),
    }
}

fn exec_command(
    pool: Arc<Pool<SystemClock>>,
    command: Command,
    include_raw_payload: bool,
) -> Result<serde_json::Value, RpcError> {
    let mut client = pool.checkout()?;
    let result = dispatch_command(&mut client, command, include_raw_payload);
    let ok = result.is_ok();
    pool.release(client, ok)?;
    result
}

fn parse_uuid_param(input: &str) -> Result<Uuid16, (StatusCode, Json<Value>)> {
    parse_uuid(input).map_err(|err| error_value(RpcError::new("bad_request", err.to_string())))
}

fn error_response(err: RpcError) -> (StatusCode, Json<RpcResponse>) {
    let status = status_from_rpc_error(&err.code);
    (status, Json(RpcResponse::from(err)))
}

fn error_value(err: RpcError) -> (StatusCode, Json<Value>) {
    let status = status_from_rpc_error(&err.code);
    (status, Json(json!({ "error": err })))
}

fn status_from_rpc_error(code: &str) -> StatusCode {
    match code {
        "bad_request" => StatusCode::BAD_REQUEST,
        "service_unavailable" => StatusCode::SERVICE_UNAVAILABLE,
        "rac_error" => StatusCode::BAD_GATEWAY,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn command_name(cmd: &Command) -> &'static str {
    match cmd {
        Command::AgentVersion => "agent.version",
        Command::ClusterList => "cluster.list",
        Command::ClusterInfo { .. } => "cluster.info",
        Command::ManagerList { .. } => "manager.list",
        Command::ManagerInfo { .. } => "manager.info",
        Command::ServerList { .. } => "server.list",
        Command::ServerInfo { .. } => "server.info",
        Command::ProcessList { .. } => "process.list",
        Command::ProcessInfo { .. } => "process.info",
        Command::InfobaseSummaryList { .. } => "infobase.summary_list",
        Command::InfobaseSummaryInfo { .. } => "infobase.summary_info",
        Command::InfobaseInfo { .. } => "infobase.info",
        Command::ConnectionList { .. } => "connection.list",
        Command::ConnectionInfo { .. } => "connection.info",
        Command::SessionList { .. } => "session.list",
        Command::SessionInfo { .. } => "session.info",
        Command::LockList { .. } => "lock.list",
        Command::ProfileList { .. } => "profile.list",
        Command::CounterList { .. } => "counter.list",
        Command::CounterInfo { .. } => "counter.info",
        Command::LimitList { .. } => "limit.list",
        Command::LimitInfo { .. } => "limit.info",
    }
}
