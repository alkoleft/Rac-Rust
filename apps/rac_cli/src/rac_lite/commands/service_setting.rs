use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{
    service_setting_apply,
    service_setting_get_service_data_dirs_for_transfer,
    service_setting_info,
    service_setting_info_no_auth,
    service_setting_insert,
    service_setting_list,
    service_setting_remove,
    service_setting_update_no_auth,
    ServiceSettingApplyRpc,
    ServiceSettingGetDataDirsRpc,
    ServiceSettingInfoRpc,
    ServiceSettingInsertRpc,
    ServiceSettingListRpc,
    ServiceSettingRemoveRpc,
    ServiceSettingUpdateRpc,
};
use rac_protocol::error::Result;

use rac_protocol::commands::cluster_auth_optional;
use crate::rac_lite::cli::ServiceSettingCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::parse_uuid_arg;

pub fn run(json: bool, cfg: &ClientConfig, command: ServiceSettingCmd) -> Result<()> {
    match command {
        ServiceSettingCmd::List {
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
            let req = ServiceSettingListRpc { cluster, server };
            let resp = service_setting_list(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::service_setting_list(&resp.records));
            client.close()?;
        }
        ServiceSettingCmd::Info {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            server,
            setting,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let server = parse_uuid_arg(&server)?;
            let setting = parse_uuid_arg(&setting)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let req = ServiceSettingInfoRpc {
                cluster,
                server,
                setting,
            };
            let resp = service_setting_info(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::service_setting_info(&resp.record));
            client.close()?;
        }
        ServiceSettingCmd::Insert {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            server,
            service_name,
            infobase_name,
            service_data_dir,
            active,
            no_active,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let server = parse_uuid_arg(&server)?;
            let active = if no_active { false } else { active };
            let active = if active { 1u16 } else { 0u16 };
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let req = ServiceSettingInsertRpc {
                cluster,
                server,
                service_name,
                infobase_name,
                service_data_dir,
                active,
            };
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = service_setting_insert(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::service_setting_insert(&resp));
            client.close()?;
        }
        ServiceSettingCmd::Update {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            server,
            setting,
            service_data_dir,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let server = parse_uuid_arg(&server)?;
            let setting = parse_uuid_arg(&setting)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let _creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let info_req = ServiceSettingInfoRpc {
                cluster,
                server,
                setting,
            };
            let info = service_setting_info_no_auth(&mut client, info_req)?;
            let active = if info.record.active { 1u16 } else { 0u16 };
            let req = ServiceSettingUpdateRpc {
                cluster,
                server,
                setting,
                service_name: info.record.service_name,
                infobase_name: info.record.infobase_name,
                service_data_dir,
                active,
            };
            let resp = service_setting_update_no_auth(&mut client, req)?;
            console::output(json, &resp, console::service_setting_update(&resp));
            client.close()?;
        }
        ServiceSettingCmd::Remove {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            server,
            setting,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let server = parse_uuid_arg(&server)?;
            let setting = parse_uuid_arg(&setting)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let req = ServiceSettingRemoveRpc {
                cluster,
                server,
                setting,
            };
            let resp = service_setting_remove(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::service_setting_remove(&resp));
            client.close()?;
        }
        ServiceSettingCmd::Apply {
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
            let req = ServiceSettingApplyRpc { cluster, server };
            let resp = service_setting_apply(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::service_setting_apply(&resp));
            client.close()?;
        }
        ServiceSettingCmd::GetServiceDataDirsForTransfer {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            server,
            service_name,
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
            let req = ServiceSettingGetDataDirsRpc {
                cluster,
                server,
                service_name,
            };
            let resp = service_setting_get_service_data_dirs_for_transfer(
                &mut client,
                creds.user,
                creds.pwd,
                req,
            )?;
            console::output(
                json,
                &resp,
                console::service_setting_get_data_dirs_for_transfer(&resp.records),
            );
            client.close()?;
        }
    }
    Ok(())
}
