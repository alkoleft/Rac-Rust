use clap::{Parser, Subcommand};

#[path = "rac_lite/console_output.rs"]
mod console_output;
#[path = "rac_lite/format.rs"]
mod format;

use console_output as console;
use rac_protocol::client::{ClientConfig, RacClient};
use rac_protocol::commands::{
    agent_version, cluster_info, cluster_list, connection_info, connection_list, counter_list,
    infobase_info, infobase_summary_info, infobase_summary_list, limit_list, lock_list,
    manager_info, manager_list, process_info, process_list, profile_list, server_info, server_list,
    session_info, session_list,
};
use rac_protocol::error::Result;
use rac_protocol::rac_wire::parse_uuid;
use rac_protocol::Uuid16;

#[derive(Parser, Debug)]
#[command(name = "rac_lite", version, about = "Minimal RAC client")]
struct Cli {
    #[arg(long)]
    json: bool,
    #[arg(long)]
    debug_raw: bool,
    #[command(subcommand)]
    command: TopCommand,
}

#[derive(Subcommand, Debug)]
enum TopCommand {
    Agent {
        #[command(subcommand)]
        command: AgentCmd,
    },
    Cluster {
        #[command(subcommand)]
        command: ClusterCmd,
    },
    Manager {
        #[command(subcommand)]
        command: ManagerCmd,
    },
    Server {
        #[command(subcommand)]
        command: ServerCmd,
    },
    Process {
        #[command(subcommand)]
        command: ProcessCmd,
    },
    Infobase {
        #[command(subcommand)]
        command: InfobaseCmd,
    },
    Connection {
        #[command(subcommand)]
        command: ConnectionCmd,
    },
    Session {
        #[command(subcommand)]
        command: SessionCmd,
    },
    Lock {
        #[command(subcommand)]
        command: LockCmd,
    },
    Profile {
        #[command(subcommand)]
        command: ProfileCmd,
    },
    Counter {
        #[command(subcommand)]
        command: CounterCmd,
    },
    Limit {
        #[command(subcommand)]
        command: LimitCmd,
    },
}

#[derive(Subcommand, Debug)]
enum AgentCmd {
    Version { addr: String },
}

#[derive(Subcommand, Debug)]
enum ClusterCmd {
    List {
        addr: String,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
}

#[derive(Subcommand, Debug)]
enum ManagerCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        manager: String,
    },
}

#[derive(Subcommand, Debug)]
enum ServerCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        server: String,
    },
}

#[derive(Subcommand, Debug)]
enum ProcessCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        process: String,
    },
}

#[derive(Subcommand, Debug)]
enum InfobaseCmd {
    SummaryList {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
    SummaryInfo {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        infobase: String,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        infobase: String,
    },
}

#[derive(Subcommand, Debug)]
enum ConnectionCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        connection: String,
    },
}

#[derive(Subcommand, Debug)]
enum SessionCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        session: String,
    },
}

#[derive(Subcommand, Debug)]
enum LockCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
}

#[derive(Subcommand, Debug)]
enum ProfileCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
}

#[derive(Subcommand, Debug)]
enum CounterCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
}

#[derive(Subcommand, Debug)]
enum LimitCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let cfg = client_cfg(&cli);
    match cli.command {
        TopCommand::Agent { command } => match command {
            AgentCmd::Version { addr } => {
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = agent_version(&mut client)?;
                console::output(cli.json, &resp, console::agent_version(&resp));
                client.close()?;
            }
        },
        TopCommand::Cluster { command } => match command {
            ClusterCmd::List { addr } => {
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = cluster_list(&mut client)?;
                console::output(cli.json, &resp, console::cluster_list(&resp));
                client.close()?;
            }
            ClusterCmd::Info { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = cluster_info(&mut client, cluster)?;
                console::output(cli.json, &resp, console::cluster_info(&resp));
                client.close()?;
            }
        },
        TopCommand::Manager { command } => match command {
            ManagerCmd::List { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = manager_list(&mut client, cluster)?;
                console::output(
                    cli.json,
                    &resp,
                    console::uuid_list("managers", &resp.managers),
                );
                client.close()?;
            }
            ManagerCmd::Info {
                addr,
                cluster,
                manager,
            } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let manager = parse_uuid_arg(&manager)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = manager_info(&mut client, cluster, manager)?;
                console::output(
                    cli.json,
                    &resp,
                    console::info("manager", &resp.manager, &resp.fields),
                );
                client.close()?;
            }
        },
        TopCommand::Server { command } => match command {
            ServerCmd::List { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = server_list(&mut client, cluster)?;
                console::output(
                    cli.json,
                    &resp,
                    console::uuid_list("servers", &resp.servers),
                );
                client.close()?;
            }
            ServerCmd::Info {
                addr,
                cluster,
                server,
            } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let server = parse_uuid_arg(&server)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = server_info(&mut client, cluster, server)?;
                console::output(
                    cli.json,
                    &resp,
                    console::info("server", &resp.server, &resp.fields),
                );
                client.close()?;
            }
        },
        TopCommand::Process { command } => match command {
            ProcessCmd::List { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = process_list(&mut client, cluster)?;
                console::output(
                    cli.json,
                    &resp,
                    console::uuid_list("processes", &resp.processes),
                );
                client.close()?;
            }
            ProcessCmd::Info {
                addr,
                cluster,
                process,
            } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let process = parse_uuid_arg(&process)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = process_info(&mut client, cluster, process)?;
                console::output(
                    cli.json,
                    &resp,
                    console::info("process", &resp.process, &resp.fields),
                );
                client.close()?;
            }
        },
        TopCommand::Infobase { command } => match command {
            InfobaseCmd::SummaryList { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = infobase_summary_list(&mut client, cluster)?;
                console::output(
                    cli.json,
                    &resp,
                    console::infobase_summary_list(&resp.summaries),
                );
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
                console::output(
                    cli.json,
                    &resp,
                    console::info("infobase", &resp.infobase, &resp.fields),
                );
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
                console::output(
                    cli.json,
                    &resp,
                    console::info("infobase", &resp.infobase, &resp.fields),
                );
                client.close()?;
            }
        },
        TopCommand::Connection { command } => match command {
            ConnectionCmd::List { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = connection_list(&mut client, cluster)?;
                console::output(
                    cli.json,
                    &resp,
                    console::uuid_list("connections", &resp.connections),
                );
                client.close()?;
            }
            ConnectionCmd::Info {
                addr,
                cluster,
                connection,
            } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let connection = parse_uuid_arg(&connection)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = connection_info(&mut client, cluster, connection)?;
                console::output(
                    cli.json,
                    &resp,
                    console::info("connection", &resp.connection, &resp.fields),
                );
                client.close()?;
            }
        },
        TopCommand::Session { command } => match command {
            SessionCmd::List { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = session_list(&mut client, cluster)?;
                console::output(cli.json, &resp, console::session_list(&resp.records));
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
                console::output(cli.json, &resp, console::session_info(&resp.record));
                client.close()?;
            }
        },
        TopCommand::Lock { command } => match command {
            LockCmd::List { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = lock_list(&mut client, cluster)?;
                console::output(cli.json, &resp, console::uuid_list("locks", &resp.locks));
                client.close()?;
            }
        },
        TopCommand::Profile { command } => match command {
            ProfileCmd::List { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = profile_list(&mut client, cluster)?;
                console::output(
                    cli.json,
                    &resp,
                    console::uuid_list("profiles", &resp.profiles),
                );
                client.close()?;
            }
        },
        TopCommand::Counter { command } => match command {
            CounterCmd::List { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = counter_list(&mut client, cluster)?;
                console::output(
                    cli.json,
                    &resp,
                    console::uuid_list("counters", &resp.counters),
                );
                client.close()?;
            }
        },
        TopCommand::Limit { command } => match command {
            LimitCmd::List { addr, cluster } => {
                let cluster = parse_uuid_arg(&cluster)?;
                let mut client = RacClient::connect(&addr, cfg.clone())?;
                let resp = limit_list(&mut client, cluster)?;
                console::output(cli.json, &resp, console::limit_list(&resp.limits));
                client.close()?;
            }
        },
    }
    Ok(())
}

fn parse_uuid_arg(input: &str) -> Result<Uuid16> {
    Ok(parse_uuid(input)?)
}

fn client_cfg(cli: &Cli) -> ClientConfig {
    let mut cfg = ClientConfig::default();
    cfg.debug_raw = cli.debug_raw;
    cfg
}
