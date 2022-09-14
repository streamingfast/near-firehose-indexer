FROM ubuntu:20.04

RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
       apt-get -y install -y \
       ca-certificates libssl1.1 vim htop iotop sysstat wget \
       dstat strace lsof curl jq tzdata && \
       rm -rf /var/cache/apt /var/lib/apt/lists/*

RUN rm /etc/localtime && ln -snf /usr/share/zoneinfo/America/Montreal /etc/localtime && dpkg-reconfigure -f noninteractive tzdata

RUN apt update && apt-get -y install ca-certificates libssl1.1 && rm -rf /var/cache/apt/*

# s5cmd is a CLI tool to manipulate S3 store (Needed to sync NEAR Foundation backup(s))
RUN mkdir /tmp/s5cmd && \
  cd /tmp/s5cmd && \
  wget -O s5cmd.tar.gz https://github.com/peak/s5cmd/releases/download/v1.4.0/s5cmd_1.4.0_Linux-64bit.tar.gz && \
  tar -xzvf s5cmd.tar.gz && \
  cp s5cmd /usr/bin/ && \
  cd / && \
  rm -rf /tmp/s5cmd


COPY near-dm-indexer-x86_64-unknown-linux-gnu /app/near-dm-indexer

ENV PATH "$PATH:/app"