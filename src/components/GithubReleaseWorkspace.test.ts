import { mount } from "@vue/test-utils";
import { describe, expect, it, vi } from "vitest";

import type { GithubRelease } from "../api";

import GithubReleaseWorkspace from "./GithubReleaseWorkspace.vue";

const release: GithubRelease = {
  tagName: "v1.0.0",
  name: "Version 1",
  isDraft: false,
  isLatest: true,
  isPrerelease: false,
  publishedAt: "now",
};

describe("GithubReleaseWorkspace", () => {
  it("emits the selected release so the shell can open its detail panel", async () => {
    const runAction = vi.fn();
    const wrapper = mount(GithubReleaseWorkspace, {
      props: { path: "D:/repo", releases: [release], busy: false, runAction },
    });

    await wrapper.get('[data-action="view-release"]').trigger("click");
    expect(wrapper.emitted("view")).toEqual([[release]]);
  });
});
