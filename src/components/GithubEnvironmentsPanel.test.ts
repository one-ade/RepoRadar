import { flushPromises, mount } from "@vue/test-utils";
import { beforeEach, describe, expect, it, vi } from "vitest";

const api = vi.hoisted(() => ({
  deleteGithubEnvironment: vi.fn(), deleteGithubEnvironmentSecret: vi.fn(),
  deleteGithubEnvironmentVariable: vi.fn(), getGithubEnvironmentConfiguration: vi.fn(),
  getGithubEnvironments: vi.fn(), saveGithubEnvironment: vi.fn(),
  setGithubEnvironmentSecret: vi.fn(), setGithubEnvironmentVariable: vi.fn(),
}));
vi.mock("../api", () => api);
vi.mock("@tauri-apps/plugin-dialog", () => ({ confirm: vi.fn().mockResolvedValue(true) }));

import GithubEnvironmentsPanel from "./GithubEnvironmentsPanel.vue";

describe("GithubEnvironmentsPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    api.getGithubEnvironments.mockResolvedValue([{
      id: 1, name: "production", createdAt: "2026-08-01", updatedAt: "2026-08-02",
      protectionRules: [], deploymentBranchPolicy: null,
    }]);
    api.getGithubEnvironmentConfiguration.mockResolvedValue({
      variables: [{ name: "REGION", value: "cn-east", updatedAt: "now" }],
      secrets: [{ name: "DEPLOY_TOKEN", updatedAt: "now" }],
    });
    api.saveGithubEnvironment.mockResolvedValue(undefined);
    api.setGithubEnvironmentVariable.mockResolvedValue(undefined);
    api.setGithubEnvironmentSecret.mockResolvedValue(undefined);
  });

  it("loads environments and scoped configuration without exposing secret values", async () => {
    const wrapper = mount(GithubEnvironmentsPanel, {
      props: { path: "D:/repo", busy: false, runAction: async (action: () => Promise<void>) => action() },
    });
    await wrapper.get('[data-action="load-environments"]').trigger("click");
    await flushPromises();
    await wrapper.get('[data-action="view-environment"]').trigger("click");
    await flushPromises();

    expect(api.getGithubEnvironmentConfiguration).toHaveBeenCalledWith("D:/repo", "production");
    expect(wrapper.text()).toContain("REGION");
    expect(wrapper.text()).toContain("DEPLOY_TOKEN");
    expect(wrapper.text()).toContain("值不可见");
    expect(wrapper.find('input[type="password"]').exists()).toBe(true);
  });

  it("creates environments and clears secret values before saving", async () => {
    let finishSecret = () => {};
    api.setGithubEnvironmentSecret.mockImplementation(() => new Promise<void>((resolve) => { finishSecret = resolve; }));
    const wrapper = mount(GithubEnvironmentsPanel, {
      props: { path: "D:/repo", busy: false, runAction: async (action: () => Promise<void>) => action() },
    });
    await wrapper.get('[data-action="load-environments"]').trigger("click");
    await flushPromises();
    await wrapper.get('[aria-label="新部署环境名称"]').setValue("staging");
    await wrapper.get('[data-action="save-environment"]').trigger("click");
    await flushPromises();
    expect(api.saveGithubEnvironment).toHaveBeenCalledWith("D:/repo", "staging");

    await wrapper.get('[data-action="view-environment"]').trigger("click");
    await flushPromises();
    await wrapper.get('[aria-label="环境密钥名"]').setValue("TOKEN");
    await wrapper.get('[aria-label="环境密钥值"]').setValue("secret-value");
    await wrapper.get('[data-action="save-environment-secret"]').trigger("click");
    expect(wrapper.get('[aria-label="环境密钥值"]').element).toHaveProperty("value", "");
    finishSecret();
    await flushPromises();
  });
});
