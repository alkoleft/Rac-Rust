use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::lock_list;
use rac_protocol::error::Result;

use crate::rac_lite::cli::LockCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::parse_uuid_arg;

pub fn run(json: bool, cfg: &ClientConfig, command: LockCmd) -> Result<()> {
    match command {
        LockCmd::List { addr, cluster } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = lock_list(&mut client, cluster)?;
            console::output(json, &resp, console::lock_list(&resp.records));
            client.close()?;
        }
    }
    Ok(())
}
