import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import AppSidebar from "./AppSidebar.vue";
import OperationsPanel from "./OperationsPanel.vue";

describe("app chrome", () => {
  it("uses the primary navigation tabs for GitHub and operations", async () => {
    const wrapper = mount(AppSidebar, {
      props: {
        activeSection: "overview",
        projectCount: 2,
        githubDisabled: true,
      },
    });
    const githubButton = wrapper.get("nav button:nth-of-type(3)");

    expect(githubButton.attributes("disabled")).toBeDefined();
    await githubButton.trigger("click");
    expect(wrapper.emitted("navigate")).toBeUndefined();

    await wrapper.setProps({ githubDisabled: false });
    await githubButton.trigger("click");
    expect(wrapper.emitted("navigate")).toEqual([["github"]]);

    await wrapper.get("nav button:nth-of-type(4)").trigger("click");
    expect(wrapper.emitted("navigate")).toEqual([["github"], ["operations"]]);
  });

  it("exposes the active view to assistive technology", async () => {
    const wrapper = mount(AppSidebar, {
      props: {
        activeSection: "overview",
        projectCount: 0,
        githubDisabled: true,
      },
    });

    expect(wrapper.get("nav button:nth-of-type(1)").attributes("aria-current")).toBe("page");
    expect(wrapper.get("nav button:nth-of-type(2)").attributes("aria-current")).toBeUndefined();

    await wrapper.setProps({ activeSection: "operations" });

    expect(wrapper.get("nav button:nth-of-type(4)").attributes("aria-current")).toBe("page");
  });

  it("emits clear from the operation panel without exposing action details", async () => {
    const wrapper = mount(OperationsPanel, {
      props: {
        scanning: false,
        operations: [
          {
            id: 1,
            label: "Push",
            state: "failed",
            startedAt: 1,
            finishedAt: 2,
            error: "rejected",
          },
        ],
      },
    });

    expect(wrapper.text()).toContain("Push");
    expect(wrapper.text()).toContain("rejected");
    expect(wrapper.text()).toContain("不保存命令参数");
    await wrapper.get(".operations-actions button").trigger("click");
    expect(wrapper.emitted("clear")).toHaveLength(1);
  });
});
