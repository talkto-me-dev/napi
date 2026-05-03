#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x
. ../sh/pid.sh
lsof -ti:3456 | xargs kill -9 2>/dev/null || true

(
  for i in {1..100}; do
    if nc -z 127.0.0.1 3456; then
      open http://127.0.0.1:3456
      break
    fi
    sleep 3
  done
) &
export RUST_LOG=info

if ! command -v watchexec &>/dev/null; then
  cargo install watchexec-cli
fi
exec watchexec -r -e rs -- cargo run --release --example server --features=verify
