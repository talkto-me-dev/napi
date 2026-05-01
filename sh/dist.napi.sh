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

# 获取更新后的版本号
VERSION=$(node -p "require('./package.json').version")

# 手动同步版本号到 optionalDependencies 和 Cargo.toml
# napi version 在没有 npm 目录时可能不会更新主 package.json
node -e "
const fs = require('fs');
const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8'));
const version = pkg.version;

if (pkg.optionalDependencies) {
  for (const name in pkg.optionalDependencies) {
    if (name.startsWith('@3-/') || name.startsWith('@napi-rs/')) {
      pkg.optionalDependencies[name] = version;
    }
  }
  fs.writeFileSync('package.json', JSON.stringify(pkg, null, 2) + '\n');
}

let cargo = fs.readFileSync('Cargo.toml', 'utf8');
cargo = cargo.replace(/^version = \".*\"/m, 'version = \"' + version + '\"');
fs.writeFileSync('Cargo.toml', cargo);
"

# 2. 提交并推送更改
cd "$DIR"
git add "$PROJECT"

# 使用 [skip ci] 避免触发其他工作流
git commit -m "chore($PROJECT): bump version to $VERSION [skip ci]"
git push

# 3. 手动触发 GitHub Action
gh workflow run "$PROJECT-publish.yml"
