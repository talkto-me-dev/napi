#!/usr/bin/env bash

# 自动发布 NAPI 项目脚本

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
ROOT=$(dirname $DIR)
cd $ROOT
set -x

# 检查并更新 npm 以支持 npm trust
if ! npm trust --help > /dev/null 2>&1; then
  echo "正在更新 npm 以支持 trust 命令..."
  npm install -g npm@latest
fi

if [ -z "$1" ]; then
  echo "用法: $0 <项目名称>"
  exit 1
fi

PROJECT=$1

if [ ! -d "$PROJECT" ]; then
  echo "项目 $PROJECT 不存在"
  exit 1
fi

# 1. 更新版本并同步依赖
cd "$PROJECT"

# 在 package.json 中增加版本号，不创建 git tag
npm version patch --no-git-tag-version

# 获取更新后的版本号
VERSION=$(node -p "require('./package.json').version")

# 手动同步版本号到 optionalDependencies 和 Cargo.toml
bun "$DIR/dist.napi.js"

# 2. 检查并初始化 npm 平台包 (仅初始化本地文件，不发布，发布由 CI 处理以避免版本冲突)
bun "$DIR/npm.init.js" --cwd . --trust

# 3. 提交并推送更改
cd "$ROOT"
git add "$PROJECT"

# 注意: 不能使用 [skip ci]，否则会导致后续的 tag push 也无法触发 publish workflow
git commit -m "chore($PROJECT): bump version to $VERSION"
git push
git tag "$PROJECT@$VERSION"
git push origin "$PROJECT@$VERSION"
