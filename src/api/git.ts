import { invoke } from "@tauri-apps/api/core";

import type { GitBranch, GitCommit, GitStatus } from "./types";

export function getGitStatus(path: string): Promise<GitStatus> {
  return invoke<GitStatus>("get_git_status", { path });
}

export function stageAll(path: string): Promise<void> {
  return invoke<void>("stage_all", { path });
}

export function unstageAll(path: string): Promise<void> {
  return invoke<void>("unstage_all", { path });
}

export function fetch(path: string): Promise<void> {
  return invoke<void>("fetch", { path });
}

export function commit(path: string, message: string): Promise<void> {
  return invoke<void>("commit", { path, message });
}

export function pull(path: string): Promise<void> {
  return invoke<void>("pull", { path });
}

export function push(path: string): Promise<void> {
  return invoke<void>("push", { path });
}

export function getGitDiff(path: string, staged = false): Promise<string> {
  return invoke<string>("get_git_diff", { path, staged });
}

export function listBranches(path: string): Promise<GitBranch[]> {
  return invoke<GitBranch[]>("list_branches", { path });
}

export function switchBranch(path: string, branch: string): Promise<void> {
  return invoke<void>("switch_branch", { path, branch });
}

export function createBranch(path: string, branch: string): Promise<void> {
  return invoke<void>("create_branch", { path, branch });
}

export function deleteBranch(path: string, branch: string): Promise<void> {
  return invoke<void>("delete_branch", { path, branch });
}

export function getGitLog(path: string, limit = 30): Promise<GitCommit[]> {
  return invoke<GitCommit[]>("git_log", { path, limit });
}
