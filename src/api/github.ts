import { invoke } from "@tauri-apps/api/core";

import type {
  GithubConfiguration,
  GithubIssueDetail,
  GithubIssueEdit,
  GithubOverview,
  GithubPullRequestDetail,
  GithubReleaseDetail,
  GithubReleaseEdit,
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
