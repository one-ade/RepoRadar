use std::sync::Arc;

use tauri::State;

use crate::{database::Database, environment};

#[tauri::command]
pub async fn get_environment(
    database: State<'_, Arc<Database>>,
) -> Result<environment::EnvironmentStatus, String> {
    let database_ready = database.is_ready();
    tauri::async_runtime::spawn_blocking(move || environment::detect_environment(database_ready))
        .await
        .map_err(|error| error.to_string())
}
