use std::{
    path::Path,
    process::{Command, Output},
};

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitStatus {
    pub branch: String,
    pub upstream: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub files: Vec<GitFile>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitFile {
    pub path: String,
    pub index_status: String,
    pub worktree_status: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitBranch {
    pub name: String,
    pub current: bool,
    pub upstream: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitCommit {
    pub hash: String,
    pub author: String,
    pub date: String,
    pub subject: String,
}

pub fn status(path: &Path) -> Result<GitStatus, String> {
    let output = run(path, ["status", "--porcelain=v2", "--branch"])?;
    parse_status(&String::from_utf8_lossy(&output.stdout))
}

pub fn stage_all(path: &Path) -> Result<(), String> {
    run(path, ["add", "--all"])?;
    Ok(())
}

pub fn unstage_all(path: &Path) -> Result<(), String> {
    run(path, ["restore", "--staged", "--", "."])?;
    Ok(())
}

pub fn fetch(path: &Path) -> Result<(), String> {
    run(path, ["fetch", "--all", "--prune"])?;
    Ok(())
}

pub fn commit(path: &Path, message: &str) -> Result<(), String> {
    let message = message.trim();
    if message.is_empty() {
        return Err("提交信息不能为空".to_owned());
    }
    let mut command = git_command(path);
    let output = command.args(["commit", "-m"]).arg(message).output();
    finish(output)?;
    Ok(())
}

pub fn pull(path: &Path) -> Result<(), String> {
    run(path, ["pull", "--ff-only"])?;
    Ok(())
}

pub fn push(path: &Path) -> Result<(), String> {
    run(path, ["push"])?;
    Ok(())
}

pub fn remote_url(path: &Path) -> Result<String, String> {
    let output = run(path, ["remote", "get-url", "origin"])?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

pub fn diff(path: &Path, staged: bool) -> Result<String, String> {
    let output = if staged {
        run(path, ["diff", "--no-ext-diff", "--staged", "--"])?
    } else {
        run(path, ["diff", "--no-ext-diff", "--"])?
    };
    Ok(String::from_utf8_lossy(&output.stdout)
        .chars()
        .take(100_000)
        .collect())
}

pub fn branches(path: &Path) -> Result<Vec<GitBranch>, String> {
    let output = run(
        path,
        [
            "for-each-ref",
            "--format=%(refname:short)%09%(HEAD)%09%(upstream:short)",
            "refs/heads",
        ],
    )?;
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut fields = line.split('\t');
            let name = fields.next()?.to_owned();
            let current = fields.next() == Some("*");
            let upstream = fields
                .next()
                .filter(|value| !value.is_empty())
                .map(str::to_owned);
            Some(GitBranch {
                name,
                current,
                upstream,
            })
        })
        .collect())
}

pub fn switch_branch(path: &Path, branch: &str) -> Result<(), String> {
    validate_branch(path, branch)?;
    run(path, ["switch", "--", branch])?;
    Ok(())
}

pub fn create_branch(path: &Path, branch: &str) -> Result<(), String> {
    validate_branch(path, branch)?;
    run(path, ["switch", "-c", branch])?;
    Ok(())
}

pub fn delete_branch(path: &Path, branch: &str) -> Result<(), String> {
    validate_branch(path, branch)?;
    run(path, ["branch", "-d", branch])?;
    Ok(())
}

pub fn log(path: &Path, limit: u32) -> Result<Vec<GitCommit>, String> {
    let limit = limit.clamp(1, 100).to_string();
    let pretty = "--pretty=format:%H%x09%an%x09%ad%x09%s";
    let mut command = git_command(path);
    let output = command
        .args(["log", "-n", &limit, "--date=iso-strict"])
        .arg(pretty)
        .output();
    let output = finish(output)?;
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut fields = line.splitn(4, '\t');
            Some(GitCommit {
                hash: fields.next()?.to_owned(),
                author: fields.next()?.to_owned(),
                date: fields.next()?.to_owned(),
                subject: fields.next()?.to_owned(),
            })
        })
        .collect())
}

fn validate_branch(path: &Path, branch: &str) -> Result<(), String> {
    if branch.trim().is_empty() {
        return Err("分支名不能为空".to_owned());
    }
    if branch.starts_with('-') {
        return Err("分支名不能以连字符开头".to_owned());
    }
    let mut command = git_command(path);
    finish(
        command
            .args(["check-ref-format", "--branch"])
            .arg(branch)
            .output(),
    )?;
    Ok(())
}

fn run<const N: usize>(path: &Path, args: [&str; N]) -> Result<Output, String> {
    finish(git_command(path).args(args).output())
}

fn git_command(path: &Path) -> Command {
    let mut command = Command::new("git");
    command.arg("-C").arg(path);
    command
}

fn finish(output: std::io::Result<Output>) -> Result<Output, String> {
    let output = output.map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(output)
    } else {
        let message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        Err(if message.is_empty() {
            format!("git exited with status {}", output.status)
        } else {
            message
        })
    }
}

fn parse_status(output: &str) -> Result<GitStatus, String> {
    let mut branch = "HEAD".to_owned();
    let mut upstream = None;
    let mut ahead = 0;
    let mut behind = 0;
    let mut files = Vec::new();

    for line in output.lines() {
        if let Some(value) = line.strip_prefix("# branch.head ") {
            branch = value.to_owned();
        } else if let Some(value) = line.strip_prefix("# branch.upstream ") {
            upstream = Some(value.to_owned());
        } else if let Some(value) = line.strip_prefix("# branch.ab ") {
            let mut values = value.split_whitespace();
            ahead = values
                .next()
                .and_then(|value| value.strip_prefix('+'))
                .and_then(|value| value.parse().ok())
                .unwrap_or_default();
            behind = values
                .next()
                .and_then(|value| value.strip_prefix('-'))
                .and_then(|value| value.parse().ok())
                .unwrap_or_default();
        } else if let Some(file) = parse_file(line) {
            files.push(file);
        }
    }

    Ok(GitStatus {
        branch,
        upstream,
        ahead,
        behind,
        files,
    })
}

fn parse_file(line: &str) -> Option<GitFile> {
    let kind = line.get(..2)?;
    match kind {
        "1 " | "2 " | "u " => {
            let mut fields = line.splitn(9, ' ');
            fields.next()?;
            let status = fields.next()?;
            let path = fields.nth(6)?.split('\t').next_back()?.to_owned();
            Some(GitFile {
                path,
                index_status: status.get(..1)?.to_owned(),
                worktree_status: status.get(1..2)?.to_owned(),
            })
        }
        "? " => Some(GitFile {
            path: line[2..].to_owned(),
            index_status: "?".to_owned(),
            worktree_status: "?".to_owned(),
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    #[test]
    fn parses_branch_counts_and_paths_with_spaces() {
        let output = "\
# branch.oid 123
# branch.head main
# branch.upstream origin/main
# branch.ab +2 -1
1 .M N... 100644 100644 100644 aaa bbb path with spaces.txt
? new file.txt
";

        let status = parse_status(output).unwrap();

        assert_eq!(status.branch, "main");
        assert_eq!(status.upstream.as_deref(), Some("origin/main"));
        assert_eq!(status.ahead, 2);
        assert_eq!(status.behind, 1);
        assert_eq!(status.files.len(), 2);
        assert_eq!(status.files[0].path, "path with spaces.txt");
        assert_eq!(status.files[1].index_status, "?");
    }

    #[test]
    fn runs_status_stage_and_commit_against_a_real_repository() {
        let path = test_directory();
        run(&path, ["init"]).unwrap();
        run(&path, ["config", "user.email", "repo-radar@example.test"]).unwrap();
        run(&path, ["config", "user.name", "RepoRadar Test"]).unwrap();
        fs::write(path.join("README.md"), "RepoRadar").unwrap();

        assert_eq!(status(&path).unwrap().files[0].index_status, "?");
        stage_all(&path).unwrap();
        assert_eq!(status(&path).unwrap().files[0].index_status, "A");
        commit(&path, "Initial commit").unwrap();
        assert!(status(&path).unwrap().files.is_empty());
        assert!(diff(&path, false).unwrap().is_empty());
        assert_eq!(branches(&path).unwrap().len(), 1);
        assert_eq!(log(&path, 10).unwrap().len(), 1);
        let original_branch = status(&path).unwrap().branch;
        create_branch(&path, "feature/test").unwrap();
        assert_eq!(status(&path).unwrap().branch, "feature/test");
        switch_branch(&path, &original_branch).unwrap();
        delete_branch(&path, "feature/test").unwrap();
        fs::write(path.join("README.md"), "RepoRadar updated").unwrap();
        stage_all(&path).unwrap();
        unstage_all(&path).unwrap();
        assert_eq!(status(&path).unwrap().files[0].worktree_status, "M");

        fs::remove_dir_all(path).unwrap();
    }

    fn test_directory() -> PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("repo-radar-git-{suffix}"));
        fs::create_dir_all(&path).unwrap();
        path
    }
}
