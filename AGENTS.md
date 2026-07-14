# Agent guidelines

## Codec: never `unimplemented!`

When a new NEAR type appears in `src/codec/mod.rs` (e.g. a new
`ActionView`, `ActionErrorKind`, `InvalidTxError`, or
`AccessKeyPermissionView` variant), do NOT write `unimplemented!` /
`todo!` / `panic!`. Those crash the indexer on the first real block
carrying the type.

Instead:

1. Add the corresponding message / enum value to the canonical protobuf
   at `sf/near/type/v1/type.proto` in the **firehose-near** repo. Look
   for it at `../firehose-near`; if it is not there, ask the user for its
   location. See PR
   https://github.com/streamingfast/firehose-near/pull/13 for the
   pattern (gas key + DelegateV2 types).
2. Regenerate the Go bindings there (`pb/generate.sh`) and open a PR.
3. Mirror the new proto types into the checked-in generated Rust file
   `src/codec/sf.near.r#type.v1.rs` (it is hand-maintained here, not
   generated at build time — the build only `#[path=...]` includes it).
4. Map the NEAR variant to the new proto type in `src/codec/mod.rs`.

Read the NEAR source for exact field types under the cargo git checkout
(`~/.cargo/git/checkouts/nearcore-*/<rev>/core/primitives/src/`).
