import type {
  GithubIssue,
  GithubPullRequest,
  GithubRelease,
  GithubRun,
} from "./api";

export type GlobalSection = "repositories" | "activity" | "diagnostics";

export type RepositoryView = "changes" | "branches" | "history" | "github";

export type GithubSection =
  | "pull-requests"
  | "issues"
  | "actions"
  | "releases"
  | "tools";

export type DetailTarget =
  | { kind: "pull-request"; item: GithubPullRequest }
  | { kind: "issue"; item: GithubIssue }
  | { kind: "release"; item: GithubRelease }
  | { kind: "run"; item: GithubRun }
  | null;

export type GithubComposer =
  | "none"
  | "pull-request"
  | "issue"
  | "release"
  | "repository"
  | "clone";
