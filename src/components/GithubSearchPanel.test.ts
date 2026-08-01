import { flushPromises, mount } from "@vue/test-utils";
import { describe, expect, it, vi } from "vitest";

const api = vi.hoisted(() => ({ searchGithub: vi.fn() }));
vi.mock("../api", () => api);

import GithubSearchPanel from "./GithubSearchPanel.vue";

describe("GithubSearchPanel", () => {
  it("runs every query through the selected global scope and preserves unknown fields", async () => {
    api.searchGithub.mockResolvedValue([{ fullName: "acme/radar", url: "https://github.com/acme/radar", futureField: { score: 9 } }]);
    const wrapper = mount(GithubSearchPanel, {
      props: { path: "D:/repo", busy: false, runAction: async (action: () => Promise<void>) => action() },
    });
    await wrapper.get('[aria-label="GitHub 搜索类型"]').setValue("repos");
    await wrapper.get('[aria-label="GitHub 搜索条件"]').setValue("radar stars:>5");
    await wrapper.get('[aria-label="仅当前仓库"]').setValue(true);
    await wrapper.get("form").trigger("submit");
    await flushPromises();

    expect(api.searchGithub).toHaveBeenCalledWith("D:/repo", "repos", "radar stars:>5", true);
    expect(wrapper.text()).toContain("acme/radar");
    expect(wrapper.text()).toContain("futureField");
  });
});
