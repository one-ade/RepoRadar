use std::path::PathBuf;

use crate::git::{self, GitBranch, GitCommit, GitStatus};

#[tauri::command]
pub async fn get_git_status(path: PathBuf) -> Result<GitStatus, String> {
    tauri::async_runtime::spawn_blocking(move || git::status(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn stage_all(path: PathBuf) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || git::stage_all(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn unstage_all(path: PathBuf) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || git::unstage_all(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn fetch(path: PathBuf) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || git::fetch(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn commit(path: PathBuf, message: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || git::commit(&path, &message))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn pull(path: PathBuf) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || git::pull(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn push(path: PathBuf) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || git::push(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_git_diff(path: PathBuf, staged: bool) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || git::diff(&path, staged))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn list_branches(path: PathBuf) -> Result<Vec<GitBranch>, String> {
    tauri::async_runtime::spawn_blocking(move || git::branches(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn switch_branch(path: PathBuf, branch: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || git::switch_branch(&path, &branch))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn create_branch(path: PathBuf, branch: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || git::create_branch(&path, &branch))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn delete_branch(path: PathBuf, branch: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || git::delete_branch(&path, &branch))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn git_log(path: PathBuf, limit: u32) -> Result<Vec<GitCommit>, String> {
    tauri::async_runtime::spawn_blocking(move || git::log(&path, limit))
        .await
        .map_err(|error| error.to_string())?
}
