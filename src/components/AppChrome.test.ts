import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";

import AppSidebar from "./AppSidebar.vue";
import OperationsPanel from "./OperationsPanel.vue";

describe("app chrome", () => {
  it("keeps GitHub navigation disabled until a project is ready", async () => {
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
    expect(wrapper.emitted("github")).toBeUndefined();

    await wrapper.setProps({ githubDisabled: false });
    await githubButton.trigger("click");
    expect(wrapper.emitted("github")).toHaveLength(1);
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
