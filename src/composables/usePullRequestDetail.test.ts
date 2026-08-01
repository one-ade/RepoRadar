import { nextTick, ref } from "vue";
import { beforeEach, describe, expect, it, vi } from "vitest";

import type { GithubOverview, GithubPullRequest, Project } from "../api";

const api = vi.hoisted(() => ({ getPullRequestDetail: vi.fn() }));
vi.mock("../api", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../api")>()),
  getPullRequestDetail: api.getPullRequestDetail,
}));

import { usePullRequestDetail } from "./usePullRequestDetail";

const project: Project = {
  id: 1,
  name: "repo",
  path: "D:/repo",
  favorite: false,
  tags: [],
  lastSeenAt: "now",
};
const pullRequest: GithubPullRequest = {
  number: 7,
  title: "Details",
  state: "OPEN",
  author: { login: "octocat" },
  headRefName: "feature",
  baseRefName: "main",
  isDraft: false,
  updatedAt: "now",
  url: "https://github.com/acme/repo/pull/7",
};

function overview(): GithubOverview {
  return {
    repository: {
      nameWithOwner: "acme/repo",
      description: null,
      isPrivate: false,
      defaultBranchRef: { name: "main" },
      url: "https://github.com/acme/repo",
      stargazerCount: 0,
      forkCount: 0,
    },
    pullRequests: [pullRequest],
    issues: [],
    workflows: [],
    runs: [],
    releases: [],
  };
}

describe("usePullRequestDetail", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    api.getPullRequestDetail.mockResolvedValue({
      fields: [{ name: "title", value: "Details" }],
    });
  });

  it("loads a selected pull request through the parent operation", async () => {
    const selectedProject = ref<Project | null>(project);
    const githubOverview = ref<GithubOverview | null>(overview());
    const runAction = vi.fn(async (action: () => Promise<void>) => action());
    const detail = usePullRequestDetail(selectedProject, githubOverview, runAction);

    await detail.viewPullRequest(pullRequest);

    expect(api.getPullRequestDetail).toHaveBeenCalledWith("D:/repo", 7);
    expect(runAction).toHaveBeenCalledOnce();
    expect(detail.selectedPullRequest.value?.number).toBe(7);
    expect(detail.pullRequestDetail.value?.fields[0]?.name).toBe("title");
  });

  it("clears stale details when the project or overview changes", async () => {
    let finishRequest = (_value: { fields: never[] }) => {};
    api.getPullRequestDetail.mockImplementation(
      () =>
        new Promise((resolve) => {
          finishRequest = resolve;
        }),
    );
    const selectedProject = ref<Project | null>(project);
    const githubOverview = ref<GithubOverview | null>(overview());
    const detail = usePullRequestDetail(
      selectedProject,
      githubOverview,
      async (action) => action(),
    );
    const loading = detail.viewPullRequest(pullRequest);

    githubOverview.value = overview();
    await nextTick();
    finishRequest({ fields: [] });
    await loading;

    expect(detail.selectedPullRequest.value).toBeNull();
    expect(detail.pullRequestDetail.value).toBeNull();
  });
});
