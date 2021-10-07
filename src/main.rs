mod codec;
mod configs;
mod dm;
mod logging;

use actix;
use clap::Clap;
use configs::{Opts, SubCommand};
use near_indexer;
use tracing::info;

fn main() {
    info!(target: "main", "Starting");

    openssl_probe::init_ssl_cert_env_vars();
    logging::init();

    let opts: Opts = Opts::parse();

    let home_dir = opts
        .home
        .unwrap_or(std::path::PathBuf::from(near_indexer::get_default_home()));


    let mut sync_mode = near_indexer::SyncModeEnum::LatestSynced;
    let start_block = opts.start_block.unwrap_or(0);
    if start_block > 0 {
        sync_mode = near_indexer::SyncModeEnum::BlockHeight(start_block)
    }

    match opts.subcmd {
        SubCommand::Run => {
            info!(target: "main", "Running");
            let indexer_config = near_indexer::IndexerConfig {
                home_dir,
                sync_mode,
                await_for_node_synced: near_indexer::AwaitForNodeSyncedEnum::StreamWhileSyncing,
            };

            let system = actix::System::new();

            system.block_on(async move {
                let indexer = near_indexer::Indexer::new(indexer_config);
                let mut stream = indexer.streamer();
                actix::spawn(async move {
                    while let Some(streamer_message) = stream.recv().await {
                        let block = codec::Block::from(&streamer_message);
                        dm::on_block(&block);
                    }
                });
            });

            system.run().unwrap();
        }
    }
}
