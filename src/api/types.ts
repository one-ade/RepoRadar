export interface ToolStatus {
  installed: boolean;
  version: string | null;
  authenticated: boolean | null;
}

export interface GithubHost {
  host: string;
  login: string;
  state: string;
  active: boolean;
  tokenSource: string | null;
  scopes: string | null;
  gitProtocol: string | null;
}

export interface EnvironmentStatus {
  git: ToolStatus;
  gh: ToolStatus;
  githubHosts: GithubHost[];
  databaseReady: boolean;
}

export interface Project {
  id: number;
  path: string;
  name: string;
  favorite: boolean;
  tags: string[];
  lastSeenAt: string;
}

export interface PathInspection {
  path: string;
  name: string;
  repositoryKind: "standard" | "worktree" | "bare" | null;
}

export interface ScanSummary {
  found: number;
  skipped: number;
  cancelled: boolean;
  projects: Project[];
}

export interface ScanProgress {
  scanned: number;
  found: number;
  skipped: number;
  currentPath: string;
}

export interface GitFile {
  path: string;
  indexStatus: string;
  worktreeStatus: string;
}

export interface GitStatus {
  branch: string;
  upstream: string | null;
  ahead: number;
  behind: number;
  files: GitFile[];
}

export interface GitBranch {
  name: string;
  current: boolean;
  upstream: string | null;
}

export interface GitCommit {
  hash: string;
  author: string;
  date: string;
  subject: string;
}

export interface GithubRepository {
  nameWithOwner: string;
  description: string | null;
  isPrivate: boolean;
  defaultBranchRef: { name: string } | null;
  url: string;
  stargazerCount: number;
  forkCount: number;
}

export interface GithubAuthor {
  login: string;
}

export interface GithubPullRequest {
  number: number;
  title: string;
  state: string;
  author: GithubAuthor | null;
  headRefName: string;
  baseRefName: string;
  isDraft: boolean;
  updatedAt: string;
  url: string;
}

export interface GithubIssue {
  number: number;
  title: string;
  state: string;
  author: GithubAuthor | null;
  labels: { name: string }[];
  updatedAt: string;
  url: string;
}

export interface GithubRun {
  databaseId: number;
  name: string | null;
  displayTitle: string;
  status: string;
  conclusion: string | null;
  headBranch: string | null;
  event: string;
  workflowName: string;
  createdAt: string;
  updatedAt: string;
  url: string;
}

export interface GithubWorkflow {
  id: number;
  name: string;
  state: string;
  path: string;
}

export interface GithubRelease {
  tagName: string;
  name: string | null;
  isDraft: boolean;
  isLatest: boolean;
  isPrerelease: boolean;
  publishedAt: string | null;
}

export interface GithubOverview {
  repository: GithubRepository;
  pullRequests: GithubPullRequest[];
  issues: GithubIssue[];
  workflows: GithubWorkflow[];
  runs: GithubRun[];
  releases: GithubRelease[];
}

export interface GithubConfiguration {
  variables: Array<{ name: string; value: string; updatedAt: string }>;
  secrets: Array<{ name: string; updatedAt: string }>;
  labels: Array<{ name: string; color: string | null; description: string | null }>;
}
