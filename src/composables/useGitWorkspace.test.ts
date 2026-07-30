import { ref } from "vue";
import { beforeEach, describe, expect, it, vi } from "vitest";

const api = vi.hoisted(() => ({
  commit: vi.fn(),
  createBranch: vi.fn(),
  deleteBranch: vi.fn(),
  fetch: vi.fn(),
  getGitDiff: vi.fn(),
  getGitLog: vi.fn(),
  getGitStatus: vi.fn(),
  listBranches: vi.fn(),
  pull: vi.fn(),
  push: vi.fn(),
  stageAll: vi.fn(),
  switchBranch: vi.fn(),
  unstageAll: vi.fn(),
}));

vi.mock("../api", () => api);
vi.mock("@tauri-apps/plugin-dialog", () => ({ confirm: vi.fn() }));

import { useGitWorkspace } from "./useGitWorkspace";

describe("useGitWorkspace", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    api.getGitStatus.mockResolvedValue({
      branch: "main",
      upstream: "origin/main",
      ahead: 0,
      behind: 0,
      files: [],
    });
    api.getGitDiff.mockResolvedValue("diff");
  });

  it("runs a Git action once, refreshes status, and reports success", async () => {
    const project = ref({
      id: 1,
      name: "repo",
      path: "D:/repo",
      favorite: false,
      lastSeenAt: "now",
    });
    const runAction = vi.fn(async (action: () => Promise<void>) => action());
    const notify = vi.fn();
    const workspace = useGitWorkspace(project, runAction, vi.fn(), notify);

    await workspace.stageProject();

    expect(api.stageAll).toHaveBeenCalledWith("D:/repo");
    expect(api.getGitStatus).toHaveBeenCalledWith("D:/repo");
    expect(api.getGitDiff).toHaveBeenCalledWith("D:/repo");
    expect(runAction).toHaveBeenCalledTimes(1);
    expect(notify).toHaveBeenCalledWith("已暂存全部修改");
  });
});
