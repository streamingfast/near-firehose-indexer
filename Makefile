.PHONY: build
build:
	cargo build

.PHONY: release
release:
	CARGO_PROFILE_RELEASE_CODEGEN_UNITS='1' CARGO_PROFILE_RELEASE_LTO='fat' CARGO_BUILD_RUSTFLAGS='-D warnings' NEAR_RELEASE_BUILD='release' cargo build --release

