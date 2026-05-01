#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
ROOT=$(dirname $DIR)
cd $ROOT
set -x

if [ -z "$1" ]; then
  echo "Usage: $0 <PROJECT>"
  exit 1
fi

PROJECT=$1

if [ ! -d "$PROJECT" ]; then
  echo "Project $PROJECT does not exist"
  exit 1
fi

# 1. 更新版本并同步依赖
cd "$PROJECT"

# 在 package.json 中增加版本号，不创建 git tag
npm version patch --no-git-tag-version

# 获取更新后的版本号
# Get the updated version number
VERSION=$(grep '"version":' package.json | head -n 1 | cut -d'"' -f4)

# 手动同步版本号到 optionalDependencies 和 Cargo.toml
# Manually sync version to optionalDependencies and Cargo.toml
bun "$DIR/dist.napi.js"

# 2. 提交并推送更改
cd "$ROOT"
git add "$PROJECT"

# 使用 [skip ci] 避免触发其他工作流
git commit -m "chore($PROJECT): bump version to $VERSION [skip ci]"
git push

# 3. 手动触发 GitHub Action
gh workflow run "$PROJECT-publish.yml"
