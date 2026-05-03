#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x
RUST_BACKTRACE=1 RUST_LOG=info cargo run --release --example main -F verify
