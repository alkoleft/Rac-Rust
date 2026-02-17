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

fn cluster_uuid_from_env_or_first(client: &mut RacClient) -> [u8; 16] {
    let from_env = std::env::var("RAC_CLUSTER")
        .ok()
        .map(|value| parse_uuid(&value).expect("RAC_CLUSTER must be a valid uuid"));
    from_env.unwrap_or_else(|| {
        let clusters = cluster_list(client).expect("cluster list").clusters;
        let first = clusters.first().expect("cluster list empty");
        first.uuid
    })
}

fn load_params() -> TestParams {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let path = format!("{}/tests/params.toml", manifest_dir);
    let data = fs::read_to_string(&path).expect("read tests/params.toml");
    toml::from_str(&data).expect("parse tests/params.toml")
}

#[test]
#[ignore]
fn live_infobase_info() {
    let params = load_params();
    let addr = std::env::var("RAC_ADDR").unwrap_or_else(|_| params.addr.clone());

    let cfg = ClientConfig {
        connect_timeout: Duration::from_secs(5),
        read_timeout: Duration::from_secs(30),
        write_timeout: Duration::from_secs(15),
        debug_raw: false,
        protocol: Default::default(),
    };
    let mut client = RacClient::connect(&addr, cfg).expect("connect");

    let _ = agent_version(&mut client);

    let cluster_uuid = cluster_uuid_from_env_or_first(&mut client);

    let list = infobase_summary_list(&mut client, cluster_uuid)
        .expect("infobase summary list")
        .infobases;
    let infobase = *list.first().expect("infobase summary list empty");

    let info = infobase_info(&mut client, cluster_uuid, infobase).expect("infobase info");
    println!(
        "infobase_info: uuid={}, fields={}",
        format_uuid(&info.infobase),
        info.fields.len()
    );

    client.close().expect("close");
}
