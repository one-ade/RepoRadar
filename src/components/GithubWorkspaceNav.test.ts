import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import GithubWorkspaceNav from "./GithubWorkspaceNav.vue";

describe("GithubWorkspaceNav", () => {
  it("emits repository-local section changes and exposes the current section", async () => {
    const wrapper = mount(GithubWorkspaceNav, { props: { section: "pull-requests" } });

    expect(wrapper.get("nav button").attributes("aria-current")).toBe("page");
    await wrapper.get("nav button:nth-of-type(3)").trigger("click");

    expect(wrapper.emitted("select")).toEqual([["actions"]]);
  });
});
