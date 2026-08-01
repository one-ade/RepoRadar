use std::path::Path;

use serde::Deserialize;

use super::client::{gh_output, repository_reference, required};

#[derive(Deserialize)]
pub struct GithubApiField {
    key: String,
    value: String,
}

pub fn safe_command(path: &Path, command: &str, extra_args: &[String]) -> Result<String, String> {
    let reference = repository_reference(path)?;
    let args = safe_command_args(command, extra_args)?;
    let arguments = args.iter().map(String::as_str).collect::<Vec<_>>();
    Ok(gh_output(&reference, path, &arguments)?
        .chars()
        .take(200_000)
        .collect())
}

pub fn api_request(
    path: &Path,
    method: &str,
    endpoint: &str,
    fields: &[GithubApiField],
) -> Result<String, String> {
    let reference = repository_reference(path)?;
    let args = api_args(method, endpoint, fields)?;
    let arguments = args.iter().map(String::as_str).collect::<Vec<_>>();
    Ok(gh_output(&reference, path, &arguments)?
        .chars()
        .take(200_000)
        .collect())
}

fn safe_command_args(command: &str, extra_args: &[String]) -> Result<Vec<String>, String> {
    let base = match command {
        "repo-view" => ["repo", "view"],
        "pr-list" => ["pr", "list"],
        "issue-list" => ["issue", "list"],
        "run-list" => ["run", "list"],
        "workflow-list" => ["workflow", "list"],
        "release-list" => ["release", "list"],
        "label-list" => ["label", "list"],
        "variable-list" => ["variable", "list"],
        "secret-list" => ["secret", "list"],
        "ruleset-list" => ["ruleset", "list"],
        _ => return Err("该 gh 命令不在只读允许列表中".into()),
    };
    let mut args = base.map(str::to_owned).to_vec();
    for argument in extra_args {
        let argument = required(argument, "gh 参数")?;
        if matches!(argument, "--web" | "-w") || argument.starts_with("--web=") {
            return Err("安全命令面板不允许打开外部窗口".into());
        }
        args.push(argument.into());
    }
    Ok(args)
}

fn api_args(
    method: &str,
    endpoint: &str,
    fields: &[GithubApiField],
) -> Result<Vec<String>, String> {
    if !matches!(method, "GET" | "POST" | "PUT" | "PATCH" | "DELETE") {
        return Err("不支持的 GitHub API 方法".into());
    }
    let endpoint = required(endpoint, "GitHub API Endpoint")?;
    if endpoint.starts_with('-')
        || endpoint.contains(char::is_whitespace)
        || endpoint.contains("..")
        || endpoint.starts_with("http://")
        || endpoint.starts_with("https://")
    {
        return Err("GitHub API Endpoint 必须是相对路径".into());
    }
    let mut args = vec![
        "api".into(),
        "--method".into(),
        method.into(),
        endpoint.into(),
    ];
    for field in fields {
        let key = required(&field.key, "API 字段名")?;
        if !key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        {
            return Err("API 字段名只能包含字母、数字、下划线和连字符".into());
        }
        args.extend(["--raw-field".into(), format!("{key}={}", field.value)]);
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_builds_allowlisted_read_only_commands() {
        let args = safe_command_args("pr-list", &["--state".into(), "all".into()]).unwrap();
        assert_eq!(&args[..2], ["pr", "list"]);
        assert!(safe_command_args("pr-merge", &[]).is_err());
        assert!(safe_command_args("repo-view", &["--web".into()]).is_err());
        assert!(safe_command_args("repo-view", &["--web=true".into()]).is_err());
    }

    #[test]
    fn validates_api_methods_endpoints_and_fields() {
        let args = api_args(
            "GET",
            "repos/{owner}/{repo}",
            &[GithubApiField {
                key: "page".into(),
                value: "1".into(),
            }],
        )
        .unwrap();
        assert!(args.contains(&"--raw-field".into()));
        assert!(api_args("TRACE", "user", &[]).is_err());
        assert!(api_args("GET", "--input", &[]).is_err());
        assert!(api_args("GET", "https://example.com", &[]).is_err());
        assert!(
            api_args(
                "POST",
                "graphql",
                &[GithubApiField {
                    key: "bad key".into(),
                    value: "x".into()
                }]
            )
            .is_err()
        );
    }
}
