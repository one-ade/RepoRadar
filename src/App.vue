<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { computed, onMounted, onUnmounted, ref } from "vue";
import AppHeader from "./components/AppHeader.vue";
import ActivityTray from "./components/ActivityTray.vue";
import AppSidebar from "./components/AppSidebar.vue";
import AppTitlebar from "./components/AppTitlebar.vue";
import DetailPanel from "./components/DetailPanel.vue";
import EnvironmentHero from "./components/EnvironmentHero.vue";
import EnvironmentStatusGrid from "./components/EnvironmentStatusGrid.vue";
import GitDetailControls from "./components/GitDetailControls.vue";
import GithubWorkspacePanel from "./components/GithubWorkspacePanel.vue";
import ProjectCatalog from "./components/ProjectCatalog.vue";
import { useGitWorkspace } from "./composables/useGitWorkspace";
import { useGithubWorkspace } from "./composables/useGithubWorkspace";
import { usePullRequestDetail } from "./composables/usePullRequestDetail";
import { useOperations } from "./composables/useOperations";
import { useProjectDiscovery } from "./composables/useProjectDiscovery";
import { userFacingErrors } from "./userFacingErrors";
import { type GithubIssue, type GithubRelease, type GithubPullRequest, type Project, type ScanProgress } from "./api";
import type { DetailTab } from "./components/GitDetailControls.vue";
import type { DetailTarget, GithubSection, GlobalSection, RepositoryView } from "./workspace";

const error = ref("");
const notice = ref("");
const projectBusy = ref(false);
const searchQuery = ref("");
const globalSection = ref<GlobalSection>("repositories");
const repositoryView = ref<RepositoryView>("changes");
const githubSection = ref<GithubSection>("pull-requests");
const activityOpen = ref(false);
const detailTarget = ref<DetailTarget>(null);
const { operations, activeOperationCount, beginOperation, finishOperation, clearOperations } =
  useOperations();
const {
  environment, projects, selectedProject, loading, projectsLoading, projectsError, scanning, scanProgress,
  refreshEnvironment, refreshProjects, chooseProject: chooseProjectFromDialog, chooseScanRoot,
  rescan, stopScan, toggleFavorite, updateTags,
} = useProjectDiscovery(runProjectAction, chooseDirectory,
  (message) => { notice.value = message; },
  (message) => { error.value = message; },
);
const {
  githubOverview, githubTitle, githubBody, githubComment, githubRunLog,
  releaseTag, releaseTitle, releaseNotes, cloneReference, repositoryName,
  repositoryVisibility, repositoryDescription, resetGithub, refreshGithub,
  viewGithubRun, rerunGithubWorkflow, dispatchGithubWorkflow, cancelGithubWorkflow,
  downloadGithubRunArtifacts, createGithubReleaseItem, downloadGithubReleaseItem,
  forkGithubRepositoryItem, syncGithubRepositoryItem, cloneGithubRepositoryItem,
  createGithubRepositoryItem, createGithubItem, reviewGithubPullRequest,
  mergeGithubPullRequest, commentGithubIssue, closeGithubIssue,
} = useGithubWorkspace(
  selectedProject, runProjectAction, chooseDirectory,
  (message) => { notice.value = message; },
);

const { selectedPullRequest, pullRequestDetail, viewPullRequest, clearPullRequest } =
  usePullRequestDetail(selectedProject, githubOverview, runProjectAction);
const {
  gitStatus, gitDiff, branches, history, commitMessage, newBranch, detailTab,
  resetGit, loadProject, selectDetailTab, stageProject, unstageProject,
  fetchProject, commitProject, pullProject, pushProject, createNewBranch,
  checkoutBranch, removeBranch,
} = useGitWorkspace(
  selectedProject, runProjectAction, refreshGithub,
  (message) => { notice.value = message; },
);

async function openPullRequestDetail(pullRequest: GithubPullRequest) {
  detailTarget.value = { kind: "pull-request", item: pullRequest };
  await viewPullRequest(pullRequest);
}

function openIssueDetail(issue: GithubIssue) {
  detailTarget.value = { kind: "issue", item: issue };
}

function openReleaseDetail(release: GithubRelease) {
  detailTarget.value = { kind: "release", item: release };
}

async function openRunDetail(databaseId: number) {
  const run = githubOverview.value?.runs.find((item) => item.databaseId === databaseId);
  if (!run) return;
  detailTarget.value = { kind: "run", item: run };
  await viewGithubRun(databaseId);
}

function closeDetailPanel() {
  detailTarget.value = null;
  clearPullRequest();
}

function navigateRepositoryView(view: RepositoryView) {
  globalSection.value = "repositories";
  activityOpen.value = false;
  repositoryView.value = view;
  if (view !== "github") {
    void selectDetailTab(view);
  } else {
    void navigateGithubWorkspace();
  }
}

async function navigateGithubWorkspace() {
  if (!selectedProject.value) return;
  globalSection.value = "repositories";
  activityOpen.value = false;
  repositoryView.value = "github";
  await selectDetailTab("github");
}

function handleNavigate(section: GlobalSection) {
  if (section === "activity") {
    activityOpen.value = true;
    globalSection.value = "activity";
    return;
  }
  activityOpen.value = false;
  globalSection.value = section;
}

function toggleOperations() {
  activityOpen.value = !activityOpen.value;
  globalSection.value = activityOpen.value ? "activity" : "repositories";
}

function selectGithubSection(section: GithubSection) {
  githubSection.value = section;
  detailTarget.value = null;
}

async function chooseProjectFromHeader() {
  await chooseProjectFromDialog();
  globalSection.value = "repositories";
  repositoryView.value = "changes";
  githubSection.value = "pull-requests";
}

async function selectWorkspaceDetailTab(tab: DetailTab) {
  if (tab === "github") {
    await navigateGithubWorkspace();
    return;
  }
  globalSection.value = "repositories";
  activityOpen.value = false;
  repositoryView.value = tab;
  await selectDetailTab(tab);
}

const filteredProjects = computed(() => {
  const query = searchQuery.value.trim().toLocaleLowerCase();
  if (!query) return projects.value;
  return projects.value.filter(
    (project) =>
      project.name.toLocaleLowerCase().includes(query) ||
      project.path.toLocaleLowerCase().includes(query) ||
      project.tags.some((tag) => tag.toLocaleLowerCase().includes(query)),
  );
});

const readiness = computed(() => {
  if (!environment.value) return 0;
  return [environment.value.git.installed, environment.value.gh.installed,
    environment.value.databaseReady].filter(Boolean).length;
});

const projectDetailLoading = computed(() => Boolean(
  selectedProject.value && !gitStatus.value && projectBusy.value,
));
const readyGitStatus = computed(() => gitStatus.value!);

async function selectProject(project: Project) {
  globalSection.value = "repositories";
  repositoryView.value = "changes";
  githubSection.value = "pull-requests";
  activityOpen.value = false;
  detailTarget.value = null;
  selectedProject.value = project;
  resetGit();
  resetGithub();
  await loadProject(project);
}

async function chooseDirectory(title: string) {
  const selection = await open({ directory: true, multiple: false, title });
  return typeof selection === "string" ? selection : null;
}

async function runProjectAction(action: () => Promise<void>, label = "项目操作") {
  const operationId = beginOperation(label);
  projectBusy.value = true;
  error.value = "";
  notice.value = "";
  try {
    await action();
    finishOperation(operationId, "success");
  } catch {
    error.value = userFacingErrors.action;
    finishOperation(operationId, "failed");
  } finally {
    projectBusy.value = false;
  }
}

let stopScanProgress: UnlistenFn | undefined;

onMounted(async () => {
  try {
    stopScanProgress = await listen<ScanProgress>("scan-progress", (event) => {
      scanProgress.value = event.payload;
    });
  } catch {
    if (Reflect.has(window, "__TAURI_INTERNALS__")) error.value = userFacingErrors.action;
  }
  try {
    await Promise.all([refreshEnvironment(), refreshProjects()]);
  } catch {
    error.value = userFacingErrors.action;
  }
});

onUnmounted(() => stopScanProgress?.());
</script>

<template>
  <div class="app-window">
    <AppTitlebar />

    <div class="app-shell">
      <AppSidebar
        :active-section="globalSection"
        :active-operation-count="activeOperationCount"
        @navigate="handleNavigate"
      />

      <ProjectCatalog
        v-model:search-query="searchQuery"
        :projects="projects"
        :visible-projects="filteredProjects"
        :loading="projectsLoading"
        :error="projectsError"
        :selected-id="selectedProject?.id"
        :busy="projectBusy"
        :save-tags="updateTags"
        :scanning="scanning"
        @add="chooseProjectFromHeader"
        @scan="chooseScanRoot"
        @stop-scan="stopScan"
        @rescan="rescan"
        @retry="refreshProjects"
        @select="selectProject"
        @favorite="toggleFavorite"
      />

      <main :class="['workspace-main', detailTarget && 'has-detail-panel']">
        <AppHeader
          :project="selectedProject"
          :status="gitStatus"
          :global-section="globalSection"
          :repository-view="repositoryView"
          :activity-open="activityOpen"
          :active-operation-count="activeOperationCount"
          :scanning="scanning"
          :busy="projectBusy"
          @toggle-operations="toggleOperations"
          @stop-scan="stopScan"
          @choose-scan-root="chooseScanRoot"
          @choose-project="chooseProjectFromHeader"
        />

        <div class="global-feedback" aria-live="polite">
          <p v-if="error" class="error-banner" role="alert">{{ error }}</p>
          <p v-if="notice" class="notice-banner" role="status">{{ notice }}</p>
        </div>

        <div class="view-viewport" :key="`${globalSection}-${repositoryView}`">
          <section v-if="globalSection === 'diagnostics'" class="view-panel overview-view diagnostics-view">
            <EnvironmentHero :loading="loading" :readiness="readiness" @refresh="refreshEnvironment" />
            <EnvironmentStatusGrid :environment="environment" :loading="loading" />
          </section>

          <section v-else class="view-panel repository-view">
            <div v-if="!selectedProject" class="detail-empty repository-empty">
              <span class="detail-empty-mark">R</span>
              <div>
                <span class="section-label">REPOSITORY WORKSPACE</span>
                <h2>选择一个仓库</h2>
                <p>从左侧项目栏选择仓库，这里会显示 Git 状态、Diff、分支、历史和 GitHub 工作流。</p>
                <button class="secondary-action" @click="chooseProjectFromHeader">添加项目</button>
              </div>
            </div>
            <div v-else-if="projectDetailLoading" class="detail-loading" aria-busy="true">
              <span class="skeleton-line medium"></span>
              <span class="skeleton-line short"></span>
              <span v-for="index in 5" :key="index" class="skeleton-row"></span>
            </div>
            <div v-else-if="!gitStatus" class="detail-empty">
              <span class="detail-empty-mark">!</span>
              <div>
                <span class="section-label">WORKSPACE ERROR</span>
                <h2>暂时无法加载项目</h2>
                <p>Git 状态读取失败，请检查仓库路径后重试。</p>
                <button class="secondary-action" @click="selectProject(selectedProject)">重新加载</button>
              </div>
            </div>
            <div v-else-if="repositoryView !== 'github'" class="git-detail">
              <GitDetailControls
                v-model:commit-message="commitMessage"
                v-model:new-branch="newBranch"
                :project="selectedProject"
                :status="readyGitStatus"
                :diff="gitDiff"
                :branches="branches"
                :history="history"
                :tab="detailTab"
                :busy="projectBusy"
                @stage="stageProject"
                @unstage="unstageProject"
                @fetch="fetchProject"
                @pull="pullProject"
                @push="pushProject"
                @commit="commitProject"
                @select-tab="selectWorkspaceDetailTab"
                @create-branch="createNewBranch"
                @checkout-branch="checkoutBranch"
                @remove-branch="removeBranch"
              />
            </div>
            <div v-else-if="projectBusy && !githubOverview" class="github-loading" aria-busy="true">
              <span class="github-loading-orb"></span>
              <strong>正在加载 GitHub 工作区…</strong>
              <small>正在读取仓库概览和远程资源</small>
            </div>
            <GithubWorkspacePanel
              v-else
              v-model:github-title="githubTitle"
              v-model:github-body="githubBody"
              v-model:github-comment="githubComment"
              v-model:release-tag="releaseTag"
              v-model:release-title="releaseTitle"
              v-model:release-notes="releaseNotes"
              v-model:clone-reference="cloneReference"
              v-model:repository-name="repositoryName"
              v-model:repository-visibility="repositoryVisibility"
              v-model:repository-description="repositoryDescription"
              :path="selectedProject.path"
              :busy="projectBusy"
              :overview="githubOverview"
              :section="githubSection"
              :selected-pull-request="selectedPullRequest"
              :pull-request-detail="pullRequestDetail"
              :run-log="githubRunLog"
              :run-action="runProjectAction"
              @clone="cloneGithubRepositoryItem"
              @create-repository="createGithubRepositoryItem"
              @fork="forkGithubRepositoryItem"
              @sync="syncGithubRepositoryItem"
              @create-item="createGithubItem"
              @create-release="createGithubReleaseItem"
              @review="reviewGithubPullRequest"
              @merge="mergeGithubPullRequest"
              @view-pull-request="openPullRequestDetail"
              @view-issue="openIssueDetail"
              @view-release="openReleaseDetail"
              @close-pull-request="closeDetailPanel"
              @comment-issue="commentGithubIssue"
              @close-issue="closeGithubIssue"
              @refresh="refreshGithub"
              @dispatch-workflow="dispatchGithubWorkflow"
              @view-run="openRunDetail"
              @rerun-run="rerunGithubWorkflow"
              @cancel-run="cancelGithubWorkflow"
              @download-artifacts="downloadGithubRunArtifacts"
              @download-release="downloadGithubReleaseItem"
              @select-section="selectGithubSection"
              @notice="notice = $event"
            />
          </section>
        </div>

        <DetailPanel
          v-if="detailTarget && selectedProject"
          :target="detailTarget"
          :path="selectedProject.path"
          :busy="projectBusy"
          :pull-request-detail="pullRequestDetail"
          :run-log="githubRunLog"
          :run-action="runProjectAction"
          @close="closeDetailPanel"
          @refresh="refreshGithub"
          @notice="notice = $event"
          @rerun-run="rerunGithubWorkflow"
          @cancel-run="cancelGithubWorkflow"
          @download-artifacts="downloadGithubRunArtifacts"
        />

        <ActivityTray
          v-if="activityOpen"
          :operations="operations"
          :scanning="scanning"
          :progress="scanProgress"
          @close="activityOpen = false; globalSection = 'repositories'"
          @stop-scan="stopScan"
          @clear="clearOperations"
        />
      </main>
    </div>
  </div>
</template>
