use std::path::PathBuf;

use crate::github::{self, GithubReleaseDetail, GithubReleaseEdit};

#[tauri::command]
pub async fn get_release_detail(path: PathBuf, tag: String) -> Result<GithubReleaseDetail, String> {
    tauri::async_runtime::spawn_blocking(move || github::release_detail(&path, &tag))
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
pub async fn edit_github_release(
    path: PathBuf,
    tag: String,
    edit: GithubReleaseEdit,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::edit_release(&path, &tag, &edit))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn upload_github_release_assets(
    path: PathBuf,
    tag: String,
    files: Vec<String>,
    clobber: bool,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::upload_release_assets(&path, &tag, &files, clobber)
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
