import { ref } from "vue";
import { beforeEach, describe, expect, it, vi } from "vitest";

const api = vi.hoisted(() => ({
  cancelGithubRun: vi.fn(),
  cloneGithubRepository: vi.fn(),
  closeIssue: vi.fn(),
  commentIssue: vi.fn(),
  createGithubRelease: vi.fn(),
  createGithubRepository: vi.fn(),
  createIssue: vi.fn(),
  createPullRequest: vi.fn(),
  downloadGithubArtifacts: vi.fn(),
  downloadGithubRelease: vi.fn(),
  forkGithubRepository: vi.fn(),
  getGithubOverview: vi.fn(),
  getGithubRunLog: vi.fn(),
  mergePullRequest: vi.fn(),
  rerunGithubRun: vi.fn(),
  reviewPullRequest: vi.fn(),
  runGithubWorkflow: vi.fn(),
  syncGithubRepository: vi.fn(),
}));
const dialog = vi.hoisted(() => ({ confirm: vi.fn() }));

vi.mock("../api", () => api);
vi.mock("@tauri-apps/plugin-dialog", () => dialog);

import { useGithubWorkspace } from "./useGithubWorkspace";

describe("useGithubWorkspace", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    api.createIssue.mockResolvedValue("https://github.com/acme/repo/issues/1");
    api.getGithubOverview.mockResolvedValue({ repository: { nameWithOwner: "acme/repo" } });
    dialog.confirm.mockResolvedValue(true);
  });

  it("creates an issue through one parent operation and clears the editor", async () => {
    const project = ref({
      id: 1,
      name: "repo",
      path: "D:/repo",
      favorite: false,
      tags: [],
      lastSeenAt: "now",
    });
    const runAction = vi.fn(async (action: () => Promise<void>) => action());
    const notify = vi.fn();
    const workspace = useGithubWorkspace(project, runAction, vi.fn(), notify);
    workspace.githubTitle.value = "Bug";
    workspace.githubBody.value = "Steps";

    await workspace.createGithubItem("issue");

    expect(api.createIssue).toHaveBeenCalledWith("D:/repo", "Bug", "Steps");
    expect(runAction).toHaveBeenCalledTimes(1);
    expect(workspace.githubTitle.value).toBe("");
    expect(workspace.githubBody.value).toBe("");
    expect(notify).toHaveBeenCalledWith(
      "Issue 已创建：https://github.com/acme/repo/issues/1",
    );
  });

  it("dispatches, reruns, and cancels workflow runs through their API boundaries", async () => {
    const project = ref({
      id: 1,
      name: "repo",
      path: "D:/repo",
      favorite: false,
      tags: [],
      lastSeenAt: "now",
    });
    const runAction = vi.fn(async (action: () => Promise<void>) => action());
    const workspace = useGithubWorkspace(project, runAction, vi.fn(), vi.fn());

    await workspace.dispatchGithubWorkflow(7);
    await workspace.rerunGithubWorkflow(8);
    await workspace.cancelGithubWorkflow(9);

    expect(api.runGithubWorkflow).toHaveBeenCalledOnce();
    expect(api.runGithubWorkflow).toHaveBeenCalledWith("D:/repo", 7);
    expect(api.rerunGithubRun).toHaveBeenCalledWith("D:/repo", 8);
    expect(api.cancelGithubRun).toHaveBeenCalledWith("D:/repo", 9);
    expect(runAction).toHaveBeenCalledTimes(3);
  });
});
