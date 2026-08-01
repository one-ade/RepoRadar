import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import type { GithubPullRequest } from "../api";
import GithubPullRequestSection from "./GithubPullRequestSection.vue";

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

describe("GithubPullRequestSection", () => {
  it("emits detail and every supported review action", async () => {
    const wrapper = mount(GithubPullRequestSection, {
      props: { pullRequests: [pullRequest], busy: false },
    });

    await wrapper.get('[data-action="view-pr"]').trigger("click");
    await wrapper.get('[data-action="request-changes"]').trigger("click");

    expect(wrapper.emitted("view")?.[0]).toEqual([pullRequest]);
    expect(wrapper.emitted("review")?.[0]).toEqual([7, "request-changes"]);
    expect(wrapper.text()).toContain("Show every detail");
  });
});
