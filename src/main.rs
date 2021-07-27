mod configs;
mod pb;

use actix;
use clap::Clap;
use configs::{init_logging, Opts, SubCommand};
use near_indexer;
use tokio::sync::mpsc;
use tracing::info;

async fn listen_blocks(mut stream: mpsc::Receiver<near_indexer::StreamerMessage>) {
    while let Some(streamer_message) = stream.recv().await {
        let wrap = pb::BlockWrapper::from(&streamer_message);

        info!(target: "indexer_example", "vp {}: {}", streamer_message.block.header.validator_proposals.len(), wrap.block.as_ref().unwrap().header.as_ref().unwrap().validator_proposals.len());

        for (count, v) in wrap
            .block
            .unwrap()
            .header
            .unwrap()
            .validator_proposals
            .iter()
            .enumerate()
        {
            info!(target: "indexer_example", "VP {}: {}", count, v.account_id);
        }
        // let mut buf = Vec::new();
        // buf.reserve(wrap.encoded_len());
        // wrap.encode(&mut buf).unwrap();
        // info!(target: "indexer_example", "{:x?}", buf);
        // info!(target: "indexer_example", "BBBBB {}", wrap);

        info!(
            target: "indexer_example",
            "#{} {} Shards: {}, Transactions: {}, Receipts: {}, ExecutionOutcomes: {}",
            streamer_message.block.header.height,
            streamer_message.block.header.hash,
            wrap.shards.len(),
            wrap.shards.iter().map(|shard| if let Some(chunk) = &shard.chunk { chunk.transactions.len() } else { 0usize }).sum::<usize>(),
            wrap.shards.iter().map(|shard| if let Some(chunk) = &shard.chunk { chunk.receipts.len() } else { 0usize }).sum::<usize>(),
            wrap.shards.iter().map(|shard| shard.receipt_execution_outcomes.len()).sum::<usize>(),
        );
    }
}

fn main() {
    // We use it to automatically search the for root certificates to perform HTTPS calls
    // (sending telemetry and downloading genesis)
    info!(target: "Main", "Starting");
    openssl_probe::init_ssl_cert_env_vars();
    init_logging();

    let opts: Opts = Opts::parse();

    let home_dir = opts
        .home_dir
        .unwrap_or(std::path::PathBuf::from(near_indexer::get_default_home()));

    match opts.subcmd {
        SubCommand::Run => {
            info!(target: "Main", "Running");
            let indexer_config = near_indexer::IndexerConfig {
                home_dir,

                sync_mode: near_indexer::SyncModeEnum::BlockHeight(43355192),
                await_for_node_synced: near_indexer::AwaitForNodeSyncedEnum::StreamWhileSyncing,
            };
            let system = actix::System::new();
            system.block_on(async move {
                let indexer = near_indexer::Indexer::new(indexer_config);
                let stream = indexer.streamer();
                actix::spawn(listen_blocks(stream));
            });
            system.run().unwrap();
        }
        SubCommand::Init(config) => near_indexer::indexer_init_configs(&home_dir, config.into()),
    }
}
