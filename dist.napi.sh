#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x

if [ -z "$1" ]; then
  echo "$0 <PROJECT>"
  exit 1
fi

PROJECT=$1
