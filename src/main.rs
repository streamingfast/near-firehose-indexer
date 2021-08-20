mod codec;
mod configs;
mod dm;
mod logging;

use actix;
use clap::Clap;
use configs::{Opts, SubCommand};
use near_indexer;
use tokio::sync::mpsc;
use tracing::info;

fn main() {
    info!(target: "main", "Starting");

    openssl_probe::init_ssl_cert_env_vars();
    logging::init();

    let opts: Opts = Opts::parse();

    let home_dir = opts
        .home
        .unwrap_or(std::path::PathBuf::from(near_indexer::get_default_home()));

    match opts.subcmd {
        SubCommand::Run => {
            info!(target: "main", "Running");
            let indexer_config = near_indexer::IndexerConfig {
                home_dir,
                // FIXME: We should configure that in the flags!
                sync_mode: near_indexer::SyncModeEnum::BlockHeight(0),
                await_for_node_synced: near_indexer::AwaitForNodeSyncedEnum::StreamWhileSyncing,
            };

            let system = actix::System::new();

            system.block_on(async move {
                let indexer = near_indexer::Indexer::new(indexer_config);
                let mut stream = indexer.streamer();
                actix::spawn(async move {
                    while let Some(streamer_message) = stream.recv().await {
                        let wrap = codec::BlockWrapper::from(&streamer_message);

                        dm::on_block(&wrap);
                    }
                });
            });

            system.run().unwrap();
        }
    }
}
