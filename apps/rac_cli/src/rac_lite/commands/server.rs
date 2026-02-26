use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{server_info, server_list};
use rac_protocol::error::Result;

use crate::rac_lite::cli::ServerCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::parse_uuid_arg;

pub fn run(json: bool, cfg: &ClientConfig, command: ServerCmd) -> Result<()> {
    match command {
        ServerCmd::List { addr, cluster } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = server_list(&mut client, cluster)?;
            console::output(json, &resp, console::server_list(&resp.servers));
            client.close()?;
        }
        ServerCmd::Info {
            addr,
            cluster,
            server,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let server = parse_uuid_arg(&server)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = server_info(&mut client, cluster, server)?;
            console::output(json, &resp, console::server_info(&resp.server));
            client.close()?;
        }
    }
    Ok(())
}
