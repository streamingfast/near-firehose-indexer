mod sf_near_v1;

use crate::pb::SignatureType::{Ed25519, Secp256k1};
use near_crypto;
use near_indexer::near_primitives;
use near_indexer::near_primitives::errors as near_errors;
use near_indexer::near_primitives::errors::ActionErrorKind;
use near_indexer::near_primitives::views as near_views;
use near_indexer::near_primitives::views::{
    DataReceiverView, ExecutionStatusView, ReceiptEnumView, ReceiptView,
};
use near_indexer::StreamerMessage;
pub use sf_near_v1::*;
use std::fmt::{Display, Formatter};

impl From<&near_indexer::StreamerMessage> for BlockWrapper {
    fn from(sm: &StreamerMessage) -> Self {
        let h = sm.block.header.clone();
        // if let Some(ap) = h.approvals.as_ref()[0] {}

        let block = Block {
            header: Some(BlockHeader::from(h)),
            author: sm.block.author.clone(),
            chunks: sm
                .block
                .chunks
                .clone()
                .into_iter()
                .map(|ch| ChunkHeader::from(ch))
                .collect(),
        };
        BlockWrapper {
            block: Some(block),
            shards: sm.shards.iter().map(|s| IndexerShard::from(s)).collect(),
            state_changes: vec![],
        }
    }
}

impl From<near_views::BlockHeaderView> for BlockHeader {
    fn from(h: near_views::BlockHeaderView) -> Self {
        BlockHeader {
            height: h.height,
            prev_height: 0, //todo: this is v3 feature, what that means?
            epoch_id: Some(CryptoHash::from(h.epoch_id)),
            next_epoch_id: Some(CryptoHash::from(h.next_epoch_id)),
            hash: Some(CryptoHash::from(h.hash)),
            prev_hash: Some(CryptoHash::from(h.prev_hash)),
            prev_state_root: Some(CryptoHash::from(h.prev_state_root)),
            chunk_receipts_root: Some(CryptoHash::from(h.chunk_receipts_root)),
            chunk_headers_root: Some(CryptoHash::from(h.chunk_headers_root)),
            chunk_tx_root: Some(CryptoHash::from(h.chunk_tx_root)),
            outcome_root: Some(CryptoHash::from(h.outcome_root)),
            chunks_included: h.chunks_included,
            challenges_root: Some(CryptoHash::from(h.challenges_root)),
            timestamp: h.timestamp,
            timestamp_nanosec: h.timestamp_nanosec,
            random_value: Some(CryptoHash::from(h.random_value)),
            validator_proposals: h
                .validator_proposals
                .into_iter()
                .map(|p| ValidatorStake::from(p))
                .collect(),
            chunk_mask: h.chunk_mask,
            gas_price: Some(BigInt::from(h.gas_price)),
            block_ordinal: 0, //todo: this is v3 feature, what that means?
            validator_reward: Some(BigInt::from(h.validator_reward)),
            total_supply: Some(BigInt::from(h.total_supply)),
            challenges_result: h
                .challenges_result
                .into_iter()
                .map(|cr| SlashedValidator::from(cr))
                .collect(),
            last_final_block: Some(CryptoHash::from(h.last_final_block)),
            last_ds_final_block: Some(CryptoHash::from(h.last_ds_final_block)),
            next_bp_hash: Some(CryptoHash::from(h.next_bp_hash)),
            block_merkle_root: Some(CryptoHash::from(h.block_merkle_root)),
            epoch_sync_data_hash: vec![], //todo: this is v3 feature, what that means?
            approvals: vec![], //todo: need to find a way to make Signature::from work with near_crypto::Signature
            signature: None, //todo: need to find a way to make Signature::from work with near_crypto::Signature
            latest_protocol_version: h.latest_protocol_version,
        }
    }
}

impl From<&near_indexer::IndexerShard> for IndexerShard {
    fn from(is: &near_indexer::IndexerShard) -> Self {
        let id = is.shard_id;

        let chunk: Option<IndexerChunk> = match &is.chunk {
            None => None,
            Some(c) => Some(IndexerChunk::from(c)),
        };

        IndexerShard {
            shard_id: id,
            chunk,
            receipt_execution_outcomes: is
                .receipt_execution_outcomes
                .iter()
                .map(|r| IndexerExecutionOutcomeWithReceipt::from(r))
                .collect(),
        }
    }
}

impl From<&near_indexer::IndexerExecutionOutcomeWithReceipt>
    for IndexerExecutionOutcomeWithReceipt
{
    fn from(r: &near_indexer::IndexerExecutionOutcomeWithReceipt) -> Self {
        IndexerExecutionOutcomeWithReceipt {
            execution_outcome: Some(ExecutionOutcomeWithIdView::from(
                r.execution_outcome.clone(),
            )),
            receipt: Some(Receipt::from(r.receipt.clone())),
        }
    }
}

impl From<near_views::ReceiptView> for Receipt {
    fn from(r: near_views::ReceiptView) -> Self {
        Receipt {
            predecessor_id: r.predecessor_id,
            receiver_id: r.receiver_id,
            receipt_id: Some(CryptoHash::from(r.receipt_id)),
            receipt: match r.receipt {
                ReceiptEnumView::Action {
                    signer_id,
                    signer_public_key,
                    gas_price,
                    output_data_receivers,
                    input_data_ids,
                    actions,
                } => Some(receipt::Receipt::Action {
                    0: ReceiptAction {
                        signer_id,
                        signer_public_key: Some(PublicKey::from(signer_public_key.key_data())),
                        gas_price: Some(BigInt::from(gas_price)),
                        output_data_receivers: output_data_receivers
                            .into_iter()
                            .map(|o| DataReceiver::from(o))
                            .collect(),
                        input_data_ids: input_data_ids
                            .into_iter()
                            .map(|i| CryptoHash::from(i))
                            .collect(),
                        actions: actions.into_iter().map(|a| Action::from(a)).collect(),
                    },
                }),
                ReceiptEnumView::Data { data_id, data } => Some(receipt::Receipt::Data {
                    0: ReceiptData {
                        data_id: Some(CryptoHash::from(data_id)),
                        data: data.unwrap_or(vec![]),
                    },
                }),
            },
        }
    }
}

impl From<near_views::DataReceiverView> for DataReceiver {
    fn from(d: DataReceiverView) -> Self {
        DataReceiver {
            data_id: Some(CryptoHash::from(d.data_id)),
            receiver_id: d.receiver_id,
        }
    }
}

impl From<&near_indexer::IndexerChunkView> for IndexerChunk {
    fn from(s: &near_indexer::IndexerChunkView) -> Self {
        IndexerChunk {
            author: s.author.clone(),
            header: Some(ChunkHeader::from(s.header.clone())),
            transactions: s
                .transactions
                .iter()
                .map(|tx| IndexerTransactionWithOutcome::from(tx.clone()))
                .collect(),
            receipts: s.receipts.into_iter().map(|r| Receipt::from(r)).collect(),
        }
    }
}

impl From<near_indexer::IndexerTransactionWithOutcome> for IndexerTransactionWithOutcome {
    fn from(tx: near_indexer::IndexerTransactionWithOutcome) -> Self {
        IndexerTransactionWithOutcome {
            transaction: Some(SignedTransaction::from(tx.transaction)),
            outcome: Some(IndexerExecutionOutcomeWithOptionalReceipt::from(tx.outcome)),
        }
    }
}

impl From<near_indexer::IndexerExecutionOutcomeWithOptionalReceipt>
    for IndexerExecutionOutcomeWithOptionalReceipt
{
    fn from(o: near_indexer::IndexerExecutionOutcomeWithOptionalReceipt) -> Self {
        IndexerExecutionOutcomeWithOptionalReceipt {
            execution_outcome: Some(ExecutionOutcomeWithIdView::from(o.execution_outcome)),
            receipt: match o.receipt {
                None => None,
                Some(r) => Some(Receipt::from(r)),
            },
        }
    }
}
impl From<near_views::ExecutionOutcomeWithIdView> for ExecutionOutcomeWithIdView {
    fn from(o: near_views::ExecutionOutcomeWithIdView) -> Self {
        ExecutionOutcomeWithIdView {
            proof: Some(MerklePath::from(o.proof)),
            block_hash: Some(CryptoHash::from(o.block_hash)),
            id: Some(CryptoHash::from(o.id)),
            outcome: Some(ExecutionOutcome::from(o.outcome)),
        }
    }
}

impl From<near_views::ExecutionOutcomeView> for ExecutionOutcome {
    fn from(o: near_views::ExecutionOutcomeView) -> Self {
        ExecutionOutcome {
            logs: o.logs,
            receipt_ids: o
                .receipt_ids
                .into_iter()
                .map(|id| CryptoHash::from(id))
                .collect(),
            gas_burnt: o.gas_burnt,
            tokens_burnt: Some(BigInt::from(o.tokens_burnt)),
            executor_id: o.executor_id,
            status: Some(execution_outcome::Status::from(o.status)),
            //todo: meta data
        }
    }
}

impl From<near_views::ExecutionStatusView> for execution_outcome::Status {
    fn from(s: near_views::ExecutionStatusView) -> Self {
        match s {
            ExecutionStatusView::Unknown => execution_outcome::Status::Unknown {
                0: UnknownExecutionStatus {},
            },
            ExecutionStatusView::SuccessValue(v) => execution_outcome::Status::SuccessValue {
                0: SuccessValueExecutionStatus { value: v },
            },
            ExecutionStatusView::SuccessReceiptId(v) => {
                execution_outcome::Status::SuccessReceiptId {
                    0: SuccessReceiptIdExecutionStatus {
                        id: Some(CryptoHash::from(v)),
                    },
                }
            }

            ExecutionStatusView::Failure(tx_err) => execution_outcome::Status::Failure {
                0: FailureExecutionStatus {
                    failure: match tx_err {
                        near_errors::TxExecutionError::ActionError(ae) => {
                            Some(failure_execution_status::Failure::ActionError {
                                0: ActionError {
                                    index: ae.index.unwrap_or(0),
                                    kind: Some(match ae.kind {
                                        ActionErrorKind::AccountAlreadyExists { account_id } => {
                                            action_error::Kind::AccountAlreadyExist {
                                                0: AccountAlreadyExistsErrorKind { account_id },
                                            }
                                        }
                                        ActionErrorKind::AccountDoesNotExist { account_id } => {
                                            action_error::Kind::AccountDoesNotExist {
                                                0: AccountDoesNotExistErrorKind { account_id },
                                            }
                                        }
                                        ActionErrorKind::CreateAccountOnlyByRegistrar {
                                            account_id,
                                            registrar_account_id,
                                            predecessor_id,
                                        } => action_error::Kind::CreateAccountOnlyByRegistrar {
                                            0: CreateAccountOnlyByRegistrarErrorKind {
                                                account_id,
                                                registrar_account_id,
                                                predecessor_id,
                                            },
                                        },
                                        ActionErrorKind::CreateAccountNotAllowed {
                                            account_id,
                                            predecessor_id,
                                        } => action_error::Kind::CreateAccountNotAllowed {
                                            0: CreateAccountNotAllowedErrorKind {
                                                account_id,
                                                predecessor_id,
                                            },
                                        },
                                        ActionErrorKind::ActorNoPermission {
                                            account_id,
                                            actor_id,
                                        } => action_error::Kind::ActorNoPermission {
                                            0: ActorNoPermissionErrorKind {
                                                account_id,
                                                actor_id,
                                            },
                                        },
                                        ActionErrorKind::DeleteKeyDoesNotExist {
                                            account_id,
                                            public_key,
                                        } => action_error::Kind::DeleteKeyDoesNotExist {
                                            0: DeleteKeyDoesNotExistErrorKind {
                                                account_id,
                                                public_key: Some(PublicKey::from(
                                                    public_key.key_data(),
                                                )),
                                            },
                                        },
                                        ActionErrorKind::AddKeyAlreadyExists {
                                            account_id,
                                            public_key,
                                        } => action_error::Kind::AddKeyAlreadyExists {
                                            0: AddKeyAlreadyExistsErrorKind {
                                                account_id,
                                                public_key: Some(PublicKey::from(
                                                    public_key.key_data(),
                                                )),
                                            },
                                        },
                                        ActionErrorKind::DeleteAccountStaking { .. } => {
                                            action_error::Kind::DeleteAccountStaking {
                                                0: DeleteAccountStakingErrorKind {
                                                    account_id: "".to_string(),
                                                },
                                            }
                                        }
                                        ActionErrorKind::LackBalanceForState {
                                            account_id,
                                            amount,
                                        } => action_error::Kind::LackBalanceForState {
                                            0: LackBalanceForStateErrorKind {
                                                account_id,
                                                balance: Some(BigInt::from(amount)),
                                            },
                                        },
                                        ActionErrorKind::TriesToUnstake { account_id } => {
                                            action_error::Kind::TriesToUnstake {
                                                0: TriesToUnstakeErrorKind { account_id },
                                            }
                                        }
                                        ActionErrorKind::TriesToStake {
                                            account_id,
                                            stake,
                                            locked,
                                            balance,
                                        } => action_error::Kind::TriesToStake {
                                            0: TriesToStakeErrorKind {
                                                account_id,
                                                stake: Some(BigInt::from(stake)),
                                                locked: Some(BigInt::from(locked)),
                                                balance: Some(BigInt::from(balance)),
                                            },
                                        },
                                        ActionErrorKind::InsufficientStake {
                                            account_id,
                                            stake,
                                            minimum_stake,
                                        } => action_error::Kind::InsufficientStake {
                                            0: InsufficientStakeErrorKind {
                                                account_id,
                                                stake: Some(BigInt::from(stake)),
                                                minimum_stake: Some(BigInt::from(minimum_stake)),
                                            },
                                        },
                                        ActionErrorKind::FunctionCallError(_) => {
                                            action_error::Kind::FunctionCall {
                                                0: FunctionCallErrorKind {},
                                            }
                                        }
                                        ActionErrorKind::NewReceiptValidationError(..) => {
                                            action_error::Kind::NewReceiptValidation {
                                                0: NewReceiptValidationErrorKind {},
                                            }
                                        }
                                        ActionErrorKind::OnlyImplicitAccountCreationAllowed {
                                            account_id,
                                        } => {
                                            action_error::Kind::OnlyImplicitAccountCreationAllowed {
                                                0: OnlyImplicitAccountCreationAllowedErrorKind {
                                                    account_id,
                                                },
                                            }
                                        }
                                        ActionErrorKind::DeleteAccountWithLargeState {
                                            account_id,
                                        } => action_error::Kind::DeleteAccountWithLargeState {
                                            0: DeleteAccountWithLargeStateErrorKind { account_id },
                                        },
                                    }),
                                },
                            })
                        }
                        near_errors::TxExecutionError::InvalidTxError(e) => {
                            Some(failure_execution_status::Failure::InvalidTxError {
                                0: match e {
                                    near_errors::InvalidTxError::InvalidAccessKeyError(..) => {
                                        InvalidTxError::InvalidAccessKeyError.into()
                                    }
                                    near_errors::InvalidTxError::InvalidSignerId { .. } => {
                                        InvalidTxError::InvalidSignerId.into()
                                    }
                                    near_errors::InvalidTxError::SignerDoesNotExist { .. } => {
                                        InvalidTxError::SignerDoesNotExist.into()
                                    }
                                    near_errors::InvalidTxError::InvalidNonce { .. } => {
                                        InvalidTxError::InvalidNonce.into()
                                    }
                                    near_errors::InvalidTxError::NonceTooLarge { .. } => {
                                        InvalidTxError::NonceTooLarge.into()
                                    }
                                    near_errors::InvalidTxError::InvalidReceiverId { .. } => {
                                        InvalidTxError::InvalidReceiverId.into()
                                    }
                                    near_errors::InvalidTxError::InvalidSignature => {
                                        InvalidTxError::InvalidSignature.into()
                                    }
                                    near_errors::InvalidTxError::NotEnoughBalance { .. } => {
                                        InvalidTxError::NotEnoughBalance.into()
                                    }
                                    near_errors::InvalidTxError::LackBalanceForState { .. } => {
                                        InvalidTxError::LackBalanceForState.into()
                                    }
                                    near_errors::InvalidTxError::CostOverflow => {
                                        InvalidTxError::CostOverflow.into()
                                    }
                                    near_errors::InvalidTxError::InvalidChain => {
                                        InvalidTxError::InvalidChain.into()
                                    }
                                    near_errors::InvalidTxError::Expired => {
                                        InvalidTxError::Expired.into()
                                    }
                                    near_errors::InvalidTxError::ActionsValidation(_) => {
                                        InvalidTxError::ActionsValidation.into()
                                    }
                                    near_errors::InvalidTxError::TransactionSizeExceeded {
                                        ..
                                    } => InvalidTxError::TransactionSizeExceeded.into(),
                                },
                            })
                        }
                    },
                },
            },
        }
    }
}

impl From<near_primitives::merkle::MerklePath> for MerklePath {
    fn from(p: near_primitives::merkle::MerklePath) -> Self {
        MerklePath {
            path: p
                .into_iter()
                .map(|item| MerklePathItem {
                    hash: Some(CryptoHash::from(item.hash)),
                    direction: match item.direction {
                        near_primitives::merkle::Direction::Left => 0,
                        near_primitives::merkle::Direction::Right => 1,
                    },
                })
                .collect(),
        }
    }
}

impl From<near_views::SignedTransactionView> for SignedTransaction {
    fn from(tx: near_views::SignedTransactionView) -> Self {
        SignedTransaction {
            signer_id: tx.signer_id,
            public_key: Some(PublicKey::from(tx.public_key.key_data())),
            nonce: tx.nonce,
            receiver_id: tx.receiver_id,
            actions: tx.actions.into_iter().map(|a| Action::from(a)).collect(),
            signature: None, //todo: need to find a way to make Signature::from work with near_crypto::Signature,
            hash: Some(CryptoHash::from(tx.hash)),
        }
    }
}

impl From<near_views::ActionView> for Action {
    fn from(a: near_views::ActionView) -> Self {
        match a {
            near_views::ActionView::CreateAccount => Action {
                action: Some(action::Action::CreateAccount {
                    0: CreateAccountAction {},
                }),
            },
            near_views::ActionView::DeployContract { code } => Action {
                action: Some(action::Action::DeployContract {
                    0: DeployContractAction { code },
                }),
            },
            near_views::ActionView::FunctionCall {
                method_name,
                args,
                gas,
                deposit,
            } => Action {
                action: Some(action::Action::FunctionCall {
                    0: FunctionCallAction {
                        method_name,
                        args,
                        gas,
                        deposit: Some(BigInt::from(deposit)),
                    },
                }),
            },
            near_views::ActionView::Transfer { deposit } => Action {
                action: Some(action::Action::Transfer {
                    0: TransferAction {
                        deposit: Some(BigInt::from(deposit)),
                    },
                }),
            },
            near_views::ActionView::Stake { stake, public_key } => Action {
                action: Some(action::Action::Stake {
                    0: StakeAction {
                        stake: Some(BigInt::from(stake)),
                        public_key: Some(PublicKey::from(public_key.key_data())),
                    },
                }),
            },
            near_views::ActionView::AddKey {
                public_key,
                access_key,
            } => Action {
                action: Some(action::Action::AddKey {
                    0: AddKeyAction {
                        public_key: Some(PublicKey::from(public_key.key_data())),
                        access_key: Some(AccessKey::from(access_key)),
                    },
                }),
            },
            near_views::ActionView::DeleteKey { public_key } => Action {
                action: Some(action::Action::DeleteKey {
                    0: DeleteKeyAction {
                        public_key: Some(PublicKey::from(public_key.key_data())),
                    },
                }),
            },
            near_views::ActionView::DeleteAccount { beneficiary_id } => Action {
                action: Some(action::Action::DeleteAccount {
                    0: DeleteAccountAction { beneficiary_id },
                }),
            },
        }
    }
}

impl From<near_views::AccessKeyView> for AccessKey {
    fn from(k: near_views::AccessKeyView) -> Self {
        AccessKey {
            nonce: k.nonce,
            permission: Some(AccessKeyPermission::from(k.permission)),
        }
    }
}

impl From<near_views::AccessKeyPermissionView> for AccessKeyPermission {
    fn from(p: near_views::AccessKeyPermissionView) -> Self {
        match p {
            near_views::AccessKeyPermissionView::FunctionCall {
                allowance,
                receiver_id,
                method_names,
            } => AccessKeyPermission {
                permission: Some(access_key_permission::Permission::FunctionCall {
                    0: FunctionCallPermission {
                        allowance: match allowance {
                            None => None,
                            Some(a) => Some(BigInt::from(a)),
                        },
                        receiver_id,
                        method_names,
                    },
                }),
            },
            near_views::AccessKeyPermissionView::FullAccess => AccessKeyPermission {
                permission: Some(access_key_permission::Permission::FullAccess {
                    0: FullAccessPermission {},
                }),
            },
        }
    }
}

impl From<near_views::ChunkHeaderView> for ChunkHeader {
    fn from(ch: near_views::ChunkHeaderView) -> Self {
        ChunkHeader {
            chunk_hash: Vec::from(ch.chunk_hash),
            prev_block_hash: Vec::from(ch.prev_block_hash),
            outcome_root: Vec::from(ch.outcome_root),
            prev_state_root: Vec::from(ch.prev_state_root),
            encoded_merkle_root: Vec::from(ch.encoded_merkle_root),
            encoded_length: ch.encoded_length,
            height_created: ch.height_created,
            height_included: ch.height_included,
            shard_id: ch.shard_id,
            gas_used: ch.gas_used,
            gas_limit: ch.gas_limit,
            validator_reward: Some(BigInt::from(ch.validator_reward)),
            balance_burnt: Some(BigInt::from(ch.balance_burnt)),
            outgoing_receipts_root: Vec::from(ch.outgoing_receipts_root),
            tx_root: Vec::from(ch.tx_root),
            validator_proposals: ch
                .validator_proposals
                .into_iter()
                .map(|vp| ValidatorStake::from(vp))
                .collect(),
            signature: None, //todo: need to find a way to make Signature::from work with near_crypto::Signature
        }
    }
}

impl From<near_crypto::Signature> for Signature {
    fn from(sign: near_crypto::Signature) -> Self {
        match sign {
            near_crypto::Signature::ED25519(s) => Signature {
                r#type: Ed25519.into(),
                bytes: Vec::from(s.to_bytes()),
            } as Signature,
            near_crypto::Signature::SECP256K1(s) => {
                let data = Vec::from(<[u8; 65]>::from(s));
                Signature {
                    r#type: Secp256k1.into(),
                    bytes: data,
                }
            }
        }
    }
}

impl From<near_primitives::challenge::SlashedValidator> for SlashedValidator {
    fn from(sv: near_primitives::challenge::SlashedValidator) -> Self {
        SlashedValidator {
            account_id: sv.account_id,
            is_double_sign: sv.is_double_sign,
        }
    }
}

impl From<near_primitives::views::validator_stake_view::ValidatorStakeView> for ValidatorStake {
    fn from(sv: near_primitives::views::validator_stake_view::ValidatorStakeView) -> Self {
        ValidatorStake {
            account_id: sv.account_id,
            public_key: Some(PublicKey::from(sv.public_key.key_data())),
            stake: Some(BigInt::from(sv.stake)),
        }
    }
}

impl From<u128> for BigInt {
    fn from(i: u128) -> Self {
        BigInt {
            bytes: Vec::from(i.to_be_bytes()),
        }
    }
}

impl From<&[u8]> for PublicKey {
    fn from(data: &[u8]) -> Self {
        PublicKey {
            bytes: Vec::from(data),
        }
    }
}

impl From<near_primitives::hash::CryptoHash> for CryptoHash {
    fn from(h: near_primitives::hash::CryptoHash) -> Self {
        CryptoHash {
            bytes: Vec::from(h),
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
