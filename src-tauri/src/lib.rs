mod commands;
mod database;
mod environment;
mod git;
mod github;
mod projects;

use std::sync::{Arc, atomic::AtomicBool};

use database::Database;
use tauri::Manager;

pub struct ScanController {
    pub(crate) running: AtomicBool,
    pub(crate) cancelled: AtomicBool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            let database = Database::open(&app_data_dir)?;
            app.manage(Arc::new(database));
            app.manage(Arc::new(ScanController {
                running: AtomicBool::new(false),
                cancelled: AtomicBool::new(false),
            }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::environment::get_environment,
            commands::projects::list_projects,
            commands::projects::inspect_project_path,
            commands::projects::add_project,
            commands::projects::add_scan_root,
            commands::projects::scan_projects,
            commands::projects::cancel_scan,
            commands::projects::set_project_favorite,
            commands::projects::set_project_tags,
            commands::git::get_git_status,
            commands::git::stage_all,
            commands::git::unstage_all,
            commands::git::fetch,
            commands::git::commit,
            commands::git::pull,
            commands::git::push,
            commands::git::get_git_diff,
            commands::git::list_branches,
            commands::git::switch_branch,
            commands::git::create_branch,
            commands::git::delete_branch,
            commands::git::git_log,
            commands::github::get_github_overview,
            commands::github::get_pull_request_detail,
            commands::github::get_issue_detail,
            commands::github::get_github_configuration,
            commands::github::set_github_variable,
            commands::github::delete_github_variable,
            commands::github::set_github_secret,
            commands::github::delete_github_secret,
            commands::github::save_github_label,
            commands::github::delete_github_label,
            commands::github::check_github_rulesets,
            commands::github::get_github_run_log,
            commands::github::rerun_github_run,
            commands::github::run_github_workflow,
            commands::github::cancel_github_run,
            commands::github::download_github_artifacts,
            commands::github::create_github_release,
            commands::github::download_github_release,
            commands::github::fork_github_repository,
            commands::github::sync_github_repository,
            commands::github::clone_github_repository,
            commands::github::create_github_repository,
            commands::github::create_pull_request,
            commands::github::review_pull_request,
            commands::github::merge_pull_request,
            commands::github::create_issue,
            commands::github::edit_issue,
            commands::github::comment_issue,
            commands::github::close_issue
        ])
        .run(tauri::generate_context!())
        .expect("error while running RepoRadar");
}
