use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{
    rule_apply,
    rule_info,
    rule_insert,
    rule_list,
    rule_remove,
    rule_update,
    RuleApplyRpc,
    RuleInfoRpc,
    RuleInsertRpc,
    RuleListRpc,
    RuleRemoveRpc,
    RuleUpdateRpc,
};
use rac_protocol::error::Result;

use rac_protocol::commands::cluster_auth_optional;
use crate::rac_lite::cli::RuleCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::{parse_rule_apply_mode, parse_uuid_arg};

pub fn run(json: bool, cfg: &ClientConfig, command: RuleCmd) -> Result<()> {
    match command {
        RuleCmd::Apply {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            mode,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mode = parse_rule_apply_mode(&mode)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let req = RuleApplyRpc { cluster, mode };
            let resp = rule_apply(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::rule_apply(&resp));
            client.close()?;
        }
        RuleCmd::List {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            server,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let server = parse_uuid_arg(&server)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let req = RuleListRpc { cluster, server };
            let resp = rule_list(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::rule_list(&resp.records));
            client.close()?;
        }
        RuleCmd::Info {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            server,
            rule,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let server = parse_uuid_arg(&server)?;
            let rule = parse_uuid_arg(&rule)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let req = RuleInfoRpc {
                cluster,
                server,
                rule,
            };
            let resp = rule_info(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::rule_info(&resp.record));
            client.close()?;
        }
        RuleCmd::Insert {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            server,
            position,
            object_type,
            infobase_name,
            rule_type,
            application_ext,
            priority,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let server = parse_uuid_arg(&server)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let req = RuleInsertRpc {
                cluster,
                server,
                rule: [0u8; 16],
                position,
                object_type,
                infobase_name,
                rule_type,
                application_ext,
                priority,
            };
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = rule_insert(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::rule_insert(&resp));
            client.close()?;
        }
        RuleCmd::Update {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            server,
            rule,
            position,
            object_type,
            infobase_name,
            rule_type,
            application_ext,
            priority,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let server = parse_uuid_arg(&server)?;
            let rule = parse_uuid_arg(&rule)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let req = RuleUpdateRpc {
                cluster,
                server,
                rule,
                position,
                object_type,
                infobase_name,
                rule_type,
                application_ext,
                priority,
            };
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = rule_update(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::rule_update(&resp));
            client.close()?;
        }
        RuleCmd::Remove {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            server,
            rule,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let server = parse_uuid_arg(&server)?;
            let rule = parse_uuid_arg(&rule)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let req = RuleRemoveRpc {
                cluster,
                server,
                rule,
            };
            let resp = rule_remove(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::rule_remove(&resp));
            client.close()?;
        }
    }
    Ok(())
}
