# Project Tags Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add persistent multi-tag editing, display, and search for discovered projects.

**Architecture:** Extend the existing SQLite `projects` model with a normalized `project_tags` table and expose one structured Tauri command that replaces a project's tags transactionally. Reuse the current Vue API facade, project-discovery composable, project catalog, operation runner, and global search without adding dependencies or global tag state.

**Tech Stack:** Rust 2024, rusqlite, Tauri 2, Vue 3, TypeScript 5.9, Vitest 4, CSS.

## Global Constraints

- Each project stores at most 8 tags.
- Trim tag names; reject empty names and names longer than 20 characters.
- Deduplicate tags case-insensitively while preserving the first spelling.
- Scans and project upserts must preserve tags.
- No colors, global tag manager, bulk editing, rename workflow, or new dependency.
- All controls retain existing busy, focus, hover, active, and reduced-motion behavior.

---

### Task 1: Persist and expose project tags

**Files:**
- Modify: `src-tauri/src/projects.rs`
- Modify: `src-tauri/src/database.rs`
- Modify: `src-tauri/src/commands/projects.rs`
- Modify: `src-tauri/src/lib.rs`

**Interfaces:**
- Consumes: existing `Database`, `Project`, and `set_project_favorite` command pattern.
- Produces: `Project.tags: Vec<String>`, `Database::set_project_tags(id, tags) -> Result<Project, String>`, and Tauri command `set_project_tags(id, tags) -> Project`.

- [ ] **Step 1: Add failing database tests**

Add tests that create a temporary `Database`, upsert a project, save `[' frontend ', 'FRONTEND', 'rust']`, and assert the returned/listed tags are `['frontend', 'rust']`. Add boundary assertions for 9 tags, an empty tag, and a 21-character tag. Extend the schema test to expect `project_tags` and `DATABASE_VERSION == 2`.

```rust
#[test]
fn project_tags_are_validated_deduplicated_and_persisted() {
    let directory = std::env::temp_dir().join(format!(
        "repo-radar-project-tags-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
    ));
    std::fs::create_dir_all(&directory).unwrap();
    let database = Database::open(&directory).unwrap();
    let project = database.upsert_project(&directory, "RepoRadar").unwrap();

    let updated = database
        .set_project_tags(
            project.id,
            vec![" frontend ".into(), "FRONTEND".into(), "rust".into()],
        )
        .unwrap();

    assert_eq!(updated.tags, vec!["frontend", "rust"]);
    assert_eq!(database.list_projects().unwrap()[0].tags, updated.tags);
    assert!(database.set_project_tags(project.id, vec![" ".into()]).is_err());
    assert!(database.set_project_tags(project.id, vec!["x".repeat(21)]).is_err());
    assert!(database.set_project_tags(project.id, (0..9).map(|n| n.to_string()).collect()).is_err());
    std::fs::remove_dir_all(directory).unwrap();
}
```

- [ ] **Step 2: Run the focused Rust test and confirm failure**

Run: `cargo test --manifest-path src-tauri/Cargo.toml project_tags_are_validated_deduplicated_and_persisted`

Expected: compilation fails because `set_project_tags` and `Project.tags` do not exist.

- [ ] **Step 3: Implement the minimal schema and database methods**

Set `DATABASE_VERSION` to `2`; add `tags: Vec<String>` to `Project`; create the table below in the existing idempotent migration.

```sql
CREATE TABLE IF NOT EXISTS project_tags (
    project_id INTEGER NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name TEXT NOT NULL COLLATE NOCASE CHECK (length(name) BETWEEN 1 AND 20),
    PRIMARY KEY (project_id, name)
);
```

Add a private `project_tags(connection, project_id)` query ordered by `rowid`, use it when mapping `list_projects` and `upsert_project`, and implement transactional replacement:

```rust
pub fn set_project_tags(&self, id: i64, tags: Vec<String>) -> Result<Project, String> {
    let mut tags = tags.into_iter().map(|tag| tag.trim().to_owned()).collect::<Vec<_>>();
    if tags.iter().any(|tag| tag.is_empty() || tag.chars().count() > 20) {
        return Err("每个项目最多 8 个标签，且每个标签须为 1 至 20 个字符".into());
    }
    let mut seen = std::collections::HashSet::new();
    tags.retain(|tag| seen.insert(tag.to_lowercase()));
    if tags.len() > 8 {
        return Err("每个项目最多 8 个标签，且每个标签须为 1 至 20 个字符".into());
    }

    let mut connection = self.connection()?;
    let transaction = connection.transaction().map_err(|error| error.to_string())?;
    transaction.execute("DELETE FROM project_tags WHERE project_id = ?1", [id])
        .map_err(|error| error.to_string())?;
    for tag in &tags {
        transaction.execute(
            "INSERT INTO project_tags (project_id, name) VALUES (?1, ?2)",
            rusqlite::params![id, tag],
        ).map_err(|error| error.to_string())?;
    }
    transaction.commit().map_err(|error| error.to_string())?;
    self.list_projects()?.into_iter().find(|project| project.id == id)
        .ok_or_else(|| "项目不存在".into())
}
```

- [ ] **Step 4: Add and register the structured command**

```rust
#[tauri::command]
pub async fn set_project_tags(
    database: State<'_, Arc<Database>>,
    id: i64,
    tags: Vec<String>,
) -> Result<Project, String> {
    let database = Arc::clone(database.inner());
    tauri::async_runtime::spawn_blocking(move || database.set_project_tags(id, tags))
        .await
        .map_err(|error| error.to_string())?
}
```

Register `commands::projects::set_project_tags` next to `set_project_favorite`.

- [ ] **Step 5: Verify, review, commit, and push backend work**

Run: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`

Run: `cargo test --manifest-path src-tauri/Cargo.toml`

Run: `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`

Expected: all commands exit successfully. Review the full diff for duplicated row mapping, unnecessary helpers, unrelated formatting, and scan/upsert tag preservation.

```bash
git add src-tauri/src/projects.rs src-tauri/src/database.rs src-tauri/src/commands/projects.rs src-tauri/src/lib.rs
git commit -m "feat: persist project tags"
git push origin main
```

### Task 2: Wire tag updates and search through the frontend

**Files:**
- Modify: `src/api/types.ts`
- Modify: `src/api/projects.ts`
- Modify: `src/composables/useProjectDiscovery.ts`
- Modify: `src/composables/useProjectDiscovery.test.ts`
- Modify: `src/App.vue`
- Modify: `src/App.test.ts`

**Interfaces:**
- Consumes: backend `set_project_tags`, existing `runAction`, `projects`, `selectedProject`, and `searchQuery`.
- Produces: `setProjectTags(id, tags)`, `updateTags(project, tags) -> Promise<boolean>`, and search matching project tags.

- [ ] **Step 1: Add failing composable and search tests**

Extend API mocks with `setProjectTags`. Assert `updateTags(project, ['rust'])` calls the API once and replaces both the list item and selected project reference with the returned object. Mount `App` with a project tagged `frontend`, enter `frontend` through the `AppHeader` stub model, and assert `ProjectCatalog.visibleProjects` contains that project.

```ts
it("updates tags through one project action", async () => {
  const updated = { ...project, tags: ["rust"] };
  api.setProjectTags.mockResolvedValue(updated);
  const discovery = useProjectDiscovery(runAction, chooseDirectory, notify, reportError);
  discovery.projects.value = [project];
  discovery.selectedProject.value = project;

  await discovery.updateTags(project, ["rust"]);

  expect(api.setProjectTags).toHaveBeenCalledWith(project.id, ["rust"]);
  expect(discovery.projects.value[0]).toEqual(updated);
  expect(discovery.selectedProject.value).toEqual(updated);
});
```

- [ ] **Step 2: Run focused frontend tests and confirm failure**

Run: `npm run test:frontend -- src/composables/useProjectDiscovery.test.ts src/App.test.ts`

Expected: failures because `tags`, `setProjectTags`, and `updateTags` do not exist and search ignores tags.

- [ ] **Step 3: Implement the typed API and state update**

```ts
export interface Project {
  id: number;
  path: string;
  name: string;
  favorite: boolean;
  tags: string[];
  lastSeenAt: string;
}

export function setProjectTags(id: number, tags: string[]): Promise<Project> {
  return invoke<Project>("set_project_tags", { id, tags });
}
```

In `useProjectDiscovery`, call the API through the existing `runAction`, replace the matching list element, and update `selectedProject` only when its id matches. Track whether the action body completed and return `Promise<boolean>` so the editor closes only after a successful save. Return `updateTags` from the composable.

- [ ] **Step 4: Include tags in existing search and wire the catalog event**

```ts
return projects.value.filter((project) =>
  project.name.toLocaleLowerCase().includes(query) ||
  project.path.toLocaleLowerCase().includes(query) ||
  project.tags.some((tag) => tag.toLocaleLowerCase().includes(query)),
);
```

Destructure `updateTags` and bind `:save-tags="updateTags"` on `ProjectCatalog`.

- [ ] **Step 5: Verify, review, commit, and push frontend state work**

Run: `npm run test:frontend -- src/composables/useProjectDiscovery.test.ts src/App.test.ts`

Run: `npm run check`

Expected: all commands exit successfully. Review that state replacement is localized and no new store or helper was added.

```bash
git add src/api/types.ts src/api/projects.ts src/composables/useProjectDiscovery.ts src/composables/useProjectDiscovery.test.ts src/App.vue src/App.test.ts
git commit -m "feat: wire project tag updates"
git push origin main
```

### Task 3: Add the project tag editor and complete the milestone item

**Files:**
- Modify: `src/components/ProjectCatalog.vue`
- Modify: `src/components/ProjectCatalog.test.ts`
- Modify: `src/styles.css`
- Modify: `docs/TODO.md`

**Interfaces:**
- Consumes: `Project.tags`, `busy`, and `saveTags(project, tags) -> Promise<boolean>` wiring from Task 2.
- Produces: accessible inline editing that retains the draft when persistence fails.

- [ ] **Step 1: Add failing component tests**

Assert chips render, the edit button opens a comma-separated input, cancel closes without saving, save passes parsed values to the callback, failed saves retain the draft, and all tag controls are disabled while busy.

```ts
it("edits project tags inline", async () => {
  const tagged = { ...project, tags: ["rust", "desktop"] };
  const saveTags = vi.fn().mockResolvedValue(true);
  const wrapper = mount(ProjectCatalog, {
    props: { projects: [tagged], visibleProjects: [tagged], busy: false, saveTags },
  });

  expect(wrapper.findAll(".project-tag").map((tag) => tag.text())).toEqual(["rust", "desktop"]);
  await wrapper.get(".tag-edit-button").trigger("click");
  await wrapper.get(".tag-input").setValue("rust, tauri");
  await wrapper.get(".tag-save-button").trigger("click");

  expect(saveTags).toHaveBeenCalledWith(tagged, ["rust", "tauri"]);
  expect(wrapper.find(".tag-editor").exists()).toBe(false);
});
```

- [ ] **Step 2: Run the focused component test and confirm failure**

Run: `npm run test:frontend -- src/components/ProjectCatalog.test.ts`

Expected: editor selectors and the `saveTags` prop are missing.

- [ ] **Step 3: Implement one inline editor with local draft state**

Use `editingId = ref<number>()` and `tagDraft = ref('')`. Opening copies `project.tags.join(', ')`; saving splits on ASCII and Chinese commas, trims values, awaits `saveTags`, and closes only on success. Keep validation authoritative in Rust; set `maxlength="167"` to bound the raw input without duplicating tag rules.

```ts
const props = defineProps<{
  projects: Project[];
  visibleProjects: Project[];
  selectedId?: number;
  busy: boolean;
  saveTags: (project: Project, tags: string[]) => Promise<boolean>;
}>();

async function submitTags(project: Project) {
  const tags = tagDraft.value.split(/[,，]/).map((tag) => tag.trim()).filter(Boolean);
  if (await props.saveTags(project, tags)) editingId.value = undefined;
}
```

Render chips below the path, an icon-sized text button with `aria-label="编辑 {name} 的标签"`, an inline format hint, and an inline form with visible “保存”“取消” buttons. Prevent row selection from editor clicks.

- [ ] **Step 4: Add minimal matching styles and update TODO**

Add only `.project-tags`, `.project-tag`, `.tag-edit-button`, `.tag-editor`, `.tag-input`, `.tag-save-button`, and `.tag-cancel-button`. Reuse existing colors, border radii, font sizes, button interaction rules, and the 1100px breakpoint. Mark `docs/TODO.md` project tags complete only after verification.

- [ ] **Step 5: Run full automated and rendered verification**

Run: `npm run test:frontend`

Run: `npm run check`

Run: `npm run build`

Run: `npm run test:rust`

Run: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`

Run: `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`

Expected: every command succeeds. Launch `npm run tauri dev`; verify add/edit/clear/search/persistence and keyboard focus at 960px, 1280px, and maximized widths. Inspect the final diff for redundant state, unused CSS, inconsistent copy, overflow, unrelated formatting, and missing TODO updates.

- [ ] **Step 6: Commit and push the completed major item**

```bash
git add src/components/ProjectCatalog.vue src/components/ProjectCatalog.test.ts src/styles.css docs/TODO.md
git commit -m "feat: add project tag editor"
git push origin main
```
