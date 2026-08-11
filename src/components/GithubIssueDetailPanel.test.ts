import { flushPromises, mount } from "@vue/test-utils";
import { beforeEach, describe, expect, it, vi } from "vitest";

import type { GithubIssue } from "../api";

const api = vi.hoisted(() => ({ getIssueDetail: vi.fn(), editIssue: vi.fn() }));
vi.mock("../api", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../api")>()),
  ...api,
}));

import GithubIssueDetailPanel from "./GithubIssueDetailPanel.vue";

const issue: GithubIssue = {
  number: 9,
  title: "Issue details",
  state: "OPEN",
  author: { login: "octocat" },
  labels: [],
  updatedAt: "now",
  url: "https://github.com/acme/repo/issues/9",
};

describe("GithubIssueDetailPanel", () => {
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

  it("loads detail and preserves the full typed edit form", async () => {
    const runAction = vi.fn(async (action: () => Promise<void>) => action());
    const wrapper = mount(GithubIssueDetailPanel, {
      props: { path: "D:/repo", issue, busy: false, runAction },
    });
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

  it("shows an error and retry action when detail loading fails", async () => {
    api.getIssueDetail.mockRejectedValueOnce(new Error("gh --token secret-token"));
    const runAction = vi.fn(async (action: () => Promise<void>) => {
      try {
        await action();
      } catch {
        // Match App.runProjectAction: the operation ledger handles the failure.
      }
    });
    const wrapper = mount(GithubIssueDetailPanel, {
      props: { path: "D:/repo", issue, busy: false, runAction },
    });
    await flushPromises();

    expect(wrapper.text()).toContain("完整详情加载失败");
    expect(wrapper.get('[data-action="retry-issue-detail"]')).toBeTruthy();
  });
});
