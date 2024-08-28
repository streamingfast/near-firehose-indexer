use crate::codec;
use tracing::info;

pub fn on_block(block: &codec::Block) {
    // FIXME: Apply stats like approach (#Height, Block Count, Total Transactions, Total Receipts, etc..)
    info!(
        target: "firehose",
        "Block {} Shards: {}, Transactions: {}, Receipts: {}, ExecutionOutcomes: {}",
        block,
        block.shards.len(),
        block.shards.iter().map(|shard| if let Some(chunk) = &shard.chunk { chunk.transactions.len() } else { 0usize }).sum::<usize>(),
        block.shards.iter().map(|shard| if let Some(chunk) = &shard.chunk { chunk.receipts.len() } else { 0usize }).sum::<usize>(),
        block.shards.iter().map(|shard| shard.receipt_execution_outcomes.len()).sum::<usize>(),
    );

    record_block(block);
}

fn record_block(block: &codec::Block) {
    let bytes = prost::Message::encode_to_vec(block);
    let header = block.header.as_ref().unwrap();

    println!(
        "FIRE BLOCK {height:} {hash:} {parent_height:} {parent_hash:} {lib:} {timestamp:} {hex:}",
        height = header.height,
        hash = header.hash.as_ref().unwrap(),
        parent_height = header.prev_height,
        parent_hash = header.prev_hash.as_ref().unwrap(),
        lib = header.last_final_block.as_ref().unwrap(),
        timestamp = header.timestamp,
        hex = hex::encode(bytes),
    );
}
