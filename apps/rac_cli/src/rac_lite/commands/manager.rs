use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{manager_info, manager_list};
use rac_protocol::error::Result;

use crate::rac_lite::cli::ManagerCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::parse_uuid_arg;

pub fn run(json: bool, cfg: &ClientConfig, command: ManagerCmd) -> Result<()> {
    match command {
        ManagerCmd::List { addr, cluster } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = manager_list(&mut client, cluster)?;
            console::output(json, &resp, console::manager_list(&resp.managers));
            client.close()?;
        }
        ManagerCmd::Info {
            addr,
            cluster,
            manager,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let manager = parse_uuid_arg(&manager)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = manager_info(&mut client, cluster, manager)?;
            console::output(json, &resp, console::manager_info(&resp.record));
            client.close()?;
        }
    }
    Ok(())
}
