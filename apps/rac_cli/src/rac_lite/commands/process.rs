use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{process_info, process_list};
use rac_protocol::error::Result;

use crate::rac_lite::cli::ProcessCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::parse_uuid_arg;

pub fn run(json: bool, cfg: &ClientConfig, command: ProcessCmd) -> Result<()> {
    match command {
        ProcessCmd::List {
            addr,
            cluster,
            licenses,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = process_list(&mut client, cluster)?;
            if licenses {
                console::output(json, &resp, console::process_list_licenses(&resp.records));
            } else {
                console::output(json, &resp, console::process_list(&resp.records));
            }
            client.close()?;
        }
        ProcessCmd::Info {
            addr,
            cluster,
            process,
            licenses,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let process = parse_uuid_arg(&process)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = process_info(&mut client, cluster, process)?;
            if licenses {
                console::output(json, &resp, console::process_info_licenses(&resp.record));
            } else {
                console::output(json, &resp, console::process_info(&resp.record));
            }
            client.close()?;
        }
    }
    Ok(())
}
