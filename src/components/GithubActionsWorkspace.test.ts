import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import GithubActionsWorkspace from "./GithubActionsWorkspace.vue";

const overview = {
  repository: {
    nameWithOwner: "acme/repo",
    description: null,
    isPrivate: true,
    defaultBranchRef: { name: "main" },
    url: "https://github.com/acme/repo",
    stargazerCount: 0,
    forkCount: 0,
  },
  pullRequests: [],
  issues: [],
  workflows: [{ id: 4, name: "CI", path: ".github/workflows/ci.yml", state: "active" }],
  runs: [{
    databaseId: 8,
    name: "CI",
    displayTitle: "Build",
    status: "completed",
    conclusion: "success",
    headBranch: "main",
    event: "push",
    workflowName: "CI",
    createdAt: "now",
    updatedAt: "now",
    url: "https://github.com/acme/repo/actions/runs/8",
  }],
  releases: [],
};

describe("GithubActionsWorkspace", () => {
  it("keeps workflow and run actions explicit", async () => {
    const wrapper = mount(GithubActionsWorkspace, {
      props: { overview, busy: false, runLog: "" },
    });

    await wrapper.get(".github-row button").trigger("click");
    await wrapper.findAll(".github-row")[1]!.get("button").trigger("click");

    expect(wrapper.emitted("dispatch-workflow")).toEqual([[4]]);
    expect(wrapper.emitted("view-run")).toEqual([[8]]);
  });
});
