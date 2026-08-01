import { flushPromises, mount } from "@vue/test-utils";
import { beforeEach, describe, expect, it, vi } from "vitest";

import type { GithubIssue } from "../api";

const api = vi.hoisted(() => ({ getIssueDetail: vi.fn(), editIssue: vi.fn() }));
vi.mock("../api", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../api")>()),
  ...api,
}));

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
  beforeEach(() => {
    vi.clearAllMocks();
    api.getIssueDetail.mockResolvedValue({
      fields: [
        { name: "title", value: "Issue details" },
        { name: "body", value: "Steps" },
        { name: "futureField", value: true },
      ],
    });
    api.editIssue.mockResolvedValue(undefined);
  });

  it("loads every detail and submits the typed edit form", async () => {
    const runAction = vi.fn(async (action: () => Promise<void>) => action());
    const wrapper = mount(GithubIssueWorkspace, {
      props: { path: "D:/repo", issues: [issue], busy: false, runAction },
    });

    await wrapper.get('[data-action="view-issue"]').trigger("click");
    await flushPromises();

    expect(api.getIssueDetail).toHaveBeenCalledWith("D:/repo", 9);
    expect(wrapper.text()).toContain("futureField");
    expect(wrapper.findAll("[data-edit-field]")).toHaveLength(20);

    await wrapper.get('[aria-label="添加标签"]').setValue("bug");
    await wrapper.get('[data-action="save-issue-edit"]').trigger("click");
    await flushPromises();

    expect(api.editIssue).toHaveBeenCalledWith(
      "D:/repo",
      9,
      expect.objectContaining({ title: "Issue details", body: "Steps", addLabels: "bug" }),
    );
    expect(wrapper.emitted("refresh")).toHaveLength(1);
  });
});
