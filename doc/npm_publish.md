# NPM 发布说明

项目已配置 GitHub Actions 自动发布，支持 **OIDC (Trusted Publisher)** 免 Token 发布。

## 1. 发布流程

1. **更新版本**: 在包目录下运行 `npm version patch` (或 minor/major)。
2. **推送代码**: `git push origin main`。
   Action 会自动完成编译、测试及 NPM 发布。

## 2. NPM OIDC 配置

在 NPM 包设置的 **Trusted Publishers** 中添加：

- **Organization**: `talkto-me-dev`
- **Repository**: `napi`
- **Workflow filename**: 对应的文件名（如 `captcha-publish.yml`）

## 3. 常见问题

- **避免循环触发**: 默认 `GITHUB_TOKEN` 推送不会触发 Action，或在 commit 中加 `[skip ci]`。
- **权限问题**: Action 已配置 `contents: write` 用于创建 GitHub Release。
- **多包支持**: 使用通用模板 `.github/workflows/napi-publish-template.yml`。

### 新包接入示例：

```yaml
jobs:
  napi-publish:
    uses: ./.github/workflows/napi-publish-template.yml
    with:
      working-directory: "your-pkg-name"
```
