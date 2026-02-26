use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{connection_info, connection_list};
use rac_protocol::error::Result;

use crate::rac_lite::cli::ConnectionCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::parse_uuid_arg;

pub fn run(json: bool, cfg: &ClientConfig, command: ConnectionCmd) -> Result<()> {
    match command {
        ConnectionCmd::List { addr, cluster } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = connection_list(&mut client, cluster)?;
            console::output(json, &resp, console::connection_list(&resp.records));
            client.close()?;
        }
        ConnectionCmd::Info {
            addr,
            cluster,
            connection,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let connection = parse_uuid_arg(&connection)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = connection_info(&mut client, cluster, connection)?;
            console::output(json, &resp, console::connection_info(&resp.record));
            client.close()?;
        }
    }
    Ok(())
}
