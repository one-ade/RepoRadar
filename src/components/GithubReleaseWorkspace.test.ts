import { flushPromises, mount } from "@vue/test-utils";
import { beforeEach, describe, expect, it, vi } from "vitest";

import type { GithubRelease } from "../api";

const api = vi.hoisted(() => ({
  editGithubRelease: vi.fn(),
  getReleaseDetail: vi.fn(),
  uploadGithubReleaseAssets: vi.fn(),
}));
const dialog = vi.hoisted(() => ({ confirm: vi.fn(), open: vi.fn() }));
vi.mock("../api", async (importOriginal) => ({
  ...(await importOriginal<typeof import("../api")>()),
  ...api,
}));
vi.mock("@tauri-apps/plugin-dialog", () => dialog);

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
  beforeEach(() => {
    vi.clearAllMocks();
    api.getReleaseDetail.mockResolvedValue({
      fields: [
        { name: "tagName", value: "v1.0.0" },
        { name: "body", value: "Notes" },
        { name: "futureField", value: true },
      ],
    });
    api.editGithubRelease.mockResolvedValue(undefined);
    api.uploadGithubReleaseAssets.mockResolvedValue(undefined);
    dialog.confirm.mockResolvedValue(true);
    dialog.open.mockResolvedValue(["D:/dist/app.zip"]);
  });

  it("loads all details and submits every supported edit field", async () => {
    const runAction = vi.fn(async (action: () => Promise<void>) => action());
    const wrapper = mount(GithubReleaseWorkspace, {
      props: { path: "D:/repo", releases: [release], busy: false, runAction },
    });

    await wrapper.get('[data-action="view-release"]').trigger("click");
    await flushPromises();

    expect(api.getReleaseDetail).toHaveBeenCalledWith("D:/repo", "v1.0.0");
    expect(wrapper.text()).toContain("futureField");
    expect(wrapper.findAll("[data-release-edit-field]")).toHaveLength(10);

    await wrapper.get('[aria-label="新 Release 标题"]').setValue("Version 1.1");
    await wrapper.get('[data-action="save-release-edit"]').trigger("click");
    await flushPromises();

    expect(api.editGithubRelease).toHaveBeenCalledWith(
      "D:/repo",
      "v1.0.0",
      expect.objectContaining({ title: "Version 1.1", verifyTag: false }),
    );
    expect(wrapper.emitted("refresh")).toHaveLength(1);
  });

  it("labels uploads and confirms destructive replacement", async () => {
    const wrapper = mount(GithubReleaseWorkspace, {
      props: {
        path: "D:/repo",
        releases: [release],
        busy: false,
        runAction: async (action: () => Promise<void>) => action(),
      },
    });
    await wrapper.get('[data-action="view-release"]').trigger("click");
    await flushPromises();
    await wrapper.get('[data-action="choose-release-assets"]').trigger("click");
    await wrapper.get('[aria-label="资源显示名称 1"]').setValue("Windows");
    await wrapper.get('[aria-label="覆盖同名资源"]').setValue(true);
    await wrapper.get('[data-action="upload-release-assets"]').trigger("click");
    await flushPromises();

    expect(dialog.confirm).toHaveBeenCalledOnce();
    expect(api.uploadGithubReleaseAssets).toHaveBeenCalledWith(
      "D:/repo",
      "v1.0.0",
      ["D:/dist/app.zip#Windows"],
      true,
    );
  });
});
