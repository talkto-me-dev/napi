#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x
(sleep 2 && open http://127.0.0.1:3456) &
RUST_LOG=info cargo run --release --example server
