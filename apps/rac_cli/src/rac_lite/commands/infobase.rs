use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{infobase_info, infobase_summary_info, infobase_summary_list};
use rac_protocol::error::Result;

use crate::rac_lite::cli::InfobaseCmd;
use crate::rac_lite::console_output as console;
use crate::rac_lite::parse::parse_uuid_arg;

pub fn run(json: bool, cfg: &ClientConfig, command: InfobaseCmd) -> Result<()> {
    match command {
        InfobaseCmd::SummaryList { addr, cluster } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = infobase_summary_list(&mut client, cluster)?;
            console::output(json, &resp, console::infobase_summary_list(&resp.summaries));
            client.close()?;
        }
        InfobaseCmd::SummaryInfo {
            addr,
            cluster,
            infobase,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let infobase = parse_uuid_arg(&infobase)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = infobase_summary_info(&mut client, cluster, infobase)?;
            let summary = &resp.summary;
            let fields = vec![summary.name.clone(), summary.descr.clone()];
            console::output(json, &resp, console::info("infobase", &summary.infobase, &fields));
            client.close()?;
        }
        InfobaseCmd::Info {
            addr,
            cluster,
            infobase,
        } => {
            let cluster = parse_uuid_arg(&cluster)?;
            let infobase = parse_uuid_arg(&infobase)?;
            let mut client = RacClient::connect(&addr, cfg.clone())?;
            let resp = infobase_info(&mut client, cluster, infobase)?;
            let info = &resp.info;
            let fields = vec![info.name.clone(), info.descr.clone()];
            console::output(json, &resp, console::info("infobase", &info.infobase, &fields));
            client.close()?;
        }
    }
    Ok(())
}
