use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{
    GithubSecret, GithubVariable,
    client::{gh_json, gh_json_list, gh_output, repository_reference, required},
};

#[derive(Deserialize)]
struct GithubEnvironmentPage {
    environments: Vec<GithubEnvironment>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all(deserialize = "snake_case", serialize = "camelCase"))]
pub struct GithubEnvironment {
    pub id: u64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub protection_rules: Vec<serde_json::Value>,
    pub deployment_branch_policy: Option<serde_json::Value>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubEnvironmentConfiguration {
    pub variables: Vec<GithubVariable>,
    pub secrets: Vec<GithubSecret>,
}

pub fn environments(path: &Path) -> Result<Vec<GithubEnvironment>, String> {
    let reference = repository_reference(path)?;
    let pages: Vec<GithubEnvironmentPage> = gh_json(
        &reference,
        path,
        &[
            "api",
            "--paginate",
            "--slurp",
            "repos/{owner}/{repo}/environments?per_page=100",
        ],
    )?;
    Ok(pages
        .into_iter()
        .flat_map(|page| page.environments)
        .collect())
}

pub fn environment_configuration(
    path: &Path,
    environment: &str,
) -> Result<GithubEnvironmentConfiguration, String> {
    let reference = repository_reference(path)?;
    let environment = required(environment, "部署环境")?;
    Ok(GithubEnvironmentConfiguration {
        variables: gh_json_list(
            &reference,
            path,
            &[
                "variable",
                "list",
                "--env",
                environment,
                "--json",
                "name,value,updatedAt",
            ],
        )?,
        secrets: gh_json_list(
            &reference,
            path,
            &[
                "secret",
                "list",
                "--env",
                environment,
                "--json",
                "name,updatedAt",
            ],
        )?,
    })
}

pub fn save_environment(path: &Path, name: &str) -> Result<(), String> {
    let reference = repository_reference(path)?;
    gh_output(
        &reference,
        path,
        &["api", "--method", "PUT", &environment_endpoint(name)?],
    )
    .map(|_| ())
}

pub fn delete_environment(path: &Path, name: &str) -> Result<(), String> {
    let reference = repository_reference(path)?;
    gh_output(
        &reference,
        path,
        &["api", "--method", "DELETE", &environment_endpoint(name)?],
    )
    .map(|_| ())
}

fn environment_endpoint(name: &str) -> Result<String, String> {
    let name = required(name, "部署环境")?;
    let encoded = name.bytes().fold(String::new(), |mut output, byte| {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            output.push(byte as char);
        } else {
            output.push_str(&format!("%{byte:02X}"));
        }
        output
    });
    Ok(format!("repos/{{owner}}/{{repo}}/environments/{encoded}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_environment_names_for_api_paths() {
        assert_eq!(
            environment_endpoint("Production / CN").unwrap(),
            "repos/{owner}/{repo}/environments/Production%20%2F%20CN"
        );
        assert!(environment_endpoint("  ").is_err());
    }

    #[test]
    fn maps_rest_field_names_to_frontend_field_names() {
        let environment: GithubEnvironment = serde_json::from_str(
            r#"{"id":1,"name":"production","created_at":"2026-08-01","updated_at":"2026-08-02","protection_rules":[],"deployment_branch_policy":null}"#,
        )
        .expect("GitHub REST environment");
        let json = serde_json::to_value(environment).expect("frontend environment");

        assert_eq!(json["createdAt"], "2026-08-01");
        assert_eq!(json["deploymentBranchPolicy"], serde_json::Value::Null);
    }
}
