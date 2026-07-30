use std::path::Path;

use super::{
    GithubConfiguration, GithubOverview,
    client::{gh_json, gh_json_list, repository_reference},
};

pub fn overview(path: &Path) -> Result<GithubOverview, String> {
    let reference = repository_reference(path)?;
    Ok(GithubOverview {
        repository: gh_json(
            &reference,
            path,
            &[
                "repo",
                "view",
                "--json",
                "nameWithOwner,description,isPrivate,defaultBranchRef,url,stargazerCount,forkCount",
            ],
        )?,
        pull_requests: gh_json_list(
            &reference,
            path,
            &[
                "pr",
                "list",
                "--limit",
                "30",
                "--json",
                "number,title,state,author,headRefName,baseRefName,isDraft,updatedAt,url",
            ],
        )?,
        issues: gh_json_list(
            &reference,
            path,
            &[
                "issue",
                "list",
                "--limit",
                "30",
                "--json",
                "number,title,state,author,labels,updatedAt,url",
            ],
        )?,
        workflows: gh_json_list(
            &reference,
            path,
            &[
                "workflow",
                "list",
                "--all",
                "--limit",
                "50",
                "--json",
                "id,name,state,path",
            ],
        )?,
        runs: gh_json_list(
            &reference,
            path,
            &[
                "run",
                "list",
                "--limit",
                "20",
                "--json",
                "databaseId,name,displayTitle,status,conclusion,headBranch,event,workflowName,createdAt,updatedAt,url",
            ],
        )?,
        releases: gh_json_list(
            &reference,
            path,
            &[
                "release",
                "list",
                "--limit",
                "20",
                "--json",
                "tagName,name,isDraft,isLatest,isPrerelease,publishedAt",
            ],
        )?,
    })
}

pub fn configuration(path: &Path) -> Result<GithubConfiguration, String> {
    let reference = repository_reference(path)?;
    Ok(GithubConfiguration {
        variables: gh_json_list(
            &reference,
            path,
            &["variable", "list", "--json", "name,value,updatedAt"],
        )?,
        secrets: gh_json_list(
            &reference,
            path,
            &["secret", "list", "--json", "name,updatedAt"],
        )?,
        labels: gh_json_list(
            &reference,
            path,
            &[
                "label",
                "list",
                "--limit",
                "100",
                "--json",
                "name,color,description",
            ],
        )?,
    })
}
