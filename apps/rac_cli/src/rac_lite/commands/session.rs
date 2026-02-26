use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{
    cluster_auth_optional,
    session_info,
    session_interrupt_current_server_call,
    session_list,
    session_terminate,
};
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
        SessionCmd::Terminate {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            session,
            error_message,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let session = parse_uuid_arg(&session)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = session_terminate(
                &mut client,
                creds.user,
                creds.pwd,
                cluster,
                session,
                error_message,
            )?;
            console::output(json, &resp, console::session_terminate(&resp));
            client.close()?;
        }
        SessionCmd::InterruptCurrentServerCall {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            session,
            error_message,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let session = parse_uuid_arg(&session)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = session_interrupt_current_server_call(
                &mut client,
                creds.user,
                creds.pwd,
                cluster,
                session,
                error_message,
            )?;
            console::output(json, &resp, console::session_interrupt_current_server_call(&resp));
            client.close()?;
        }
    }
    Ok(())
}
