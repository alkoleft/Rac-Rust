use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rac_lite", version, about = "Minimal RAC client")]
pub struct Cli {
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub debug_raw: bool,
    #[command(subcommand)]
    pub command: TopCommand,
}

#[derive(Subcommand, Debug)]
pub enum TopCommand {
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
    Rule {
        #[command(subcommand)]
        command: RuleCmd,
    },
    ServiceSetting {
        #[command(subcommand)]
        command: ServiceSettingCmd,
    },
}

#[derive(Subcommand, Debug)]
pub enum AgentCmd {
    Version { addr: String },
    Admin {
        #[command(subcommand)]
        command: AgentAdminCmd,
    },
}

#[derive(Subcommand, Debug)]
pub enum AgentAdminCmd {
    List {
        addr: String,
        #[arg(long)]
        agent_user: Option<String>,
        #[arg(long)]
        agent_pwd: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ClusterCmd {
    List {
        addr: String,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
    Admin {
        #[command(subcommand)]
        command: ClusterAdminCmd,
    },
}

#[derive(Subcommand, Debug)]
pub enum ClusterAdminCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
    },
    Register {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        name: String,
        #[arg(long)]
        pwd: String,
        #[arg(long, default_value = "")]
        descr: String,
        #[arg(long, default_value = "pwd")]
        auth: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ManagerCmd {
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
pub enum ServerCmd {
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
pub enum ProcessCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        licenses: bool,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        process: String,
        #[arg(long)]
        licenses: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum InfobaseCmd {
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
pub enum ConnectionCmd {
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
pub enum SessionCmd {
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
pub enum LockCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProfileCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum CounterCmd {
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
        counter: String,
    },
    Clear {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        counter: String,
        #[arg(long, default_value = "")]
        object: String,
    },
    Remove {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        name: String,
    },
    Values {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        counter: String,
        #[arg(long, default_value = "")]
        object: String,
    },
    AccumulatedValues {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        counter: String,
        #[arg(long, default_value = "")]
        object: String,
    },
    Update {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        name: String,
        #[arg(long)]
        collection_time: u64,
        #[arg(long)]
        group: String,
        #[arg(long)]
        filter_type: String,
        #[arg(long)]
        filter: String,
        #[arg(long)]
        duration: String,
        #[arg(long)]
        cpu_time: String,
        #[arg(long)]
        duration_dbms: String,
        #[arg(long)]
        service: String,
        #[arg(long)]
        memory: String,
        #[arg(long)]
        read: String,
        #[arg(long)]
        write: String,
        #[arg(long)]
        dbms_bytes: String,
        #[arg(long)]
        call: String,
        #[arg(long)]
        number_of_active_sessions: String,
        #[arg(long)]
        number_of_sessions: String,
        #[arg(long, default_value = "")]
        descr: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum LimitCmd {
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
        limit: String,
    },
    Update {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        name: String,
        #[arg(long)]
        action: String,
        #[arg(long, default_value = "")]
        counter: String,
        #[arg(long, default_value_t = 0)]
        duration: u64,
        #[arg(long, default_value_t = 0)]
        cpu_time: u64,
        #[arg(long, default_value_t = 0)]
        memory: u64,
        #[arg(long, default_value_t = 0)]
        read: u64,
        #[arg(long, default_value_t = 0)]
        write: u64,
        #[arg(long, default_value_t = 0)]
        duration_dbms: u64,
        #[arg(long, default_value_t = 0)]
        dbms_bytes: u64,
        #[arg(long, default_value_t = 0)]
        service: u64,
        #[arg(long, default_value_t = 0)]
        call: u64,
        #[arg(long, default_value_t = 0)]
        number_of_active_sessions: u64,
        #[arg(long, default_value_t = 0)]
        number_of_sessions: u64,
        #[arg(long, default_value = "")]
        error_message: String,
        #[arg(long, default_value = "")]
        descr: String,
    },
    Remove {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum RuleCmd {
    Apply {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long, default_value = "full")]
        mode: String,
    },
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
        #[arg(long)]
        rule: String,
    },
    Insert {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
        #[arg(long)]
        position: u32,
        #[arg(long)]
        object_type: u32,
        #[arg(long, default_value = "")]
        infobase_name: String,
        #[arg(long, default_value_t = 0)]
        rule_type: u8,
        #[arg(long, default_value = "")]
        application_ext: String,
        #[arg(long, default_value_t = 0)]
        priority: u32,
    },
    Update {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
        #[arg(long)]
        rule: String,
        #[arg(long)]
        position: u32,
        #[arg(long)]
        object_type: u32,
        #[arg(long, default_value = "")]
        infobase_name: String,
        #[arg(long, default_value_t = 0)]
        rule_type: u8,
        #[arg(long, default_value = "")]
        application_ext: String,
        #[arg(long, default_value_t = 0)]
        priority: u32,
    },
    Remove {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
        #[arg(long)]
        rule: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ServiceSettingCmd {
    List {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
    },
    Info {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
        #[arg(long)]
        setting: String,
    },
    Insert {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
        #[arg(long)]
        service_name: String,
        #[arg(long, default_value = "")]
        infobase_name: String,
        #[arg(long, default_value = "")]
        service_data_dir: String,
        #[arg(long, default_value_t = false)]
        active: bool,
        #[arg(long = "no-active", default_value_t = false)]
        no_active: bool,
    },
    Update {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
        #[arg(long)]
        setting: String,
        #[arg(long)]
        service_data_dir: String,
    },
    Remove {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
        #[arg(long)]
        setting: String,
    },
    Apply {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
    },
    GetServiceDataDirsForTransfer {
        addr: String,
        #[arg(long)]
        cluster: String,
        #[arg(long)]
        cluster_user: Option<String>,
        #[arg(long)]
        cluster_pwd: Option<String>,
        #[arg(long)]
        server: String,
        #[arg(long, default_value = "")]
        service_name: String,
    },
}
