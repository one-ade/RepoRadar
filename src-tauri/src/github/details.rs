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

pub fn pull_request_detail(path: &Path, number: u64) -> Result<GithubPullRequestDetail, String> {
    let reference = repository_reference(path)?;
    let number = number.to_string();
    let requested_fields = PULL_REQUEST_DETAIL_FIELDS.join(",");
    let output = gh_output(
        &reference,
        path,
        &["pr", "view", &number, "--json", &requested_fields],
    )?;
    parse_pull_request_detail(&output)
}

fn parse_pull_request_detail(raw: &str) -> Result<GithubPullRequestDetail, String> {
    let mut values: BTreeMap<String, GithubDetailValue> =
        serde_json::from_str(raw).map_err(|error| error.to_string())?;
    let fields = PULL_REQUEST_DETAIL_FIELDS
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
        let detail = parse_pull_request_detail(&raw).expect("complete detail");

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
        assert!(parse_pull_request_detail(&raw).is_err());
    }
}
