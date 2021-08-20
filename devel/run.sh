#!/usr/bin/env bash

ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

build=

main() {
  pushd "$ROOT" &> /dev/null

  while getopts "hb" opt; do
    case $opt in
      h) usage && exit 0;;
      b) build=true;;
      \?) usage_error "Invalid option: -$OPTARG";;
    esac
  done
  shift $((OPTIND-1))

  network="$1"; shift
  if [[ $network == "" ]]; then
    usage_error "parameter <network> is required"
  fi

  if [[ ! -d "$network" ]]; then
    usage_error "Invalid network"
  fi

  if [[ $build == true ]]; then
    set -e
    cargo build
    set +e
  fi

  near_dm_indexer_bin=${NEAR_DM_INDEXER_BIN:-"../target/debug/near-dm-indexer"}

  config_dir="$network"
  run_dir="run/$network"

  rm -rf "$run_dir" &> /dev/null
  mkdir -p "$run_dir"

  cp "$config_dir/genesis.json" "$run_dir/"
  cp "$config_dir/config.json" "$run_dir/"
  cp "$config_dir/node_key.json" "$run_dir/"

  RUST_BACKTRACE=1 "$near_dm_indexer_bin" -h "$ROOT/$run_dir" run
}

usage_error() {
  message="$1"
  exit_code="$2"

  echo "ERROR: $message"
  echo ""
  usage
  exit ${exit_code:-1}
}

usage() {
  echo "usage: run.sh <network>"
  echo ""
  echo "Launch NEAR indexer against the given <network>, the <network> must be one of"
  echo "the available repository which contains files required to connect to desired"
  echo "network."
  echo ""
  echo "Options"
  echo "    -b          Build target binary prior running actual syncing phase"
  echo "    -h          Display help about this script"
}

main "$@"