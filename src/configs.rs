use clap::{Parser, Subcommand};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(
    version = VERSION.unwrap_or("unknown"),
    author = "StreamingFast Developers <dev@streamingfast.io>"
)]
pub(crate) struct Opts {
    #[clap(short, long)]
    pub home: Option<std::path::PathBuf>,
    #[clap(short, long)]
    pub start_block: Option<u64>,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub(crate) enum SubCommand {
    Run,
}
