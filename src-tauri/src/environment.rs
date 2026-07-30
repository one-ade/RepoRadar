use std::{collections::HashMap, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentStatus {
    pub git: ToolStatus,
    pub gh: ToolStatus,
    pub github_hosts: Vec<GithubHost>,
    pub database_ready: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubHost {
    pub host: String,
    pub login: String,
    pub state: String,
    pub active: bool,
    pub token_source: Option<String>,
    pub scopes: Option<String>,
    pub git_protocol: Option<String>,
}

#[derive(Deserialize)]
struct AuthStatus {
    hosts: HashMap<String, Vec<GithubHost>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolStatus {
    pub installed: bool,
    pub version: Option<String>,
    pub authenticated: Option<bool>,
}

pub fn detect_environment(database_ready: bool) -> EnvironmentStatus {
    EnvironmentStatus {
        git: detect_tool("git", &["--version"], false),
        gh: detect_tool("gh", &["--version"], true),
        github_hosts: detect_github_hosts(),
        database_ready,
    }
}

fn detect_github_hosts() -> Vec<GithubHost> {
    let Ok(output) = Command::new("gh")
        .args(["auth", "status", "--json", "hosts"])
        .output()
    else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }
    parse_github_hosts(&output.stdout)
}

fn parse_github_hosts(output: &[u8]) -> Vec<GithubHost> {
    let Ok(status) = serde_json::from_slice::<AuthStatus>(output) else {
        return Vec::new();
    };
    status.hosts.into_values().flatten().collect()
}

fn detect_tool(program: &str, version_args: &[&str], checks_auth: bool) -> ToolStatus {
    let Ok(output) = Command::new(program).args(version_args).output() else {
        return ToolStatus {
            installed: false,
            version: None,
            authenticated: checks_auth.then_some(false),
        };
    };

    let version = output.status.success().then(|| {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .unwrap_or_default()
            .trim()
            .to_owned()
    });
    let authenticated = checks_auth.then(|| {
        Command::new(program)
            .args(["auth", "status"])
            .output()
            .is_ok_and(|output| output.status.success())
    });

    ToolStatus {
        installed: output.status.success(),
        version,
        authenticated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_program_is_reported_without_error() {
        let status = detect_tool(
            "repo-radar-program-that-does-not-exist",
            &["--version"],
            true,
        );

        assert!(!status.installed);
        assert_eq!(status.version, None);
        assert_eq!(status.authenticated, Some(false));
    }

    #[test]
    fn parses_hosts_without_token_fields() {
        let hosts = parse_github_hosts(
            br#"{"hosts":{"github.com":[{"state":"success","active":true,"host":"github.com","login":"alice","tokenSource":"keyring","scopes":"repo","gitProtocol":"https"}]}}"#,
        );
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].host, "github.com");
        assert_eq!(hosts[0].login, "alice");
    }
}
