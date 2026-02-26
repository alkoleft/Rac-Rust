use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{agent_admin_list, agent_version};
use rac_protocol::error::Result;

use crate::rac_lite::cli::{AgentAdminCmd, AgentCmd};
use crate::rac_lite::console_output as console;

pub fn run(json: bool, cfg: &ClientConfig, command: AgentCmd) -> Result<()> {
    match command {
        AgentCmd::Version { addr } => {
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = agent_version(&mut client)?;
            console::output(json, &resp, console::agent_version(&resp.version));
            client.close()?;
        }
        AgentCmd::Admin { command } => match command {
            AgentAdminCmd::List {
                addr,
                agent_user,
                agent_pwd,
            } => {
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = agent_admin_list(&mut client, &agent_user, &agent_pwd)?;
                console::output(json, &resp, console::agent_admin_list(&resp.admins));
                client.close()?;
            }
        },
    }
    Ok(())
}
