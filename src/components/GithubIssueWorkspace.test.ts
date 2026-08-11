import { mount } from "@vue/test-utils";
import { describe, expect, it, vi } from "vitest";

import type { GithubIssue } from "../api";

import GithubIssueWorkspace from "./GithubIssueWorkspace.vue";

const issue: GithubIssue = {
  number: 9,
  title: "Issue details",
  state: "OPEN",
  author: { login: "octocat" },
  labels: [],
  updatedAt: "now",
  url: "https://github.com/acme/repo/issues/9",
};

describe("GithubIssueWorkspace", () => {
  it("emits the selected issue so the shell can open its detail panel", async () => {
    const runAction = vi.fn();
    const wrapper = mount(GithubIssueWorkspace, {
      props: { path: "D:/repo", issues: [issue], busy: false, runAction },
    });

    await wrapper.get('[data-action="view-issue"]').trigger("click");

    expect(wrapper.emitted("view")).toEqual([[issue]]);
  });
});
