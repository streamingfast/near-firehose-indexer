## v2.14.0-rc.2-fh3.0

* Bumped to [2.14.0-rc.2](https://github.com/near/nearcore/releases/tag/2.14.0-rc.2).

* Bumped `rust-toolchain.toml` to `1.95`, matching the minimum supported Rust version nearcore 2.14.0 requires.

* Added codecs for the `UniversalStateInit` action and for the new `TotalPromiseInputSizeExceeded`, `ReceiptStorageProofSizeExceeded`, `MalformedUniversalStateInit` and `AccountNotInitialized` action error kinds.

* `IndexerConfig.skip_broken_blocks` is set to `false`. nearcore 2.14.0 added the field and gives it no default, so it has to be chosen explicitly. When `true`, a block whose `StreamerMessage` still fails to build after the retry budget is skipped, its height recorded as synced, and the stream resumes past it — leaving a permanent hole in the Firehose block stream that nothing goes back for. `false` panics instead, so the reader restarts and retries the same height. This is stricter than 2.13.4, which skipped such a height immediately and without retrying; nearcore's own reference indexer (`tools/indexer/example`) also uses `false`, while `true` exists for the `mirror` traffic generator.

* `DelegateV2` codecs are retained. nearcore 2.14.0 rejects the action at validation but keeps the type, and receipts created before the protocol upgrade still execute.

## v2.13.4-fh3.0

* Bumped to [2.13.4](https://github.com/near/nearcore/releases/tag/2.13.4).

## v2.13.3-fh3.0

* Bumped to [2.13.3](https://github.com/near/nearcore/releases/tag/2.13.3).

## v2.13.2-fh3.0

* Bumped to [2.13.2](https://github.com/near/nearcore/releases/tag/2.13.2).

## v2.13.1-fh3.0

* Bumped to [2.13.1](https://github.com/near/nearcore/releases/tag/2.13.1).

## v2.13.0-fh3.0

* Bumped to [2.13.0](https://github.com/near/nearcore/releases/tag/2.13.0).

## v2.13.0-rc.2-fh3.0-2

* implemented gas key codecs (actions, access key permissions, action/tx errors) and DelegateV2, replacing the `unimplemented!` panics

## v2.13.0-rc.2-fh3.0-1

* fixed unimplemented mldsa65 curve

## v2.13.0-rc.2-fh3.0

* Bumped to [2.13.0-rc.2](https://github.com/near/nearcore/releases/tag/2.13.0-rc.2).

## v2.13.0-rc.1-fh3.0

* Bumped to [2.13.0-rc.1](https://github.com/near/nearcore/releases/tag/2.13.0-rc.1).

## v2.12.0-fh3.0

* Bumped to [2.12.0](https://github.com/near/nearcore/releases/tag/2.12.0).

## v2.12.0-rc.2-fh3.0

* Bumped to [2.12.0-rc.2](https://github.com/near/nearcore/releases/tag/2.12.0-rc.2).

## v2.12.0-rc.1-fh3.0

* Bumped to [2.12.0-rc.1](https://github.com/near/nearcore/releases/tag/2.12.0-rc.1).

## v2.11.1-fh3.0

* Bumped to [2.11.1](https://github.com/near/nearcore/releases/tag/2.11.1).

## v2.11.0-fh3.0

* Bumped to [2.11.0](https://github.com/near/nearcore/releases/tag/2.11.0).

## v2.11.0-rc.5-fh3.0

* Bumped to [2.11.0-rc.5](https://github.com/near/nearcore/releases/tag/2.11.0-rc.5).

## v2.11.0-rc.4-fh3.0

* Bumped to [2.11.0-rc.4](https://github.com/near/nearcore/releases/tag/2.11.0-rc.4).

## v2.11.0-rc.3-fh3.0

* Bumped to [2.11.0-rc.3](https://github.com/near/nearcore/releases/tag/2.11.0-rc.3).

## v2.11.0-rc.2-fh3.0

* Bumped to [2.11.0-rc.2](https://github.com/near/nearcore/releases/tag/2.11.0-rc.2).

## v2.11.0-rc.1-fh3.0

* Bumped to [2.11.0-rc.1](https://github.com/near/nearcore/releases/tag/2.11.0-rc.1).

## v2.10.7-fh3.0

* Bumped to [2.10.7](https://github.com/near/nearcore/releases/tag/2.10.7).

## v2.10.6-fh3.0

* Bumped to [2.10.6](https://github.com/near/nearcore/releases/tag/2.10.6).

## v2.10.5-fh3.0

* Bumped to [2.10.5](https://github.com/near/nearcore/releases/tag/2.10.5).

## v2.10.4-fh3.0

* Bumped to [2.10.4](https://github.com/near/nearcore/releases/tag/2.10.4).

## v2.10.3-fh3.0

* Bumped to [2.10.3](https://github.com/near/nearcore/releases/tag/2.10.3).

## v2.10.1-fh3.0

* Bumped to [2.10.1](https://github.com/near/nearcore/releases/tag/2.10.1).

## v2.10.0-fh3.0

* Bumped to [2.10.0](https://github.com/near/nearcore/releases/tag/2.10.0).

## v2.10.0-rc.5-fh3.0

* Bumped to [2.10.0-rc.5](https://github.com/near/nearcore/releases/tag/2.10.0-rc.5).

## v2.10.0-rc.4-fh3.0

* Bumped to [2.10.0-rc.4](https://github.com/near/nearcore/releases/tag/2.10.0-rc.4).

## v2.9.0-fh3.0

* Bumped to [2.9.0](https://github.com/near/nearcore/releases/tag/2.9.0).

## v2.8.0-fh3.0

* Bumped to [2.8.0](https://github.com/near/nearcore/releases/tag/2.8.0).

## v2.8.0-rc.1-fh3.0

* Bumped to [2.8.0-rc.1](https://github.com/near/nearcore/releases/tag/2.8.0-rc.1).

## v2.7.1-fh3.0

* Bumped to [2.7.1](https://github.com/near/nearcore/releases/tag/2.7.1).

## v2.7.0-fh3.0

* Bumped to [2.7.0](https://github.com/near/nearcore/releases/tag/2.7.0).

## v2.7.0-rc.4-fh3.0-5

This is a *re-release* of `v2.7.0-rc.4-fh3.0-3` and `v2.7.0-rc.4-fh3.0-4` with CI build fix.

#### v2.7.0-rc.4-fh3.0-4

This is a *re-release* of `v2.7.0-rc.4-fh3.0-3` with CI build fix.

#### v2.7.0-rc.4-fh3.0-3

Release should now have correct binary attached now.

## v2.7.0-rc.4-fh3.0-2

This is a *re-release* of `v2.7.0-rc.4-fh3.0`

#### v2.7.0-rc.4-fh3.0-1

This is a *re-release* of `v2.7.0-rc.4-fh3.0`

#### v2.7.0-rc.4-fh3.0

* Docker image entrypoint is now `firenear` which should be how the indexer should be run anyway.

* Removed all `clone` by using move operations instead.

* Fixed release pipeline for new tag formats.

* Bumped to [2.7.0-rc.4](https://github.com/near/nearcore/releases/tag/2.7.0-rc.4).

## 1.37.0-rc.3-fire

CODE_COLOR: CODE_RED_TESTNET
RELEASE_VERSION: 1.37.0-rc.3
PROTOCOL_UPGRADE: FALSE
DATABASE_UPGRADE: FALSE
SECURITY_UPGRADE: TRUE

## 1.37.0-rc.2-fire

CODE_COLOR: CODE_GREEN_TESTNET
RELEASE_VERSION: 1.37.0-rc.2
PROTOCOL_UPGRADE: FALSE
DATABASE_UPGRADE: FALSE
SECURITY_UPGRADE: FALSE

## 1.37.0-rc.1-fire

CODE_COLOR: CODE_YELLOW_TESTNET
RELEASE_VERSION: 1.37.0-rc.1
PROTOCOL_UPGRADE: TRUE
DATABASE_UPGRADE: FALSE
SECURITY_UPGRADE: FALSE

## 1.36.4-fire

CODE_COLOR: CODE_RED_MAINNET
RELEASE_VERSION: 1.36.4
PROTOCOL_UPGRADE: FALSE
DATABASE_UPGRADE: FALSE
SECURITY_UPGRADE: TRUE

## 1.36.3-fire

CODE_COLOR: CODE_RED_MAINNET
RELEASE_VERSION: 1.36.3
PROTOCOL_UPGRADE: FALSE
DATABASE_UPGRADE: FALSE
SECURITY_UPGRADE: TRUE

## 1.36.2-fire

CODE_COLOR: CODE_RED_MAINNET
RELEASE_VERSION: 1.36.2
PROTOCOL_UPGRADE: FALSE
DATABASE_UPGRADE: FALSE
SECURITY_UPGRADE: TRUE

## 1.36.1-fire

CODE_COLOR: CODE_GREEN_MAINNET
RELEASE_VERSION: 1.36.1
PROTOCOL_UPGRADE: FALSE
DATABASE_UPGRADE: FALSE
SECURITY_UPGRADE: FALSE

## 1.36.0-fire

CODE_COLOR: CODE_YELLOW_MAINNET
RELEASE_VERSION: 1.36.0
PROTOCOL_UPGRADE: TRUE
DATABASE_UPGRADE: TRUE
SECURITY_UPGRADE: TRUE
