use std::path::Path;

use serde::Deserialize;

use super::client::{gh_output, repository_reference, required};

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubIssueEdit {
    title: Option<String>,
    body: Option<String>,
    add_assignees: Option<String>,
    remove_assignees: Option<String>,
    add_labels: Option<String>,
    remove_labels: Option<String>,
    add_projects: Option<String>,
    remove_projects: Option<String>,
    milestone: Option<String>,
    remove_milestone: bool,
    issue_type: Option<String>,
    remove_type: bool,
    parent: Option<String>,
    remove_parent: bool,
    add_sub_issues: Option<String>,
    remove_sub_issues: Option<String>,
    add_blocked_by: Option<String>,
    remove_blocked_by: Option<String>,
    add_blocking: Option<String>,
    remove_blocking: Option<String>,
}

pub fn create_issue(path: &Path, title: &str, body: &str) -> Result<String, String> {
    let title = required(title, "Issue 标题")?;
    let reference = repository_reference(path)?;
    gh_output(
        &reference,
        path,
        &["issue", "create", "--title", title, "--body", body.trim()],
    )
}

pub fn comment_issue(path: &Path, number: u64, body: &str) -> Result<(), String> {
    let body = required(body, "评论内容")?;
    let reference = repository_reference(path)?;
    let number = number.to_string();
    gh_output(
        &reference,
        path,
        &["issue", "comment", &number, "--body", body],
    )
    .map(|_| ())
}

pub fn close_issue(path: &Path, number: u64) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let number = number.to_string();
    gh_output(&reference, path, &["issue", "close", &number]).map(|_| ())
}

pub fn edit_issue(path: &Path, number: u64, edit: &GithubIssueEdit) -> Result<(), String> {
    let reference = repository_reference(path)?;
    let args = issue_edit_args(number, edit)?;
    let arguments = args.iter().map(String::as_str).collect::<Vec<_>>();
    gh_output(&reference, path, &arguments).map(|_| ())
}

fn issue_edit_args(number: u64, edit: &GithubIssueEdit) -> Result<Vec<String>, String> {
    reject_conflict(&edit.milestone, edit.remove_milestone, "里程碑")?;
    reject_conflict(&edit.issue_type, edit.remove_type, "Issue 类型")?;
    reject_conflict(&edit.parent, edit.remove_parent, "父 Issue")?;
    let mut args = vec!["issue".into(), "edit".into(), number.to_string()];
    if let Some(title) = edit.title.as_deref() {
        push_argument(&mut args, "--title", required(title, "Issue 标题")?);
    }
    if let Some(body) = edit.body.as_deref() {
        push_argument(&mut args, "--body", body.trim());
    }
    for (flag, value) in [
        ("--add-assignee", &edit.add_assignees),
        ("--remove-assignee", &edit.remove_assignees),
        ("--add-label", &edit.add_labels),
        ("--remove-label", &edit.remove_labels),
        ("--add-project", &edit.add_projects),
        ("--remove-project", &edit.remove_projects),
        ("--milestone", &edit.milestone),
        ("--type", &edit.issue_type),
        ("--parent", &edit.parent),
        ("--add-sub-issue", &edit.add_sub_issues),
        ("--remove-sub-issue", &edit.remove_sub_issues),
        ("--add-blocked-by", &edit.add_blocked_by),
        ("--remove-blocked-by", &edit.remove_blocked_by),
        ("--add-blocking", &edit.add_blocking),
        ("--remove-blocking", &edit.remove_blocking),
    ] {
        if let Some(value) = value
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            push_argument(&mut args, flag, value);
        }
    }
    for (flag, enabled) in [
        ("--remove-milestone", edit.remove_milestone),
        ("--remove-type", edit.remove_type),
        ("--remove-parent", edit.remove_parent),
    ] {
        if enabled {
            args.push(flag.into());
        }
    }
    if args.len() == 3 {
        Err("至少选择一项 Issue 修改".into())
    } else {
        Ok(args)
    }
}

fn push_argument(args: &mut Vec<String>, flag: &str, value: &str) {
    args.push(flag.into());
    args.push(value.into());
}

fn reject_conflict(value: &Option<String>, remove: bool, label: &str) -> Result<(), String> {
    if remove
        && value
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
    {
        Err(format!("不能同时设置和移除{label}"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_every_supported_issue_edit_flag() {
        let edit = GithubIssueEdit {
            title: Some("New title".into()),
            body: Some("New body".into()),
            add_assignees: Some("@me,octocat".into()),
            remove_assignees: Some("hubot".into()),
            add_labels: Some("bug,help wanted".into()),
            remove_labels: Some("wontfix".into()),
            add_projects: Some("Roadmap".into()),
            remove_projects: Some("Backlog".into()),
            milestone: Some("v1".into()),
            issue_type: Some("Bug".into()),
            parent: Some("100".into()),
            add_sub_issues: Some("101,102".into()),
            remove_sub_issues: Some("103".into()),
            add_blocked_by: Some("200".into()),
            remove_blocked_by: Some("201".into()),
            add_blocking: Some("300".into()),
            remove_blocking: Some("301".into()),
            ..GithubIssueEdit::default()
        };
        let args = issue_edit_args(7, &edit).expect("valid edit");
        for flag in [
            "--title",
            "--body",
            "--add-assignee",
            "--remove-assignee",
            "--add-label",
            "--remove-label",
            "--add-project",
            "--remove-project",
            "--milestone",
            "--type",
            "--parent",
            "--add-sub-issue",
            "--remove-sub-issue",
            "--add-blocked-by",
            "--remove-blocked-by",
            "--add-blocking",
            "--remove-blocking",
        ] {
            assert!(
                args.iter().any(|argument| argument == flag),
                "missing {flag}"
            );
        }
        let removals = GithubIssueEdit {
            remove_milestone: true,
            remove_type: true,
            remove_parent: true,
            ..GithubIssueEdit::default()
        };
        let args = issue_edit_args(7, &removals).expect("valid removals");
        for flag in ["--remove-milestone", "--remove-type", "--remove-parent"] {
            assert!(
                args.iter().any(|argument| argument == flag),
                "missing {flag}"
            );
        }
    }

    #[test]
    fn rejects_empty_and_conflicting_issue_edits() {
        assert!(issue_edit_args(7, &GithubIssueEdit::default()).is_err());
        assert!(
            issue_edit_args(
                7,
                &GithubIssueEdit {
                    milestone: Some("v1".into()),
                    remove_milestone: true,
                    ..GithubIssueEdit::default()
                }
            )
            .is_err()
        );
    }
}
