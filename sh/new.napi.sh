#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
ROOT=$(dirname $DIR)
cd $ROOT

if [ -z "$1" ]; then
  echo "用法: $0 <项目名称>"
  exit 1
fi

PROJECT=$1

if [ -d "$PROJECT" ]; then
  echo "错误: 目录 $PROJECT 已存在。"
  exit 1
fi

echo "正在创建新项目: $PROJECT"

# 同步模板到新项目目录
rsync -av --exclude='node_modules' --exclude='target' --exclude='.git' _tmpl/napi/ "$PROJECT/"

# 使用 JS 脚本替换模板字符串
node "$DIR/new.napi.js" "$PROJECT"

echo "完成！项目 $PROJECT 已创建。"
