import { invoke } from "@tauri-apps/api/core";

import type {
  GithubConfiguration,
  GithubEnvironment,
  GithubEnvironmentConfiguration,
  GithubIssueDetail,
  GithubIssueEdit,
  GithubOverview,
  GithubPullRequestDetail,
  GithubProject,
  GithubReleaseDetail,
  GithubReleaseEdit,
  GithubDiscussion,
  GithubCodespace,
  GithubDetailValue,
} from "./types";

export function getGithubOverview(path: string): Promise<GithubOverview> {
  return invoke<GithubOverview>("get_github_overview", { path });
}

export function getPullRequestDetail(
  path: string,
  number: number,
): Promise<GithubPullRequestDetail> {
  return invoke<GithubPullRequestDetail>("get_pull_request_detail", { path, number });
}

export function getIssueDetail(path: string, number: number): Promise<GithubIssueDetail> {
  return invoke<GithubIssueDetail>("get_issue_detail", { path, number });
}

export function getGithubConfiguration(path: string): Promise<GithubConfiguration> {
  return invoke<GithubConfiguration>("get_github_configuration", { path });
}

export function getGithubEnvironments(path: string): Promise<GithubEnvironment[]> {
  return invoke<GithubEnvironment[]>("get_github_environments", { path });
}

export function getGithubEnvironmentConfiguration(
  path: string,
  environment: string,
): Promise<GithubEnvironmentConfiguration> {
  return invoke<GithubEnvironmentConfiguration>("get_github_environment_configuration", {
    path,
    environment,
  });
}

export function saveGithubEnvironment(path: string, name: string): Promise<void> {
  return invoke("save_github_environment", { path, name });
}

export function deleteGithubEnvironment(path: string, name: string): Promise<void> {
  return invoke("delete_github_environment", { path, name });
}

export function setGithubEnvironmentVariable(
  path: string, environment: string, name: string, value: string,
): Promise<void> {
  return invoke("set_github_environment_variable", { path, environment, name, value });
}

export function deleteGithubEnvironmentVariable(
  path: string, environment: string, name: string,
): Promise<void> {
  return invoke("delete_github_environment_variable", { path, environment, name });
}

export function setGithubEnvironmentSecret(
  path: string, environment: string, name: string, value: string,
): Promise<void> {
  return invoke("set_github_environment_secret", { path, environment, name, value });
}

export function deleteGithubEnvironmentSecret(
  path: string, environment: string, name: string,
): Promise<void> {
  return invoke("delete_github_environment_secret", { path, environment, name });
}

export function setGithubVariable(path: string, name: string, value: string): Promise<void> {
  return invoke("set_github_variable", { path, name, value });
}

export function deleteGithubVariable(path: string, name: string): Promise<void> {
  return invoke("delete_github_variable", { path, name });
}

export function setGithubSecret(path: string, name: string, value: string): Promise<void> {
  return invoke("set_github_secret", { path, name, value });
}

export function deleteGithubSecret(path: string, name: string): Promise<void> {
  return invoke("delete_github_secret", { path, name });
}

export function saveGithubLabel(
  path: string,
  name: string,
  color: string,
  description: string,
): Promise<void> {
  return invoke("save_github_label", { path, name, color, description });
}

export function deleteGithubLabel(path: string, name: string): Promise<void> {
  return invoke("delete_github_label", { path, name });
}

export function checkGithubRulesets(path: string): Promise<string> {
  return invoke<string>("check_github_rulesets", { path });
}

export function getGithubRunLog(path: string, databaseId: number): Promise<string> {
  return invoke<string>("get_github_run_log", { path, databaseId });
}

export function rerunGithubRun(path: string, databaseId: number): Promise<void> {
  return invoke("rerun_github_run", { path, databaseId });
}

export function runGithubWorkflow(path: string, workflowId: number): Promise<void> {
  return invoke("run_github_workflow", { path, workflowId });
}

export function cancelGithubRun(path: string, databaseId: number): Promise<void> {
  return invoke("cancel_github_run", { path, databaseId });
}

export function downloadGithubArtifacts(
  path: string,
  databaseId: number,
  targetDir: string,
  artifactName = "",
): Promise<void> {
  return invoke("download_github_artifacts", {
    path,
    databaseId,
    targetDir,
    artifactName,
  });
}

export function createGithubRelease(
  path: string,
  tag: string,
  title: string,
  notes: string,
  draft = false,
  prerelease = false,
): Promise<string> {
  return invoke<string>("create_github_release", {
    path,
    tag,
    title,
    notes,
    draft,
    prerelease,
  });
}

export function getReleaseDetail(path: string, tag: string): Promise<GithubReleaseDetail> {
  return invoke<GithubReleaseDetail>("get_release_detail", { path, tag });
}

export function editGithubRelease(
  path: string,
  tag: string,
  edit: GithubReleaseEdit,
): Promise<void> {
  return invoke("edit_github_release", { path, tag, edit });
}

export function uploadGithubReleaseAssets(
  path: string,
  tag: string,
  files: readonly string[],
  clobber = false,
): Promise<void> {
  return invoke("upload_github_release_assets", { path, tag, files, clobber });
}

export function downloadGithubRelease(
  path: string,
  tag: string,
  targetDir: string,
  pattern = "",
): Promise<void> {
  return invoke("download_github_release", { path, tag, targetDir, pattern });
}

export function forkGithubRepository(path: string, organization = ""): Promise<string> {
  return invoke<string>("fork_github_repository", { path, organization });
}

export function syncGithubRepository(path: string, branch = ""): Promise<void> {
  return invoke("sync_github_repository", { path, branch });
}

export function cloneGithubRepository(reference: string, targetDir: string): Promise<string> {
  return invoke<string>("clone_github_repository", { reference, targetDir });
}

export function createGithubRepository(
  path: string,
  name: string,
  visibility: "public" | "private" | "internal",
  description = "",
): Promise<string> {
  return invoke<string>("create_github_repository", {
    path,
    name,
    visibility,
    description,
  });
}

export function createPullRequest(
  path: string,
  title: string,
  body: string,
  draft = false,
): Promise<string> {
  return invoke<string>("create_pull_request", { path, title, body, draft });
}

export function reviewPullRequest(
  path: string,
  number: number,
  action: "approve" | "comment" | "request-changes",
  body: string,
): Promise<void> {
  return invoke("review_pull_request", { path, number, action, body });
}

export function mergePullRequest(
  path: string,
  number: number,
  method: "merge" | "squash" | "rebase" = "squash",
  deleteBranch = true,
): Promise<void> {
  return invoke("merge_pull_request", { path, number, method, deleteBranch });
}

export function createIssue(path: string, title: string, body: string): Promise<string> {
  return invoke<string>("create_issue", { path, title, body });
}

export function editIssue(path: string, number: number, edit: GithubIssueEdit): Promise<void> {
  return invoke("edit_issue", { path, number, edit });
}

export function commentIssue(path: string, number: number, body: string): Promise<void> {
  return invoke("comment_issue", { path, number, body });
}

export function closeIssue(path: string, number: number): Promise<void> {
  return invoke("close_issue", { path, number });
}

export function getGithubProjects(path: string): Promise<GithubProject[]> {
  return invoke<GithubProject[]>("get_github_projects", { path });
}

export function getGithubProjectItems(
  path: string, number: number, query = "",
): Promise<GithubDetailValue> {
  return invoke<GithubDetailValue>("get_github_project_items", { path, number, query });
}

export function getGithubDiscussions(path: string): Promise<GithubDiscussion[]> {
  return invoke<GithubDiscussion[]>("get_github_discussions", { path });
}

export function getGithubCodespaces(path: string): Promise<GithubCodespace[]> {
  return invoke<GithubCodespace[]>("get_github_codespaces", { path });
}

export function getGithubCodespaceLog(path: string, name: string): Promise<string> {
  return invoke<string>("get_github_codespace_log", { path, name });
}

export function stopGithubCodespace(path: string, name: string): Promise<void> {
  return invoke("stop_github_codespace", { path, name });
}

export function deleteGithubCodespace(
  path: string, name: string, force: boolean,
): Promise<void> {
  return invoke("delete_github_codespace", { path, name, force });
}

export function searchGithub(
  path: string,
  kind: "code" | "commits" | "issues" | "prs" | "repos",
  query: string,
  currentRepository = false,
): Promise<GithubDetailValue[]> {
  return invoke<GithubDetailValue[]>("search_github", {
    path,
    kind,
    query,
    currentRepository,
  });
}

export function runSafeGithubCommand(
  path: string, command: string, extraArgs: readonly string[],
): Promise<string> {
  return invoke<string>("run_safe_github_command", { path, command, extraArgs });
}

export function runGithubApiRequest(
  path: string,
  method: "GET" | "POST" | "PUT" | "PATCH" | "DELETE",
  endpoint: string,
  fields: readonly { key: string; value: string }[],
): Promise<string> {
  return invoke<string>("run_github_api_request", { path, method, endpoint, fields });
}
