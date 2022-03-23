# NEAR Deepmind Indexer

Near Deepmind Indexer is an application that leverages [Near Indexer's micro-framework](https://github.com/near/nearcore/tree/master/chain/indexer) to output protobuf files

## Requirements

Before you proceed, make sure you have the following software installed:
* [rustup](https://rustup.rs/) or Rust version that is mentioned in `rust-toolchain` file in the root of nearcore project.

## Configuring NEAR Deepmind Indexer

To run the NEAR Deepmind Indexer connected to a network we need to have configs and keys prepopulated. The configs and keys must be placed in a home directory, which has the following structure:

homedir
├── config.json     *required - node configuration file*
├── genesis.json    *required - node genesis configuration file*
├── node_key.json   *required - node node-key file*
├── data
│   └── ...         *optional - you could extra your snapshot here to start the process quicker*

### Localnet

To generate configs for localnet do the following:

```bash
$ git clone git@github.com:nearprotocol/nearcore.git
$ cd nearcore/tools/indexer/example
$ cargo run --release -- --home-dir ~/.near/localnet init
```

### Testnet / Betanet

To generate configs for testnet / betanet do the following

```bash
$ cargo run --release -- --home-dir ~/.near/testnet init --chain-id testnet --download
```

The above code will download the official genesis config and generate necessary configs. You can replace `testnet` in the command above to different network ID `betanet`.

To run the NEAR Indexer connected to testnet or betanet we need to have configs and keys prepopulated, you can get them with the NEAR Indexer Example like above with a little change. Follow the instructions below to run non-validating node (leaving account ID empty).

```bash
$ cargo run --release -- --home-dir ~/.near/testnet init --chain-id testnet --download
```

```bash
$ git clone git@github.com:nearprotocol/nearcore.git
$ cd nearcore/tools/indexer/example
$ cargo run --release -- --home-dir ~/.near/localnet init
```

```
$ cargo run --release -- --home-dir ~/.near/localnet init

near-dm-indexer --home-dir ./near-home-mainnet run
```

### Protobuf Definitions

The protobuf definitions in this project requires you to have

- https://github.com/streamingfast/proto
- https://github.com/streamingfast/proto-near

Cloned as sibling of this project. So that ultimately, all three projects
are part of parent folder all at the same depth.

To re-generate the Rust bindings of the definitions, use:

```
cargo run -p deepmind-pb
```
