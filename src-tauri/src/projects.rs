use std::{
    collections::{HashSet, VecDeque},
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
};

use serde::Serialize;

use crate::database::Database;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i64,
    pub path: String,
    pub name: String,
    pub favorite: bool,
    pub tags: Vec<String>,
    pub last_seen_at: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanRoot {
    pub id: i64,
    pub path: String,
    pub max_depth: u32,
    pub enabled: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathInspection {
    pub path: String,
    pub name: String,
    pub repository_kind: Option<RepositoryKind>,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RepositoryKind {
    Standard,
    Worktree,
    Bare,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanSummary {
    pub found: usize,
    pub skipped: usize,
    pub cancelled: bool,
    pub projects: Vec<Project>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanProgress {
    pub scanned: usize,
    pub found: usize,
    pub skipped: usize,
    pub current_path: String,
}

pub fn inspect_path(path: &Path) -> Result<PathInspection, String> {
    let canonical_path = canonical_directory(path)?;
    Ok(PathInspection {
        name: project_name(&canonical_path),
        path: canonical_path.to_string_lossy().into_owned(),
        repository_kind: repository_kind(&canonical_path),
    })
}

pub fn add_project(database: &Database, path: &Path, initialize: bool) -> Result<Project, String> {
    let canonical_path = canonical_directory(path)?;
    if repository_kind(&canonical_path).is_none() {
        if !initialize {
            return Err("NOT_A_GIT_REPOSITORY".to_owned());
        }
        initialize_repository(&canonical_path)?;
    }

    database.upsert_project(&canonical_path, &project_name(&canonical_path))
}

pub fn add_scan_root(database: &Database, path: &Path, max_depth: u32) -> Result<ScanRoot, String> {
    if !(1..=64).contains(&max_depth) {
        return Err("扫描深度必须在 1 到 64 之间".to_owned());
    }
    let canonical_path = canonical_directory(path)?;
    database.upsert_scan_root(&canonical_path, max_depth)
}

pub fn scan_projects_with_progress<F>(
    database: &Database,
    cancelled: &AtomicBool,
    mut on_progress: F,
) -> Result<ScanSummary, String>
where
    F: FnMut(ScanProgress),
{
    let mut seen = HashSet::new();
    let mut paths = Vec::new();
    let mut skipped = 0;
    let mut scanned = 0;

    for root in database.list_scan_roots()? {
        let (found_paths, skipped_paths, was_cancelled) = scan_root_with_progress(
            Path::new(&root.path),
            root.max_depth,
            cancelled,
            &mut |path| {
                scanned += 1;
                on_progress(ScanProgress {
                    scanned,
                    found: paths.len(),
                    skipped,
                    current_path: path.to_string_lossy().into_owned(),
                });
            },
        );
        skipped += skipped_paths;
        for path in found_paths {
            if seen.insert(path.clone()) {
                paths.push(path);
            }
        }
        if was_cancelled {
            break;
        }
    }

    for path in &paths {
        database.upsert_project(path, &project_name(path))?;
    }

    Ok(ScanSummary {
        found: paths.len(),
        skipped,
        cancelled: cancelled.load(Ordering::Relaxed),
        projects: database.list_projects()?,
    })
}

fn canonical_directory(path: &Path) -> Result<PathBuf, String> {
    let path = dunce::canonicalize(path).map_err(|error| error.to_string())?;
    if !path.is_dir() {
        return Err("所选路径不是目录".to_owned());
    }
    Ok(path)
}

fn initialize_repository(path: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .args(["-C"])
        .arg(path)
        .arg("init")
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
    }
}

fn repository_kind(path: &Path) -> Option<RepositoryKind> {
    let git_path = path.join(".git");
    if git_path.is_dir() {
        Some(RepositoryKind::Standard)
    } else if git_path.is_file() {
        Some(RepositoryKind::Worktree)
    } else if path.join("HEAD").is_file()
        && path.join("objects").is_dir()
        && path.join("refs").is_dir()
    {
        Some(RepositoryKind::Bare)
    } else {
        None
    }
}

fn project_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned())
}

fn scan_root_with_progress<F>(
    root: &Path,
    max_depth: u32,
    cancelled: &AtomicBool,
    on_progress: &mut F,
) -> (Vec<PathBuf>, usize, bool)
where
    F: FnMut(&Path),
{
    let mut queue = VecDeque::from([(root.to_path_buf(), 0)]);
    let mut projects = Vec::new();
    let mut skipped = 0;

    while let Some((path, depth)) = queue.pop_front() {
        if cancelled.load(Ordering::Relaxed) {
            return (projects, skipped, true);
        }
        on_progress(&path);
        if repository_kind(&path).is_some() {
            projects.push(path);
            continue;
        }
        if depth >= max_depth {
            continue;
        }

        let Ok(entries) = fs::read_dir(&path) else {
            skipped += 1;
            continue;
        };
        for entry in entries.flatten() {
            if cancelled.load(Ordering::Relaxed) {
                return (projects, skipped, true);
            }
            let Ok(file_type) = entry.file_type() else {
                skipped += 1;
                continue;
            };
            if file_type.is_dir() && !file_type.is_symlink() && !should_skip(&entry.file_name()) {
                queue.push_back((entry.path(), depth + 1));
            }
        }
    }

    (projects, skipped, false)
}

fn should_skip(name: &OsStr) -> bool {
    matches!(
        name.to_string_lossy().to_ascii_lowercase().as_str(),
        ".git"
            | ".cache"
            | ".idea"
            | ".venv"
            | "$recycle.bin"
            | "appdata"
            | "build"
            | "dist"
            | "node_modules"
            | "program files"
            | "program files (x86)"
            | "programdata"
            | "system volume information"
            | "target"
            | "venv"
            | "windows"
    )
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn scan_detects_repository_kinds_and_skips_dependencies() {
        let root = test_directory();
        let standard = root.join("standard");
        let worktree = root.join("worktree");
        let bare = root.join("bare.git");
        let ignored = root.join("node_modules").join("ignored");
        fs::create_dir_all(standard.join(".git")).unwrap();
        fs::create_dir_all(&worktree).unwrap();
        fs::write(worktree.join(".git"), "gitdir: ../actual").unwrap();
        fs::create_dir_all(bare.join("objects")).unwrap();
        fs::create_dir_all(bare.join("refs")).unwrap();
        fs::write(bare.join("HEAD"), "ref: refs/heads/main").unwrap();
        fs::create_dir_all(ignored.join(".git")).unwrap();

        let cancelled = AtomicBool::new(false);
        let (mut projects, skipped, _) = scan_root_with_progress(&root, 4, &cancelled, &mut |_| {});
        projects.sort();

        assert_eq!(projects, vec![bare, standard, worktree]);
        assert_eq!(skipped, 0);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn add_project_initializes_and_persists_non_git_directory() {
        let root = test_directory();
        let project_path = root.join("new-project");
        let database = Database::open(&root.join("database")).unwrap();
        fs::create_dir_all(&project_path).unwrap();

        let project = add_project(&database, &project_path, true).unwrap();

        assert!(project_path.join(".git").is_dir());
        assert_eq!(project.name, "new-project");
        assert_eq!(database.list_projects().unwrap().len(), 1);
        database.set_project_favorite(project.id, true).unwrap();
        assert!(database.list_projects().unwrap()[0].favorite);
        drop(database);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn scan_stops_when_cancelled_before_start() {
        let root = test_directory();
        fs::create_dir_all(root.join("repo").join(".git")).unwrap();
        let cancelled = AtomicBool::new(true);

        let (projects, skipped, was_cancelled) =
            scan_root_with_progress(&root, 4, &cancelled, &mut |_| {});

        assert!(projects.is_empty());
        assert_eq!(skipped, 0);
        assert!(was_cancelled);
        fs::remove_dir_all(root).unwrap();
    }

    fn test_directory() -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("repo-radar-{suffix}"));
        fs::create_dir_all(&path).unwrap();
        path
    }
}
