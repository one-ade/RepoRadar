import { confirm } from "@tauri-apps/plugin-dialog";
import type { Ref } from "vue";
import { ref } from "vue";

import {
  commit,
  createBranch,
  deleteBranch,
  fetch,
  getGitDiff,
  getGitLog,
  getGitStatus,
  listBranches,
  pull,
  push,
  stageAll,
  switchBranch,
  unstageAll,
  type Project,
} from "../api";
import type { DetailTab } from "../components/GitDetailControls.vue";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;

export function useGitWorkspace(
  project: Ref<Project | null>,
  runAction: RunAction,
  loadGithub: () => Promise<void>,
  notify: (message: string) => void,
) {
  const gitStatus = ref<Awaited<ReturnType<typeof getGitStatus>> | null>(null);
  const gitDiff = ref("");
  const branches = ref<Awaited<ReturnType<typeof listBranches>>>([]);
  const history = ref<Awaited<ReturnType<typeof getGitLog>>>([]);
  const commitMessage = ref("");
  const newBranch = ref("");
  const detailTab = ref<DetailTab>("changes");

  function resetGit() {
    gitStatus.value = null;
    gitDiff.value = "";
    branches.value = [];
    history.value = [];
    detailTab.value = "changes";
  }

  async function loadProject(selected: Project) {
    await runAction(async () => {
      gitStatus.value = await getGitStatus(selected.path);
      gitDiff.value = await getGitDiff(selected.path);
    });
  }

  async function selectDetailTab(tab: DetailTab) {
    if (!project.value) return;
    detailTab.value = tab;
    await runAction(async () => {
      if (tab === "changes") {
        gitDiff.value = await getGitDiff(project.value!.path);
      } else if (tab === "branches") {
        branches.value = await listBranches(project.value!.path);
      } else if (tab === "history") {
        history.value = await getGitLog(project.value!.path);
      } else {
        await loadGithub();
      }
    });
  }

  async function runGitAction(action: () => Promise<void>, message: string) {
    await runAction(async () => {
      await action();
      if (project.value) {
        gitStatus.value = await getGitStatus(project.value.path);
        if (detailTab.value === "changes") {
          gitDiff.value = await getGitDiff(project.value.path);
        }
      }
      notify(message);
    }, message);
  }

  async function stageProject() {
    if (project.value) {
      await runGitAction(() => stageAll(project.value!.path), "已暂存全部修改");
    }
  }

  async function unstageProject() {
    if (project.value) {
      await runGitAction(() => unstageAll(project.value!.path), "已取消暂存");
    }
  }

  async function fetchProject() {
    if (project.value) {
      await runGitAction(() => fetch(project.value!.path), "Fetch 成功");
    }
  }

  async function commitProject() {
    if (!project.value) return;
    await runGitAction(
      () => commit(project.value!.path, commitMessage.value),
      "提交成功",
    );
    commitMessage.value = "";
  }

  async function pullProject() {
    if (project.value) {
      await runGitAction(() => pull(project.value!.path), "Pull 成功");
    }
  }

  async function pushProject() {
    if (project.value) {
      await runGitAction(() => push(project.value!.path), "Push 成功");
    }
  }

  async function createNewBranch() {
    if (!project.value) return;
    await runGitAction(
      () => createBranch(project.value!.path, newBranch.value),
      "分支已创建并切换",
    );
    newBranch.value = "";
    await selectDetailTab("branches");
  }

  async function checkoutBranch(branch: string) {
    if (!project.value) return;
    await runGitAction(
      () => switchBranch(project.value!.path, branch),
      `已切换到 ${branch}`,
    );
    await selectDetailTab("branches");
  }

  async function removeBranch(branch: string) {
    if (!project.value) return;
    const confirmed = await confirm(`确认删除本地分支 ${branch}？`, {
      title: "删除分支",
      kind: "warning",
    });
    if (!confirmed) return;
    await runGitAction(
      () => deleteBranch(project.value!.path, branch),
      `已删除 ${branch}`,
    );
    await selectDetailTab("branches");
  }

  return {
    gitStatus,
    gitDiff,
    branches,
    history,
    commitMessage,
    newBranch,
    detailTab,
    resetGit,
    loadProject,
    selectDetailTab,
    stageProject,
    unstageProject,
    fetchProject,
    commitProject,
    pullProject,
    pushProject,
    createNewBranch,
    checkoutBranch,
    removeBranch,
  };
}
