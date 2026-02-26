use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{
    agent_admin_list,
    agent_admin_register,
    agent_admin_remove,
    agent_auth_optional,
    agent_version,
};
use rac_protocol::error::Result;

use crate::rac_lite::cli::{AgentAdminCmd, AgentCmd};
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::parse_auth_flags;

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
                let creds = agent_auth_optional(
                    &mut client,
                    agent_user.as_deref(),
                    agent_pwd.as_deref(),
                )?;
                let resp = agent_admin_list(&mut client, creds.user, creds.pwd)?;
                console::output(json, &resp, console::agent_admin_list(&resp.admins));
                client.close()?;
            }
            AgentAdminCmd::Register {
                addr,
                agent_user,
                agent_pwd,
                name,
                pwd,
                descr,
                auth,
                os_user,
            } => {
                let auth_flags = parse_auth_flags(&auth)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let creds = agent_auth_optional(
                    &mut client,
                    agent_user.as_deref(),
                    agent_pwd.as_deref(),
                )?;
                let resp = agent_admin_register(
                    &mut client,
                    creds.user,
                    creds.pwd,
                    name,
                    descr,
                    pwd,
                    auth_flags,
                    os_user,
                )?;
                console::output(json, &resp, console::agent_admin_register(&resp));
                client.close()?;
            }
            AgentAdminCmd::Remove {
                addr,
                agent_user,
                agent_pwd,
                name,
            } => {
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let creds = agent_auth_optional(
                    &mut client,
                    agent_user.as_deref(),
                    agent_pwd.as_deref(),
                )?;
                let resp = agent_admin_remove(&mut client, creds.user, creds.pwd, &name)?;
                console::output(json, &resp, console::agent_admin_remove(&resp));
                client.close()?;
            }
        },
    }
    Ok(())
}
