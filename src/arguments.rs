use clap::Parser;

/// PostgreSQL DML change management tool
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Config file location
    #[clap(short, long)]
    #[clap(default_value_t = String::from("pg-sync.toml"))]
    pub config: String,
    /// Log level - Off, Error, Warn, Info, Debug, Trace,
    #[clap(short, long)]
    #[clap(default_value_t = log::LevelFilter::Off)]
    pub log_level: log::LevelFilter,
    ///Apply changes without confirmation
    #[clap(short, long)]
    pub apply: bool,

}