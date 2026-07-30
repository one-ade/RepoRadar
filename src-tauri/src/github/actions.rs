use std::path::Path;

use super::client::{gh_output, repository_reference, required};

pub fn check_default_branch_rules(path: &Path) -> Result<String, String> {
    let reference = repository_reference(path)?;
    gh_output(&reference, path, &["ruleset", "check", "--default"])
}

pub fn run_log(path: &Path, database_id: u64) -> Result<String, String> {
    let reference = repository_reference(path)?;
    let database_id = database_id.to_string();
    let output = gh_output(&reference, path, &["run", "view", &database_id, "--log"])?;
    Ok(output.chars().take(100_000).collect())
}

pub fn rerun(path: &Path, database_id: u64) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let database_id = database_id.to_string();
    gh_output(&reference, path, &["run", "rerun", &database_id]).map(|_| ())
}

pub fn run_workflow(path: &Path, workflow_id: u64) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let workflow_id = workflow_id.to_string();
    gh_output(&reference, path, &["workflow", "run", &workflow_id]).map(|_| ())
}

pub fn cancel_run(path: &Path, database_id: u64) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let database_id = database_id.to_string();
    gh_output(&reference, path, &["run", "cancel", &database_id]).map(|_| ())
}

pub fn download_run_artifacts(
    path: &Path,
    database_id: u64,
    target_dir: &str,
    artifact_name: &str,
) -> Result<(), String> {
    let target_dir = required(target_dir, "Artifact 目标目录")?;
    let reference = repository_reference(path)?;
    let database_id = database_id.to_string();
    let mut args = vec!["run", "download", &database_id, "--dir", target_dir];
    if !artifact_name.trim().is_empty() {
        args.push("--name");
        args.push(artifact_name.trim());
    }
    gh_output(&reference, path, &args).map(|_| ())
}

pub fn create_release(
    path: &Path,
    tag: &str,
    title: &str,
    notes: &str,
    draft: bool,
    prerelease: bool,
) -> Result<String, String> {
    let tag = required(tag, "Release Tag")?;
    let reference = repository_reference(path)?;
    let mut args = vec!["release", "create", tag];
    if !title.trim().is_empty() {
        args.push("--title");
        args.push(title.trim());
    }
    if !notes.trim().is_empty() {
        args.push("--notes");
        args.push(notes.trim());
    } else {
        args.push("--generate-notes");
    }
    if draft {
        args.push("--draft");
    }
    if prerelease {
        args.push("--prerelease");
    }
    gh_output(&reference, path, &args)
}

pub fn download_release(
    path: &Path,
    tag: &str,
    target_dir: &str,
    pattern: &str,
) -> Result<(), String> {
    let tag = required(tag, "Release Tag")?;
    let target_dir = required(target_dir, "Release 目标目录")?;
    let reference = repository_reference(path)?;
    let mut args = vec!["release", "download", tag, "--dir", target_dir];
    if !pattern.trim().is_empty() {
        args.push("--pattern");
        args.push(pattern.trim());
    }
    args.push("--skip-existing");
    gh_output(&reference, path, &args).map(|_| ())
}
