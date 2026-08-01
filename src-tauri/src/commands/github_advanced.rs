use std::path::PathBuf;

use crate::github::{self, GithubApiField};

#[tauri::command]
pub async fn run_safe_github_command(
    path: PathBuf,
    command: String,
    extra_args: Vec<String>,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || github::safe_command(&path, &command, &extra_args))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn run_github_api_request(
    path: PathBuf,
    method: String,
    endpoint: String,
    fields: Vec<GithubApiField>,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::api_request(&path, &method, &endpoint, &fields)
    })
    .await
    .map_err(|error| error.to_string())?
}
