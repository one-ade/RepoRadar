use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use serde::Deserialize;

use crate::git;

pub(super) fn repository_reference(path: &Path) -> Result<String, String> {
    let remote = git::remote_url(path)?;
    parse_repository_reference(&remote).ok_or_else(|| "origin 不是 GitHub 仓库地址".to_owned())
}

pub(super) fn gh_json<T>(reference: &str, path: &Path, args: &[&str]) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_str(&gh_output(reference, path, args)?).map_err(|error| error.to_string())
}

pub(super) fn gh_json_list<T>(reference: &str, path: &Path, args: &[&str]) -> Result<Vec<T>, String>
where
    T: for<'de> Deserialize<'de>,
{
    parse_json_list(&gh_output(reference, path, args)?)
}

fn parse_json_list<T>(output: &str) -> Result<Vec<T>, String>
where
    T: for<'de> Deserialize<'de>,
{
    if output.is_empty() {
        return Ok(Vec::new());
    }
    serde_json::from_str(output).map_err(|error| error.to_string())
}

pub(super) fn gh_output(reference: &str, path: &Path, args: &[&str]) -> Result<String, String> {
    run_gh(path, args, Some(reference), None)
}

pub(super) fn gh_raw_output(path: &Path, args: &[&str]) -> Result<String, String> {
    run_gh(path, args, None, None)
}

pub(super) fn run_gh(
    path: &Path,
    args: &[&str],
    reference: Option<&str>,
    input: Option<&str>,
) -> Result<String, String> {
    let mut command = Command::new("gh");
    if let Some(reference) = reference {
        command.env("GH_REPO", reference);
    }
    command.current_dir(path).args(args);
    let output = if let Some(input) = input {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| error.to_string())?;
        child
            .stdin
            .take()
            .ok_or_else(|| "无法写入 gh 标准输入".to_owned())?
            .write_all(input.as_bytes())
            .map_err(|error| error.to_string())?;
        child
            .wait_with_output()
            .map_err(|error| error.to_string())?
    } else {
        command.output().map_err(|error| error.to_string())?
    };
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

pub(super) fn required<'a>(value: &'a str, label: &str) -> Result<&'a str, String> {
    let value = value.trim();
    (!value.is_empty())
        .then_some(value)
        .ok_or_else(|| format!("{label}不能为空"))
}

pub(super) fn label_color(value: &str) -> Result<&str, String> {
    let color = value.trim().strip_prefix('#').unwrap_or(value.trim());
    (color.len() == 6 && color.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then_some(color)
        .ok_or_else(|| "标签颜色必须是 6 位十六进制值".to_owned())
}

pub(super) fn review_flag(action: &str) -> Result<&'static str, String> {
    match action {
        "approve" => Ok("--approve"),
        "comment" => Ok("--comment"),
        "request-changes" => Ok("--request-changes"),
        _ => Err("不支持的 Review 操作".to_owned()),
    }
}

pub(super) fn merge_flag(method: &str) -> Result<&'static str, String> {
    match method {
        "merge" => Ok("--merge"),
        "squash" => Ok("--squash"),
        "rebase" => Ok("--rebase"),
        _ => Err("不支持的合并方式".to_owned()),
    }
}

pub(super) fn repository_visibility_flag(visibility: &str) -> Result<&'static str, String> {
    match visibility {
        "public" => Ok("--public"),
        "private" => Ok("--private"),
        "internal" => Ok("--internal"),
        _ => Err("不支持的仓库可见性".to_owned()),
    }
}

fn parse_repository_reference(remote: &str) -> Option<String> {
    let remote = remote.trim().trim_end_matches('/');
    let without_scheme = remote
        .strip_prefix("https://")
        .or_else(|| remote.strip_prefix("http://"))
        .or_else(|| remote.strip_prefix("ssh://"))
        .unwrap_or(remote);
    let without_user = without_scheme
        .strip_prefix("git@")
        .unwrap_or(without_scheme);
    let (host, path) = without_user
        .split_once(':')
        .or_else(|| without_user.split_once('/'))?;
    let path = path.trim_start_matches('/').trim_end_matches(".git");
    let mut parts = path.split('/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    if owner.is_empty() || repo.is_empty() || parts.next().is_some() {
        return None;
    }
    Some(format!("{host}/{owner}/{repo}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_https_and_ssh_repository_urls() {
        assert_eq!(
            parse_repository_reference("https://github.com/owner/repo.git"),
            Some("github.com/owner/repo".to_owned())
        );
        assert_eq!(
            parse_repository_reference("git@github.com:owner/repo.git"),
            Some("github.com/owner/repo".to_owned())
        );
        assert_eq!(
            parse_repository_reference("ssh://git@github.example.com/owner/repo"),
            Some("github.example.com/owner/repo".to_owned())
        );
    }

    #[test]
    fn rejects_non_repository_urls() {
        assert_eq!(
            parse_repository_reference("https://example.com/owner"),
            None
        );
    }

    #[test]
    fn validates_mutating_operation_kinds() {
        assert_eq!(review_flag("approve"), Ok("--approve"));
        assert_eq!(merge_flag("squash"), Ok("--squash"));
        assert_eq!(repository_visibility_flag("private"), Ok("--private"));
        assert!(review_flag("delete").is_err());
        assert!(merge_flag("force").is_err());
        assert!(repository_visibility_flag("secret").is_err());
        assert_eq!(label_color("#7C3AED"), Ok("7C3AED"));
        assert!(label_color("#abc").is_err());
        assert!(label_color("purple").is_err());
        assert!(required("   ", "name").is_err());
    }

    #[test]
    fn treats_empty_github_lists_as_empty() {
        let values: Vec<String> = parse_json_list("").expect("empty list should parse");
        assert!(values.is_empty());
    }
}
