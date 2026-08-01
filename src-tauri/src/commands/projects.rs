use std::{
    path::PathBuf,
    sync::{Arc, atomic::Ordering},
};

use tauri::{Emitter, State};

use crate::{
    ScanController,
    database::Database,
    projects::{self, PathInspection, Project, ScanProgress, ScanRoot, ScanSummary},
};

#[tauri::command]
pub async fn list_projects(database: State<'_, Arc<Database>>) -> Result<Vec<Project>, String> {
    let database = Arc::clone(database.inner());
    tauri::async_runtime::spawn_blocking(move || database.list_projects())
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn inspect_project_path(path: PathBuf) -> Result<PathInspection, String> {
    tauri::async_runtime::spawn_blocking(move || projects::inspect_path(&path))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn add_project(
    database: State<'_, Arc<Database>>,
    path: PathBuf,
    initialize: bool,
) -> Result<Project, String> {
    let database = Arc::clone(database.inner());
    tauri::async_runtime::spawn_blocking(move || {
        projects::add_project(&database, &path, initialize)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn add_scan_root(
    database: State<'_, Arc<Database>>,
    path: PathBuf,
    max_depth: u32,
) -> Result<ScanRoot, String> {
    let database = Arc::clone(database.inner());
    tauri::async_runtime::spawn_blocking(move || {
        projects::add_scan_root(&database, &path, max_depth)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn scan_projects(
    app: tauri::AppHandle,
    controller: State<'_, Arc<ScanController>>,
    database: State<'_, Arc<Database>>,
) -> Result<ScanSummary, String> {
    if controller
        .running
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return Err("扫描已经在进行中".to_owned());
    }
    controller.cancelled.store(false, Ordering::Release);
    let controller = Arc::clone(controller.inner());
    let worker_controller = Arc::clone(&controller);
    let database = Arc::clone(database.inner());
    let result = tauri::async_runtime::spawn_blocking(move || {
        projects::scan_projects_with_progress(
            &database,
            &worker_controller.cancelled,
            |progress: ScanProgress| {
                let _ = app.emit("scan-progress", progress);
            },
        )
    })
    .await
    .map_err(|error| error.to_string());
    controller.running.store(false, Ordering::Release);
    result?
}

#[tauri::command]
pub async fn cancel_scan(controller: State<'_, Arc<ScanController>>) -> Result<(), String> {
    if !controller.running.load(Ordering::Acquire) {
        return Err("当前没有进行中的扫描".to_owned());
    }
    controller.cancelled.store(true, Ordering::Release);
    Ok(())
}

#[tauri::command]
pub async fn set_project_favorite(
    database: State<'_, Arc<Database>>,
    id: i64,
    favorite: bool,
) -> Result<(), String> {
    let database = Arc::clone(database.inner());
    tauri::async_runtime::spawn_blocking(move || database.set_project_favorite(id, favorite))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn set_project_tags(
    database: State<'_, Arc<Database>>,
    id: i64,
    tags: Vec<String>,
) -> Result<Project, String> {
    let database = Arc::clone(database.inner());
    tauri::async_runtime::spawn_blocking(move || database.set_project_tags(id, tags))
        .await
        .map_err(|error| error.to_string())?
}
