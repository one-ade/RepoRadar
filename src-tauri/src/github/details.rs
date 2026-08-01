use std::{collections::BTreeMap, path::Path};

use serde::{Deserialize, Serialize};

use super::client::{gh_output, repository_reference};

const PULL_REQUEST_DETAIL_FIELDS: &[&str] = &[
    "additions",
    "assignees",
    "author",
    "autoMergeRequest",
    "baseRefName",
    "baseRefOid",
    "body",
    "changedFiles",
    "closed",
    "closedAt",
    "closingIssuesReferences",
    "comments",
    "commits",
    "createdAt",
    "deletions",
    "files",
    "fullDatabaseId",
    "headRefName",
    "headRefOid",
    "headRepository",
    "headRepositoryOwner",
    "id",
    "isCrossRepository",
    "isDraft",
    "labels",
    "latestReviews",
    "maintainerCanModify",
    "mergeCommit",
    "mergeStateStatus",
    "mergeable",
    "mergedAt",
    "mergedBy",
    "milestone",
    "number",
    "potentialMergeCommit",
    "projectCards",
    "projectItems",
    "reactionGroups",
    "reviewDecision",
    "reviewRequests",
    "reviews",
    "state",
    "statusCheckRollup",
    "title",
    "updatedAt",
    "url",
];

const ISSUE_DETAIL_FIELDS: &[&str] = &[
    "assignees",
    "author",
    "blockedBy",
    "blocking",
    "body",
    "closed",
    "closedAt",
    "closedByPullRequestsReferences",
    "comments",
    "createdAt",
    "id",
    "isPinned",
    "issueType",
    "labels",
    "milestone",
    "number",
    "parent",
    "projectCards",
    "projectItems",
    "reactionGroups",
    "state",
    "stateReason",
    "subIssues",
    "subIssuesSummary",
    "title",
    "updatedAt",
    "url",
];

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum GithubDetailValue {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Array(Vec<GithubDetailValue>),
    Object(BTreeMap<String, GithubDetailValue>),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct GithubDetailField {
    pub name: String,
    pub value: GithubDetailValue,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct GithubPullRequestDetail {
    pub fields: Vec<GithubDetailField>,
}

pub type GithubIssueDetail = GithubPullRequestDetail;

pub fn pull_request_detail(path: &Path, number: u64) -> Result<GithubPullRequestDetail, String> {
    fetch_detail(path, number, "pr", PULL_REQUEST_DETAIL_FIELDS)
}

pub fn issue_detail(path: &Path, number: u64) -> Result<GithubIssueDetail, String> {
    fetch_detail(path, number, "issue", ISSUE_DETAIL_FIELDS)
}

fn fetch_detail(
    path: &Path,
    number: u64,
    kind: &str,
    fields: &[&str],
) -> Result<GithubPullRequestDetail, String> {
    let reference = repository_reference(path)?;
    let number = number.to_string();
    let requested_fields = fields.join(",");
    let output = gh_output(
        &reference,
        path,
        &[kind, "view", &number, "--json", &requested_fields],
    )?;
    parse_detail(&output, fields)
}

fn parse_detail(raw: &str, requested_fields: &[&str]) -> Result<GithubPullRequestDetail, String> {
    let mut values: BTreeMap<String, GithubDetailValue> =
        serde_json::from_str(raw).map_err(|error| error.to_string())?;
    let fields = requested_fields
        .iter()
        .map(|name| {
            values
                .remove(*name)
                .map(|value| GithubDetailField {
                    name: (*name).into(),
                    value,
                })
                .ok_or_else(|| format!("GitHub response is missing pull request field `{name}`"))
        })
        .collect::<Result<_, _>>()?;

    Ok(GithubPullRequestDetail { fields })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn requests_every_supported_pull_request_field_once() {
        assert_eq!(PULL_REQUEST_DETAIL_FIELDS.len(), 46);
        assert_eq!(
            PULL_REQUEST_DETAIL_FIELDS
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                .len(),
            PULL_REQUEST_DETAIL_FIELDS.len()
        );
        assert!(PULL_REQUEST_DETAIL_FIELDS.contains(&"comments"));
        assert!(PULL_REQUEST_DETAIL_FIELDS.contains(&"statusCheckRollup"));
    }

    #[test]
    fn requests_every_supported_issue_field_once() {
        assert_eq!(ISSUE_DETAIL_FIELDS.len(), 27);
        assert_eq!(
            ISSUE_DETAIL_FIELDS
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                .len(),
            ISSUE_DETAIL_FIELDS.len()
        );
        assert!(ISSUE_DETAIL_FIELDS.contains(&"subIssues"));
        assert!(ISSUE_DETAIL_FIELDS.contains(&"blockedBy"));
    }

    #[test]
    fn preserves_recursive_json_shapes_in_field_order() {
        let value: GithubDetailValue =
            serde_json::from_str(r#"[null,false,2,"Ready",{"name":"bug"}]"#)
                .expect("valid recursive detail value");

        assert_eq!(
            value,
            GithubDetailValue::Array(vec![
                GithubDetailValue::Null,
                GithubDetailValue::Bool(false),
                GithubDetailValue::Number(2.into()),
                GithubDetailValue::String("Ready".into()),
                GithubDetailValue::Object(
                    [("name".into(), GithubDetailValue::String("bug".into()))]
                        .into_iter()
                        .collect(),
                ),
            ])
        );
    }

    #[test]
    fn parses_every_requested_field_and_rejects_missing_fields() {
        let mut values = PULL_REQUEST_DETAIL_FIELDS
            .iter()
            .map(|name| ((*name).to_owned(), serde_json::Value::Null))
            .collect::<serde_json::Map<_, _>>();
        let raw = serde_json::to_string(&values).expect("serializable fixture");
        let detail = parse_detail(&raw, PULL_REQUEST_DETAIL_FIELDS).expect("complete detail");

        assert_eq!(
            detail
                .fields
                .iter()
                .map(|field| field.name.as_str())
                .collect::<Vec<_>>(),
            PULL_REQUEST_DETAIL_FIELDS
        );

        values.remove("comments");
        let raw = serde_json::to_string(&values).expect("serializable fixture");
        assert!(parse_detail(&raw, PULL_REQUEST_DETAIL_FIELDS).is_err());
    }

    #[test]
    fn parses_issue_fields_with_the_shared_detail_parser() {
        let values = ISSUE_DETAIL_FIELDS
            .iter()
            .map(|name| ((*name).to_owned(), serde_json::Value::Null))
            .collect::<serde_json::Map<_, _>>();
        let raw = serde_json::to_string(&values).expect("serializable fixture");

        let detail = parse_detail(&raw, ISSUE_DETAIL_FIELDS).expect("complete issue detail");

        assert_eq!(detail.fields.len(), 27);
        assert_eq!(detail.fields[0].name, "assignees");
    }
}
