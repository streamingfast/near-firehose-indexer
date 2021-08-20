use crate::codec;
use tracing::info;

pub fn on_block(wrap: &codec::BlockWrapper) {
    // FIXME: Apply stats like approach (#Height, Block Count, Total Transactions, Total Receipts, etc..)
    info!(
        target: "dm",
        "Block {} Shards: {}, Transactions: {}, Receipts: {}, ExecutionOutcomes: {}",
        wrap,
        wrap.shards.len(),
        wrap.shards.iter().map(|shard| if let Some(chunk) = &shard.chunk { chunk.transactions.len() } else { 0usize }).sum::<usize>(),
        wrap.shards.iter().map(|shard| if let Some(chunk) = &shard.chunk { chunk.receipts.len() } else { 0usize }).sum::<usize>(),
        wrap.shards.iter().map(|shard| shard.receipt_execution_outcomes.len()).sum::<usize>(),
    );

    record_block(wrap);
}

fn record_block(wrap: &codec::BlockWrapper) {
    let bytes = prost::Message::encode_to_vec(wrap);
    let header = wrap.block.as_ref().unwrap().header.as_ref().unwrap();

    println!(
        "DMLOG BLOCK {height:} {hash:} {hex:}",
        height = header.height,
        hash = header.hash.as_ref().unwrap(),
        hex = hex::encode(bytes),
    );
}
