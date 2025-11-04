# 统一发布指南

本项目使用统一的版本管理系统，一次发布可同时更新 TypeScript 和 Rust 两个 SDK。

## 🚀 快速发布

### 一键发布（推荐）

```bash
# 运行统一发布脚本
./scripts/release-all.sh

# 脚本会自动：
# 1. 更新 VERSION 文件
# 2. 同步版本到 TypeScript SDK (package.json)
# 3. 同步版本到 Rust SDK (Cargo.toml)
# 4. 运行所有测试
# 5. 创建 git tag
# 6. 推送到 GitHub
# 7. 触发自动发布到 npm 和 crates.io
```

## 📋 版本管理

### 版本文件位置

所有版本信息统一存储在：
- **主版本文件**: `VERSION` (根目录)
- **TypeScript SDK**: `typescript-sdk/package.json`
- **Rust SDK**: `rust-sdk/Cargo.toml`

### 版本同步

修改版本只需要编辑 `VERSION` 文件，然后运行：

```bash
./scripts/sync-version.sh
```

或者使用统一发布脚本（自动同步）：

```bash
./scripts/release-all.sh
```

## 🔖 标签规范

**统一标签格式**: `v*.*.*`

- ✅ `v1.0.0` - 同时发布 TypeScript 和 Rust SDK
- ✅ `v1.0.1` - 同时发布两个 SDK 的补丁版本
- ✅ `v2.0.0` - 同时发布两个 SDK 的主版本

**旧标签格式**（已废弃）:
- ❌ `rust-v1.0.0` - 不再使用
- ❌ TypeScript 单独标签 - 不再使用

## 📦 发布流程

### 自动化流程

```
开发者
  ↓
运行 ./scripts/release-all.sh
  ↓
选择版本类型 (patch/minor/major/custom)
  ↓
├─ 更新 VERSION 文件
├─ 同步到 package.json
├─ 同步到 Cargo.toml
├─ 运行 TypeScript 测试
├─ 运行 Rust 测试
├─ 构建 TypeScript SDK
└─ 提交版本更改
  ↓
创建 tag (v1.0.0)
  ↓
推送到 GitHub
  ↓
GitHub Actions 触发
  ↓
├─ TypeScript SDK → npm
│   ├─ 运行测试
│   ├─ 构建包
│   ├─ 发布到 @zen_tools/x-sdk
│   └─ 创建 GitHub Release
│
└─ Rust SDK → crates.io
    ├─ 运行测试
    ├─ 发布到 x-sdk
    └─ 创建 GitHub Release
  ↓
✅ 两个 SDK 同时发布完成
```

## 🔧 前置配置

### 1. GitHub Secrets

需要在 GitHub 仓库设置中配置：

- **NPM_TOKEN**: npm 发布 token (用于 @zen_tools/x-sdk)
- **CARGO_TOKEN**: crates.io 发布 token (用于 x-sdk)

### 2. npm Organization

确保你是 `zen_tools` organization 的成员：
- 访问: https://www.npmjs.com/settings/zen_tools/members

### 3. crates.io 账户

确保你有权限发布 `x-sdk` crate：
- 访问: https://crates.io/settings/tokens

## 📝 版本号规范

遵循 [Semantic Versioning](https://semver.org/):

| 版本类型 | 格式 | 说明 | 示例 |
|---------|------|------|------|
| **MAJOR** | x.0.0 | 不兼容的 API 变更 | 1.0.0 → 2.0.0 |
| **MINOR** | 0.x.0 | 向后兼容的功能新增 | 1.0.0 → 1.1.0 |
| **PATCH** | 0.0.x | 向后兼容的问题修复 | 1.0.0 → 1.0.1 |

## 🎯 使用示例

### 场景 1: 发布补丁版本（Bug 修复）

```bash
# 当前版本: 1.0.0
./scripts/release-all.sh

# 选择: 1) patch
# 新版本: 1.0.1
# 自动发布两个 SDK v1.0.1
```

### 场景 2: 发布新功能（Minor 版本）

```bash
# 当前版本: 1.0.1
./scripts/release-all.sh

# 选择: 2) minor
# 新版本: 1.1.0
# 自动发布两个 SDK v1.1.0
```

### 场景 3: 发布破坏性变更（Major 版本）

```bash
# 当前版本: 1.1.0
./scripts/release-all.sh

# 选择: 3) major
# 新版本: 2.0.0
# 自动发布两个 SDK v2.0.0
```

### 场景 4: 手动指定版本

```bash
./scripts/release-all.sh

# 选择: 4) custom
# 输入: 1.2.3-beta.1
# 发布预发布版本
```

## 🔍 验证发布

### 检查版本同步

```bash
# 查看主版本
cat VERSION

# 查看 TypeScript SDK 版本
cat typescript-sdk/package.json | grep version

# 查看 Rust SDK 版本
cat rust-sdk/Cargo.toml | grep ^version
```

### 验证 npm 包

```bash
npm view @zen_tools/x-sdk version
npm install @zen_tools/x-sdk
```

### 验证 crates.io

```bash
cargo search x-sdk
cargo add x-sdk
```

## 📊 发布状态监控

发布后可以在这些地方查看状态：

- **GitHub Actions**: https://github.com/0xCryptoZen/x-sdks/actions
- **npm 包**: https://www.npmjs.com/package/@zen_tools/x-sdk
- **crates.io**: https://crates.io/crates/x-sdk
- **docs.rs**: https://docs.rs/x-sdk
- **GitHub Releases**: https://github.com/0xCryptoZen/x-sdks/releases

## ⚠️ 注意事项

1. **单一真实来源**: VERSION 文件是版本的唯一来源
2. **自动同步**: 不要手动修改 package.json 或 Cargo.toml 的版本
3. **测试必须通过**: 发布前所有测试必须通过
4. **标签唯一性**: 每个版本号只能发布一次
5. **不可回退**: npm 和 crates.io 不允许删除已发布的版本

## 🆘 问题排查

### 版本不同步

```bash
# 重新同步版本
./scripts/sync-version.sh
```

### 发布失败

```bash
# 查看 GitHub Actions 日志
# https://github.com/0xCryptoZen/x-sdks/actions

# 检查 tokens 配置
# Settings → Secrets → Actions
```

### 撤销发布（发布前）

```bash
# 如果还没有推送
git reset --hard HEAD~1
git tag -d v1.0.0

# 如果已经推送但还没发布
git push origin :refs/tags/v1.0.0
git reset --hard HEAD~1
git push -f origin main
```

## 📚 相关文档

- [TypeScript SDK 发布指南](./PUBLISHING.md)
- [Rust SDK 发布指南](./PUBLISHING_RUST.md)
- [贡献指南](./CLAUDE.md)

## 🎉 快速参考

```bash
# 查看当前版本
cat VERSION

# 同步版本
./scripts/sync-version.sh

# 统一发布
./scripts/release-all.sh

# 监控发布状态
# https://github.com/0xCryptoZen/x-sdks/actions
```
