import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import type { GithubPullRequest, GithubPullRequestDetail } from "../api";
import GithubPullRequestDetailPanel from "./GithubPullRequestDetailPanel.vue";

const pullRequest: GithubPullRequest = {
  number: 7,
  title: "Show every detail",
  state: "OPEN",
  author: { login: "octocat" },
  headRefName: "feature",
  baseRefName: "main",
  isDraft: false,
  updatedAt: "now",
  url: "https://github.com/acme/repo/pull/7",
};

describe("GithubPullRequestDetailPanel", () => {
  it("renders scalar, complex, and unmatched fields without dropping data", async () => {
    const detail: GithubPullRequestDetail = {
      fields: [
        { name: "title", value: "Show every detail" },
        { name: "comments", value: [{ author: { login: "octocat" }, body: "Ship it" }] },
        { name: "futureField", value: true },
      ],
    };
    const wrapper = mount(GithubPullRequestDetailPanel, {
      props: { pullRequest, detail },
    });

    expect(wrapper.text()).toContain("Show every detail");
    expect(wrapper.text()).toContain("futureField");
    expect(wrapper.text()).toContain("其他");
    expect(wrapper.find("details pre").text()).toContain("Ship it");
    await wrapper.get('[aria-label="关闭 Pull Request 详情"]').trigger("click");
    expect(wrapper.emitted("close")).toHaveLength(1);
  });

  it("shows a stable loading state while details are fetched", () => {
    const wrapper = mount(GithubPullRequestDetailPanel, {
      props: { pullRequest, detail: null },
    });

    expect(wrapper.text()).toContain("正在加载完整详情");
  });
});
