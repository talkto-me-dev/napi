#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
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

# 同步版本号到 Cargo.toml 并更新 optionalDependencies（平台特定包）
bun x napi version

# 2. 提交并推送更改
cd "$DIR"
git add "$PROJECT"
VERSION=$(grep '"version":' "$PROJECT/package.json" | head -n 1 | cut -d'"' -f4)

# 使用 [skip ci] 避免触发其他工作流
git commit -m "chore($PROJECT): bump version to $VERSION [skip ci]"
git push

# 3. 手动触发 GitHub Action
gh workflow run "$PROJECT-publish.yml"
