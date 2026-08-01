use std::path::Path;

use super::client::{gh_json_list, repository_reference, repository_slug, required};

pub fn search(
    path: &Path,
    kind: &str,
    query: &str,
    current_repository: bool,
) -> Result<Vec<serde_json::Value>, String> {
    let reference = repository_reference(path)?;
    let args = search_args(
        kind,
        query,
        current_repository
            .then(|| repository_slug(path))
            .transpose()?
            .as_deref(),
    )?;
    let arguments = args.iter().map(String::as_str).collect::<Vec<_>>();
    gh_json_list(&reference, path, &arguments)
}

fn search_args(kind: &str, query: &str, repository: Option<&str>) -> Result<Vec<String>, String> {
    let fields = match kind {
        "code" => "path,repository,sha,textMatches,url",
        "commits" => "author,commit,committer,id,parents,repository,sha,url",
        "issues" => {
            "assignees,author,authorAssociation,body,closedAt,commentsCount,createdAt,id,isLocked,isPullRequest,labels,number,repository,state,title,updatedAt,url"
        }
        "prs" => {
            "assignees,author,authorAssociation,body,closedAt,commentsCount,createdAt,id,isDraft,isLocked,isPullRequest,labels,number,repository,state,title,updatedAt,url"
        }
        "repos" => {
            "createdAt,defaultBranch,description,forksCount,fullName,id,isArchived,isDisabled,isFork,isPrivate,language,license,name,openIssuesCount,owner,pushedAt,size,stargazersCount,updatedAt,url,visibility,watchersCount"
        }
        _ => return Err("不支持的 GitHub 搜索类型".into()),
    };
    let mut args = vec![
        "search".into(),
        kind.into(),
        required(query, "搜索条件")?.into(),
        "--limit".into(),
        "50".into(),
        "--json".into(),
        fields.into(),
    ];
    if let Some(repository) = repository {
        args.extend(["--repo".into(), repository.into()]);
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_every_search_kind_and_optional_repository_scope() {
        for kind in ["code", "commits", "issues", "prs", "repos"] {
            let args = search_args(kind, "rust stars:>100", Some("acme/repo")).unwrap();
            assert_eq!(args[1], kind);
            assert!(args.contains(&"--json".into()));
            assert!(args.contains(&"acme/repo".into()));
        }
        assert!(search_args("users", "octocat", None).is_err());
        assert!(search_args("repos", " ", None).is_err());
    }
}
