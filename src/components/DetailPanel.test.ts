import { mount } from "@vue/test-utils";
import { describe, expect, it, vi } from "vitest";

import type { DetailTarget } from "../workspace";
import DetailPanel from "./DetailPanel.vue";

const issue = {
  number: 1,
  title: "Issue",
  state: "OPEN",
  author: null,
  labels: [],
  updatedAt: "now",
  url: "https://github.com/acme/repo/issues/1",
};
const release = {
  tagName: "v1.0.0",
  name: "Version 1",
  isDraft: false,
  isLatest: true,
  isPrerelease: false,
  publishedAt: "now",
};
const run = {
  databaseId: 3,
  name: "CI",
  displayTitle: "Build",
  status: "completed",
  conclusion: "success",
  headBranch: "main",
  event: "push",
  workflowName: "CI",
  createdAt: "now",
  updatedAt: "now",
  url: "https://github.com/acme/repo/actions/runs/3",
};

describe("DetailPanel", () => {
  it("renders each discriminated target and closes without changing context", async () => {
    const target: DetailTarget = { kind: "issue", item: issue };
    const wrapper = mount(DetailPanel, {
      props: {
        target,
        path: "D:/repo",
        busy: false,
        pullRequestDetail: null,
        runLog: "",
        runAction: vi.fn(),
      },
      global: {
        stubs: {
          GithubIssueDetailPanel: { template: "<div data-detail-type=issue></div>" },
          GithubReleaseDetailPanel: { template: "<div data-detail-type=release></div>" },
          GithubRunDetailPanel: { template: "<div data-detail-type=run></div>" },
        },
      },
    });

    expect(wrapper.find('[data-detail-type="issue"]').exists()).toBe(true);
    await wrapper.get('[aria-label="关闭详情面板"]').trigger("click");
    expect(wrapper.emitted("close")).toHaveLength(1);

    await wrapper.setProps({ target: { kind: "release", item: release } });
    expect(wrapper.find('[data-detail-type="release"]').exists()).toBe(true);
    await wrapper.setProps({ target: { kind: "run", item: run } });
    expect(wrapper.find('[data-detail-type="run"]').exists()).toBe(true);
  });
});
