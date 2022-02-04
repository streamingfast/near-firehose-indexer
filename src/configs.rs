use clap::Clap;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Clap, Debug)]
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

#[derive(Clap, Debug)]
pub(crate) enum SubCommand {
    Run,
}
