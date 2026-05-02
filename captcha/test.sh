#!/usr/bin/env bash
set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x

bun run build:debug

bun run test
