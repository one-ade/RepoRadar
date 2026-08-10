import { mount } from "@vue/test-utils";
import { describe, expect, it, vi } from "vitest";

import type { Project } from "../api";
import ProjectCatalog from "./ProjectCatalog.vue";

const project: Project = {
  id: 1,
  name: "RepoRadar",
  path: "D:/dev-code/RepoRadar",
  favorite: false,
  tags: ["rust"],
  lastSeenAt: "2026-07-30",
};

describe("ProjectCatalog", () => {
  it("shows a stable loading skeleton before the project list is ready", () => {
    const wrapper = mount(ProjectCatalog, {
      props: {
        projects: [],
        visibleProjects: [],
        loading: true,
        busy: false,
        saveTags: async () => true,
      },
    });

    expect(wrapper.find(".project-skeleton").exists()).toBe(true);
    expect(wrapper.find(".empty-content").exists()).toBe(false);
  });

  it("shows a retryable error instead of an indistinguishable empty state", async () => {
    const wrapper = mount(ProjectCatalog, {
      props: {
        projects: [],
        visibleProjects: [],
        loading: false,
        error: "数据库暂不可用",
        busy: false,
        saveTags: async () => true,
      },
    });

    expect(wrapper.get(".project-error-content").text()).toContain("项目加载失败");
    expect(wrapper.find(".empty-content").exists()).toBe(false);
    await wrapper.get(".retry-action").trigger("click");

    expect(wrapper.emitted("retry")).toHaveLength(1);
  });

  it("emits selection and favorite actions when ready", async () => {
    const wrapper = mount(ProjectCatalog, {
      props: {
        projects: [project],
        visibleProjects: [project],
        loading: false,
        busy: false,
        saveTags: async () => true,
      },
    });

    await wrapper.get(".project-select").trigger("click");
    await wrapper.get(".favorite-toggle").trigger("click");

    expect(wrapper.emitted("select")?.[0]).toEqual([project]);
    expect(wrapper.emitted("favorite")?.[0]).toEqual([project]);
  });

  it("blocks project actions while busy", async () => {
    const wrapper = mount(ProjectCatalog, {
      props: {
        projects: [project],
        visibleProjects: [project],
        loading: false,
        busy: true,
        saveTags: async () => true,
      },
    });

    await wrapper.get(".project-select").trigger("click");
    await wrapper.get(".favorite-toggle").trigger("click");
    await wrapper.get(".tag-edit").trigger("click");
    await wrapper.get(".text-button").trigger("click");

    expect(wrapper.emitted("select")).toBeUndefined();
    expect(wrapper.emitted("favorite")).toBeUndefined();
    expect(wrapper.emitted("rescan")).toBeUndefined();
    expect(wrapper.find(".tag-editor").exists()).toBe(false);
  });

  it("shows tags and saves a comma-separated draft", async () => {
    const saveTags = vi.fn().mockResolvedValue(true);
    const wrapper = mount(ProjectCatalog, {
      props: {
        projects: [project],
        visibleProjects: [project],
        loading: false,
        busy: false,
        saveTags,
      },
    });

    expect(wrapper.get(".project-tag").text()).toBe("rust");
    await wrapper.get(".tag-edit").trigger("click");
    await wrapper.get(".tag-editor input").setValue("rust, frontend，desktop");
    await wrapper.get(".tag-editor").trigger("submit");

    expect(saveTags).toHaveBeenCalledWith(project, ["rust", "frontend", "desktop"]);
    expect(wrapper.find(".tag-editor").exists()).toBe(false);
  });

  it("keeps a failed tag draft and lets the user cancel", async () => {
    const saveTags = vi.fn().mockResolvedValue(false);
    const wrapper = mount(ProjectCatalog, {
      props: {
        projects: [project],
        visibleProjects: [project],
        loading: false,
        busy: false,
        saveTags,
      },
    });

    await wrapper.get(".tag-edit").trigger("click");
    await wrapper.get(".tag-editor input").setValue("needs-review");
    await wrapper.get(".tag-editor").trigger("submit");

    expect(wrapper.get<HTMLInputElement>(".tag-editor input").element.value).toBe("needs-review");
    await wrapper.get(".tag-cancel").trigger("click");
    expect(wrapper.find(".tag-editor").exists()).toBe(false);
  });
});
