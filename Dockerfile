ARG FIRENEAR_VERSION=latest

FROM ghcr.io/streamingfast/firehose-near:${FIRENEAR_VERSION} AS firenear

FROM firenear AS rust-base

RUN apt-get update -qq && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    git cmake g++ pkg-config curl llvm clang libssl-dev

COPY ./rust-toolchain.toml ./rust-toolchain.toml

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain none && \
    export TOOLCHAIN_VERSION=$(cat rust-toolchain.toml | grep -oE 'channel\s+=\s+".*"' | cut -d'"' -f2) && \
    rustup toolchain install $TOOLCHAIN_VERSION-x86_64-unknown-linux-gnu

FROM rust-base AS build

COPY . .

RUN CARGO_TARGET_DIR=/tmp/target make release && \
    chmod +x /tmp/target/release/near-firehose-indexer

FROM firenear

COPY --from=build /tmp/target/release/near-firehose-indexer /app/near-firehose-indexer
