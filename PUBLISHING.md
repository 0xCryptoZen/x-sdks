# Publishing Guide

本指南说明如何发布新版本的 X SDKs 到 npm。

## 前置准备

### 1. 配置 NPM Token

首先需要在 GitHub 仓库中配置 NPM 访问令牌：

1. 在 [npmjs.com](https://www.npmjs.com/) 登录你的账户
2. 进入 **Access Tokens** 页面
3. 点击 **Generate New Token** → 选择 **Automation**
4. 复制生成的 token

5. 在 GitHub 仓库中：
   - 进入 **Settings** → **Secrets and variables** → **Actions**
   - 点击 **New repository secret**
   - Name: `NPM_TOKEN`
   - Value: 粘贴你的 npm token
   - 点击 **Add secret**

### 2. npm 包权限

确保你的 npm 账户有权限发布 `@zen_tools/x-sdk` 包：

```bash
# 登录 npm
npm login

# 检查当前用户
npm whoami

# 确保你是 zen_tools organization 的成员
# 访问 https://www.npmjs.com/settings/zen_tools/members
# 或者创建新组织: https://www.npmjs.com/org/create
```

## 发布新版本

### 方法一：使用 Git Tag（推荐）

这是最简单和安全的方法：

```bash
# 1. 确保所有修改已提交
git status

# 2. 更新版本号（自动更新 package.json）
cd typescript-sdk
npm version patch  # 0.1.0 -> 0.1.1
# 或者
npm version minor  # 0.1.0 -> 0.2.0
# 或者
npm version major  # 0.1.0 -> 1.0.0

# 3. 推送代码和标签
git push origin main
git push origin --tags

# GitHub Actions 会自动：
# - 运行测试
# - 构建包
# - 发布到 npm
# - 创建 GitHub Release
```

### 方法二：手动指定版本

```bash
# 1. 手动更新 typescript-sdk/package.json 中的版本号
vim typescript-sdk/package.json

# 2. 提交修改
git add typescript-sdk/package.json
git commit -m "chore: bump version to 0.2.0"

# 3. 创建并推送标签
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

### 方法三：手动触发（测试用）

1. 访问 GitHub 仓库的 **Actions** 页面
2. 选择 **Publish to npm** workflow
3. 点击 **Run workflow**
4. 选择分支并运行

## 版本号规范

遵循 [Semantic Versioning](https://semver.org/) (SemVer) 规范：

- **MAJOR** (1.0.0): 不兼容的 API 变更
- **MINOR** (0.1.0): 向后兼容的功能新增
- **PATCH** (0.0.1): 向后兼容的问题修复

### 示例

```bash
# Bug 修复
npm version patch   # 0.1.0 → 0.1.1

# 新功能（向后兼容）
npm version minor   # 0.1.0 → 0.2.0

# 破坏性变更
npm version major   # 0.1.0 → 1.0.0

# 预发布版本
npm version prerelease --preid=beta  # 0.1.0 → 0.1.1-beta.0
```

## 发布流程详解

GitHub Actions 执行的步骤：

1. **测试阶段**
   - 检出代码
   - 安装依赖
   - 运行测试套件
   - 构建 TypeScript 代码

2. **发布阶段**（测试通过后）
   - 安装依赖
   - 构建生产版本
   - 发布到 npm registry
   - 创建 GitHub Release

## 发布后验证

发布成功后，验证包是否正常：

```bash
# 1. 检查 npm 上的版本
npm view @zen_tools/x-sdk version

# 2. 在新项目中测试安装
mkdir test-install
cd test-install
npm init -y
npm install @zen_tools/x-sdk

# 3. 测试导入
node -e "const sdk = require('@zen_tools/x-sdk'); console.log('OK')"
```

## 常见问题

### 发布失败：403 Forbidden

**原因**：NPM_TOKEN 无效或权限不足

**解决**：
1. 检查 NPM_TOKEN 是否正确设置
2. 确认 token 类型为 "Automation"
3. 验证账户有发布权限

### 发布失败：版本已存在

**原因**：尝试发布已存在的版本号

**解决**：
```bash
# 更新到新版本
cd typescript-sdk
npm version patch
git push origin --tags
```

### 测试失败导致无法发布

**原因**：代码中有错误或测试未通过

**解决**：
```bash
# 本地运行测试
cd typescript-sdk
npm test

# 修复问题后重新提交
git add .
git commit -m "fix: resolve test failures"
git push origin main
```

## 回滚版本

如果发布的版本有问题：

```bash
# 1. 发布新的补丁版本修复问题（推荐）
npm version patch
git push origin --tags

# 2. 或者废弃有问题的版本
npm deprecate @zen_tools/x-sdk@0.2.0 "This version has bugs, use 0.2.1 instead"

# 注意：npm 不允许删除已发布超过 72 小时的包
```

## 发布检查清单

在发布前确认：

- [ ] 所有测试通过
- [ ] 文档已更新
- [ ] CHANGELOG.md 已更新
- [ ] 版本号符合 SemVer 规范
- [ ] 代码已合并到 main 分支
- [ ] NPM_TOKEN 已正确配置

## 手动发布（备用方案）

如果 GitHub Actions 不可用，可以手动发布：

```bash
cd typescript-sdk

# 1. 安装依赖
npm ci

# 2. 运行测试
npm test

# 3. 构建
npm run build

# 4. 登录 npm
npm login

# 5. 发布
npm publish --access public

# 6. 创建 git tag
cd ..
git tag v0.2.0
git push origin v0.2.0
```

## 相关链接

- [npm Documentation](https://docs.npmjs.com/)
- [Semantic Versioning](https://semver.org/)
- [GitHub Actions](https://docs.github.com/en/actions)
- [npm Access Tokens](https://docs.npmjs.com/about-access-tokens)
