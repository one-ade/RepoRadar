use std::path::PathBuf;

use crate::github::{self, GithubEnvironment, GithubEnvironmentConfiguration};

#[tauri::command]
pub async fn get_github_environments(path: PathBuf) -> Result<Vec<GithubEnvironment>, String> {
    tauri::async_runtime::spawn_blocking(move || github::environments(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_github_environment_configuration(
    path: PathBuf,
    environment: String,
) -> Result<GithubEnvironmentConfiguration, String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::environment_configuration(&path, &environment)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn save_github_environment(path: PathBuf, name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::save_environment(&path, &name))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn delete_github_environment(path: PathBuf, name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || github::delete_environment(&path, &name))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn set_github_environment_variable(
    path: PathBuf,
    environment: String,
    name: String,
    value: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::set_environment_variable(&path, &environment, &name, &value)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn delete_github_environment_variable(
    path: PathBuf,
    environment: String,
    name: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::delete_environment_variable(&path, &environment, &name)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn set_github_environment_secret(
    path: PathBuf,
    environment: String,
    name: String,
    value: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::set_environment_secret(&path, &environment, &name, &value)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn delete_github_environment_secret(
    path: PathBuf,
    environment: String,
    name: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        github::delete_environment_secret(&path, &environment, &name)
    })
    .await
    .map_err(|error| error.to_string())?
}
