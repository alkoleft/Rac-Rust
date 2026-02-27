use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::profile_list;
use rac_protocol::error::Result;

use crate::rac_lite::cli::ProfileCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::parse_uuid_arg;

pub fn run(json: bool, cfg: &ClientConfig, command: ProfileCmd) -> Result<()> {
    match command {
        ProfileCmd::List { addr, cluster } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = profile_list(&mut client, cluster)?;
            console::output(json, &resp, console::profile_list(&resp.profiles));
            client.close()?;
        }
    }
    Ok(())
}
