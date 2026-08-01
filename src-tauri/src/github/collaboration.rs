use std::path::Path;

use super::client::{gh_output, merge_flag, repository_reference, required, review_flag};

pub fn create_pull_request(
    path: &Path,
    title: &str,
    body: &str,
    draft: bool,
) -> Result<String, String> {
    let title = required(title, "Pull Request 标题")?;
    let reference = repository_reference(path)?;
    let mut args = vec!["pr", "create", "--title", title, "--body", body.trim()];
    if draft {
        args.push("--draft");
    }
    gh_output(&reference, path, &args)
}

pub fn review_pull_request(
    path: &Path,
    number: u64,
    action: &str,
    body: &str,
) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let number = number.to_string();
    gh_output(
        &reference,
        path,
        &[
            "pr",
            "review",
            &number,
            review_flag(action)?,
            "--body",
            body.trim(),
        ],
    )
    .map(|_| ())
}

pub fn merge_pull_request(
    path: &Path,
    number: u64,
    method: &str,
    delete_branch: bool,
) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let number = number.to_string();
    let mut args = vec!["pr", "merge", &number, merge_flag(method)?];
    if delete_branch {
        args.push("--delete-branch");
    }
    gh_output(&reference, path, &args).map(|_| ())
}
