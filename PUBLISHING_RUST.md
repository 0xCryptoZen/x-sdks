# Publishing Rust SDK to crates.io

本指南说明如何发布 Rust SDK 到 crates.io。

## 前置准备

### 1. 配置 Cargo Token

首先需要在 GitHub 仓库中配置 crates.io 访问令牌：

1. 在 [crates.io](https://crates.io/) 登录你的账户
2. 进入 **Account Settings** → **API Tokens**
3. 点击 **New Token**
   - Token name: `GitHub Actions - x-sdk`
   - 点击 **Generate**
4. 复制生成的 token

5. 在 GitHub 仓库中：
   - 进入 **Settings** → **Secrets and variables** → **Actions**
   - 点击 **New repository secret**
   - Name: `CARGO_TOKEN`
   - Value: 粘贴你的 crates.io token
   - 点击 **Add secret**

### 2. crates.io 账户权限

确保你的 crates.io 账户有权限发布 `x-sdk` crate：

```bash
# 登录 crates.io
cargo login

# 检查当前配置
cat ~/.cargo/credentials.toml
```

## 发布新版本

### 方法一：使用发布脚本（推荐）

这是最简单和安全的方法：

```bash
# 运行 Rust SDK 发布脚本
./scripts/release-rust.sh

# 脚本会：
# 1. 检查工作区状态
# 2. 运行测试
# 3. 运行 clippy 检查
# 4. 检查代码格式
# 5. 让你选择版本类型（patch/minor/major）
# 6. 自动创建 tag 并推送
```

### 方法二：手动发布

```bash
# 1. 更新版本号
cd rust-sdk
# 编辑 Cargo.toml 中的 version

# 2. 运行测试
cargo test --all-features --workspace

# 3. 运行 clippy
cargo clippy --all-features --workspace -- -D warnings

# 4. 检查格式
cargo fmt --all -- --check

# 5. 更新 Cargo.lock
cargo check

# 6. 提交修改
cd ..
git add rust-sdk/Cargo.toml rust-sdk/Cargo.lock
git commit -m "chore(rust-sdk): bump version to 0.2.0"

# 7. 创建并推送标签
git tag rust-v0.2.0
git push origin main
git push origin rust-v0.2.0
```

### 方法三：手动触发工作流

1. 访问 GitHub 仓库的 **Actions** 页面
2. 选择 **Publish to crates.io** workflow
3. 点击 **Run workflow**
4. 选择分支并运行

## 版本号规范

遵循 [Semantic Versioning](https://semver.org/) (SemVer) 规范：

- **MAJOR** (1.0.0): 不兼容的 API 变更
- **MINOR** (0.1.0): 向后兼容的功能新增
- **PATCH** (0.0.1): 向后兼容的问题修复

### 示例

```bash
cd rust-sdk

# Bug 修复
# 0.1.0 → 0.1.1
sed -i 's/version = "0.1.0"/version = "0.1.1"/' Cargo.toml

# 新功能（向后兼容）
# 0.1.0 → 0.2.0
sed -i 's/version = "0.1.0"/version = "0.2.0"/' Cargo.toml

# 破坏性变更
# 0.1.0 → 1.0.0
sed -i 's/version = "0.1.0"/version = "1.0.0"/' Cargo.toml
```

## 标签命名规范

Rust SDK 使用 `rust-v*` 前缀的标签：

- ✅ `rust-v0.1.0`
- ✅ `rust-v0.2.0`
- ✅ `rust-v1.0.0`
- ❌ `v0.1.0` (这是 TypeScript SDK 的标签)

## 发布流程详解

GitHub Actions 执行的步骤：

1. **测试阶段**
   - 检出代码
   - 设置 Rust 工具链
   - 缓存依赖
   - 运行所有测试
   - 运行 clippy 检查
   - 检查代码格式

2. **发布阶段**（测试通过后）
   - 发布到 crates.io
   - 创建 GitHub Release
   - 生成发布说明

## 发布前检查清单

在发布前确认：

- [ ] 所有测试通过 (`cargo test --all-features --workspace`)
- [ ] Clippy 无警告 (`cargo clippy --all-features --workspace -- -D warnings`)
- [ ] 代码已格式化 (`cargo fmt --all -- --check`)
- [ ] 文档已更新
- [ ] CHANGELOG.md 已更新
- [ ] 版本号符合 SemVer 规范
- [ ] 代码已合并到 main 分支
- [ ] CARGO_TOKEN 已正确配置

## 发布后验证

发布成功后，验证 crate 是否正常：

```bash
# 1. 检查 crates.io 上的版本
cargo search x-sdk

# 2. 在新项目中测试安装
cargo new test-install
cd test-install
cargo add x-sdk

# 3. 测试编译
cargo build

# 4. 查看文档
# 访问 https://docs.rs/x-sdk
```

## 常见问题

### 发布失败：authentication required

**原因**：CARGO_TOKEN 无效或未设置

**解决**：
1. 检查 CARGO_TOKEN 是否正确设置
2. 重新生成 crates.io token
3. 验证 token 有发布权限

### 发布失败：crate already exists

**原因**：尝试发布已存在的版本号

**解决**：
```bash
# 更新到新版本
cd rust-sdk
# 编辑 Cargo.toml 更新版本
cargo check
cd ..
git add rust-sdk/Cargo.toml rust-sdk/Cargo.lock
git commit -m "chore(rust-sdk): bump version"
git tag rust-v0.1.1
git push origin main rust-v0.1.1
```

### 测试失败导致无法发布

**原因**：代码中有错误或测试未通过

**解决**：
```bash
# 本地运行测试
cd rust-sdk
cargo test --all-features --workspace

# 修复问题后重新提交
git add .
git commit -m "fix: resolve test failures"
git push origin main
```

### Clippy 警告

**原因**：代码质量检查未通过

**解决**：
```bash
# 运行 clippy
cd rust-sdk
cargo clippy --all-features --workspace -- -D warnings

# 修复警告
# 然后重新提交
```

## 回滚版本

如果发布的版本有问题：

```bash
# 1. 发布新的补丁版本修复问题（推荐）
./scripts/release-rust.sh

# 2. 或者使用 cargo yank（不删除，但不推荐使用）
cargo yank --version 0.2.0 x-sdk

# 3. 取消 yank
cargo yank --undo --version 0.2.0 x-sdk

# 注意：crates.io 不允许删除已发布的版本
# 只能 yank（标记为不推荐使用）
```

## 手动发布（备用方案）

如果 GitHub Actions 不可用，可以手动发布：

```bash
cd rust-sdk/x-sdk

# 1. 运行测试
cargo test --all-features

# 2. 运行 clippy
cargo clippy --all-features -- -D warnings

# 3. 检查格式
cargo fmt --all -- --check

# 4. 登录 crates.io
cargo login

# 5. 发布
cargo publish

# 6. 创建 git tag
cd ../..
git tag rust-v0.2.0
git push origin rust-v0.2.0
```

## 文档更新

发布后，文档会自动在以下位置更新：

- **docs.rs**: https://docs.rs/x-sdk
- **crates.io**: https://crates.io/crates/x-sdk
- **GitHub**: https://github.com/0xCryptoZen/x-sdks/tree/main/rust-sdk

## 相关链接

- [crates.io Documentation](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Semantic Versioning](https://semver.org/)
- [GitHub Actions](https://docs.github.com/en/actions)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [docs.rs](https://docs.rs/)

## 与 TypeScript SDK 发布的区别

| 特性 | Rust SDK | TypeScript SDK |
|------|----------|----------------|
| **Registry** | crates.io | npm |
| **Token Secret** | `CARGO_TOKEN` | `NPM_TOKEN` |
| **Tag 前缀** | `rust-v*` | `v*` |
| **脚本** | `./scripts/release-rust.sh` | `./scripts/release.sh` |
| **包名** | `x-sdk` | `@zen_tools/x-sdk` |
| **安装命令** | `cargo add x-sdk` | `npm install @zen_tools/x-sdk` |
