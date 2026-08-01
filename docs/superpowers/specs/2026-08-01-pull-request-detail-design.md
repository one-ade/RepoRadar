# Pull Request 全字段详情设计

## 状态

已批准。用户要求覆盖当前 `gh pr view --json` 支持的全部字段，并授权在持续 Goal 内自主采用最小可维护默认方案。

## 目标

在现有 GitHub 工作台中查看 Pull Request 完整详情，覆盖本机 `gh` 当前提供的 46 个 JSON 字段，同时保持信息可扫读、错误可见、界面风格统一。

## 范围

- PR 列表提供“详情”入口。
- 详情面板展示 `gh pr view --json` 当前支持的全部 46 个字段。
- 字段按内容、参与者、分支与仓库、合并与审查、变更、Checks、关联信息分组。
- 标量直接显示；数组和对象使用原生 `<details>` 折叠，并以格式化 JSON 展示完整值。
- 未被已知分组匹配的字段进入“其他”，不得静默遗漏。
- 复用现有 Review 与合并后端，并提供评论、批准、请求修改和合并入口。

不新增路由、弹窗、Markdown 渲染器、Diff 编辑器或依赖。Issue 与 Release 的详情和编辑由后续里程碑独立实现。

## 后端设计

新增 `get_pull_request_detail(path, number)` Tauri 命令。命令复用现有 GitHub Remote 解析和 `gh` 执行边界，执行：

```text
gh pr view <number> --json <46 个字段> -R <host/owner/repo>
```

字段名称由单一常量维护并按固定顺序传递。返回值解析为：

- `GithubPullRequestDetail`：有序字段列表。
- `GithubDetailField`：字段名和字段值。
- `GithubDetailValue`：可序列化的递归联合值，覆盖 null、布尔、数字、字符串、数组和对象。

不把 `serde_json::Value` 直接传过应用边界，不拼接 Shell 命令。PR 编号使用无符号整数，仓库引用继续由现有解析器生成。

## 前端设计

新增独立 `GithubPullRequestDetailPanel.vue`，避免继续扩大现有 GitHub 工作台组件。面板接收详情、busy 状态和关闭事件，只负责展示。

`useGithubWorkspace` 新增当前 PR 详情状态与加载动作。选择详情时先清空旧值，再通过现有 `runProjectAction` 调用 IPC。切换项目、刷新 GitHub 概览、Review 或合并完成后清空详情，避免显示过期状态。

`GithubWorkspacePanel.vue` 只增加“详情”事件和详情面板插槽数据，不承担字段格式化逻辑。根组件继续只负责连线。

## 分组

- 内容：number、title、body、state、isDraft、url、id、fullDatabaseId、createdAt、updatedAt、closed、closedAt。
- 参与与讨论：author、assignees、reviewRequests、reviews、latestReviews、comments、mergedBy。
- 分支与仓库：baseRefName、baseRefOid、headRefName、headRefOid、headRepository、headRepositoryOwner、isCrossRepository、maintainerCanModify。
- 合并与审查：mergeable、mergeStateStatus、reviewDecision、autoMergeRequest、mergeCommit、potentialMergeCommit、mergedAt。
- 变更：additions、deletions、changedFiles、commits、files。
- Checks：statusCheckRollup。
- 关联信息：labels、milestone、closingIssuesReferences、projectCards、projectItems、reactionGroups。
- 其他：任何未匹配字段。

## 错误与空状态

- 加载开始时清空旧详情。
- `gh` 失败、字段不兼容或 JSON 无效时，由现有错误横幅和操作中心显示完整失败；不渲染部分结果。
- null、空数组和空对象仍显示字段名及空状态，以证明字段没有被遗漏。
- 无选中详情时不占用额外界面空间。

## 验证

- Rust 单元测试锁定全部 46 个字段并覆盖六类递归 JSON 值。
- 前端 composable 测试覆盖加载、旧值清理和项目重置。
- Vue 组件测试覆盖入口、分组、全部字段计数、复杂值折叠、关闭和 busy 禁用。
- 运行前端完整测试、类型检查、生产构建、Rust 测试、`cargo fmt --check`、Clippy 和 `git diff --check`。
- 使用真实 Vue 组件 fixture 完成 960px、1280px 与最大化视觉检查；不为测试创建远程 PR。

## 后续扩展

同一递归字段模型可复用于 Issue 全字段详情，但本次不提前抽象公共领域组件；等第二个真实调用方出现时再提取。
