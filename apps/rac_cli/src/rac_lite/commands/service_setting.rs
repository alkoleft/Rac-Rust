use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{
    cluster_auth, service_setting_apply, service_setting_get_service_data_dirs_for_transfer,
    service_setting_info, service_setting_info_no_auth, service_setting_insert,
    service_setting_list, service_setting_remove, service_setting_update_no_auth,
    ServiceSettingInsertReq, ServiceSettingUpdateReq,
};
use rac_protocol::error::Result;

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
            let resp = service_setting_list(
                &mut client,
                cluster,
                &cluster_user,
                &cluster_pwd,
                server,
            )?;
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
            let resp = service_setting_info(
                &mut client,
                cluster,
                &cluster_user,
                &cluster_pwd,
                server,
                setting,
            )?;
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
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let req = ServiceSettingInsertReq {
                server,
                service_name,
                infobase_name,
                service_data_dir,
                active,
            };
            let resp = service_setting_insert(
                &mut client,
                cluster,
                &cluster_user,
                &cluster_pwd,
                req,
            )?;
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
            cluster_auth(&mut client, cluster, &cluster_user, &cluster_pwd)?;
            let info = service_setting_info_no_auth(&mut client, cluster, server, setting)?;
            let req = ServiceSettingUpdateReq {
                server,
                setting,
                service_name: info.record.service_name,
                infobase_name: info.record.infobase_name,
                service_data_dir,
                active: info.record.active,
            };
            let resp = service_setting_update_no_auth(&mut client, cluster, req)?;
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
            let resp = service_setting_remove(
                &mut client,
                cluster,
                &cluster_user,
                &cluster_pwd,
                server,
                setting,
            )?;
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
            let resp = service_setting_apply(
                &mut client,
                cluster,
                &cluster_user,
                &cluster_pwd,
                server,
            )?;
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
            let resp = service_setting_get_service_data_dirs_for_transfer(
                &mut client,
                cluster,
                &cluster_user,
                &cluster_pwd,
                server,
                &service_name,
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
