import { flushPromises, mount } from "@vue/test-utils";
import { describe, expect, it, vi } from "vitest";

const unlisten = vi.hoisted(() => vi.fn());
const listen = vi.hoisted(() => vi.fn().mockResolvedValue(unlisten));
const api = vi.hoisted(() => {
  const fn = () => vi.fn();
  return {
    addProject: fn(),
    addScanRoot: fn(),
    cancelScan: fn(),
    setProjectFavorite: fn(),
    getEnvironment: vi.fn().mockResolvedValue({
      git: { installed: true, version: "git", authenticated: null },
      gh: { installed: true, version: "gh", authenticated: true },
      githubHosts: [],
      databaseReady: true,
    }),
    inspectProjectPath: fn(),
    listProjects: vi.fn().mockResolvedValue([]),
    scanProjects: fn(),
    commit: fn(),
    createBranch: fn(),
    deleteBranch: fn(),
    fetch: fn(),
    getGitDiff: fn(),
    getGitLog: fn(),
    getGitStatus: fn(),
    listBranches: fn(),
    pull: fn(),
    push: fn(),
    stageAll: fn(),
    switchBranch: fn(),
    unstageAll: fn(),
    cancelGithubRun: fn(),
    cloneGithubRepository: fn(),
    closeIssue: fn(),
    commentIssue: fn(),
    createGithubRelease: fn(),
    createGithubRepository: fn(),
    createIssue: fn(),
    createPullRequest: fn(),
    downloadGithubArtifacts: fn(),
    downloadGithubRelease: fn(),
    forkGithubRepository: fn(),
    getGithubOverview: fn(),
    getGithubRunLog: fn(),
    mergePullRequest: fn(),
    rerunGithubRun: fn(),
    reviewPullRequest: fn(),
    runGithubWorkflow: fn(),
    syncGithubRepository: fn(),
  };
});

vi.mock("@tauri-apps/api/event", () => ({ listen }));
vi.mock("@tauri-apps/plugin-dialog", () => ({
  confirm: vi.fn(),
  open: vi.fn(),
}));
vi.mock("./api", () => api);

import App from "./App.vue";

describe("App", () => {
  it("owns the scan listener lifecycle while child components stay presentational", async () => {
    const wrapper = mount(App, {
      global: {
        stubs: {
          AppTitlebar: { template: "<div class='titlebar'>RepoRadar</div>" },
          AppSidebar: true,
          AppHeader: true,
          EnvironmentHero: true,
          EnvironmentStatusGrid: true,
          ScanProgressBanner: true,
          OperationsPanel: true,
          ProjectCatalog: true,
          GitDetailControls: true,
          GithubWorkspacePanel: true,
          ProjectRoadmap: true,
        },
      },
    });
    await flushPromises();

    expect(wrapper.text()).toContain("RepoRadar");
    expect(listen).toHaveBeenCalledWith("scan-progress", expect.any(Function));

    wrapper.unmount();
    expect(unlisten).toHaveBeenCalledOnce();
  });
});
