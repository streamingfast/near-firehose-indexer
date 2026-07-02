ARG FIRENEAR_VERSION=v2.5.2
ARG FIRECORE_VERSION=v1.14.6

FROM ghcr.io/streamingfast/firehose-near:${FIRENEAR_VERSION} AS firenear
FROM ghcr.io/streamingfast/firehose-core:${FIRECORE_VERSION} AS firecore

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

# We start from firehose-core base so we get proper motd and reader scripts
FROM firecore

ENTRYPOINT [ "/app/firenear" ]

COPY --from=build /tmp/target/release/near-firehose-indexer /app/near-firehose-indexer
COPY --from=firenear /app/firenear /app/firenear
