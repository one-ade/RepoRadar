import { beforeEach, describe, expect, it, vi } from "vitest";

const api = vi.hoisted(() => ({
  addProject: vi.fn(),
  addScanRoot: vi.fn(),
  cancelScan: vi.fn(),
  getEnvironment: vi.fn(),
  inspectProjectPath: vi.fn(),
  listProjects: vi.fn(),
  scanProjects: vi.fn(),
  setProjectFavorite: vi.fn(),
}));
const dialog = vi.hoisted(() => ({ confirm: vi.fn() }));

vi.mock("../api", () => api);
vi.mock("@tauri-apps/plugin-dialog", () => dialog);

import { useProjectDiscovery } from "./useProjectDiscovery";

describe("useProjectDiscovery", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    api.inspectProjectPath.mockResolvedValue({
      path: "D:/new-repo",
      name: "new-repo",
      repositoryKind: null,
    });
    api.addProject.mockResolvedValue({
      id: 1,
      path: "D:/new-repo",
      name: "new-repo",
      favorite: false,
      lastSeenAt: "now",
    });
    api.listProjects.mockResolvedValue([]);
    dialog.confirm.mockResolvedValue(true);
  });

  it("confirms git init before adding a non-repository directory", async () => {
    const runAction = vi.fn(async (action: () => Promise<void>) => action());
    const notify = vi.fn();
    const discovery = useProjectDiscovery(
      runAction,
      vi.fn().mockResolvedValue("D:/new-repo"),
      notify,
      vi.fn(),
    );

    await discovery.chooseProject();

    expect(dialog.confirm).toHaveBeenCalledOnce();
    expect(api.addProject).toHaveBeenCalledWith("D:/new-repo", true);
    expect(runAction).toHaveBeenCalledTimes(1);
    expect(notify).toHaveBeenCalledWith("已添加 new-repo");
  });
});
