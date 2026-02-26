use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{session_info, session_list};
use rac_protocol::error::Result;

use crate::rac_lite::cli::SessionCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::parse_uuid_arg;

pub fn run(json: bool, cfg: &ClientConfig, command: SessionCmd) -> Result<()> {
    match command {
        SessionCmd::List { addr, cluster } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = session_list(&mut client, cluster)?;
            console::output(json, &resp, console::session_list(&resp.records));
            client.close()?;
        }
        SessionCmd::Info {
            addr,
            cluster,
            session,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let session = parse_uuid_arg(&session)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = session_info(&mut client, cluster, session)?;
            console::output(json, &resp, console::session_info(&resp.record));
            client.close()?;
        }
    }
    Ok(())
}
