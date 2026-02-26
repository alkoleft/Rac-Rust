use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{
    cluster_admin_list, cluster_admin_register, cluster_auth, cluster_info, cluster_list,
};
use rac_protocol::error::Result;

use crate::rac_lite::cli::{ClusterAdminCmd, ClusterCmd};
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::{parse_auth_flags, parse_uuid_arg};

pub fn run(json: bool, cfg: &ClientConfig, command: ClusterCmd) -> Result<()> {
    match command {
        ClusterCmd::List { addr } => {
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = cluster_list(&mut client)?;
            console::output(json, &resp, console::cluster_list(&resp));
            client.close()?;
        }
        ClusterCmd::Info { addr, cluster } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = cluster_info(&mut client, cluster)?;
            console::output(json, &resp, console::cluster_info(&resp));
            client.close()?;
        }
        ClusterCmd::Admin { command } => match command {
            ClusterAdminCmd::List {
                addr,
                cluster,
                cluster_user,
                cluster_pwd,
            } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                cluster_auth(&mut client, cluster, &cluster_user, &cluster_pwd)?;
                let resp = cluster_admin_list(&mut client, cluster)?;
                console::output(json, &resp, console::cluster_admin_list(&resp));
                client.close()?;
            }
            ClusterAdminCmd::Register {
                addr,
                cluster,
                cluster_user,
                cluster_pwd,
                name,
                pwd,
                descr,
                auth,
            } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let auth_flags = parse_auth_flags(&auth)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                cluster_auth(&mut client, cluster, &cluster_user, &cluster_pwd)?;
                let resp =
                    cluster_admin_register(&mut client, cluster, name, descr, pwd, auth_flags)?;
                console::output(json, &resp, console::cluster_admin_register(resp));
                client.close()?;
            }
        },
    }
    Ok(())
}
