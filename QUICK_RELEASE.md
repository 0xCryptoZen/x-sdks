# 快速发布指南

三种发布方式，从最简单到最灵活。

## 🚀 方式 1: 一行命令发布（最简单）

```bash
./scripts/tag-release.sh v2.0.3
```

**流程:**
1. 输入版本标签（如 v2.0.3）
2. 自动更新 VERSION、package.json、Cargo.toml
3. 运行测试
4. 创建 commit 和 tag
5. 询问是否推送
6. 推送后自动触发 GitHub Actions 发布

**优点:**
- ✅ 一条命令完成所有步骤
- ✅ 自动运行测试
- ✅ 有确认步骤，安全可控

---

## 🎯 方式 2: Git Tag + Hook（最自动化）

### 首次设置（只需一次）

```bash
./scripts/setup-hooks.sh
```

### 使用

```bash
# 1. 创建标签
git tag v2.0.3

# 2. 推送标签
git push origin v2.0.3
```

**流程:**
1. 创建 tag
2. 推送 tag 时 pre-push hook 自动触发
3. Hook 检测版本不匹配
4. 自动更新 VERSION、package.json、Cargo.toml
5. 自动创建 commit
6. 自动移动 tag 到新 commit
7. 继续推送，触发 GitHub Actions

**优点:**
- ✅ 最少的命令
- ✅ 完全自动化
- ✅ 符合直觉的 git 工作流

**注意:**
- 需要先运行 `./scripts/setup-hooks.sh` 安装 hook

---

## 🔧 方式 3: 手动控制（最灵活）

```bash
# 1. 手动编辑 VERSION 文件
echo "2.0.3" > VERSION

# 2. 同步版本
./scripts/sync-version.sh

# 3. 运行测试
cd typescript-sdk && npm test && cd ..
cd rust-sdk && cargo test --all-features --workspace && cd ..

# 4. 提交
git add -A
git commit -m "chore: bump version to 2.0.3"

# 5. 创建标签
git tag v2.0.3

# 6. 推送
git push origin main
git push origin v2.0.3
```

**优点:**
- ✅ 完全掌控每一步
- ✅ 可以在任意步骤暂停检查
- ✅ 适合调试问题

---

## 📊 三种方式对比

| 特性 | 方式 1<br>tag-release.sh | 方式 2<br>Git Hook | 方式 3<br>手动 |
|------|------------------------|-------------------|--------------|
| **命令数** | 1 条 | 2 条 | 6+ 条 |
| **自动化** | 高 | 最高 | 低 |
| **灵活性** | 中 | 低 | 最高 |
| **安全确认** | ✅ 有 | ❌ 无 | ✅ 手动 |
| **运行测试** | ✅ 自动 | ❌ 不运行 | ✅ 手动 |
| **适合场景** | 日常发布 | 快速发布 | 调试/特殊情况 |

---

## 🎯 推荐使用

### 日常发布 → 方式 1

```bash
./scripts/tag-release.sh v2.0.4
```

简单、安全、有测试保护。

### 快速热修复 → 方式 2

```bash
git tag v2.0.5
git push origin v2.0.5
```

最快速度发布紧急修复。

### 调试问题 → 方式 3

手动执行每一步，便于定位问题。

---

## 🔄 工作流示例

### 场景 1: 计划发布新版本

```bash
# 开发完成，准备发布
./scripts/tag-release.sh v2.1.0

# 脚本会：
# ✓ 更新版本文件
# ✓ 运行测试
# ✓ 创建 commit 和 tag
# ✓ 询问确认
# ✓ 推送到 GitHub
# ✓ 自动发布到 npm 和 crates.io
```

### 场景 2: 紧急热修复

```bash
# 修复 bug
git add .
git commit -m "fix: critical bug"

# 快速发布（已安装 hook）
git tag v2.0.6
git push origin v2.0.6

# Hook 自动处理版本同步
```

### 场景 3: 预发布版本

```bash
# 创建 beta 版本
./scripts/tag-release.sh v2.1.0-beta.1

# 或手动
echo "2.1.0-beta.1" > VERSION
./scripts/sync-version.sh
git add -A
git commit -m "chore: release beta version"
git tag v2.1.0-beta.1
git push origin main v2.1.0-beta.1
```

---

## ⚙️ 安装 Git Hook

### 自动安装（推荐）

```bash
./scripts/setup-hooks.sh
```

### 手动安装

```bash
git config core.hooksPath .githooks
```

### 验证安装

```bash
git config core.hooksPath
# 输出: .githooks
```

---

## 🔍 故障排查

### Hook 没有触发

```bash
# 检查 hook 配置
git config core.hooksPath

# 重新安装
./scripts/setup-hooks.sh

# 确认 hook 可执行
ls -la .githooks/pre-push
```

### 版本同步失败

```bash
# 手动同步
./scripts/sync-version.sh

# 检查版本
cat VERSION
grep version typescript-sdk/package.json
grep version rust-sdk/Cargo.toml
```

### 测试失败

```bash
# TypeScript 测试
cd typescript-sdk
npm test

# Rust 测试
cd rust-sdk
cargo test --all-features --workspace
```

---

## 📝 版本号规范

遵循 [Semantic Versioning](https://semver.org/):

```
v<major>.<minor>.<patch>[-prerelease]

v2.0.3          ✅ 正式版本
v2.1.0-beta.1   ✅ 预发布版本
v3.0.0-rc.2     ✅ 候选版本
2.0.3           ❌ 缺少 'v' 前缀
v2.0            ❌ 缺少 patch 版本
```

---

## 🎉 快速参考

```bash
# 最简单的发布方式
./scripts/tag-release.sh v2.0.4

# 或安装 hook 后
git tag v2.0.4 && git push origin v2.0.4

# 监控发布状态
# https://github.com/0xCryptoZen/x-sdks/actions
```

---

## 📚 相关文档

- [RELEASE.md](./RELEASE.md) - 完整发布指南
- [PUBLISHING.md](./PUBLISHING.md) - TypeScript SDK 发布
- [PUBLISHING_RUST.md](./PUBLISHING_RUST.md) - Rust SDK 发布
