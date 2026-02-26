use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{limit_info, limit_list, limit_remove, limit_update, LimitUpdateReq};
use rac_protocol::error::Result;

use crate::rac_lite::auth::cluster_auth_optional;
use crate::rac_lite::cli::LimitCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::{parse_limit_action, parse_uuid_arg};

pub fn run(json: bool, cfg: &ClientConfig, command: LimitCmd) -> Result<()> {
    match command {
        LimitCmd::List { addr, cluster } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = limit_list(&mut client, cluster)?;
            console::output(json, &resp, console::limit_list(&resp.limits));
            client.close()?;
        }
        LimitCmd::Info {
            addr,
            cluster,
            limit,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = limit_info(&mut client, cluster, &limit)?;
            console::output(json, &resp, console::limit_info(&resp.record));
            client.close()?;
        }
        LimitCmd::Update {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            name,
            action,
            counter,
            duration,
            cpu_time,
            memory,
            read,
            write,
            duration_dbms,
            dbms_bytes,
            service,
            call,
            number_of_active_sessions,
            number_of_sessions,
            error_message,
            descr,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let req = LimitUpdateReq {
                name,
                counter,
                action: parse_limit_action(&action)?,
                duration,
                cpu_time,
                memory,
                read,
                write,
                duration_dbms,
                dbms_bytes,
                service,
                call,
                number_of_active_sessions,
                number_of_sessions,
                error_message,
                descr,
            };
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = limit_update(&mut client, cluster, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::limit_update(&resp));
            client.close()?;
        }
        LimitCmd::Remove {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            name,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = limit_remove(&mut client, cluster, creds.user, creds.pwd, &name)?;
            console::output(json, &resp, console::limit_remove(&resp));
            client.close()?;
        }
    }
    Ok(())
}
