import { flushPromises, mount } from "@vue/test-utils";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { GithubConfiguration } from "../api";
import GithubConfigurationPanel from "./GithubConfigurationPanel.vue";

const configuration: GithubConfiguration = {
  variables: [{ name: "DEPLOY_REGION", value: "cn-east", updatedAt: "2026-07-30" }],
  secrets: [{ name: "DEPLOY_TOKEN", updatedAt: "2026-07-30" }],
  labels: [{ name: "bug", color: "d73a4a", description: "缺陷" }],
};

const api = vi.hoisted(() => ({
  checkGithubRulesets: vi.fn(),
  deleteGithubLabel: vi.fn(),
  deleteGithubSecret: vi.fn(),
  deleteGithubVariable: vi.fn(),
  getGithubConfiguration: vi.fn(),
  saveGithubLabel: vi.fn(),
  setGithubSecret: vi.fn(),
  setGithubVariable: vi.fn(),
}));

vi.mock("../api", () => api);
vi.mock("@tauri-apps/plugin-dialog", () => ({ confirm: vi.fn() }));

describe("GithubConfigurationPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    api.getGithubConfiguration.mockResolvedValue(configuration);
  });

  it("loads repository configuration without exposing secret values", async () => {
    const wrapper = mount(GithubConfigurationPanel, {
      props: {
        path: "D:/repo",
        busy: false,
        runAction: async (action: () => Promise<void>) => action(),
      },
    });

    await wrapper.get("[data-testid='load-github-configuration']").trigger("click");
    await flushPromises();

    expect(wrapper.text()).toContain("Variables · 1");
    expect(wrapper.text()).toContain("DEPLOY_REGION");
    expect(wrapper.text()).toContain("Secrets · 1");
    expect(wrapper.text()).toContain("DEPLOY_TOKEN");
    expect(wrapper.text()).toContain("值不可见");
    expect(wrapper.text()).not.toContain("secret-value");
  });

  it("clears the secret input before the save request finishes", async () => {
    let finishRequest = () => {};
    api.setGithubSecret.mockImplementation(
      () =>
        new Promise<void>((resolve) => {
          finishRequest = resolve;
        }),
    );
    const wrapper = mount(GithubConfigurationPanel, {
      props: {
        path: "D:/repo",
        busy: false,
        runAction: async (action: () => Promise<void>) => action(),
      },
    });
    await wrapper.get("[data-testid='load-github-configuration']").trigger("click");
    await flushPromises();
    await wrapper.get("[aria-label='GitHub 密钥名']").setValue("DEPLOY_TOKEN");
    await wrapper.get("[aria-label='GitHub 密钥值']").setValue("secret-value");
    const saveButton = wrapper.findAll("button").find((button) => button.text() === "保存密钥");
    expect(saveButton).toBeDefined();
    if (!saveButton) return;

    await saveButton.trigger("click");

    expect(wrapper.get("[aria-label='GitHub 密钥值']").element).toHaveProperty("value", "");
    finishRequest();
    await flushPromises();
  });

  it("records one parent operation when saving a variable", async () => {
    const runAction = vi.fn(async (action: () => Promise<void>) => action());
    const wrapper = mount(GithubConfigurationPanel, {
      props: { path: "D:/repo", busy: false, runAction },
    });
    await wrapper.get("[data-testid='load-github-configuration']").trigger("click");
    await flushPromises();
    runAction.mockClear();
    await wrapper.get("[aria-label='GitHub 变量名']").setValue("DEPLOY_REGION");
    await wrapper.get("[aria-label='GitHub 变量值']").setValue("cn-north");
    const saveButton = wrapper.findAll("button").find((button) => button.text() === "保存变量");
    expect(saveButton).toBeDefined();
    if (!saveButton) return;

    await saveButton.trigger("click");
    await flushPromises();

    expect(runAction).toHaveBeenCalledTimes(1);
  });

  it("emits a success notice after saving a variable", async () => {
    const wrapper = mount(GithubConfigurationPanel, {
      props: {
        path: "D:/repo",
        busy: false,
        runAction: async (action: () => Promise<void>) => action(),
      },
    });
    await wrapper.get("[data-testid='load-github-configuration']").trigger("click");
    await flushPromises();
    await wrapper.get("[aria-label='GitHub 变量名']").setValue("DEPLOY_REGION");
    await wrapper.get("[aria-label='GitHub 变量值']").setValue("cn-south");
    const saveButton = wrapper.findAll("button").find((button) => button.text() === "保存变量");
    expect(saveButton).toBeDefined();
    if (!saveButton) return;

    await saveButton.trigger("click");
    await flushPromises();

    expect(wrapper.emitted("notice")).toEqual([["GitHub 变量已保存"]]);
  });

  it("uses a valid native color input value", async () => {
    const wrapper = mount(GithubConfigurationPanel, {
      props: {
        path: "D:/repo",
        busy: false,
        runAction: async (action: () => Promise<void>) => action(),
      },
    });
    await wrapper.get("[data-testid='load-github-configuration']").trigger("click");
    await flushPromises();

    expect(wrapper.get("[aria-label='GitHub 标签颜色']").element).toHaveProperty(
      "value",
      "#7c3aed",
    );
  });
});
