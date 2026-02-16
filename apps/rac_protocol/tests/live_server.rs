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
}

fn load_params() -> TestParams {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let path = format!("{}/tests/params.toml", manifest_dir);
    let data = fs::read_to_string(&path).expect("read tests/params.toml");
    toml::from_str(&data).expect("parse tests/params.toml")
}

fn load_cluster_uuid() -> Option<[u8; 16]> {
    let value = std::env::var("RAC_CLUSTER").ok()?;
    parse_uuid(&value).ok()
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
    let addr = std::env::var("RAC_ADDR").unwrap_or_else(|_| params.addr.clone());
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
    let addr = std::env::var("RAC_ADDR").unwrap_or_else(|_| params.addr.clone());
    let mut client = RacClient::connect(&addr, client_cfg()).expect("connect");

    let resp = agent_version(&mut client).expect("agent version");
    assert_eq!(
        resp.version.as_deref(),
        Some(params.expected_agent_version.as_str())
    );

    let clusters = cluster_list(&mut client).expect("cluster list").clusters;
    if clusters.is_empty() {
        eprintln!("cluster list empty; skipping manager list");
        client.close().expect("close");
        return;
    }

    let cluster = clusters[0].uuid;
    let reply = match manager_list(&mut client, cluster) {
        Ok(resp) => resp,
        Err(err) => {
            if let rac_protocol::error::RacError::Io(io_err) = &err {
                if io_err.kind() == std::io::ErrorKind::WouldBlock {
                    eprintln!("manager list: no response (timeout), skipping");
                    let _ = client.close();
                    return;
                }
            }
            panic!("manager list: {err:?}");
        }
    };
    let _ = reply.managers;

    client.close().expect("close");
}

#[test]
fn live_infobase_summary_list() {
    let Some(cluster_uuid) = load_cluster_uuid() else {
        eprintln!("RAC_CLUSTER is not set; skipping live_infobase_summary_list");
        return;
    };

    let params = load_params();
    let addr = std::env::var("RAC_ADDR").unwrap_or_else(|_| params.addr.clone());
    let mut client = RacClient::connect(&addr, client_cfg()).expect("connect");

    let _ = agent_version(&mut client);

    let reply = match infobase_summary_list(&mut client, cluster_uuid) {
        Ok(resp) => resp,
        Err(err) => {
            if let rac_protocol::error::RacError::Io(io_err) = &err {
                if io_err.kind() == std::io::ErrorKind::WouldBlock {
                    eprintln!("infobase summary list: no response (timeout), skipping");
                    let _ = client.close();
                    return;
                }
            }
            panic!("infobase summary list: {err:?}");
        }
    };

    assert!(
        !reply.summaries.is_empty() || reply.infobases.is_empty(),
        "unexpected empty summaries with non-empty infobases"
    );

    client.close().expect("close");
}
