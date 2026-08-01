use std::path::Path;

use serde::{Deserialize, Serialize};

use super::client::{
    gh_json_list, gh_output, repository_owner, repository_reference, repository_slug, required,
};

#[derive(Deserialize)]
struct GithubProjectList {
    projects: Vec<GithubProject>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubCount {
    pub total_count: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubProject {
    pub number: u64,
    pub title: String,
    pub url: String,
    pub short_description: String,
    pub public: bool,
    pub closed: bool,
    pub id: String,
    pub items: GithubCount,
    pub fields: GithubCount,
}

#[derive(Deserialize)]
struct GithubDiscussionResponse {
    data: GithubDiscussionData,
}

#[derive(Deserialize)]
struct GithubDiscussionData {
    repository: Option<GithubDiscussionRepository>,
}

#[derive(Deserialize)]
struct GithubDiscussionRepository {
    discussions: GithubDiscussionConnection,
}

#[derive(Deserialize)]
struct GithubDiscussionConnection {
    nodes: Vec<GithubDiscussion>,
}

#[derive(Deserialize, Serialize)]
pub struct GithubLogin {
    pub login: String,
}

#[derive(Deserialize, Serialize)]
pub struct GithubDiscussionCategory {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubDiscussion {
    pub id: String,
    pub number: u64,
    pub title: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub is_answered: bool,
    pub answer_chosen_at: Option<String>,
    pub author: Option<GithubLogin>,
    pub category: GithubDiscussionCategory,
    pub comments: GithubCount,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubCodespace {
    pub name: String,
    pub display_name: String,
    pub state: String,
    pub machine_name: String,
    pub created_at: String,
    pub last_used_at: String,
    pub repository: serde_json::Value,
    pub git_status: serde_json::Value,
    pub owner: serde_json::Value,
}

const DISCUSSIONS_QUERY: &str = r#"query($owner:String!,$name:String!){repository(owner:$owner,name:$name){discussions(first:50,orderBy:{field:UPDATED_AT,direction:DESC}){nodes{id number title url createdAt updatedAt isAnswered answerChosenAt author{login} category{name} comments{totalCount}}}}}"#;

pub fn projects(path: &Path) -> Result<Vec<GithubProject>, String> {
    let reference = repository_reference(path)?;
    let owner = repository_owner(path)?;
    let output = gh_output(
        &reference,
        path,
        &[
            "project", "list", "--owner", &owner, "--closed", "--limit", "100", "--format", "json",
        ],
    )?;
    parse_projects(&output)
}

pub fn project_items(path: &Path, number: u64, query: &str) -> Result<serde_json::Value, String> {
    let reference = repository_reference(path)?;
    let owner = repository_owner(path)?;
    let number = number.to_string();
    let mut args = vec![
        "project",
        "item-list",
        &number,
        "--owner",
        &owner,
        "--limit",
        "100",
        "--format",
        "json",
    ];
    if !query.trim().is_empty() {
        args.extend(["--query", query.trim()]);
    }
    serde_json::from_str(&gh_output(&reference, path, &args)?).map_err(|error| error.to_string())
}

pub fn discussions(path: &Path) -> Result<Vec<GithubDiscussion>, String> {
    let reference = repository_reference(path)?;
    let parts = reference.split('/').collect::<Vec<_>>();
    let owner = format!("owner={}", parts[1]);
    let name = format!("name={}", parts[2]);
    let output = gh_output(
        &reference,
        path,
        &[
            "api",
            "graphql",
            "-F",
            &owner,
            "-F",
            &name,
            "-f",
            &format!("query={DISCUSSIONS_QUERY}"),
        ],
    )?;
    parse_discussions(&output)
}

pub fn codespaces(path: &Path) -> Result<Vec<GithubCodespace>, String> {
    let reference = repository_reference(path)?;
    let slug = repository_slug(path)?;
    gh_json_list(
        &reference,
        path,
        &[
            "codespace",
            "list",
            "--repo",
            &slug,
            "--limit",
            "100",
            "--json",
            "name,displayName,state,machineName,createdAt,lastUsedAt,repository,gitStatus,owner",
        ],
    )
}

pub fn codespace_log(path: &Path, name: &str) -> Result<String, String> {
    let reference = repository_reference(path)?;
    let args = codespace_args("logs", name, false)?;
    let arguments = args.iter().map(String::as_str).collect::<Vec<_>>();
    Ok(gh_output(&reference, path, &arguments)?
        .chars()
        .take(100_000)
        .collect())
}

pub fn stop_codespace(path: &Path, name: &str) -> Result<(), String> {
    run_codespace(path, "stop", name, false)
}

pub fn delete_codespace(path: &Path, name: &str, force: bool) -> Result<(), String> {
    run_codespace(path, "delete", name, force)
}

fn run_codespace(path: &Path, action: &str, name: &str, force: bool) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let args = codespace_args(action, name, force)?;
    let arguments = args.iter().map(String::as_str).collect::<Vec<_>>();
    gh_output(&reference, path, &arguments).map(|_| ())
}

fn parse_projects(raw: &str) -> Result<Vec<GithubProject>, String> {
    serde_json::from_str::<GithubProjectList>(raw)
        .map(|list| list.projects)
        .map_err(|error| error.to_string())
}

fn parse_discussions(raw: &str) -> Result<Vec<GithubDiscussion>, String> {
    serde_json::from_str::<GithubDiscussionResponse>(raw)
        .map(|response| {
            response
                .data
                .repository
                .map_or_else(Vec::new, |repository| repository.discussions.nodes)
        })
        .map_err(|error| error.to_string())
}

fn codespace_args(action: &str, name: &str, force: bool) -> Result<Vec<String>, String> {
    if !matches!(action, "stop" | "delete" | "logs") {
        return Err("不支持的 Codespace 操作".into());
    }
    let mut args = vec![
        "codespace".into(),
        action.into(),
        "--codespace".into(),
        required(name, "Codespace 名称")?.into(),
    ];
    if action == "delete" && force {
        args.push("--force".into());
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_project_and_discussion_lists() {
        let projects = parse_projects(r#"{"projects":[{"number":7,"title":"Roadmap","url":"https://github.com/users/acme/projects/7","shortDescription":"Plan","public":true,"closed":false,"id":"PVT_1","items":{"totalCount":3},"fields":{"totalCount":4}}],"totalCount":1}"#).unwrap();
        assert_eq!(projects[0].number, 7);
        assert_eq!(projects[0].items.total_count, 3);

        let discussions = parse_discussions(r#"{"data":{"repository":{"discussions":{"nodes":[{"id":"D_1","number":2,"title":"Ideas","url":"https://github.com/acme/repo/discussions/2","createdAt":"now","updatedAt":"now","isAnswered":false,"answerChosenAt":null,"author":{"login":"octocat"},"category":{"name":"Ideas"},"comments":{"totalCount":5}}]}}}}"#).unwrap();
        assert_eq!(discussions[0].comments.total_count, 5);
    }

    #[test]
    fn validates_codespace_lifecycle_arguments() {
        assert_eq!(
            codespace_args("stop", "silver-space", false).unwrap(),
            ["codespace", "stop", "--codespace", "silver-space"]
        );
        assert!(codespace_args("delete", " ", true).is_err());
        assert!(codespace_args("ssh", "silver-space", false).is_err());
    }
}
