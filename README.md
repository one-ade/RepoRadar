# RepoRadar

RepoRadar 是一个使用 Rust、Tauri 2 和 Vue 3 构建的本地 Git/GitHub 项目管理桌面应用。

## 当前版本

`0.1.0` 已完成 Windows、macOS 和 Linux 构建验证。可从 [GitHub Releases](https://github.com/SANgaojie/RepoRadar/releases) 下载对应平台的桌面包。

## 能力

- 扫描、添加、搜索、收藏并标记本地 Git 仓库，支持普通仓库、bare 仓库和 worktree。
- 查看工作区、Diff 与提交历史，执行暂存、提交、分支、Fetch、Pull 和 Push。
- 管理 Repository、Pull Request、Issue、Actions、Release、环境、Secret、Variable、Label 和 Ruleset。
- 接入 Projects、Discussions、Codespaces、GitHub 全局搜索、安全只读 `gh` 命令和 `gh api` 请求构造器。
- 识别 GitHub.com 与 GitHub Enterprise Server remote，展示多 Host 登录状态而不读取或保存 Token。

## 开发

要求：

- Node.js 20.19+ 或 22.12+
- 最新稳定版 Rust
- Git
- GitHub CLI（可选，GitHub 功能需要）

```bash
npm ci
npm run tauri dev
```

验证：

```bash
npm run check
npm run test:frontend
npm run build
npm run test:rust
npm run tauri build
```

## 发布

- GitHub Actions 在 Pull Request、`main` 推送和手动触发时验证三平台构建。
- 推送 `v*` 标签会创建 GitHub Release，并上传 Windows MSI/NSIS、macOS DMG/App 和 Linux AppImage/DEB/RPM。
- Windows 安装包需要代码签名证书才能消除系统的未签名提示；当前公开构建未配置签名证书。

## 架构

Vue 前端只通过明确的 Tauri Command 调用 Rust。Rust 负责文件系统、SQLite、`git` 和 `gh`，不向前端开放任意 Shell 执行权限。
