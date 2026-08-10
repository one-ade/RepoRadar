# RepoRadar 0.1.2

RepoRadar 0.1.2 提供一个统一的本地 Git 与 GitHub 桌面工作台，并修复 Windows Release 运行体验问题。

## 主要能力

- 扫描、添加、搜索、收藏并标记本地仓库，支持普通仓库、bare 仓库和 worktree。
- 查看工作区、Diff 与提交历史，执行暂存、提交、分支、Fetch、Pull 和 Push。
- 管理 Repository、Pull Request、Issue、Actions、Release、环境、Secret、Variable、Label 与 Ruleset。
- 接入 Projects、Discussions、Codespaces、GitHub 全局搜索、安全只读 `gh` 命令和 `gh api` 请求构造器。
- 识别 GitHub.com 与 GitHub Enterprise Server remote，并展示多 Host 登录状态而不读取或保存 Token。

## 修复

- Windows Release 使用 GUI 子系统，并隐藏 Git/GitHub CLI 子进程的控制台窗口。
- 应用启动、切换项目、执行 Git/GitHub 操作和初始化仓库时不再闪现终端窗口。

## 发布验证

- Windows x64：生成 MSI 与 NSIS 安装包。
- macOS、Ubuntu 和 Windows：GitHub Actions 执行前端测试、Rust 测试及原生 Tauri bundle 构建。
- Windows Release 应用：验证真实启动、环境检测 IPC、窗口响应、性能与无障碍树。

## 使用要求

- 系统已安装 Git 和 GitHub CLI。
- GitHub 功能使用现有 `gh auth` 凭据；Projects 和 Codespaces 等能力需要相应 OAuth scope。
- 当前 Windows 安装包未使用代码签名证书签名。
