ARG CORE_VERSION=latest

FROM ubuntu:20.04 as build

ENV TZ="Etc/UTC"
RUN apt-get update -qq
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y tzdata

RUN DEBIAN_FRONTEND=noninteractive apt-get install -y \
    git \
    cmake \
    g++ \
    pkg-config \
    libssl-dev \
    curl \
    llvm \
    clang \
    && rm -rf /var/lib/apt/lists/*

COPY ./rust-toolchain.toml /tmp/rust-toolchain.toml

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- -y --no-modify-path --default-toolchain none

COPY . .

RUN CARGO_TARGET_DIR=/tmp/target make release

RUN ls -la /tmp/target

FROM ubuntu:20.04

ENV TZ="Etc/UTC"
RUN apt-get update -qq
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y tzdata

RUN DEBIAN_FRONTEND=noninteractive \
       apt-get -y install -y \
       ca-certificates libssl-dev vim htop iotop sysstat wget \
       dstat strace lsof curl jq tzdata && \
       rm -rf /var/cache/apt /var/lib/apt/lists/*

RUN rm /etc/localtime && ln -snf /usr/share/zoneinfo/America/Montreal /etc/localtime && dpkg-reconfigure -f noninteractive tzdata

# s5cmd is a CLI tool to manipulate S3 store (Needed to sync NEAR Foundation backup(s))
RUN mkdir /tmp/s5cmd && \
  cd /tmp/s5cmd && \
  wget -O s5cmd.tar.gz https://github.com/peak/s5cmd/releases/download/v2.0.0/s5cmd_2.0.0_Linux-64bit.tar.gz && \
  tar -xzvf s5cmd.tar.gz && \
  cp s5cmd /usr/bin/ && \
  cd / && \
  rm -rf /tmp/s5cmd



COPY --from=build /tmp/target/near-firehose-indexer /app/neard
RUN chmod +x /app/near-firehose-indexer

ENV PATH "$PATH:/app"