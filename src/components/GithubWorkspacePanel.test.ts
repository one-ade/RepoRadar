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
      props: { ...baseProps, overview },
      global: {
        stubs: { GithubConfigurationPanel: true },
      },
    });

    const createIssue = wrapper
      .findAll("button")
      .find((button) => button.text() === "创建 Issue");
    await createIssue!.trigger("click");

    expect(wrapper.text()).toContain("acme/repo");
    expect(wrapper.emitted("create-item")?.[0]).toEqual(["issue"]);
  });
});
