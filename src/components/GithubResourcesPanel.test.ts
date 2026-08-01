import { flushPromises, mount } from "@vue/test-utils";
import { beforeEach, describe, expect, it, vi } from "vitest";

const api = vi.hoisted(() => ({
  deleteGithubCodespace: vi.fn(), getGithubCodespaceLog: vi.fn(),
  getGithubCodespaces: vi.fn(), getGithubDiscussions: vi.fn(),
  getGithubProjectItems: vi.fn(), getGithubProjects: vi.fn(), stopGithubCodespace: vi.fn(),
}));
const dialog = vi.hoisted(() => ({ confirm: vi.fn() }));
vi.mock("../api", () => api);
vi.mock("@tauri-apps/plugin-dialog", () => dialog);

import GithubResourcesPanel from "./GithubResourcesPanel.vue";

describe("GithubResourcesPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    api.getGithubProjects.mockResolvedValue([{ number: 7, title: "Roadmap", url: "project", shortDescription: "Plan", public: true, closed: false, id: "P_1", items: { totalCount: 3 }, fields: { totalCount: 4 } }]);
    api.getGithubProjectItems.mockResolvedValue({ totalCount: 1, items: [{ title: "Ship" }] });
    api.getGithubDiscussions.mockResolvedValue([{ id: "D_1", number: 2, title: "Ideas", url: "discussion", createdAt: "now", updatedAt: "now", isAnswered: false, answerChosenAt: null, author: { login: "octocat" }, category: { name: "Ideas" }, comments: { totalCount: 5 } }]);
    api.getGithubCodespaces.mockResolvedValue([{ name: "silver-space", displayName: "RepoRadar", state: "Available", machineName: "basicLinux32gb", createdAt: "now", lastUsedAt: "now", repository: {}, gitStatus: {}, owner: {} }]);
    api.getGithubCodespaceLog.mockResolvedValue("ready");
    api.stopGithubCodespace.mockResolvedValue(undefined);
    api.deleteGithubCodespace.mockResolvedValue(undefined);
    dialog.confirm.mockResolvedValue(true);
  });

  it("loads each resource independently and queries project items", async () => {
    const wrapper = mount(GithubResourcesPanel, {
      props: { path: "D:/repo", busy: false, runAction: async (action: () => Promise<void>) => action() },
    });
    for (const action of ["load-projects", "load-discussions", "load-codespaces"]) {
      await wrapper.get(`[data-action="${action}"]`).trigger("click");
      await flushPromises();
    }
    await wrapper.get('[aria-label="Project 项过滤条件"]').setValue("status:Todo");
    await wrapper.get('[data-action="view-project-items"]').trigger("click");
    await flushPromises();

    expect(wrapper.text()).toContain("Roadmap");
    expect(wrapper.text()).toContain("Ideas");
    expect(wrapper.text()).toContain("silver-space");
    expect(api.getGithubProjectItems).toHaveBeenCalledWith("D:/repo", 7, "status:Todo");
  });

  it("shows logs and confirms forced Codespace deletion", async () => {
    const wrapper = mount(GithubResourcesPanel, {
      props: { path: "D:/repo", busy: false, runAction: async (action: () => Promise<void>) => action() },
    });
    await wrapper.get('[data-action="load-codespaces"]').trigger("click");
    await flushPromises();
    await wrapper.get('[data-action="codespace-log"]').trigger("click");
    await flushPromises();
    expect(wrapper.text()).toContain("ready");
    await wrapper.get('[data-action="delete-codespace"]').trigger("click");
    await flushPromises();

    expect(dialog.confirm).toHaveBeenCalledOnce();
    expect(api.deleteGithubCodespace).toHaveBeenCalledWith("D:/repo", "silver-space", true);
  });
});
