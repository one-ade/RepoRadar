# Pull Request Full Details Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a full-width Pull Request detail surface that exposes every JSON field supported by the installed `gh pr view` contract.

**Architecture:** A new Rust GitHub detail module requests all 46 fields and converts JSON into an ordered recursive value model. A focused frontend composable owns selection/loading, while dedicated PR section and detail components keep the existing workspace, root component, and composable below the 250 effective-line ceiling.

**Tech Stack:** Rust, serde/serde_json, Tauri 2, Vue 3 Composition API, TypeScript, Vitest, existing CSS.

## Global Constraints

- Cover all 46 fields reported by the current `gh pr view --help`; unmatched fields must render under “其他”.
- Do not add dependencies, routes, modals, Markdown renderers, or a Diff editor.
- Do not construct shell command strings; pass PR number and repository reference as separate `gh` arguments.
- Clear detail state on project or GitHub overview changes so Review/merge results cannot leave stale data.
- Keep every modified TypeScript/Rust/Vue source file at or below 250 effective lines.
- Preserve the existing dark indigo visual system, focus states, busy disabling, and reduced-motion behavior.

---

### Task 1: Rust full-field detail contract and Tauri command

**Files:**
- Create: `src-tauri/src/github/details.rs`
- Modify: `src-tauri/src/github.rs`
- Modify: `src-tauri/src/commands/github.rs`
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Produces: `github::pull_request_detail(path: &Path, number: u64) -> Result<GithubPullRequestDetail, String>`
- Produces: Tauri command `get_pull_request_detail(path: PathBuf, number: u64)`
- Produces: serialized `GithubPullRequestDetail { fields: Vec<GithubDetailField> }`

- [ ] **Step 1: Write failing Rust tests for the 46-field contract and recursive values**

Add tests in `src-tauri/src/github/details.rs` before implementation:

```rust
#[test]
fn pull_request_detail_contract_contains_every_supported_field_once() {
    assert_eq!(PULL_REQUEST_DETAIL_FIELDS.len(), 46);
    let unique = PULL_REQUEST_DETAIL_FIELDS.iter().collect::<BTreeSet<_>>();
    assert_eq!(unique.len(), PULL_REQUEST_DETAIL_FIELDS.len());
    assert!(PULL_REQUEST_DETAIL_FIELDS.contains(&"statusCheckRollup"));
    assert!(PULL_REQUEST_DETAIL_FIELDS.contains(&"comments"));
}

#[test]
fn detail_value_parses_every_json_shape() {
    let value: GithubDetailValue = serde_json::from_str(
        r#"{"null":null,"bool":true,"number":3,"string":"x","array":[],"object":{}}"#,
    )
    .unwrap();
    assert!(matches!(value, GithubDetailValue::Object(values) if values.len() == 6));
}
```

- [ ] **Step 2: Run the focused Rust tests and verify RED**

Run: `cargo test --manifest-path src-tauri/Cargo.toml github::details -- --nocapture`

Expected: compilation fails because the details module, field constant, and recursive types do not exist.

- [ ] **Step 3: Implement the minimal ordered detail module**

Create `src-tauri/src/github/details.rs` with:

```rust
use std::{collections::BTreeMap, path::Path};

use serde::{Deserialize, Serialize};

use super::client::{gh_output, repository_reference};

pub(super) const PULL_REQUEST_DETAIL_FIELDS: &[&str] = &[
    "additions", "assignees", "author", "autoMergeRequest", "baseRefName",
    "baseRefOid", "body", "changedFiles", "closed", "closedAt",
    "closingIssuesReferences", "comments", "commits", "createdAt", "deletions",
    "files", "fullDatabaseId", "headRefName", "headRefOid", "headRepository",
    "headRepositoryOwner", "id", "isCrossRepository", "isDraft", "labels",
    "latestReviews", "maintainerCanModify", "mergeCommit", "mergeStateStatus",
    "mergeable", "mergedAt", "mergedBy", "milestone", "number",
    "potentialMergeCommit", "projectCards", "projectItems", "reactionGroups",
    "reviewDecision", "reviewRequests", "reviews", "state", "statusCheckRollup",
    "title", "updatedAt", "url",
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GithubDetailValue {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Array(Vec<GithubDetailValue>),
    Object(BTreeMap<String, GithubDetailValue>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubDetailField {
    pub name: String,
    pub value: GithubDetailValue,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GithubPullRequestDetail {
    pub fields: Vec<GithubDetailField>,
}
```

Implement `parse_pull_request_detail` by deserializing a `BTreeMap<String, GithubDetailValue>`, removing each required name in constant order, and returning `missing Pull Request detail field: {name}` if a key is absent. Implement `pull_request_detail` with `repository_reference`, `PULL_REQUEST_DETAIL_FIELDS.join(",")`, and `gh_output`.

Declare `mod details;` and re-export the public types/function from `src-tauri/src/github.rs`. Make `client::gh_output` available to sibling modules with its existing `pub(super)` visibility.

- [ ] **Step 4: Add and register the Tauri adapter**

Add to `src-tauri/src/commands/github.rs`:

```rust
#[tauri::command]
pub async fn get_pull_request_detail(
    path: PathBuf,
    number: u64,
) -> Result<github::GithubPullRequestDetail, String> {
    tauri::async_runtime::spawn_blocking(move || github::pull_request_detail(&path, number))
        .await
        .map_err(|error| error.to_string())?
}
```

Register `commands::github::get_pull_request_detail` beside `get_github_overview` in `src-tauri/src/lib.rs`.

- [ ] **Step 5: Verify GREEN and backend quality gates**

Run:

```text
cargo test --manifest-path src-tauri/Cargo.toml github::details -- --nocapture
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
```

Expected: detail tests pass; formatting and Clippy exit 0.

- [ ] **Step 6: Review, stage, commit, and push the backend checkpoint**

Review the diff for the exact 46-field set, missing-field failure, argument separation, serialization names, effective line counts, and duplicate helpers.

```text
git add src-tauri/src/github/details.rs src-tauri/src/github.rs src-tauri/src/commands/github.rs src-tauri/src/lib.rs
git commit -m "feat: expose pull request details"
git push origin main
```

---

### Task 2: Frontend IPC and isolated detail state

**Files:**
- Modify: `src/api/types.ts`
- Modify: `src/api/github.ts`
- Create: `src/composables/usePullRequestDetail.ts`
- Create: `src/composables/usePullRequestDetail.test.ts`

**Interfaces:**
- Consumes: Tauri command `get_pull_request_detail`
- Produces: `getPullRequestDetail(path: string, number: number): Promise<GithubPullRequestDetail>`
- Produces: `usePullRequestDetail(project, overview, runAction)` with `selectedPullRequest`, `pullRequestDetail`, `viewPullRequest`, and `clearPullRequestDetail`

- [ ] **Step 1: Write the failing composable test**

Mock `getPullRequestDetail`, create real `ref<Project | null>` and `ref<GithubOverview | null>`, then assert:

```typescript
const detail = { fields: [{ name: "title", value: "Improve details" }] };
api.getPullRequestDetail.mockResolvedValue(detail);

await workspace.viewPullRequest(pullRequest);

expect(api.getPullRequestDetail).toHaveBeenCalledWith(project.path, pullRequest.number);
expect(workspace.selectedPullRequest.value).toEqual(pullRequest);
expect(workspace.pullRequestDetail.value).toEqual(detail);

overview.value = githubOverview;
await nextTick();
expect(workspace.pullRequestDetail.value).toBeNull();
```

- [ ] **Step 2: Run the focused test and verify RED**

Run: `npm run test:frontend -- src/composables/usePullRequestDetail.test.ts`

Expected: FAIL because the API and composable do not exist.

- [ ] **Step 3: Add strict recursive frontend types and IPC**

Add to `src/api/types.ts`:

```typescript
export type GithubDetailValue =
  | null
  | boolean
  | number
  | string
  | readonly GithubDetailValue[]
  | { readonly [key: string]: GithubDetailValue };

export interface GithubDetailField {
  readonly name: string;
  readonly value: GithubDetailValue;
}

export interface GithubPullRequestDetail {
  readonly fields: readonly GithubDetailField[];
}
```

Add to `src/api/github.ts`:

```typescript
export function getPullRequestDetail(path: string, number: number) {
  return invoke<GithubPullRequestDetail>("get_pull_request_detail", { path, number });
}
```

- [ ] **Step 4: Implement the focused composable**

Create `src/composables/usePullRequestDetail.ts`:

```typescript
export function usePullRequestDetail(
  project: Ref<Project | null>,
  overview: Ref<GithubOverview | null>,
  runAction: RunAction,
) {
  const selectedPullRequest = ref<GithubPullRequest | null>(null);
  const pullRequestDetail = ref<GithubPullRequestDetail | null>(null);

  function clearPullRequestDetail() {
    selectedPullRequest.value = null;
    pullRequestDetail.value = null;
  }

  async function viewPullRequest(pullRequest: GithubPullRequest) {
    const selected = project.value;
    if (!selected) return;
    clearPullRequestDetail();
    selectedPullRequest.value = pullRequest;
    await runAction(async () => {
      pullRequestDetail.value = await getPullRequestDetail(selected.path, pullRequest.number);
    }, `加载 PR #${pullRequest.number} 详情`);
  }

  watch([project, overview], clearPullRequestDetail);

  return { selectedPullRequest, pullRequestDetail, viewPullRequest, clearPullRequestDetail };
}
```

Use type-only imports and no assertions or non-null operators.

- [ ] **Step 5: Verify GREEN, type checking, and file sizes**

Run:

```text
npm run test:frontend -- src/composables/usePullRequestDetail.test.ts
npm run check
```

Expected: focused tests and `vue-tsc --noEmit` pass; all changed source files remain at or below 250 effective lines.

- [ ] **Step 6: Review, commit, and push the state checkpoint**

```text
git add src/api/types.ts src/api/github.ts src/composables/usePullRequestDetail.ts src/composables/usePullRequestDetail.test.ts
git commit -m "feat: load pull request details"
git push origin main
```

---

### Task 3: Full-width PR section and complete field viewer

**Files:**
- Create: `src/components/GithubPullRequestDetailPanel.vue`
- Create: `src/components/GithubPullRequestDetailPanel.test.ts`
- Create: `src/components/GithubPullRequestSection.vue`
- Create: `src/components/GithubPullRequestSection.test.ts`
- Modify: `src/components/GithubWorkspacePanel.vue`
- Modify: `src/components/GithubWorkspacePanel.test.ts`
- Modify: `src/App.vue`
- Modify: `src/App.test.ts`
- Modify: `src/styles.css`

**Interfaces:**
- Consumes: `GithubPullRequest`, `GithubPullRequestDetail`, and the Task 2 composable
- Produces: full-width PR list/detail UI with `view`, `review`, `merge`, and `close-detail` events

- [ ] **Step 1: Write failing component tests**

For `GithubPullRequestDetailPanel.test.ts`, mount a detail containing scalar, null, empty array, object, and 46 total fields. Assert `.pr-detail-field` count is 46, the known fields appear in their expected groups, the unmatched field appears under “其他”, complex values use `<details>`, and close emits once.

For `GithubPullRequestSection.test.ts`, assert “详情” emits the full PR object; comment, approve, request-changes, and merge emit their exact tuples; all buttons are disabled when busy; and the detail panel appears only when selection and detail are present.

- [ ] **Step 2: Run component tests and verify RED**

Run:

```text
npm run test:frontend -- src/components/GithubPullRequestDetailPanel.test.ts src/components/GithubPullRequestSection.test.ts
```

Expected: FAIL because both components are missing.

- [ ] **Step 3: Implement the detail panel**

In `GithubPullRequestDetailPanel.vue`, define the seven exact field groups from the design as an `as const` array. Build groups with a `Map(detail.fields.map(...))`, delete matched names, then append remaining entries as “其他”. Render:

```vue
<article class="pr-detail-panel">
  <header class="pr-detail-heading">
    <div>
      <span class="section-label">PULL REQUEST DETAIL</span>
      <h4>#{{ pullRequest.number }} · {{ pullRequest.title }}</h4>
    </div>
    <button class="text-button" :disabled="busy" @click="emit('close')">关闭</button>
  </header>
  <section v-for="group in groups" :key="group.label" class="pr-detail-group">
    <h5>{{ group.label }}</h5>
    <div v-for="field in group.fields" :key="field.name" class="pr-detail-field">
      <span>{{ field.name }}</span>
      <details v-if="isComplex(field.value)">
        <summary>{{ valueSummary(field.value) }}</summary>
        <pre>{{ JSON.stringify(field.value, null, 2) }}</pre>
      </details>
      <strong v-else>{{ scalarText(field.value) }}</strong>
    </div>
  </section>
</article>
```

`scalarText(null)` returns `空`; booleans return `是`/`否`; strings and numbers use `String(value)`. `valueSummary` returns `数组 · N 项` or `对象 · N 项`.

- [ ] **Step 4: Implement the extracted PR section and workspace replacement**

Move the existing PR list markup from `GithubWorkspacePanel.vue` into `GithubPullRequestSection.vue`, add “详情” and “请求修改” buttons, and render `GithubPullRequestDetailPanel` below the list. Replace the original inline PR `<section>` with the new component and forward exact events.

Add workspace props `selectedPullRequest` and `pullRequestDetail`, plus emits `view-pr` and `close-pr-detail`. Keep the remaining Issue/Workflow/Run/Release grid unchanged.

- [ ] **Step 5: Wire the root component without crossing 250 effective lines**

Import and initialize `usePullRequestDetail` in `src/App.vue` with `selectedProject`, `githubOverview`, and `runProjectAction`. Pass selection/detail to `GithubWorkspacePanel`, connect `@view-pr="viewPullRequest"` and `@close-pr-detail="clearPullRequestDetail"`, and keep the root at or below 250 effective lines by using the existing compact orchestration formatting.

Update App/component test stubs with the new props and emitted events; do not weaken required props to optional.

- [ ] **Step 6: Add minimal cohesive CSS**

Reuse `.github-row`, `.github-row-actions`, `.text-button`, `.small-action`, existing input/pre colors, and existing focus styles. Add only layout rules for `.github-pr-section`, `.pr-detail-panel`, `.pr-detail-groups`, `.pr-detail-group`, `.pr-detail-field`, and nested `<details>/<pre>`. At `max-width: 1100px`, collapse detail groups to one column. Do not add animation or new color tokens.

- [ ] **Step 7: Verify GREEN and full frontend gates**

Run:

```text
npm run test:frontend
npm run check
npm run build
git diff --check
```

Expected: all frontend tests pass without Vue warnings, type checking succeeds, Vite builds, and the diff has no whitespace errors.

- [ ] **Step 8: Perform the major UI/code review and checkpoint commit**

Review all changed files for single responsibility, full 46-field rendering, stale-state prevention, accessible names, busy behavior, duplicate CSS, and effective line limits. Fix every critical/important issue before proceeding.

```text
git add src/components/GithubPullRequestDetailPanel.vue src/components/GithubPullRequestDetailPanel.test.ts src/components/GithubPullRequestSection.vue src/components/GithubPullRequestSection.test.ts src/components/GithubWorkspacePanel.vue src/components/GithubWorkspacePanel.test.ts src/App.vue src/App.test.ts src/styles.css
git commit -m "feat: display pull request details"
git push origin main
```

---

### Task 4: Runtime verification and TODO completion

**Files:**
- Modify: `docs/TODO.md`

**Interfaces:**
- Produces: verified Pull Request detail milestone and rollback checkpoint

- [ ] **Step 1: Run fresh full-project verification**

Run:

```text
npm run test:frontend
npm run build
cargo test --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
git diff --check
```

Expected: zero failing tests, successful build, zero formatter/Clippy/diff errors.

- [ ] **Step 2: Verify the real component visually**

Run the Tauri/Vite surface, mount the real PR section with a fixture containing all value shapes, and inspect at 960px, 1280px, and maximized width. Verify no clipping, nested-scroll trap, unreadable JSON, abnormal wrapping, missing focus outline, or style mismatch. Exercise open, expand, close, and busy states.

- [ ] **Step 3: Mark the TODO item complete**

Change only:

```markdown
- [x] Pull Request 详情
```

- [ ] **Step 4: Final code review, commit, and push**

Re-read the design and implementation plan requirement by requirement, inspect `git diff`, confirm the worktree contains no visual-test artifacts, then:

```text
git add docs/TODO.md
git commit -m "docs: complete pull request details"
git push origin main
```
