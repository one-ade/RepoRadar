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
  setProjectTags: vi.fn(),
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

  it("updates tags through one project action", async () => {
    const project = {
      id: 1,
      path: "D:/new-repo",
      name: "new-repo",
      favorite: false,
      tags: [],
      lastSeenAt: "now",
    };
    const updated = { ...project, tags: ["rust"] };
    const runAction = vi.fn(async (action: () => Promise<void>) => action());
    api.setProjectTags.mockResolvedValue(updated);
    const discovery = useProjectDiscovery(runAction, vi.fn(), vi.fn(), vi.fn());
    discovery.projects.value = [project];
    discovery.selectedProject.value = project;

    const saved = await discovery.updateTags(project, ["rust"]);

    expect(saved).toBe(true);
    expect(api.setProjectTags).toHaveBeenCalledWith(project.id, ["rust"]);
    expect(discovery.projects.value[0]).toEqual(updated);
    expect(discovery.selectedProject.value).toEqual(updated);
    expect(runAction).toHaveBeenCalledTimes(1);
  });

  it("tracks project loading independently from environment loading", async () => {
    const discovery = useProjectDiscovery(vi.fn(), vi.fn(), vi.fn(), vi.fn());

    expect(discovery.projectsLoading.value).toBe(true);

    await discovery.refreshProjects();

    expect(discovery.projectsLoading.value).toBe(false);
  });

  it("keeps a project loading error available for a retryable view", async () => {
    api.listProjects.mockRejectedValueOnce(new Error("database unavailable"));
    const discovery = useProjectDiscovery(vi.fn(), vi.fn(), vi.fn(), vi.fn());

    await expect(discovery.refreshProjects()).rejects.toThrow("database unavailable");

    expect(discovery.projectsLoading.value).toBe(false);
    expect(discovery.projectsError.value).toContain("database unavailable");
  });
});
