import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import type { Project } from "../api";
import ProjectCatalog from "./ProjectCatalog.vue";

const project: Project = {
  id: 1,
  name: "RepoRadar",
  path: "D:/dev-code/RepoRadar",
  favorite: false,
  tags: [],
  lastSeenAt: "2026-07-30",
};

describe("ProjectCatalog", () => {
  it("emits selection and favorite actions when ready", async () => {
    const wrapper = mount(ProjectCatalog, {
      props: {
        projects: [project],
        visibleProjects: [project],
        busy: false,
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
        busy: true,
      },
    });

    await wrapper.get(".project-select").trigger("click");
    await wrapper.get(".favorite-toggle").trigger("click");
    await wrapper.get(".text-button").trigger("click");

    expect(wrapper.emitted("select")).toBeUndefined();
    expect(wrapper.emitted("favorite")).toBeUndefined();
    expect(wrapper.emitted("rescan")).toBeUndefined();
  });
});
