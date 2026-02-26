use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{
    counter_accumulated_values, counter_clear, counter_info, counter_list, counter_remove,
    counter_update, counter_values, CounterAccumulatedValuesRpc, CounterClearRpc,
    CounterRemoveRpc, CounterUpdateRpc, CounterValuesRpc,
};
use rac_protocol::error::Result;

use rac_protocol::commands::cluster_auth_optional;
use crate::rac_lite::cli::CounterCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::{
    parse_counter_analyze_flag, parse_counter_filter_type, parse_counter_group, parse_uuid_arg,
};

pub fn run(json: bool, cfg: &ClientConfig, command: CounterCmd) -> Result<()> {
    match command {
        CounterCmd::List { addr, cluster } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = counter_list(&mut client, cluster)?;
            console::output(json, &resp, console::counter_list(&resp.records));
            client.close()?;
        }
        CounterCmd::Info {
            addr,
            cluster,
            counter,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = counter_info(&mut client, cluster, &counter)?;
            console::output(json, &resp, console::counter_info(&resp.record));
            client.close()?;
        }
        CounterCmd::Clear {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            counter,
            object,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = counter_clear(
                &mut client,
                creds.user,
                creds.pwd,
                CounterClearRpc {
                    cluster,
                    counter,
                    object,
                },
            )?;
            console::output(json, &resp, console::counter_clear(&resp));
            client.close()?;
        }
        CounterCmd::Remove {
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
            let resp = counter_remove(
                &mut client,
                creds.user,
                creds.pwd,
                CounterRemoveRpc { cluster, name },
            )?;
            console::output(json, &resp, console::counter_remove(&resp));
            client.close()?;
        }
        CounterCmd::Values {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            counter,
            object,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = counter_values(
                &mut client,
                creds.user,
                creds.pwd,
                CounterValuesRpc {
                    cluster,
                    counter,
                    object,
                },
            )?;
            console::output(json, &resp, console::counter_values(&resp.records));
            client.close()?;
        }
        CounterCmd::Update {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            name,
            collection_time,
            group,
            filter_type,
            filter,
            duration,
            cpu_time,
            duration_dbms,
            service,
            memory,
            read,
            write,
            dbms_bytes,
            call,
            number_of_active_sessions,
            number_of_sessions,
            descr,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let req = CounterUpdateRpc {
                cluster,
                name,
                collection_time,
                group: parse_counter_group(&group)?,
                filter_type: parse_counter_filter_type(&filter_type)?,
                filter,
                duration: parse_counter_analyze_flag("duration", &duration)?,
                cpu_time: parse_counter_analyze_flag("cpu-time", &cpu_time)?,
                duration_dbms: parse_counter_analyze_flag("duration-dbms", &duration_dbms)?,
                service: parse_counter_analyze_flag("service", &service)?,
                memory: parse_counter_analyze_flag("memory", &memory)?,
                read: parse_counter_analyze_flag("read", &read)?,
                write: parse_counter_analyze_flag("write", &write)?,
                dbms_bytes: parse_counter_analyze_flag("dbms-bytes", &dbms_bytes)?,
                call: parse_counter_analyze_flag("call", &call)?,
                number_of_active_sessions: parse_counter_analyze_flag(
                    "number-of-active-sessions",
                    &number_of_active_sessions,
                )?,
                number_of_sessions: parse_counter_analyze_flag(
                    "number-of-sessions",
                    &number_of_sessions,
                )?,
                descr,
            };
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = counter_update(&mut client, creds.user, creds.pwd, req)?;
            console::output(json, &resp, console::counter_update(&resp));
            client.close()?;
        }
        CounterCmd::AccumulatedValues {
            addr,
            cluster,
            cluster_user,
            cluster_pwd,
            counter,
            object,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let creds = cluster_auth_optional(
                &mut client,
                cluster,
                cluster_user.as_deref(),
                cluster_pwd.as_deref(),
            )?;
            let resp = counter_accumulated_values(
                &mut client,
                creds.user,
                creds.pwd,
                CounterAccumulatedValuesRpc {
                    cluster,
                    counter,
                    object,
                },
            )?;
            console::output(
                json,
                &resp,
                console::counter_accumulated_values(&resp.records),
            );
            client.close()?;
        }
    }
    Ok(())
}
