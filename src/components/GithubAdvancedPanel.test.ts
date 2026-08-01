import { flushPromises, mount } from "@vue/test-utils";
import { describe, expect, it, vi } from "vitest";

const api = vi.hoisted(() => ({ runGithubApiRequest: vi.fn(), runSafeGithubCommand: vi.fn() }));
const dialog = vi.hoisted(() => ({ confirm: vi.fn() }));
vi.mock("../api", () => api);
vi.mock("@tauri-apps/plugin-dialog", () => dialog);

import GithubAdvancedPanel from "./GithubAdvancedPanel.vue";

describe("GithubAdvancedPanel", () => {
  it("passes read-only arguments as discrete values without a shell", async () => {
    api.runSafeGithubCommand.mockResolvedValue("[]");
    const wrapper = mount(GithubAdvancedPanel, { props: { path: "D:/repo", busy: false, runAction: async (action: () => Promise<void>) => action() } });
    await wrapper.get('[aria-label="只读 gh 参数"]').setValue("--state\nall");
    await wrapper.get('[data-action="run-safe-gh"]').trigger("click");
    await flushPromises();
    expect(api.runSafeGithubCommand).toHaveBeenCalledWith("D:/repo", "repo-view", ["--state", "all"]);
  });

  it("confirms API mutations and parses literal fields", async () => {
    dialog.confirm.mockResolvedValue(true);
    api.runGithubApiRequest.mockResolvedValue("{}");
    const wrapper = mount(GithubAdvancedPanel, { props: { path: "D:/repo", busy: false, runAction: async (action: () => Promise<void>) => action() } });
    await wrapper.get('[aria-label="GitHub API 方法"]').setValue("PATCH");
    await wrapper.get('[aria-label="GitHub API Endpoint"]').setValue("repos/{owner}/{repo}");
    await wrapper.get('[aria-label="GitHub API 字段"]').setValue("name=RepoRadar\ndescription=Local=GitHub");
    await wrapper.get('[data-action="run-gh-api"]').trigger("click");
    await flushPromises();
    expect(dialog.confirm).toHaveBeenCalledOnce();
    expect(api.runGithubApiRequest).toHaveBeenCalledWith("D:/repo", "PATCH", "repos/{owner}/{repo}", [
      { key: "name", value: "RepoRadar" }, { key: "description", value: "Local=GitHub" },
    ]);
  });
});
