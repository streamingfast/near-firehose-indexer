#[path = "sf.near.codec.v1.rs"]
mod codec;

pub use codec::*;
use near_crypto;
use near_indexer::near_primitives;
use near_indexer::near_primitives::errors as near_errors;
use near_indexer::near_primitives::errors::ActionErrorKind;
use near_indexer::near_primitives::views as near_views;
use near_indexer::near_primitives::views::{
    DataReceiverView, ExecutionMetadataView, ExecutionStatusView, ReceiptEnumView,
};
use near_indexer::StreamerMessage;

use hex;
use std::fmt::{Display, Formatter};

use self::failure_execution_status::Failure;

impl From<near_indexer::StreamerMessage> for Block {
    fn from(sm: StreamerMessage) -> Self {
        Block {
            header: Some(BlockHeader::from(sm.block.header)),
            shards: sm.shards.map_into(),
            author: sm.block.author.into(),
            chunk_headers: sm.block.chunks.map_into(),
            state_changes: vec![],
        }
    }
}

impl From<near_views::BlockHeaderView> for BlockHeader {
    fn from(h: near_views::BlockHeaderView) -> Self {
        BlockHeader {
            hash: Some(CryptoHash::from(h.hash)),
            height: h.height,
            prev_hash: Some(CryptoHash::from(h.prev_hash)),
            timestamp_nanosec: h.timestamp_nanosec,
            prev_height: h.prev_height.unwrap_or(0),
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
            validator_proposals: h.validator_proposals.map_into(),
            chunk_mask: h.chunk_mask,
            gas_price: Some(BigInt::from(h.gas_price)),
            block_ordinal: 0, //todo: this is v3 feature, what that means?
            total_supply: Some(BigInt::from(h.total_supply)),
            challenges_result: h.challenges_result.map_into(),
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
                // I think it's wrong to `filter_map` here. Indeed, the original `approvals` is
                // `Vec<Option<Signature>>` which means it can contain `None` values. If the original
                // contains `None`, why would we decide to filter them out exactly using `filter_map`,
                // that seems like the wrong behavior the transformed array could have less value than
                // the original one ...
                .filter_map(|s| s.map(Into::into))
                .collect(),
            signature: Some(h.signature.into()),
            latest_protocol_version: h.latest_protocol_version,
        }
    }
}

impl From<near_indexer::IndexerShard> for IndexerShard {
    fn from(is: near_indexer::IndexerShard) -> Self {
        IndexerShard {
            shard_id: is.shard_id,
            chunk: is.chunk.map(Into::into),
            receipt_execution_outcomes: is.receipt_execution_outcomes.map_into(),
        }
    }
}

impl From<near_indexer::IndexerExecutionOutcomeWithReceipt> for IndexerExecutionOutcomeWithReceipt {
    fn from(r: near_indexer::IndexerExecutionOutcomeWithReceipt) -> Self {
        IndexerExecutionOutcomeWithReceipt {
            execution_outcome: Some(r.execution_outcome.into()),
            receipt: Some(Receipt::from(r.receipt)),
        }
    }
}

impl From<near_views::ReceiptView> for Receipt {
    fn from(r: near_views::ReceiptView) -> Self {
        Receipt {
            predecessor_id: r.predecessor_id.into(),
            receiver_id: r.receiver_id.into(),
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
                        signer_id: signer_id.into(),
                        signer_public_key: Some(PublicKey::from(signer_public_key)),
                        gas_price: Some(BigInt::from(gas_price)),
                        output_data_receivers: output_data_receivers.map_into(),
                        input_data_ids: input_data_ids.map_into(),
                        actions: actions.map_into(),
                    },
                }),
                ReceiptEnumView::Data { data_id, data } => Some(receipt::Receipt::Data {
                    0: ReceiptData {
                        data_id: Some(CryptoHash::from(data_id)),
                        data: data.unwrap_or_else(|| vec![]),
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
            receiver_id: d.receiver_id.into(),
        }
    }
}

impl From<near_indexer::IndexerChunkView> for IndexerChunk {
    fn from(s: near_indexer::IndexerChunkView) -> Self {
        IndexerChunk {
            author: s.author.into(),
            header: Some(ChunkHeader::from(s.header)),
            transactions: s.transactions.map_into(),
            receipts: s.receipts.map_into(),
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
            receipt: o.receipt.map(Into::into),
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
            receipt_ids: o.receipt_ids.map_into(),
            gas_burnt: o.gas_burnt,
            tokens_burnt: Some(BigInt::from(o.tokens_burnt)),
            executor_id: o.executor_id.into(),
            status: Some(execution_outcome::Status::from(o.status)),

            // There is a problem here, metadata as now version and `gas_profile` but we do nothing with them ...
            metadata: match o.metadata {
                ExecutionMetadataView { .. } => ExecutionMetadata::V1.into(),
            },
        }
    }
}

impl From<near_views::ExecutionStatusView> for execution_outcome::Status {
    fn from(s: near_views::ExecutionStatusView) -> Self {
        match s {
            ExecutionStatusView::Unknown => {
                execution_outcome::Status::Unknown(UnknownExecutionStatus {})
            }
            ExecutionStatusView::SuccessValue(v) => {
                execution_outcome::Status::SuccessValue(SuccessValueExecutionStatus { value: v })
            }
            ExecutionStatusView::SuccessReceiptId(v) => {
                execution_outcome::Status::SuccessReceiptId(SuccessReceiptIdExecutionStatus {
                    id: Some(CryptoHash::from(v)),
                })
            }
            ExecutionStatusView::Failure(tx_err) => {
                execution_outcome::Status::Failure(FailureExecutionStatus {
                    failure: Some(tx_err.into()),
                })
            }
        }
    }
}

impl From<near_primitives::errors::TxExecutionError> for Failure {
    fn from(err: near_primitives::errors::TxExecutionError) -> Self {
        match err {
            near_errors::TxExecutionError::ActionError(ae) => {
                failure_execution_status::Failure::ActionError(
                    ActionError {
                        index: ae.index.unwrap_or(0),
                        kind: Some(match ae.kind {
                            ActionErrorKind::AccountAlreadyExists { account_id } => {
                                action_error::Kind::AccountAlreadyExist(
                                    AccountAlreadyExistsErrorKind {
                                        account_id: account_id.into(),
                                    },
                                )
                            }
                            ActionErrorKind::AccountDoesNotExist { account_id } => {
                                action_error::Kind::AccountDoesNotExist(
                                    AccountDoesNotExistErrorKind {
                                        account_id: account_id.into(),
                                    },
                                )
                            }
                            ActionErrorKind::CreateAccountOnlyByRegistrar {
                                account_id,
                                registrar_account_id,
                                predecessor_id,
                            } => action_error::Kind::CreateAccountOnlyByRegistrar(
                                CreateAccountOnlyByRegistrarErrorKind {
                                    account_id: account_id.into(),
                                    registrar_account_id: registrar_account_id
                                        .into(),
                                    predecessor_id: predecessor_id.into(),
                                },
                            ),
                            ActionErrorKind::CreateAccountNotAllowed {
                                account_id,
                                predecessor_id,
                            } => action_error::Kind::CreateAccountNotAllowed(
                                CreateAccountNotAllowedErrorKind {
                                    account_id: account_id.into(),
                                    predecessor_id: predecessor_id.into(),
                                },
                            ),
                            ActionErrorKind::ActorNoPermission {
                                account_id,
                                actor_id,
                            } => action_error::Kind::ActorNoPermission(
                                ActorNoPermissionErrorKind {
                                    account_id: account_id.into(),
                                    actor_id: actor_id.into(),
                                },
                            ),
                            ActionErrorKind::DeleteKeyDoesNotExist {
                                account_id,
                                public_key,
                            } => action_error::Kind::DeleteKeyDoesNotExist(
                                DeleteKeyDoesNotExistErrorKind {
                                    account_id: account_id.into(),
                                    public_key: Some(PublicKey::from(public_key)),
                                },
                            ),
                            ActionErrorKind::AddKeyAlreadyExists {
                                account_id,
                                public_key,
                            } => action_error::Kind::AddKeyAlreadyExists(
                                AddKeyAlreadyExistsErrorKind {
                                    account_id: account_id.into(),
                                    public_key: Some(PublicKey::from(public_key)),
                                },
                            ),
                            ActionErrorKind::DeleteAccountStaking { account_id } => {
                                action_error::Kind::DeleteAccountStaking(
                                    DeleteAccountStakingErrorKind {
                                        // This was `account_id: "".to_string()` before which I imagine was an error
                                        account_id: account_id.into(),
                                    },
                                )
                            }
                            ActionErrorKind::LackBalanceForState {
                                account_id,
                                amount,
                            } => action_error::Kind::LackBalanceForState(
                                LackBalanceForStateErrorKind {
                                    account_id: account_id.into(),
                                    balance: Some(BigInt::from(amount)),
                                },
                            ),
                            ActionErrorKind::TriesToUnstake { account_id } => {
                                action_error::Kind::TriesToUnstake(
                                    TriesToUnstakeErrorKind {
                                        account_id: account_id.into(),
                                    },
                                )
                            }
                            ActionErrorKind::TriesToStake {
                                account_id,
                                stake,
                                locked,
                                balance,
                            } => action_error::Kind::TriesToStake(
                                TriesToStakeErrorKind {
                                    account_id: account_id.into(),
                                    stake: Some(BigInt::from(stake)),
                                    locked: Some(BigInt::from(locked)),
                                    balance: Some(BigInt::from(balance)),
                                },
                            ),
                            ActionErrorKind::InsufficientStake {
                                account_id,
                                stake,
                                minimum_stake,
                            } => action_error::Kind::InsufficientStake(
                                InsufficientStakeErrorKind {
                                    account_id: account_id.into(),
                                    stake: Some(BigInt::from(stake)),
                                    minimum_stake: Some(BigInt::from(minimum_stake)),
                                },
                            ),
                            ActionErrorKind::FunctionCallError(fce) => {
                                action_error::Kind::FunctionCall(
                                    FunctionCallErrorKind { error: match fce {
                                        near_vm_errors::FunctionCallErrorSer::CompilationError(_) => {
                                            FunctionCallErrorSer::CompilationError as i32
                                        }
                                        near_vm_errors::FunctionCallErrorSer::LinkError { .. } => {
                                            FunctionCallErrorSer::LinkError as i32
                                        }
                                        near_vm_errors::FunctionCallErrorSer::MethodResolveError(_) => {
                                            FunctionCallErrorSer::MethodResolveError as i32
                                        }
                                        near_vm_errors::FunctionCallErrorSer::WasmTrap(_) => {
                                            FunctionCallErrorSer::WasmTrap as i32
                                        }
                                        near_vm_errors::FunctionCallErrorSer::WasmUnknownError => {
                                            FunctionCallErrorSer::WasmUnknownError as i32
                                        }
                                        near_vm_errors::FunctionCallErrorSer::HostError(_) => {
                                            FunctionCallErrorSer::HostError as i32
                                        }
                                        near_vm_errors::FunctionCallErrorSer::_EVMError => {
                                            FunctionCallErrorSer::EvmError as i32
                                        }
                                        near_vm_errors::FunctionCallErrorSer::ExecutionError(_) => {
                                            FunctionCallErrorSer::ExecutionError as i32
                                        }
                                    } },
                                )
                            }
                            ActionErrorKind::NewReceiptValidationError(rve) => {
                                action_error::Kind::NewReceiptValidation(
                                    NewReceiptValidationErrorKind { error: match rve {
                                        near_errors::ReceiptValidationError::InvalidPredecessorId { .. } => {
                                            ReceiptValidationError::InvalidPredecessorId as i32
                                        }
                                        near_errors::ReceiptValidationError::InvalidReceiverId { .. } => {
                                            ReceiptValidationError::InvalidReceiverAccountId as i32
                                        }
                                        near_errors::ReceiptValidationError::InvalidSignerId { .. } => {
                                            ReceiptValidationError::InvalidSignerAccountId as i32
                                        }
                                        near_errors::ReceiptValidationError::InvalidDataReceiverId { .. } => {
                                            ReceiptValidationError::InvalidDataReceiverId as i32
                                        }
                                        near_errors::ReceiptValidationError::ReturnedValueLengthExceeded { .. } => {
                                            ReceiptValidationError::ReturnedValueLengthExceeded as i32
                                        }
                                        near_errors::ReceiptValidationError::NumberInputDataDependenciesExceeded { .. } => {
                                            ReceiptValidationError::NumberInputDataDependenciesExceeded as i32
                                        }
                                        near_errors::ReceiptValidationError::ActionsValidation(_) => {
                                            ReceiptValidationError::ActionsValidationError as i32
                                        }
                                    }}
                                )
                            }
                            ActionErrorKind::OnlyImplicitAccountCreationAllowed {
                                account_id,
                            } => {
                                action_error::Kind::OnlyImplicitAccountCreationAllowed(
                                    OnlyImplicitAccountCreationAllowedErrorKind {
                                        account_id: account_id.into(),
                                    },
                                )
                            }
                            ActionErrorKind::DeleteAccountWithLargeState {
                                account_id,
                            } => action_error::Kind::DeleteAccountWithLargeState(
                                DeleteAccountWithLargeStateErrorKind {
                                    account_id: account_id.into(),
                                },
                            ),
                        }),
                    },
                )
            }
            near_errors::TxExecutionError::InvalidTxError(e) => {
                failure_execution_status::Failure::InvalidTxError {
                    0: match e {
                        near_errors::InvalidTxError::InvalidAccessKeyError(..) => {
                            InvalidTxError::InvalidAccessKeyError as i32
                        }
                        near_errors::InvalidTxError::InvalidSignerId { .. } => {
                            InvalidTxError::InvalidSignerId as i32
                        }
                        near_errors::InvalidTxError::SignerDoesNotExist { .. } => {
                            InvalidTxError::SignerDoesNotExist as i32
                        }
                        near_errors::InvalidTxError::InvalidNonce { .. } => {
                            InvalidTxError::InvalidNonce as i32
                        }
                        near_errors::InvalidTxError::NonceTooLarge { .. } => {
                            InvalidTxError::NonceTooLarge as i32
                        }
                        near_errors::InvalidTxError::InvalidReceiverId { .. } => {
                            InvalidTxError::InvalidReceiverId as i32
                        }
                        near_errors::InvalidTxError::InvalidSignature => {
                            InvalidTxError::InvalidSignature as i32
                        }
                        near_errors::InvalidTxError::NotEnoughBalance { .. } => {
                            InvalidTxError::NotEnoughBalance as i32
                        }
                        near_errors::InvalidTxError::LackBalanceForState { .. } => {
                            InvalidTxError::LackBalanceForState as i32
                        }
                        near_errors::InvalidTxError::CostOverflow => {
                            InvalidTxError::CostOverflow as i32
                        }
                        near_errors::InvalidTxError::InvalidChain => {
                            InvalidTxError::InvalidChain as i32
                        }
                        near_errors::InvalidTxError::Expired => InvalidTxError::Expired as i32,
                        near_errors::InvalidTxError::ActionsValidation(_) => {
                            InvalidTxError::ActionsValidation as i32
                        }
                        near_errors::InvalidTxError::TransactionSizeExceeded { .. } => {
                            InvalidTxError::TransactionSizeExceeded as i32
                        }
                    },
                }
            }
        }
    }
}

impl From<near_primitives::merkle::MerklePath> for MerklePath {
    fn from(p: near_primitives::merkle::MerklePath) -> Self {
        MerklePath { path: p.map_into() }
    }
}

impl From<near_primitives::merkle::MerklePathItem> for MerklePathItem {
    fn from(p: near_primitives::merkle::MerklePathItem) -> Self {
        Self {
            hash: Some(CryptoHash::from(p.hash)),
            direction: match p.direction {
                near_primitives::merkle::Direction::Left => 0,
                near_primitives::merkle::Direction::Right => 1,
            },
        }
    }
}

impl From<near_views::SignedTransactionView> for SignedTransaction {
    fn from(tx: near_views::SignedTransactionView) -> Self {
        SignedTransaction {
            signer_id: tx.signer_id.into(),
            public_key: Some(PublicKey::from(tx.public_key)),
            nonce: tx.nonce,
            receiver_id: tx.receiver_id.into(),
            actions: tx.actions.map_into(),
            signature: Some(tx.signature.into()),
            hash: Some(CryptoHash::from(tx.hash)),
        }
    }
}

impl From<near_views::ActionView> for Action {
    fn from(a: near_views::ActionView) -> Self {
        match a {
            near_views::ActionView::CreateAccount => Action {
                action: Some(action::Action::CreateAccount(CreateAccountAction {})),
            },
            near_views::ActionView::DeployContract { code } => Action {
                action: Some(action::Action::DeployContract(DeployContractAction {
                    code,
                })),
            },
            near_views::ActionView::FunctionCall {
                method_name,
                args,
                gas,
                deposit,
            } => Action {
                action: Some(action::Action::FunctionCall(FunctionCallAction {
                    method_name,
                    args,
                    gas,
                    deposit: Some(BigInt::from(deposit)),
                })),
            },
            near_views::ActionView::Transfer { deposit } => Action {
                action: Some(action::Action::Transfer(TransferAction {
                    deposit: Some(BigInt::from(deposit)),
                })),
            },
            near_views::ActionView::Stake { stake, public_key } => Action {
                action: Some(action::Action::Stake(StakeAction {
                    stake: Some(BigInt::from(stake)),
                    public_key: Some(PublicKey::from(public_key)),
                })),
            },
            near_views::ActionView::AddKey {
                public_key,
                access_key,
            } => Action {
                action: Some(action::Action::AddKey(AddKeyAction {
                    public_key: Some(PublicKey::from(public_key)),
                    access_key: Some(AccessKey::from(access_key)),
                })),
            },
            near_views::ActionView::DeleteKey { public_key } => Action {
                action: Some(action::Action::DeleteKey(DeleteKeyAction {
                    public_key: Some(PublicKey::from(public_key)),
                })),
            },
            near_views::ActionView::DeleteAccount { beneficiary_id } => Action {
                action: Some(action::Action::DeleteAccount(DeleteAccountAction {
                    beneficiary_id: beneficiary_id.into(),
                })),
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
        use access_key_permission::Permission;

        match p {
            near_views::AccessKeyPermissionView::FunctionCall {
                allowance,
                receiver_id,
                method_names,
            } => AccessKeyPermission {
                permission: Some(Permission::FunctionCall(FunctionCallPermission {
                    allowance: allowance.map(Into::into),
                    receiver_id,
                    method_names,
                })),
            },
            near_views::AccessKeyPermissionView::FullAccess => AccessKeyPermission {
                permission: Some(Permission::FullAccess(FullAccessPermission {})),
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
            validator_proposals: ch.validator_proposals.map_into(),
            signature: Some(ch.signature.into()),
        }
    }
}

impl From<near_crypto::signature::Signature> for Signature {
    fn from(sign: near_crypto::signature::Signature) -> Self {
        match sign {
            near_crypto::signature::Signature::ED25519(s) => Signature {
                r#type: CurveKind::Ed25519 as i32,
                bytes: Vec::from(s.to_bytes()),
            } as Signature,
            near_crypto::signature::Signature::SECP256K1(s) => {
                let data = Vec::from(<[u8; 65]>::from(s));
                Signature {
                    r#type: CurveKind::Secp256k1 as i32,
                    bytes: data,
                }
            }
        }
    }
}

impl From<near_crypto::signature::PublicKey> for PublicKey {
    fn from(key: near_crypto::signature::PublicKey) -> Self {
        match key {
            near_crypto::signature::PublicKey::ED25519(s) => {
                let data = Vec::from(<[u8; 32]>::from(s));
                PublicKey {
                    r#type: CurveKind::Ed25519 as i32,
                    bytes: data,
                }
            }
            near_crypto::signature::PublicKey::SECP256K1(s) => {
                let data = Vec::from(<[u8; 64]>::from(s));
                PublicKey {
                    r#type: CurveKind::Secp256k1 as i32,
                    bytes: data,
                }
            }
        }
    }
}

impl From<near_primitives::challenge::SlashedValidator> for SlashedValidator {
    fn from(sv: near_primitives::challenge::SlashedValidator) -> Self {
        SlashedValidator {
            account_id: sv.account_id.into(),
            is_double_sign: sv.is_double_sign,
        }
    }
}

impl From<near_primitives::views::validator_stake_view::ValidatorStakeView> for ValidatorStake {
    fn from(sv: near_primitives::views::validator_stake_view::ValidatorStakeView) -> Self {
        match sv {
            near_primitives::views::validator_stake_view::ValidatorStakeView::V1(v) => {
                ValidatorStake {
                    account_id: v.account_id.into(),
                    public_key: Some(PublicKey::from(v.public_key)),
                    stake: Some(BigInt::from(v.stake)),
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
        CryptoHash { bytes: h.into() }
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

trait MapInto<R, T: Into<R>> {
    /// Shortcut for `x.into_iter().map(Into::into).collect()`.
    fn map_into<I: FromIterator<R>>(self) -> I;
}

impl<R, T: Into<R>> MapInto<R, T> for Vec<T> {
    fn map_into<I: FromIterator<R>>(self) -> I {
        self.into_iter().map(Into::into).collect()
    }
}
