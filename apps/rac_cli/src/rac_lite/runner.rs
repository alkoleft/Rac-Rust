use rac_protocol::client::ClientConfig;
use rac_protocol::error::Result;

use crate::rac_lite::cli::{Cli, TopCommand};
use crate::rac_lite::commands;

pub fn run(cli: Cli) -> Result<()> {
    let json = cli.json;
    let cfg = client_cfg(&cli);

    match cli.command {
        TopCommand::Agent { command } => commands::agent::run(json, &cfg, command)?,
        TopCommand::Cluster { command } => commands::cluster::run(json, &cfg, command)?,
        TopCommand::Manager { command } => commands::manager::run(json, &cfg, command)?,
        TopCommand::Server { command } => commands::server::run(json, &cfg, command)?,
        TopCommand::Process { command } => commands::process::run(json, &cfg, command)?,
        TopCommand::Infobase { command } => commands::infobase::run(json, &cfg, command)?,
        TopCommand::Connection { command } => commands::connection::run(json, &cfg, command)?,
        TopCommand::Session { command } => commands::session::run(json, &cfg, command)?,
        TopCommand::Lock { command } => commands::lock::run(json, &cfg, command)?,
        TopCommand::Profile { command } => commands::profile::run(json, &cfg, command)?,
        TopCommand::Counter { command } => commands::counter::run(json, &cfg, command)?,
        TopCommand::Limit { command } => commands::limit::run(json, &cfg, command)?,
        TopCommand::Rule { command } => commands::rule::run(json, &cfg, command)?,
        TopCommand::ServiceSetting { command } => {
            commands::service_setting::run(json, &cfg, command)?
        }
    }

    Ok(())
}

fn client_cfg(cli: &Cli) -> ClientConfig {
    let mut cfg = ClientConfig::default();
    cfg.debug_raw = cli.debug_raw;
    cfg
}
