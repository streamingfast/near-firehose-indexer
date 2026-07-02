#[path = "sf.near.r#type.v1.rs"]
mod codec;

pub use codec::*;
use near_crypto::PublicKey as NearPublicKey;
use near_crypto::Signature as NearSignature;
use near_indexer::StreamerMessage;
use near_indexer::near_primitives;
use near_indexer::near_primitives::action::GlobalContractIdentifier;
use near_indexer::near_primitives::errors as near_errors;
use near_indexer::near_primitives::errors::ActionErrorKind;
use near_indexer::near_primitives::views as near_views;
use near_indexer::near_primitives::views::{
    DataReceiverView, ExecutionMetadataView, ExecutionStatusView, ReceiptEnumView,
};

use base64::{Engine as _, engine::general_purpose};
use hex;
use std::fmt::{Display, Formatter};

// nearcore 2.11.0 fixed receipt_execution_outcomes ordering (https://github.com/near/nearcore/pull/15184): outcomes were previously
// sorted by CryptoHash, now they preserve execution order. To keep firehose output
// deterministic for blocks produced before the 2.11.0 deployment, we re-apply the old BTreeMap
// sort for any block strictly below legacy_receipt_ordering_last_block
const LEGACY_RECEIPT_ORDERING_LAST_BLOCK: u64 = 193444226;

impl From<near_indexer::StreamerMessage> for Block {
    fn from(sm: StreamerMessage) -> Self {
        let block_height = sm.block.header.height;
        Block {
            header: Some(BlockHeader::from(sm.block.header)),
            shards: sm
                .shards
                .into_iter()
                .map(|shard| indexer_shard_from(shard, block_height))
                .collect(),
            author: sm.block.author.to_string(),
            chunk_headers: sm.block.chunks.into_iter().map(ChunkHeader::from).collect(),
            state_changes: vec![],
        }
    }
}

fn indexer_shard_from(is: near_indexer::IndexerShard, block_height: u64) -> IndexerShard {
    let mut outcomes: Vec<IndexerExecutionOutcomeWithReceipt> = is
        .receipt_execution_outcomes
        .into_iter()
        .map(IndexerExecutionOutcomeWithReceipt::from)
        .collect();

    if block_height < LEGACY_RECEIPT_ORDERING_LAST_BLOCK {
        outcomes.sort_by(|a, b| {
            let a_id = a
                .execution_outcome
                .as_ref()
                .and_then(|o| o.id.as_ref())
                .map(|h| h.bytes.as_slice())
                .unwrap_or(&[]);
            let b_id = b
                .execution_outcome
                .as_ref()
                .and_then(|o| o.id.as_ref())
                .map(|h| h.bytes.as_slice())
                .unwrap_or(&[]);
            a_id.cmp(b_id)
        });
    }

    IndexerShard {
        shard_id: is.shard_id.into(),
        chunk: is.chunk.map(IndexerChunk::from),
        receipt_execution_outcomes: outcomes,
    }
}

impl From<near_views::BlockHeaderView> for BlockHeader {
    fn from(h: near_views::BlockHeaderView) -> Self {
        let challenges_result = h.challenges_result;
        let validator_proposals = h.validator_proposals;

        BlockHeader {
            hash: Some(CryptoHash::from(h.hash)),
            height: h.height,
            prev_hash: Some(CryptoHash::from(h.prev_hash)),
            timestamp_nanosec: h.timestamp_nanosec,
            prev_height: match h.prev_height {
                None => 0,
                Some(ph) => ph.into(),
            },
            epoch_id: Some(CryptoHash::from(h.epoch_id)),
            next_epoch_id: Some(CryptoHash::from(h.next_epoch_id)),
            prev_state_root: Some(CryptoHash::from(h.prev_state_root)),
            chunk_receipts_root: Some(CryptoHash::from(h.chunk_receipts_root)),
            chunk_headers_root: Some(CryptoHash::from(h.chunk_headers_root)),
            chunk_tx_root: Some(CryptoHash::from(h.chunk_tx_root)),
            outcome_root: Some(CryptoHash::from(h.outcome_root)),
            chunks_included: h.chunks_included,
            challenges_root: Some(CryptoHash::from(h.challenges_root)),
            timestamp: h.timestamp,
            random_value: Some(CryptoHash::from(h.random_value)),
            validator_proposals: validator_proposals
                .into_iter()
                .map(ValidatorStake::from)
                .collect(),
            chunk_mask: h.chunk_mask,
            gas_price: Some(BigInt::from(h.gas_price.as_yoctonear())),
            block_ordinal: 0, //todo: this is v3 feature, what that means?
            total_supply: Some(BigInt::from(h.total_supply.as_yoctonear())),
            challenges_result: challenges_result
                .into_iter()
                .map(SlashedValidator::from)
                .collect(),
            last_final_block_height: 0,
            last_final_block: Some(CryptoHash::from(h.last_final_block)),
            last_ds_final_block_height: 0,
            last_ds_final_block: Some(CryptoHash::from(h.last_ds_final_block)),
            next_bp_hash: Some(CryptoHash::from(h.next_bp_hash)),
            block_merkle_root: Some(CryptoHash::from(h.block_merkle_root)),
            epoch_sync_data_hash: vec![], //todo: this is v3 feature, what that means?
            approvals: h
                .approvals
                .into_iter()
                .filter_map(|s| s.map(|sig| sig.into()))
                .collect(),
            signature: Some(h.signature.into()),
            latest_protocol_version: h.latest_protocol_version,
        }
    }
}


impl From<near_indexer::IndexerExecutionOutcomeWithReceipt> for IndexerExecutionOutcomeWithReceipt {
    fn from(r: near_indexer::IndexerExecutionOutcomeWithReceipt) -> Self {
        IndexerExecutionOutcomeWithReceipt {
            execution_outcome: Some(ExecutionOutcomeWithId::from(r.execution_outcome)),
            receipt: Some(Receipt::from(r.receipt)),
        }
    }
}

impl From<near_views::ReceiptView> for Receipt {
    fn from(r: near_views::ReceiptView) -> Self {
        Receipt {
            predecessor_id: r.predecessor_id.to_string(),
            receiver_id: r.receiver_id.to_string(),
            receipt_id: Some(CryptoHash::from(r.receipt_id)),
            receipt: match r.receipt {
                ReceiptEnumView::Action {
                    signer_id,
                    signer_public_key,
                    gas_price,
                    output_data_receivers,
                    input_data_ids,
                    actions,
                    ..
                } => Some(receipt::Receipt::Action {
                    0: ReceiptAction {
                        signer_id: signer_id.to_string(),
                        signer_public_key: Some(PublicKey::from(signer_public_key)),
                        gas_price: Some(BigInt::from(gas_price.as_yoctonear())),
                        output_data_receivers: output_data_receivers
                            .into_iter()
                            .map(DataReceiver::from)
                            .collect(),
                        input_data_ids: input_data_ids
                            .into_iter()
                            .map(|i| CryptoHash::from(i))
                            .collect(),
                        actions: actions.into_iter().map(Action::from).collect(),
                    },
                }),
                ReceiptEnumView::Data { data_id, data, .. } => Some(receipt::Receipt::Data {
                    0: ReceiptData {
                        data_id: Some(CryptoHash::from(data_id)),
                        data: data.unwrap_or(vec![]),
                    },
                }),
                ReceiptEnumView::GlobalContractDistribution {
                    id,
                    target_shard,
                    already_delivered_shards,
                    code,
                    nonce: _,
                } => Some(receipt::Receipt::GlobalContractDistribution {
                    0: ReceiptGlobalContractDistribution {
                        id: Some(match id {
                            GlobalContractIdentifier::CodeHash(hash) => {
                                receipt_global_contract_distribution::Id::CodeHash(hash.into())
                            }
                            GlobalContractIdentifier::AccountId(id) => {
                                receipt_global_contract_distribution::Id::AccountId(id.into())
                            }
                        }),
                        target_shard: target_shard.into(),
                        already_delivered_shards: already_delivered_shards
                            .iter()
                            .map(|s| (*s).into())
                            .collect(),
                        code: code.into(),
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
            receiver_id: d.receiver_id.to_string(),
        }
    }
}

impl From<near_indexer::IndexerChunkView> for IndexerChunk {
    fn from(s: near_indexer::IndexerChunkView) -> Self {
        IndexerChunk {
            author: s.author.to_string(),
            header: Some(ChunkHeader::from(s.header)),
            transactions: s
                .transactions
                .into_iter()
                .map(IndexerTransactionWithOutcome::from)
                .collect(),
            receipts: s.receipts.into_iter().map(Receipt::from).collect(),
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
            execution_outcome: Some(ExecutionOutcomeWithId::from(o.execution_outcome)),
            receipt: match o.receipt {
                None => None,
                Some(r) => Some(Receipt::from(r)),
            },
        }
    }
}
impl From<near_views::ExecutionOutcomeWithIdView> for ExecutionOutcomeWithId {
    fn from(o: near_views::ExecutionOutcomeWithIdView) -> Self {
        ExecutionOutcomeWithId {
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
            gas_burnt: o.gas_burnt.as_gas(),
            tokens_burnt: Some(BigInt::from(o.tokens_burnt.as_yoctonear())),
            executor_id: o.executor_id.to_string(),
            status: Some(execution_outcome::Status::from(o.status)),
            metadata: match o.metadata {
                ExecutionMetadataView { .. } => ExecutionMetadata::V1.into(),
            },
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
                0: SuccessValueExecutionStatus { value: v.into() },
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
                                                0: AccountAlreadyExistsErrorKind {
                                                    account_id: account_id.to_string(),
                                                },
                                            }
                                        }
                                        ActionErrorKind::AccountDoesNotExist { account_id } => {
                                            action_error::Kind::AccountDoesNotExist {
                                                0: AccountDoesNotExistErrorKind {
                                                    account_id: account_id.to_string(),
                                                },
                                            }
                                        }
                                        ActionErrorKind::CreateAccountOnlyByRegistrar {
                                            account_id,
                                            registrar_account_id,
                                            predecessor_id,
                                        } => action_error::Kind::CreateAccountOnlyByRegistrar {
                                            0: CreateAccountOnlyByRegistrarErrorKind {
                                                account_id: account_id.to_string(),
                                                registrar_account_id: registrar_account_id
                                                    .to_string(),
                                                predecessor_id: predecessor_id.to_string(),
                                            },
                                        },
                                        ActionErrorKind::CreateAccountNotAllowed {
                                            account_id,
                                            predecessor_id,
                                        } => action_error::Kind::CreateAccountNotAllowed {
                                            0: CreateAccountNotAllowedErrorKind {
                                                account_id: account_id.to_string(),
                                                predecessor_id: predecessor_id.to_string(),
                                            },
                                        },
                                        ActionErrorKind::ActorNoPermission {
                                            account_id,
                                            actor_id,
                                        } => action_error::Kind::ActorNoPermission {
                                            0: ActorNoPermissionErrorKind {
                                                account_id: account_id.to_string(),
                                                actor_id: actor_id.to_string(),
                                            },
                                        },
                                        ActionErrorKind::DeleteKeyDoesNotExist {
                                            account_id,
                                            public_key,
                                        } => action_error::Kind::DeleteKeyDoesNotExist {
                                            0: DeleteKeyDoesNotExistErrorKind {
                                                account_id: account_id.to_string(),
                                                public_key: Some(PublicKey::from(*public_key)),
                                            },
                                        },
                                        ActionErrorKind::AddKeyAlreadyExists {
                                            account_id,
                                            public_key,
                                        } => action_error::Kind::AddKeyAlreadyExists {
                                            0: AddKeyAlreadyExistsErrorKind {
                                                account_id: account_id.to_string(),
                                                public_key: Some(PublicKey::from(*public_key)),
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
                                                account_id: account_id.to_string(),
                                                balance: Some(BigInt::from(amount.as_yoctonear())),
                                            },
                                        },
                                        ActionErrorKind::TriesToUnstake { account_id } => {
                                            action_error::Kind::TriesToUnstake {
                                                0: TriesToUnstakeErrorKind {
                                                    account_id: account_id.to_string(),
                                                },
                                            }
                                        }
                                        ActionErrorKind::TriesToStake {
                                            account_id,
                                            stake,
                                            locked,
                                            balance,
                                        } => action_error::Kind::TriesToStake {
                                            0: TriesToStakeErrorKind {
                                                account_id: account_id.to_string(),
                                                stake: Some(BigInt::from(stake.as_yoctonear())),
                                                locked: Some(BigInt::from(locked.as_yoctonear())),
                                                balance: Some(BigInt::from(balance.as_yoctonear())),
                                            },
                                        },
                                        ActionErrorKind::InsufficientStake {
                                            account_id,
                                            stake,
                                            minimum_stake,
                                        } => action_error::Kind::InsufficientStake {
                                            0: InsufficientStakeErrorKind {
                                                account_id: account_id.to_string(),
                                                stake: Some(BigInt::from(stake.as_yoctonear())),
                                                minimum_stake: Some(BigInt::from(minimum_stake.as_yoctonear())),
                                            },
                                        },
                                        ActionErrorKind::FunctionCallError(fce) => {
                                            action_error::Kind::FunctionCall {
                                                0: FunctionCallErrorKind { error: match fce {
                                                    near_errors::FunctionCallError::CompilationError(_) => {
                                                        FunctionCallErrorSer::CompilationError.into()
                                                    }
                                                    near_errors::FunctionCallError::LinkError { .. } => {
                                                        FunctionCallErrorSer::LinkError.into()
                                                    }
                                                    near_errors::FunctionCallError::MethodResolveError(_) => {
                                                        FunctionCallErrorSer::MethodResolveError.into()
                                                    }
                                                    near_errors::FunctionCallError::WasmTrap(_) => {
                                                        FunctionCallErrorSer::WasmTrap.into()
                                                    }
                                                    near_errors::FunctionCallError::WasmUnknownError => {
                                                        FunctionCallErrorSer::WasmUnknownError.into()
                                                    }
                                                    near_errors::FunctionCallError::HostError(_) => {
                                                        FunctionCallErrorSer::HostError.into()
                                                    }
                                                    near_errors::FunctionCallError::ExecutionError(_) => {
                                                        FunctionCallErrorSer::ExecutionError.into()
                                                    }
                                                    near_errors::FunctionCallError::_EVMError => {
                                                        FunctionCallErrorSer::EvmError.into()
                                                    }
                                                } },
                                            }
                                        }
                                        ActionErrorKind::NewReceiptValidationError(rve) => {
                                            action_error::Kind::NewReceiptValidation {
                                                0: NewReceiptValidationErrorKind { error: match rve {
                                                    near_errors::ReceiptValidationError::InvalidPredecessorId { .. } => {
                                                        ReceiptValidationError::InvalidPredecessorId.into()
                                                    }
                                                    near_errors::ReceiptValidationError::InvalidReceiverId { .. } => {
                                                        ReceiptValidationError::InvalidReceiverAccountId.into()
                                                    }
                                                    near_errors::ReceiptValidationError::InvalidSignerId { .. } => {
                                                        ReceiptValidationError::InvalidSignerAccountId.into()
                                                    }
                                                    near_errors::ReceiptValidationError::InvalidDataReceiverId { .. } => {
                                                        ReceiptValidationError::InvalidDataReceiverId.into()
                                                    }
                                                    near_errors::ReceiptValidationError::ReturnedValueLengthExceeded { .. } => {
                                                        ReceiptValidationError::ReturnedValueLengthExceeded.into()
                                                    }
                                                    near_errors::ReceiptValidationError::NumberInputDataDependenciesExceeded { .. } => {
                                                        ReceiptValidationError::NumberInputDataDependenciesExceeded.into()
                                                    }
                                                    near_errors::ReceiptValidationError::ActionsValidation(_) => {
                                                        ReceiptValidationError::ActionsValidationError.into()
                                                    }
                                                    near_primitives::errors::ReceiptValidationError::ReceiptSizeExceeded { .. } => {
                                                        ReceiptValidationError::ReceiptSizeExceeded.into()
                                                    }
                                                    near_primitives::errors::ReceiptValidationError::InvalidRefundTo { .. } => {
                                                        ReceiptValidationError::InvalidRefundTo.into()
                                                    }
                                                }}
                                            }
                                        }
                                        ActionErrorKind::OnlyImplicitAccountCreationAllowed {
                                            account_id,
                                        } => {
                                            action_error::Kind::OnlyImplicitAccountCreationAllowed {
                                                0: OnlyImplicitAccountCreationAllowedErrorKind {
                                                    account_id: account_id.to_string(),
                                                },
                                            }
                                        }
                                        ActionErrorKind::DeleteAccountWithLargeState {
                                            account_id,
                                        } => action_error::Kind::DeleteAccountWithLargeState {
                                            0: DeleteAccountWithLargeStateErrorKind {
                                                account_id: account_id.to_string(),
                                            },
                                        },
                                        ActionErrorKind::DelegateActionInvalidSignature => {
                                            action_error::Kind::DelegateActionInvalidSignature {
                                                0: Default::default(),
                                            }
                                        }
                                        ActionErrorKind::DelegateActionSenderDoesNotMatchTxReceiver {
                                            sender_id,
                                            receiver_id,
                                        } => {
                                            action_error::Kind::DelegateActionSenderDoesNotMatchTxReceiver {
                                                0: DelegateActionSenderDoesNotMatchTxReceiverKind {
                                                    sender_id: sender_id.to_string(),
                                                    receiver_id: receiver_id.to_string()
                                                },
                                            }
                                        }
                                        ActionErrorKind::DelegateActionExpired => {
                                            action_error::Kind::DelegateActionExpired {
                                                0: Default::default(),
                                            }
                                        }
                                        ActionErrorKind::DelegateActionAccessKeyError(_) => {
                                            action_error::Kind::DelegateActionAccessKeyError {
                                                0: Default::default(),
                                            }
                                        }
                                        ActionErrorKind::DelegateActionInvalidNonce { delegate_nonce, ak_nonce} => {
                                            action_error::Kind::DelegateActionInvalidNonce {
                                                0: DelegateActionInvalidNonceKind {
                                                    delegate_nonce: delegate_nonce.into(),
                                                    ak_nonce: ak_nonce.into(),
                                                },
                                            }
                                        }
                                        ActionErrorKind::DelegateActionNonceTooLarge { delegate_nonce, upper_bound } => {
                                            action_error::Kind::DelegateActionNonceTooLarge {
                                                0: DelegateActionNonceTooLargeKind {
                                                    delegate_nonce: delegate_nonce.into(),
                                                    upper_bound: upper_bound.into(),
                                                },
                                            }
                                        }
                                        ActionErrorKind::GlobalContractDoesNotExist { identifier } => {
                                            action_error::Kind::GlobalContractDoesNotExist {
                                                0: GlobalContractDoesNotExist {
                                                    identifier: Some(match identifier {
                                                        GlobalContractIdentifier::CodeHash(hash) => global_contract_does_not_exist::Identifier::CodeHash(hash.into()),
                                                        GlobalContractIdentifier::AccountId(id) => global_contract_does_not_exist::Identifier::AccountId(id.into()),
                                                    }),
                                                },

                                            }
                                        }
                                        ActionErrorKind::GasKeyDoesNotExist { .. }
                                        | ActionErrorKind::InsufficientGasKeyBalance { .. }
                                        | ActionErrorKind::GasKeyBalanceTooHigh { .. } => {
                                            unimplemented!("GasKey action error kinds are not supported yet")
                                        }
                                        ActionErrorKind::DelegateActionInvalidNonceIndex { .. } => {
                                            unimplemented!("DelegateActionInvalidNonceIndex is not supported yet")
                                        }
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
                                    near_primitives::errors::InvalidTxError::InvalidTransactionVersion {..} => {
                                        InvalidTxError::InvalidTransactionVersion.into()
                                    }
                                    near_primitives::errors::InvalidTxError::StorageError(_) => {
                                        InvalidTxError::StorageError.into()
                                    }
                                    near_primitives::errors::InvalidTxError::ShardCongested {..} => {
                                        InvalidTxError::ShardCongested.into()
                                    }
                                    near_primitives::errors::InvalidTxError::ShardStuck {..} => {
                                        InvalidTxError::ShardStuck.into()
                                    }
                                    near_primitives::errors::InvalidTxError::InvalidNonceIndex { .. }
                                    | near_primitives::errors::InvalidTxError::NotEnoughGasKeyBalance { .. }
                                    | near_primitives::errors::InvalidTxError::NotEnoughBalanceForDeposit { .. } => {
                                        unimplemented!("GasKey invalid tx error kinds are not supported yet")
                                    }
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
            signer_id: tx.signer_id.to_string(),
            public_key: Some(PublicKey::from(tx.public_key)),
            nonce: tx.nonce,
            receiver_id: tx.receiver_id.to_string(),
            actions: tx.actions.into_iter().map(|a| Action::from(a)).collect(),
            signature: Some(tx.signature.into()),
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
                    0: DeployContractAction { code: code.into() },
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
                        args: args.into(),
                        gas: gas.as_gas(),
                        deposit: Some(BigInt::from(deposit.as_yoctonear())),
                    },
                }),
            },
            near_views::ActionView::Transfer { deposit } => Action {
                action: Some(action::Action::Transfer {
                    0: TransferAction {
                        deposit: Some(BigInt::from(deposit.as_yoctonear())),
                    },
                }),
            },
            near_views::ActionView::Stake { stake, public_key } => Action {
                action: Some(action::Action::Stake {
                    0: StakeAction {
                        stake: Some(BigInt::from(stake.as_yoctonear())),
                        public_key: Some(PublicKey::from(public_key)),
                    },
                }),
            },
            near_views::ActionView::AddKey {
                public_key,
                access_key,
            } => Action {
                action: Some(action::Action::AddKey {
                    0: AddKeyAction {
                        public_key: Some(PublicKey::from(public_key)),
                        access_key: Some(AccessKey::from(access_key)),
                    },
                }),
            },
            near_views::ActionView::DeleteKey { public_key } => Action {
                action: Some(action::Action::DeleteKey {
                    0: DeleteKeyAction {
                        public_key: Some(PublicKey::from(public_key)),
                    },
                }),
            },
            near_views::ActionView::DeleteAccount { beneficiary_id } => Action {
                action: Some(action::Action::DeleteAccount {
                    0: DeleteAccountAction {
                        beneficiary_id: beneficiary_id.to_string(),
                    },
                }),
            },
            near_views::ActionView::Delegate {
                delegate_action,
                signature,
            } => Action {
                action: Some(action::Action::Delegate {
                    0: SignedDelegateAction {
                        delegate_action: Some(DelegateAction {
                            sender_id: delegate_action.sender_id.to_string(),
                            receiver_id: delegate_action.receiver_id.to_string(),
                            actions: delegate_action
                                .actions
                                .into_iter()
                                .map(|a| a.into())
                                .collect(),
                            nonce: delegate_action.nonce.into(),
                            max_block_height: delegate_action.max_block_height.into(),
                            public_key: Some(PublicKey::from(delegate_action.public_key)),
                        }),
                        signature: Some(signature.into()),
                    },
                }),
            },
            near_views::ActionView::DeployGlobalContract { code } => Action {
                action: Some(action::Action::DeployGlobalContract {
                    0: DeployGlobalContractAction { code: code.into() },
                }),
            },
            near_views::ActionView::DeployGlobalContractByAccountId { code } => Action {
                action: Some(action::Action::DeployGlobalContractByAccountId {
                    0: DeployGlobalContractByAccountIdAction { code: code.into() },
                }),
            },
            near_views::ActionView::UseGlobalContract { code_hash } => Action {
                action: Some(action::Action::UseGlobalContract {
                    0: UseGlobalContractAction {
                        code_hash: Some(CryptoHash::from(code_hash)),
                    },
                }),
            },
            near_views::ActionView::UseGlobalContractByAccountId { account_id } => Action {
                action: Some(action::Action::UseGlobalContractByAccountId {
                    0: UseGlobalContractByAccountIdAction {
                        account_id: account_id.to_string(),
                    },
                }),
            },
            near_views::ActionView::DeterministicStateInit {
                code,
                data,
                deposit,
            } => Action {
                action: Some(action::Action::DeterministicStateInit {
                    0: DeterministicStateInit {
                        code: Some(GlobalContractIdentifierView::from(code)),
                        data: data
                            .into_iter()
                            .map(|(k, v)| (general_purpose::STANDARD.encode(k), v))
                            .collect(),
                        deposit: Some(BigInt::from(deposit.as_yoctonear())),
                    },
                }),
            },
            near_views::ActionView::TransferToGasKey { .. }
            | near_views::ActionView::WithdrawFromGasKey { .. } => {
                unimplemented!("GasKey action view kinds are not supported yet")
            }
            near_views::ActionView::DelegateV2 { .. } => {
                unimplemented!("DelegateV2 action view is not supported yet")
            }
        }
    }
}

impl From<near_indexer::near_primitives::action::delegate::NonDelegateAction> for Action {
    fn from(value: near_primitives::action::delegate::NonDelegateAction) -> Self {
        let near_act: near_primitives::transaction::Action = value.into();
        let near_act_view: near_views::ActionView = near_act.into();
        near_act_view.into()
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
                            Some(a) => Some(BigInt::from(a.as_yoctonear())),
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
            near_views::AccessKeyPermissionView::GasKeyFunctionCall { .. }
            | near_views::AccessKeyPermissionView::GasKeyFullAccess { .. } => {
                unimplemented!("GasKey access key permission view kinds are not supported yet")
            }
        }
    }
}

impl From<near_views::ChunkHeaderView> for ChunkHeader {
    fn from(ch: near_views::ChunkHeaderView) -> Self {
        let validator_proposals = ch.validator_proposals;

        ChunkHeader {
            chunk_hash: Vec::from(ch.chunk_hash),
            prev_block_hash: Vec::from(ch.prev_block_hash),
            outcome_root: Vec::from(ch.outcome_root),
            prev_state_root: Vec::from(ch.prev_state_root),
            encoded_merkle_root: Vec::from(ch.encoded_merkle_root),
            encoded_length: ch.encoded_length,
            height_created: ch.height_created,
            height_included: ch.height_included,
            shard_id: ch.shard_id.into(),
            gas_used: ch.gas_used.as_gas(),
            gas_limit: ch.gas_limit.as_gas(),
            validator_reward: Some(BigInt::from(ch.validator_reward.as_yoctonear())),
            balance_burnt: Some(BigInt::from(ch.balance_burnt.as_yoctonear())),
            outgoing_receipts_root: Vec::from(ch.outgoing_receipts_root),
            tx_root: Vec::from(ch.tx_root),
            validator_proposals: validator_proposals
                .into_iter()
                .map(ValidatorStake::from)
                .collect(),
            signature: Some(ch.signature.into()),
        }
    }
}

impl From<Box<NearSignature>> for Signature {
    fn from(sig: Box<NearSignature>) -> Self {
        match *sig {
            NearSignature::ED25519(s) => Signature {
                r#type: CurveKind::Ed25519.into(),
                bytes: Vec::from(s.to_bytes()),
            },
            NearSignature::SECP256K1(s) => {
                let data = Vec::from(<[u8; 65]>::from(s));
                Signature {
                    r#type: CurveKind::Secp256k1.into(),
                    bytes: data,
                }
            }
            NearSignature::MLDSA65(s) => Signature {
                r#type: CurveKind::Mldsa65.into(),
                bytes: s.0.to_vec(),
            },
        }
    }
}

impl From<NearSignature> for Signature {
    fn from(sig: NearSignature) -> Self {
        match sig {
            NearSignature::ED25519(s) => Signature {
                r#type: CurveKind::Ed25519.into(),
                bytes: Vec::from(s.to_bytes()),
            },
            NearSignature::SECP256K1(s) => {
                let data = Vec::from(<[u8; 65]>::from(s));
                Signature {
                    r#type: CurveKind::Secp256k1.into(),
                    bytes: data,
                }
            }
            NearSignature::MLDSA65(s) => Signature {
                r#type: CurveKind::Mldsa65.into(),
                bytes: s.0.to_vec(),
            },
        }
    }
}

impl From<NearPublicKey> for PublicKey {
    fn from(key: NearPublicKey) -> Self {
        match key {
            NearPublicKey::ED25519(s) => PublicKey {
                r#type: CurveKind::Ed25519.into(),
                bytes: s.0.into(),
            },
            NearPublicKey::SECP256K1(s) => PublicKey {
                r#type: CurveKind::Secp256k1.into(),
                bytes: s.as_ref().into(),
            },
            NearPublicKey::MLDSA65(s) => PublicKey {
                r#type: CurveKind::Mldsa65.into(),
                bytes: s.0.to_vec(),
            },
        }
    }
}

impl From<near_primitives::challenge::SlashedValidator> for SlashedValidator {
    fn from(sv: near_primitives::challenge::SlashedValidator) -> Self {
        SlashedValidator {
            account_id: sv.account_id.to_string(),
            is_double_sign: sv.is_double_sign,
        }
    }
}

impl From<near_primitives::views::validator_stake_view::ValidatorStakeView> for ValidatorStake {
    fn from(sv: near_primitives::views::validator_stake_view::ValidatorStakeView) -> Self {
        match sv {
            near_primitives::views::validator_stake_view::ValidatorStakeView::V1(v) => {
                ValidatorStake {
                    account_id: v.account_id.to_string(),
                    public_key: Some(PublicKey::from(v.public_key)),
                    stake: Some(BigInt::from(v.stake.as_yoctonear())),
                }
            }
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

impl From<near_primitives::hash::CryptoHash> for CryptoHash {
    fn from(h: near_primitives::hash::CryptoHash) -> Self {
        CryptoHash {
            bytes: Vec::from(h),
        }
    }
}

impl Display for CryptoHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.bytes))
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let header = self.header.as_ref().unwrap();

        write!(f, "#{} ({})", header.height, header.hash.as_ref().unwrap())
    }
}

impl From<near_primitives::views::GlobalContractIdentifierView> for GlobalContractIdentifierView {
    fn from(view: near_primitives::views::GlobalContractIdentifierView) -> Self {
        match view {
            near_primitives::views::GlobalContractIdentifierView::CodeHash(hash) => {
                GlobalContractIdentifierView {
                    identifier: Some(global_contract_identifier_view::Identifier::CodeHash(
                        CryptoHash::from(hash),
                    )),
                }
            }
            near_primitives::views::GlobalContractIdentifierView::AccountId(account_id) => {
                GlobalContractIdentifierView {
                    identifier: Some(global_contract_identifier_view::Identifier::AccountId(
                        account_id.to_string(),
                    )),
                }
            }
        }
    }
}
