mod actions;
mod advanced;
mod client;
mod collaboration;
mod configuration;
mod details;
mod environments;
mod issues;
mod overview;
mod releases;
mod repositories;
mod resources;
mod search;

use serde::{Deserialize, Serialize};

pub use actions::{
    cancel_run, check_default_branch_rules, download_run_artifacts, rerun, run_log, run_workflow,
};
pub use advanced::{GithubApiField, api_request, safe_command};
pub use collaboration::{create_pull_request, merge_pull_request, review_pull_request};
pub use configuration::{
    delete_environment_secret, delete_environment_variable, delete_label, delete_secret,
    delete_variable, save_label, set_environment_secret, set_environment_variable, set_secret,
    set_variable,
};
pub use details::{
    GithubIssueDetail, GithubPullRequestDetail, GithubReleaseDetail, issue_detail,
    pull_request_detail, release_detail,
};
pub use environments::{
    GithubEnvironment, GithubEnvironmentConfiguration, delete_environment,
    environment_configuration, environments, save_environment,
};
pub use issues::{GithubIssueEdit, close_issue, comment_issue, create_issue, edit_issue};
pub use overview::{configuration, overview};
pub use releases::{
    GithubReleaseEdit, create_release, download_release, edit_release, upload_release_assets,
};
pub use repositories::{clone_repository, create_repository, fork_repository, sync_repository};
pub use resources::{
    GithubCodespace, GithubDiscussion, GithubProject, codespace_log, codespaces, delete_codespace,
    discussions, project_items, projects, stop_codespace,
};
pub use search::search;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubOverview {
    pub repository: GithubRepository,
    pub pull_requests: Vec<GithubPullRequest>,
    pub issues: Vec<GithubIssue>,
    pub workflows: Vec<GithubWorkflow>,
    pub runs: Vec<GithubRun>,
    pub releases: Vec<GithubRelease>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubConfiguration {
    pub variables: Vec<GithubVariable>,
    pub secrets: Vec<GithubSecret>,
    pub labels: Vec<GithubLabel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubVariable {
    pub name: String,
    pub value: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubSecret {
    pub name: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubRepository {
    pub name_with_owner: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub default_branch_ref: Option<GithubBranchRef>,
    pub url: String,
    pub stargazer_count: u64,
    pub fork_count: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GithubBranchRef {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubPullRequest {
    pub number: u64,
    pub title: String,
    pub state: String,
    pub author: Option<GithubAuthor>,
    pub head_ref_name: String,
    pub base_ref_name: String,
    pub is_draft: bool,
    pub updated_at: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubIssue {
    pub number: u64,
    pub title: String,
    pub state: String,
    pub author: Option<GithubAuthor>,
    pub labels: Vec<GithubLabel>,
    pub updated_at: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct GithubAuthor {
    pub login: String,
}

#[derive(Serialize, Deserialize)]
pub struct GithubLabel {
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubRun {
    pub database_id: u64,
    pub name: Option<String>,
    pub display_title: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub head_branch: Option<String>,
    pub event: String,
    pub workflow_name: String,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubWorkflow {
    pub id: u64,
    pub name: String,
    pub state: String,
    pub path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubRelease {
    pub tag_name: String,
    pub name: Option<String>,
    pub is_draft: bool,
    pub is_latest: bool,
    pub is_prerelease: bool,
    pub published_at: Option<String>,
}
