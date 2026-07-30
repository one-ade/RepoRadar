use std::path::Path;

use super::client::{
    gh_output, gh_raw_output, repository_reference, repository_visibility_flag, required,
};

pub fn fork_repository(path: &Path, organization: &str) -> Result<String, String> {
    let reference = repository_reference(path)?;
    let mut args = vec!["repo", "fork", "--remote"];
    if !organization.trim().is_empty() {
        args.push("--org");
        args.push(organization.trim());
    }
    gh_output(&reference, path, &args)
}

pub fn sync_repository(path: &Path, branch: &str) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let mut args = vec!["repo", "sync"];
    if !branch.trim().is_empty() {
        args.push("--branch");
        args.push(branch.trim());
    }
    gh_output(&reference, path, &args).map(|_| ())
}

pub fn clone_repository(reference: &str, target_dir: &str) -> Result<String, String> {
    let reference = required(reference, "GitHub 仓库")?;
    let target_dir = required(target_dir, "克隆目标目录")?;
    gh_raw_output(Path::new("."), &["repo", "clone", reference, target_dir])
}

pub fn create_repository(
    path: &Path,
    name: &str,
    visibility: &str,
    description: &str,
) -> Result<String, String> {
    let name = required(name, "仓库名称")?;
    let visibility_flag = repository_visibility_flag(visibility)?;
    let mut args = vec![
        "repo",
        "create",
        name,
        visibility_flag,
        "--source",
        ".",
        "--push",
    ];
    if !description.trim().is_empty() {
        args.push("--description");
        args.push(description.trim());
    }
    gh_raw_output(path, &args)
}
