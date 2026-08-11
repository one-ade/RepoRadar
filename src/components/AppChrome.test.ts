import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import AppSidebar from "./AppSidebar.vue";
import OperationsPanel from "./OperationsPanel.vue";

describe("app chrome", () => {
  it("uses the global rail for repositories, activity, and diagnostics", async () => {
    const wrapper = mount(AppSidebar, {
      props: {
        activeSection: "repositories",
        activeOperationCount: 2,
      },
    });
    await wrapper.get("nav button:nth-of-type(2)").trigger("click");
    await wrapper.get("nav button:nth-of-type(3)").trigger("click");

    expect(wrapper.emitted("navigate")).toEqual([["activity"], ["diagnostics"]]);
  });

  it("exposes the active view to assistive technology", async () => {
    const wrapper = mount(AppSidebar, {
      props: {
        activeSection: "repositories",
        activeOperationCount: 0,
      },
    });

    expect(wrapper.get("nav button:nth-of-type(1)").attributes("aria-current")).toBe("page");
    expect(wrapper.get("nav button:nth-of-type(2)").attributes("aria-current")).toBeUndefined();

    await wrapper.setProps({ activeSection: "activity" });

    expect(wrapper.get("nav button:nth-of-type(2)").attributes("aria-current")).toBe("page");
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
          },
        ],
      },
    });

    expect(wrapper.text()).toContain("Push");
    expect(wrapper.text()).not.toContain("rejected");
    expect(wrapper.text()).toContain("不保存命令参数");
    await wrapper.get(".operations-actions button").trigger("click");
    expect(wrapper.emitted("clear")).toHaveLength(1);
  });
});
