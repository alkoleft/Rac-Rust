use std::fs;
use std::time::Duration;

use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{agent_version, cluster_list, infobase_summary_list, manager_list};
use rac_protocol::rac_wire::parse_uuid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestParams {
    addr: String,
    expected_agent_version: String,
    cluster_uuid: String,
}

fn load_params() -> TestParams {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let path = format!("{}/tests/params.toml", manifest_dir);
    let data = fs::read_to_string(&path).expect("read tests/params.toml");
    toml::from_str(&data).expect("parse tests/params.toml")
}

fn require_cluster_uuid(params: &TestParams) -> [u8; 16] {
    parse_uuid(&params.cluster_uuid).expect("cluster_uuid must be a valid uuid")
}

fn client_cfg() -> ClientConfig {
    ClientConfig {
        connect_timeout: Duration::from_secs(5),
        read_timeout: Duration::from_secs(15),
        write_timeout: Duration::from_secs(15),
        debug_raw: false,
        protocol: Default::default(),
    }
}

#[test]
fn live_agent_version_only() {
    let params = load_params();
    let addr = params.addr.clone();
    let mut client = RacClient::connect(&addr, client_cfg()).expect("connect");

    let resp = agent_version(&mut client).expect("agent version");
    assert_eq!(
        resp.version.as_deref(),
        Some(params.expected_agent_version.as_str())
    );

    client.close().expect("close");
}

#[test]
fn live_agent_version_and_cluster_list() {
    let params = load_params();
    let addr = params.addr.clone();
    let mut client = RacClient::connect(&addr, client_cfg()).expect("connect");

    let resp = agent_version(&mut client).expect("agent version");
    assert_eq!(
        resp.version.as_deref(),
        Some(params.expected_agent_version.as_str())
    );

    let clusters = cluster_list(&mut client).expect("cluster list");
    assert!(!clusters.is_empty(), "cluster list empty");

    let cluster = clusters[0].uuid;
    let reply = manager_list(&mut client, cluster).expect("manager list");
    let _ = reply.managers;

    client.close().expect("close");
}

#[test]
fn live_infobase_summary_list() {
    let params = load_params();
    let addr = params.addr.clone();
    let cluster_uuid = require_cluster_uuid(&params);
    let mut client = RacClient::connect(&addr, client_cfg()).expect("connect");

    let resp = agent_version(&mut client).expect("agent version");
    assert_eq!(
        resp.version.as_deref(),
        Some(params.expected_agent_version.as_str())
    );
    let reply = infobase_summary_list(&mut client, cluster_uuid).expect("infobase summary list");

    assert_eq!(reply.summaries.len(), reply.infobases.len());

    client.close().expect("close");
}
