use std::path::PathBuf;

use crate::github::{
    self, GithubConfiguration, GithubIssueDetail, GithubIssueEdit, GithubOverview,
    GithubPullRequestDetail,
};

#[tauri::command]
pub async fn get_github_overview(path: PathBuf) -> Result<GithubOverview, String> {
    tauri::async_runtime::spawn_blocking(move || github::overview(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_pull_request_detail(
    path: PathBuf,
    number: u64,
) -> Result<GithubPullRequestDetail, String> {
    tauri::async_runtime::spawn_blocking(move || github::pull_request_detail(&path, number))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_issue_detail(path: PathBuf, number: u64) -> Result<GithubIssueDetail, String> {
    tauri::async_runtime::spawn_blocking(move || github::issue_detail(&path, number))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_github_configuration(path: PathBuf) -> Result<GithubConfiguration, String> {
    tauri::async_runtime::spawn_blocking(move || github::configuration(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn set_github_variable(path: PathBuf, name: String, value: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::set_variable(&path, &name, &value))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn delete_github_variable(path: PathBuf, name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::delete_variable(&path, &name))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn set_github_secret(path: PathBuf, name: String, value: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::set_secret(&path, &name, &value))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn delete_github_secret(path: PathBuf, name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::delete_secret(&path, &name))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn save_github_label(
    path: PathBuf,
    name: String,
    color: String,
    description: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::save_label(&path, &name, &color, &description)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn delete_github_label(path: PathBuf, name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::delete_label(&path, &name))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn check_github_rulesets(path: PathBuf) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || github::check_default_branch_rules(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_github_run_log(path: PathBuf, database_id: u64) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || github::run_log(&path, database_id))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn rerun_github_run(path: PathBuf, database_id: u64) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::rerun(&path, database_id))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn run_github_workflow(path: PathBuf, workflow_id: u64) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::run_workflow(&path, workflow_id))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn cancel_github_run(path: PathBuf, database_id: u64) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::cancel_run(&path, database_id))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn download_github_artifacts(
    path: PathBuf,
    database_id: u64,
    target_dir: String,
    artifact_name: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::download_run_artifacts(&path, database_id, &target_dir, &artifact_name)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn create_github_release(
    path: PathBuf,
    tag: String,
    title: String,
    notes: String,
    draft: bool,
    prerelease: bool,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::create_release(&path, &tag, &title, &notes, draft, prerelease)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn download_github_release(
    path: PathBuf,
    tag: String,
    target_dir: String,
    pattern: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::download_release(&path, &tag, &target_dir, &pattern)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn fork_github_repository(path: PathBuf, organization: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || github::fork_repository(&path, &organization))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn sync_github_repository(path: PathBuf, branch: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::sync_repository(&path, &branch))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn clone_github_repository(
    reference: String,
    target_dir: String,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || github::clone_repository(&reference, &target_dir))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn create_github_repository(
    path: PathBuf,
    name: String,
    visibility: String,
    description: String,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::create_repository(&path, &name, &visibility, &description)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn create_pull_request(
    path: PathBuf,
    title: String,
    body: String,
    draft: bool,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::create_pull_request(&path, &title, &body, draft)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn review_pull_request(
    path: PathBuf,
    number: u64,
    action: String,
    body: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::review_pull_request(&path, number, &action, &body)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn merge_pull_request(
    path: PathBuf,
    number: u64,
    method: String,
    delete_branch: bool,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::merge_pull_request(&path, number, &method, delete_branch)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn create_issue(path: PathBuf, title: String, body: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || github::create_issue(&path, &title, &body))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn edit_issue(path: PathBuf, number: u64, edit: GithubIssueEdit) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::edit_issue(&path, number, &edit))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn comment_issue(path: PathBuf, number: u64, body: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::comment_issue(&path, number, &body))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn close_issue(path: PathBuf, number: u64) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::close_issue(&path, number))
        .await
        .map_err(|error| error.to_string())?
}
