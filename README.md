# RepoRadar

RepoRadar 是一个使用 Rust、Tauri 2 和 Vue 3 构建的本地 Git/GitHub 项目管理桌面应用。

## 当前状态

项目正在按 [代办清单](docs/TODO.md) 分阶段实现。首个里程碑提供：

- Tauri + Vue 3 桌面应用骨架
- SQLite 数据库初始化与迁移
- Git 和 GitHub CLI 环境检测
- GitHub CLI 登录状态检测
- RepoRadar 仪表盘

## 开发

要求：

- Node.js 20.19+ 或 22.12+
- 最新稳定版 Rust
- Git
- GitHub CLI（可选，GitHub 功能需要）

```bash
npm install
npm run tauri dev
```

验证：

```bash
npm run check
npm run build
npm run test:rust
```

## 架构

Vue 前端只通过明确的 Tauri Command 调用 Rust。Rust 负责文件系统、SQLite、`git` 和 `gh`，不向前端开放任意 Shell 执行权限。
