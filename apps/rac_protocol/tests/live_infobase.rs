use std::fs;
use std::time::Duration;

use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{agent_version, cluster_list, infobase_info, infobase_summary_list};
use rac_protocol::rac_wire::{format_uuid, parse_uuid};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestParams {
    addr: String,
}

fn load_cluster_uuid() -> Option<[u8; 16]> {
    let value = std::env::var("RAC_CLUSTER").ok()?;
    parse_uuid(&value).ok()
}

fn load_params() -> TestParams {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set");
    let path = format!("{}/tests/params.toml", manifest_dir);
    let data = fs::read_to_string(&path).expect("read tests/params.toml");
    toml::from_str(&data).expect("parse tests/params.toml")
}

#[test]
fn live_infobase_info() {
    let params = load_params();
    let addr = std::env::var("RAC_ADDR").unwrap_or_else(|_| params.addr.clone());

    let cfg = ClientConfig {
        connect_timeout: Duration::from_secs(5),
        read_timeout: Duration::from_secs(30),
        write_timeout: Duration::from_secs(15),
        debug_raw: false,
    };
    let mut client = RacClient::connect(&addr, cfg).expect("connect");

    let _ = agent_version(&mut client);

    let cluster_uuid = match load_cluster_uuid() {
        Some(uuid) => uuid,
        None => {
            let clusters = match cluster_list(&mut client) {
                Ok(resp) => resp.clusters,
                Err(err) => {
                    eprintln!("cluster list failed: {err:?}");
                    let _ = client.close();
                    return;
                }
            };
            let Some(first) = clusters.first() else {
                eprintln!("cluster list empty; skipping infobase info");
                let _ = client.close();
                return;
            };
            first.uuid
        }
    };

    let list = match infobase_summary_list(&mut client, cluster_uuid) {
        Ok(resp) => resp.infobases,
        Err(err) => {
            eprintln!("infobase summary list failed: {err:?}");
            let _ = client.close();
            return;
        }
    };

    if list.is_empty() {
        eprintln!("infobase summary list empty; skipping infobase info");
        client.close().expect("close");
        return;
    }

    let infobase = list[0];
    let info = infobase_info(&mut client, cluster_uuid, infobase).expect("infobase info");
    println!(
        "infobase_info: uuid={}, fields={}",
        format_uuid(&info.infobase),
        info.fields.len()
    );

    client.close().expect("close");
}
