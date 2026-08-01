# RepoRadar 代办清单

## 里程碑 1：应用底座

- [x] 确定 Rust + Tauri 2 + Vue 3 + TypeScript 技术栈
- [x] 创建 Tauri 与 Vue 工程骨架
- [x] 建立 SQLite 数据库和版本迁移
- [x] 检测 Git、GitHub CLI 和 GitHub 登录状态
- [x] 创建首个可视化仪表盘
- [x] 安装依赖并通过前端类型检查
- [x] 通过 Rust 单元测试和编译检查
- [x] 实际启动桌面应用并完成界面检查
- [x] 去除 Tauri 系统窗口边框并加入自定义标题栏
- [x] 固定标题栏、主内容独立纵向滚动，避免页面双重滚动
- [x] 主内容与内部面板使用统一自定义滚动条样式
- [x] 完成里程碑代码审查

- [x] 按钮提供悬停、按压、禁用和键盘焦点反馈
- [x] 校正项目行与 GitHub 工具布局，统一最小字号并避免异常换行
- [x] 完成 960px、1280px 与最大化窗口的桌面视觉复核
- [x] 建立 Vitest + jsdom 的 Vue 组件测试基线
- [x] 拆分 GitHub 配置面板并覆盖敏感值、操作记录与原生控件回归
- [x] 覆盖操作中心的成功、失败、数量上限与清空行为
- [x] 拆分侧边栏、页头、环境状态、扫描进度与操作中心组件
- [x] 拆分项目目录组件并覆盖 busy 禁用行为
- [x] 拆分本地 Git 详情组件及状态 composable

## 里程碑 2：项目发现

- [x] 管理扫描根目录
- [x] 扫描普通 Git 仓库、bare 仓库和 worktree
- [x] 排除系统目录、依赖目录和符号链接循环
- [x] 手动添加现有 Git 项目
- [x] 为非 Git 目录执行 `git init`
- [x] 完成项目发现代码审查

## 项目发现增强

- [x] 展示扫描进度并支持取消
- [x] 项目搜索和收藏
- [x] 项目标签
- [x] 操作中心、扫描任务取消和安全日志

## 里程碑 3：本地 Git 管理

- [x] 工作区和暂存区状态
- [x] 查看文件 Diff
- [x] 暂存和取消暂存
- [x] 提交
- [x] 分支创建、切换和删除
- [x] Fetch、Pull、Push
- [x] 提交历史
- [x] 完成本地 Git 代码审查

## 里程碑 4：GitHub 核心能力

- [x] 解析 GitHub / Enterprise remote，并验证 Repository、PR、Issue、Actions、Release 概览
- [x] Actions 工作流列表、Run 日志、重跑和取消
- [x] Actions Artifact 下载、Release 创建和按 Tag 下载
- [x] Repository Fork 和安全同步
- [x] Repository 克隆和从本地项目创建
- [x] GitHub 多 Host 认证状态（不展示 Token）
- [x] Repository 查看、创建、克隆、Fork 和同步
- [x] Pull Request 列表、创建、Review 和合并
- [x] Pull Request 详情
- [x] Issue 列表、创建、评论和关闭
- [x] Issue 详情与编辑
- [x] Actions Workflow、Run、日志和 Artifact
- [x] Release 列表、创建和下载
- [x] Release 编辑与资源上传
- [x] 拆分 GitHub 后端能力模块并通过单测与 Clippy 审查
- [x] 拆分前端 IPC 门面并保持命令名称兼容
- [x] 将 Tauri 命令适配层按领域拆分并保持前端命令名称兼容
- [x] 将 GitHub 工作台动作迁移到独立 composable 并覆盖单次父操作回归
- [x] 拆分 GitHub 工作台展示组件并覆盖 Workflow 运行、重跑和取消
- [x] 将根组件收口为 300 行以内的页面编排层

- [x] 从 Workflow 列表触发 workflow_dispatch

## 里程碑 5：GitHub 扩展能力

- [x] Repository Secret 和 Variable 查看、写入与删除（Secret 不回显）
- [x] Repository Label 查看、创建、更新和删除
- [x] 检查默认分支适用的 Ruleset
- [x] 部署环境
- [x] Projects、Discussions 和 Codespaces
- [x] GitHub 搜索和全局工作台
- [ ] 安全的高级 `gh` 命令面板
- [ ] `gh api` 请求构造器
- [x] GitHub Enterprise Server Remote 解析与多 Host 认证状态
- [ ] 完成扩展能力代码审查

## 里程碑 6：发布

- [ ] Windows 安装包
- [ ] macOS 和 Linux 构建验证
- [ ] 端到端桌面操作验证
- [ ] 性能、无障碍和安全检查
- [ ] 最终代码审查和发布说明
