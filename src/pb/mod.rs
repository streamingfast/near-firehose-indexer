mod sf_near_v1;
use near_indexer::StreamerMessage;
pub use sf_near_v1::*;
use std::fmt::{Display, Formatter};

impl From<&near_indexer::StreamerMessage> for BlockWrapper {
    fn from(sm: &StreamerMessage) -> Self {
        let h = sm.block.header.clone();
        let header = BlockHeader {
            height: h.height,
            prev_height: 0, //todo: this is v3 feature, what that means?
            epoch_id: Some(CryptoHash {
                bytes: Vec::from(h.epoch_id),
            }),
            next_epoch_id: Some(CryptoHash {
                bytes: Vec::from(h.next_epoch_id),
            }),
            hash: Some(CryptoHash {
                bytes: Vec::from(h.hash),
            }),
            prev_hash: Some(CryptoHash {
                bytes: Vec::from(h.prev_hash),
            }),
            prev_state_root: Some(CryptoHash {
                bytes: Vec::from(h.prev_state_root),
            }),
            chunk_receipts_root: Some(CryptoHash {
                bytes: Vec::from(h.chunk_receipts_root),
            }),
            chunk_headers_root: Some(CryptoHash {
                bytes: Vec::from(h.chunk_headers_root),
            }),
            chunk_tx_root: Some(CryptoHash {
                bytes: Vec::from(h.chunk_tx_root),
            }),
            outcome_root: Some(CryptoHash {
                bytes: Vec::from(h.outcome_root),
            }),
            chunks_included: h.chunks_included,
            challenges_root: Some(CryptoHash {
                bytes: Vec::from(h.challenges_root),
            }),
            timestamp: h.timestamp,
            timestamp_nanosec: h.timestamp_nanosec,
            random_value: Some(CryptoHash {
                bytes: Vec::from(h.random_value),
            }),
            validator_proposals: vec![],
            chunk_mask: h.chunk_mask,
            gas_price: None,
            block_ordinal: 0,
            validator_reward: None,
            total_supply: None,
            challenges_result: vec![],
            last_final_block: None,
            last_ds_final_block: None,
            next_bp_hash: None,
            block_merkle_root: None,
            epoch_sync_data_hash: vec![],
            approvals: vec![],
            signature: None,
            latest_protocol_version: 0,
        };

        let block = Block {
            header: Some(header),
            author: sm.block.author.clone(),
            chunks: vec![],
        };

        BlockWrapper {
            block: Some(block),
            shards: vec![],
            state_changes: vec![],
        }
    }
}

impl Display for BlockWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "here block {}",
            self.block.as_ref().unwrap().header.as_ref().unwrap().height
        )
    }
}
