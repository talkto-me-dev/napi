#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x
(sleep 2 && open http://127.0.0.1:3000) &
. ../sh/pid.sh
lsof -ti:3000 | xargs kill -9 2>/dev/null || true
export RUST_LOG=info

if ! command -v watchexec &> /dev/null; then
  cargo install watchexec-cli
fi
exec watchexec -r -e rs -- cargo run --release --example server
