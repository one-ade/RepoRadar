use std::path::PathBuf;

use crate::github::{self, GithubCodespace, GithubDiscussion, GithubProject};

#[tauri::command]
pub async fn get_github_projects(path: PathBuf) -> Result<Vec<GithubProject>, String> {
    tauri::async_runtime::spawn_blocking(move || github::projects(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_github_project_items(
    path: PathBuf,
    number: u64,
    query: String,
) -> Result<serde_json::Value, String> {
    tauri::async_runtime::spawn_blocking(move || github::project_items(&path, number, &query))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_github_discussions(path: PathBuf) -> Result<Vec<GithubDiscussion>, String> {
    tauri::async_runtime::spawn_blocking(move || github::discussions(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_github_codespaces(path: PathBuf) -> Result<Vec<GithubCodespace>, String> {
    tauri::async_runtime::spawn_blocking(move || github::codespaces(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_github_codespace_log(path: PathBuf, name: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || github::codespace_log(&path, &name))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn stop_github_codespace(path: PathBuf, name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::stop_codespace(&path, &name))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn delete_github_codespace(
    path: PathBuf,
    name: String,
    force: bool,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::delete_codespace(&path, &name, force))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn search_github(
    path: PathBuf,
    kind: String,
    query: String,
    current_repository: bool,
) -> Result<Vec<serde_json::Value>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::search(&path, &kind, &query, current_repository)
    })
    .await
    .map_err(|error| error.to_string())?
}
