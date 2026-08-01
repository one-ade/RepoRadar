import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import GitDetailControls from "./GitDetailControls.vue";

const props = {
  project: {
    id: 1,
    name: "RepoRadar",
    path: "D:/RepoRadar",
    favorite: false,
    tags: [],
    lastSeenAt: "now",
  },
  status: {
    branch: "main",
    upstream: "origin/main",
    ahead: 0,
    behind: 0,
    files: [],
  },
  diff: "",
  branches: [],
  history: [],
  tab: "changes" as const,
  commitMessage: "",
  newBranch: "",
};

describe("GitDetailControls", () => {
  it("emits tab and editor changes through typed component boundaries", async () => {
    const wrapper = mount(GitDetailControls, { props: { ...props, busy: false } });

    await wrapper.get('[aria-label="提交信息"]').setValue("feat: split controls");
    await wrapper.findAll(".detail-tab")[1]!.trigger("click");

    expect(wrapper.emitted("update:commitMessage")?.at(-1)).toEqual([
      "feat: split controls",
    ]);
    expect(wrapper.emitted("select-tab")?.[0]).toEqual(["branches"]);
  });

  it("does not emit Git actions from disabled buttons", async () => {
    const wrapper = mount(GitDetailControls, { props: { ...props, busy: true } });

    await wrapper.get(".git-actions button").trigger("click");

    expect(wrapper.emitted("stage")).toBeUndefined();
  });
});
