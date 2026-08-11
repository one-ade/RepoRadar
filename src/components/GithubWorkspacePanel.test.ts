import { mount } from "@vue/test-utils";
import { describe, expect, it, vi } from "vitest";

import type { GithubOverview } from "../api";
import GithubWorkspacePanel from "./GithubWorkspacePanel.vue";

const overview: GithubOverview = {
  repository: {
    nameWithOwner: "acme/repo",
    description: "Demo",
    isPrivate: true,
    defaultBranchRef: { name: "main" },
    url: "https://github.com/acme/repo",
    stargazerCount: 2,
    forkCount: 1,
  },
  pullRequests: [],
  issues: [],
  workflows: [],
  runs: [],
  releases: [],
};

const pullRequest = {
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

const baseProps = {
  path: "D:/repo",
  busy: false,
  runLog: "",
  runAction: vi.fn(),
  githubTitle: "",
  githubBody: "",
  githubComment: "",
  releaseTag: "",
  releaseTitle: "",
  releaseNotes: "",
  cloneReference: "",
  repositoryName: "",
  repositoryVisibility: "private" as const,
  repositoryDescription: "",
  selectedPullRequest: null,
  pullRequestDetail: null,
};

describe("GithubWorkspacePanel", () => {
  it("renders the disconnected repository tools without leaking secret inputs", () => {
    const wrapper = mount(GithubWorkspacePanel, {
      props: { ...baseProps, overview: null },
    });

    expect(wrapper.text()).toContain("为当前本地项目连接 GitHub");
    expect(wrapper.find('input[type="password"]').exists()).toBe(false);
  });

  it("emits explicit GitHub actions from the loaded workspace", async () => {
    const wrapper = mount(GithubWorkspacePanel, {
      props: { ...baseProps, overview: { ...overview, pullRequests: [pullRequest] } },
      global: {
        stubs: { GithubConfigurationPanel: true },
      },
    });

    await wrapper
      .findAll("button")
      .find((button) => button.text() === "新建 Issue")!
      .trigger("click");
    await wrapper
      .findAll("button")
      .find((button) => button.text() === "创建 Issue")!
      .trigger("click");
    await wrapper.get('[data-action="view-pr"]').trigger("click");

    expect(wrapper.text()).toContain("acme/repo");
    expect(wrapper.emitted("create-item")?.[0]).toEqual(["issue"]);
    expect(wrapper.emitted("view-pull-request")?.[0]).toEqual([pullRequest]);
  });

  it("mounts low-frequency GitHub tools only inside the Tools section", async () => {
    const wrapper = mount(GithubWorkspacePanel, {
      props: { ...baseProps, overview, section: "pull-requests" },
      global: {
        stubs: {
          GithubConfigurationPanel: { template: '<div data-test="configuration-tool"></div>' },
          GithubEnvironmentsPanel: { template: '<div data-test="environment-tool"></div>' },
          GithubResourcesPanel: { template: '<div data-test="resource-tool"></div>' },
          GithubSearchPanel: { template: '<div data-test="search-tool"></div>' },
          GithubAdvancedPanel: { template: '<div data-test="advanced-tool"></div>' },
        },
      },
    });

    expect(wrapper.find('[data-test="configuration-tool"]').exists()).toBe(false);
    await wrapper.setProps({ section: "tools" });
    expect(wrapper.find('[data-test="configuration-tool"]').exists()).toBe(true);
  });
});
