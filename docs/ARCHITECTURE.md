# RepoRadar 架构

## 前端目录

- `src/components/`：自定义标题栏、应用壳、项目目录、本地 Git 详情与 GitHub 工作台等独立界面边界。
- `src/composables/`：项目发现、本地 Git、GitHub 和操作记录等领域状态。
- `src/api.ts`：稳定的 Tauri API 导出门面。
- `src/api/`：按环境、项目、本地 Git、GitHub 和共享类型拆分的 IPC 调用。
- `src/**/*.test.ts`：Vitest + jsdom 组件回归测试；Tauri IPC 仅在测试边界替换。
- `src/App.vue`：300 行以内的页面编排层，只组合领域状态和界面组件。

## Rust 目录

- `src-tauri/src/lib.rs`：Tauri 应用初始化、状态注入与命令注册。
- `src-tauri/src/commands/`：按环境、项目、本地 Git 和 GitHub 分组的薄命令适配层。
- `src-tauri/src/github.rs`：GitHub 数据类型与稳定公开入口。
- `src-tauri/src/github/client.rs`：唯一的 `gh` 进程、输出解析和参数校验边界。
- `src-tauri/src/github/overview.rs`：Repository、PR、Issue、Actions 与 Release 概览。
- `src-tauri/src/github/configuration.rs`：Variable、Secret 与 Label 配置。
- `src-tauri/src/github/actions.rs`：Workflow Run、Artifact、Ruleset 与 Release 操作。
- `src-tauri/src/github/repositories.rs`：Repository 创建、克隆、Fork 与同步。
- `src-tauri/src/github/collaboration.rs`：Pull Request 与 Issue 写操作。

## 边界

- Vue 负责展示和用户交互。
- Rust 负责可信边界内的本地能力。
- SQLite 由 Rust 独占访问。
- `git` 处理本地仓库操作。
- `gh` 处理 GitHub 平台操作。
- RepoRadar 不保存 GitHub Token、Git 密码或 SSH 私钥。
- GitHub Secret 与 Variable 的值通过标准输入传给 `gh`，不进入进程参数；Secret 不回显、不落库，也不进入操作日志。
- Tauri 使用无原生装饰窗口；Vue 提供标题栏、窗口控制和拖拽区域。
- 页面由固定标题栏与独立可滚动主内容区组成，避免 body 和内容面板同时承担页面滚动。
- 所有滚动容器使用统一的深色细滚动条，并预留滚动槽避免内容宽度跳动。
- 操作中心在前端记录动作名称、状态和时间；不保存命令参数、Token 或仓库内容。扫描取消继续由 Rust 原子状态控制。

- TypeScript 暂留在 `5.9.3`：`7.0.2` 已实测与 `vue-tsc 3.3.8` 不兼容；待 Vue 工具链支持其公开编译入口后再升级。

## 命令执行

前端不能提交任意 Shell 字符串。每项能力都由明确的 Tauri Command 暴露，Rust 使用程序名和参数数组启动子进程。
命令适配层只负责 Tauri 参数与阻塞任务调度，实际行为保留在对应领域模块。

高频 GitHub 功能提供专用界面。低频功能最终进入高级 `gh` 面板，但仍只允许执行 `gh`，并在执行前展示参数和风险级别。

## 数据

SQLite 位于 Tauri 应用数据目录。当前数据库版本为 1，包含：

- `projects`：已发现或手动添加的项目
- `scan_roots`：用户配置的扫描根目录
- `settings`：简单应用配置

瞬时 Git 状态不持久化，避免展示过期数据。

## 依赖原则

只有出现真实需求时才增加依赖。首个里程碑不引入状态管理、路由、UI 组件库、ORM 或异步数据库连接池。
