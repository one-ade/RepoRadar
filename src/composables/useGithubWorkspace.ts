import { confirm } from "@tauri-apps/plugin-dialog";
import type { Ref } from "vue";
import { ref } from "vue";

import {
  cancelGithubRun, cloneGithubRepository, closeIssue, commentIssue,
  createGithubRelease, createGithubRepository, createIssue, createPullRequest,
  downloadGithubArtifacts, downloadGithubRelease, forkGithubRepository,
  getGithubOverview, getGithubRunLog, mergePullRequest, rerunGithubRun,
  reviewPullRequest, runGithubWorkflow, syncGithubRepository, type Project,
} from "../api";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;
type ChooseDirectory = (title: string) => Promise<string | null>;

export function useGithubWorkspace(
  project: Ref<Project | null>,
  runAction: RunAction,
  chooseDirectory: ChooseDirectory,
  notify: (message: string) => void,
) {
  const githubOverview = ref<Awaited<ReturnType<typeof getGithubOverview>> | null>(null);
  const githubTitle = ref("");
  const githubBody = ref("");
  const githubComment = ref("");
  const githubRunLog = ref("");
  const releaseTag = ref("");
  const releaseTitle = ref("");
  const releaseNotes = ref("");
  const cloneReference = ref("");
  const repositoryName = ref("");
  const repositoryVisibility = ref<"public" | "private" | "internal">("private");
  const repositoryDescription = ref("");
  let githubRunRequestId = 0;

  function resetGithub() {
    githubOverview.value = null;
    githubRunRequestId += 1;
    githubRunLog.value = "";
  }

  async function refreshGithub() {
    if (project.value) {
      githubOverview.value = await getGithubOverview(project.value.path);
    }
  }

  async function viewGithubRun(databaseId: number) {
    if (!project.value) return;
    const currentRequest = ++githubRunRequestId;
    githubRunLog.value = "";
    await runAction(async () => {
      const log = await getGithubRunLog(project.value!.path, databaseId);
      if (currentRequest === githubRunRequestId) githubRunLog.value = log;
    });
  }

  async function rerunGithubWorkflow(databaseId: number) {
    if (!project.value) return;
    await runAction(async () => {
      await rerunGithubRun(project.value!.path, databaseId);
      await refreshGithub();
      notify(`Actions Run #${databaseId} 已请求重跑`);
    });
  }

  async function dispatchGithubWorkflow(workflowId: number) {
    if (!project.value) return;
    await runAction(async () => {
      await runGithubWorkflow(project.value!.path, workflowId);
      notify(`Workflow #${workflowId} 已触发`);
    }, `触发 Workflow #${workflowId}`);
  }

  async function cancelGithubWorkflow(databaseId: number) {
    if (!project.value) return;
    const confirmed = await confirm(`确定取消 Actions Run #${databaseId}？`, {
      title: "取消 Actions Run",
      kind: "warning",
    });
    if (!confirmed) return;
    await runAction(async () => {
      await cancelGithubRun(project.value!.path, databaseId);
      await refreshGithub();
      notify(`Actions Run #${databaseId} 已取消`);
    });
  }

  async function downloadGithubRunArtifacts(databaseId: number) {
    if (!project.value) return;
    const targetDir = await chooseDirectory("选择 Artifact 下载目录");
    if (!targetDir) return;
    await runAction(async () => {
      await downloadGithubArtifacts(project.value!.path, databaseId, targetDir);
      notify(`Actions Run #${databaseId} 的 Artifact 已下载`);
    });
  }

  async function createGithubReleaseItem() {
    if (!project.value) return;
    await runAction(async () => {
      const url = await createGithubRelease(
        project.value!.path,
        releaseTag.value,
        releaseTitle.value,
        releaseNotes.value,
      );
      releaseTag.value = "";
      releaseTitle.value = "";
      releaseNotes.value = "";
      await refreshGithub();
      notify(`Release 已创建：${url}`);
    });
  }

  async function downloadGithubReleaseItem(tag: string) {
    if (!project.value) return;
    const targetDir = await chooseDirectory("选择 Release 下载目录");
    if (!targetDir) return;
    await runAction(async () => {
      await downloadGithubRelease(project.value!.path, tag, targetDir);
      notify(`Release ${tag} 已下载`);
    });
  }

  async function forkGithubRepositoryItem() {
    if (!project.value) return;
    await runAction(async () => {
      const result = await forkGithubRepository(project.value!.path);
      notify(`Fork 已完成：${result}`);
    });
  }

  async function syncGithubRepositoryItem() {
    if (!project.value) return;
    await runAction(async () => {
      await syncGithubRepository(project.value!.path);
      await refreshGithub();
      notify("GitHub 仓库已同步");
    });
  }

  async function cloneGithubRepositoryItem() {
    const reference = cloneReference.value.trim();
    if (!reference) return;
    const targetDir = await chooseDirectory("选择克隆目标目录");
    if (!targetDir) return;
    await runAction(async () => {
      const result = await cloneGithubRepository(reference, targetDir);
      cloneReference.value = "";
      notify(`仓库已克隆：${result}`);
    }, "克隆 GitHub 仓库");
  }

  async function createGithubRepositoryItem() {
    if (!project.value) return;
    await runAction(async () => {
      const result = await createGithubRepository(
        project.value!.path,
        repositoryName.value,
        repositoryVisibility.value,
        repositoryDescription.value,
      );
      repositoryName.value = "";
      repositoryDescription.value = "";
      await refreshGithub();
      notify(`GitHub 仓库已创建：${result}`);
    }, "创建 GitHub 仓库");
  }

  async function createGithubItem(kind: "pr" | "issue") {
    if (!project.value) return;
    await runAction(async () => {
      const url =
        kind === "pr"
          ? await createPullRequest(project.value!.path, githubTitle.value, githubBody.value)
          : await createIssue(project.value!.path, githubTitle.value, githubBody.value);
      githubTitle.value = "";
      githubBody.value = "";
      await refreshGithub();
      notify(`${kind === "pr" ? "Pull Request" : "Issue"} 已创建：${url}`);
    });
  }

  async function reviewGithubPullRequest(
    number: number,
    action: "approve" | "comment" | "request-changes",
  ) {
    if (!project.value) return;
    await runAction(async () => {
      await reviewPullRequest(project.value!.path, number, action, githubComment.value);
      githubComment.value = "";
      await refreshGithub();
      notify(`Pull Request #${number} Review 已提交`);
    });
  }

  async function mergeGithubPullRequest(number: number) {
    if (!project.value) return;
    const confirmed = await confirm(`确定以 squash 方式合并 Pull Request #${number}？`, {
      title: "合并 Pull Request",
      kind: "warning",
    });
    if (!confirmed) return;
    await runAction(async () => {
      await mergePullRequest(project.value!.path, number);
      await refreshGithub();
      notify(`Pull Request #${number} 已合并`);
    });
  }

  async function commentGithubIssue(number: number) {
    if (!project.value) return;
    await runAction(async () => {
      await commentIssue(project.value!.path, number, githubComment.value);
      githubComment.value = "";
      notify(`Issue #${number} 评论已提交`);
    });
  }

  async function closeGithubIssue(number: number) {
    if (!project.value) return;
    const confirmed = await confirm(`确定关闭 Issue #${number}？`, {
      title: "关闭 Issue",
      kind: "warning",
    });
    if (!confirmed) return;
    await runAction(async () => {
      await closeIssue(project.value!.path, number);
      await refreshGithub();
      notify(`Issue #${number} 已关闭`);
    });
  }

  return {
    githubOverview,
    githubTitle,
    githubBody,
    githubComment,
    githubRunLog,
    releaseTag,
    releaseTitle,
    releaseNotes,
    cloneReference,
    repositoryName,
    repositoryVisibility,
    repositoryDescription,
    resetGithub,
    refreshGithub,
    viewGithubRun,
    rerunGithubWorkflow,
    dispatchGithubWorkflow,
    cancelGithubWorkflow,
    downloadGithubRunArtifacts,
    createGithubReleaseItem,
    downloadGithubReleaseItem,
    forkGithubRepositoryItem,
    syncGithubRepositoryItem,
    cloneGithubRepositoryItem,
    createGithubRepositoryItem,
    createGithubItem,
    reviewGithubPullRequest,
    mergeGithubPullRequest,
    commentGithubIssue,
    closeGithubIssue,
  };
}
