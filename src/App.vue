<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { computed, onMounted, onUnmounted, ref } from "vue";
import AppHeader from "./components/AppHeader.vue";
import AppSidebar, { type WorkspaceSection } from "./components/AppSidebar.vue";
import AppTitlebar from "./components/AppTitlebar.vue";
import EnvironmentHero from "./components/EnvironmentHero.vue";
import EnvironmentStatusGrid from "./components/EnvironmentStatusGrid.vue";
import GitDetailControls from "./components/GitDetailControls.vue";
import GithubWorkspacePanel from "./components/GithubWorkspacePanel.vue";
import OperationsPanel from "./components/OperationsPanel.vue";
import ProjectCatalog from "./components/ProjectCatalog.vue";
import ProjectRoadmap from "./components/ProjectRoadmap.vue";
import ScanProgressBanner from "./components/ScanProgressBanner.vue";
import { useGitWorkspace } from "./composables/useGitWorkspace";
import { useGithubWorkspace } from "./composables/useGithubWorkspace";
import { usePullRequestDetail } from "./composables/usePullRequestDetail";
import { useOperations } from "./composables/useOperations";
import { useProjectDiscovery } from "./composables/useProjectDiscovery";
import { type Project, type ScanProgress } from "./api";
import type { DetailTab } from "./components/GitDetailControls.vue";

const error = ref("");
const notice = ref("");
const projectBusy = ref(false);
const searchQuery = ref("");
const activeSection = ref<WorkspaceSection>("overview");
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

const showOperations = computed(() => activeSection.value === "operations");

const viewMeta: Record<WorkspaceSection, {
  eyebrow: string;
  title: string;
  subtitle: string;
  searchPlaceholder: string;
}> = {
  overview: {
    eyebrow: "WORKSPACE OVERVIEW",
    title: "掌握每一个代码仓库",
    subtitle: "本地 Git 与 GitHub 工作流，从清晰界面开始。",
    searchPlaceholder: "搜索项目…",
  },
  projects: {
    eyebrow: "PROJECT RADAR",
    title: "项目工作区",
    subtitle: "选择仓库，查看状态、分支与提交。",
    searchPlaceholder: "搜索项目、路径或标签…",
  },
  github: {
    eyebrow: "GITHUB WORKSPACE",
    title: "连接 GitHub 工作流",
    subtitle: "在当前仓库上下文中处理 Pull Request、Issue、Actions 和资源。",
    searchPlaceholder: "搜索项目、路径或标签…",
  },
  operations: {
    eyebrow: "OPERATIONS",
    title: "操作中心",
    subtitle: "集中查看扫描、Git 和 GitHub 动作的运行结果。",
    searchPlaceholder: "搜索项目…",
  },
};
const currentViewMeta = computed(() => viewMeta[activeSection.value]);
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

function navigateWorkspace(section: WorkspaceSection) {
  if (section === "github") return;
  if (section === "projects" && detailTab.value === "github") detailTab.value = "changes";
  activeSection.value = section;
}

async function navigateGithubWorkspace() {
  if (!selectedProject.value) return;
  activeSection.value = "github";
  await selectDetailTab("github");
}

function handleNavigate(section: WorkspaceSection) {
  if (section === "github") {
    void navigateGithubWorkspace();
    return;
  }
  navigateWorkspace(section);
}

function toggleOperations() {
  navigateWorkspace(showOperations.value ? "overview" : "operations");
}

async function chooseProjectFromHeader() {
  await chooseProjectFromDialog();
  activeSection.value = "projects";
}

async function selectWorkspaceDetailTab(tab: DetailTab) {
  if (tab === "github") {
    await navigateGithubWorkspace();
    return;
  }
  activeSection.value = "projects";
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
  activeSection.value = "projects";
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
  } catch (cause) {
    error.value = String(cause);
    finishOperation(operationId, "failed", cause);
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
  } catch (cause) {
    if (Reflect.has(window, "__TAURI_INTERNALS__")) error.value = String(cause);
  }
  try {
    await Promise.all([refreshEnvironment(), refreshProjects()]);
  } catch (cause) {
    error.value = String(cause);
  }
});

onUnmounted(() => stopScanProgress?.());
</script>

<template>
  <div class="app-window">
    <AppTitlebar />

    <div class="app-shell">
      <AppSidebar
        :active-section="activeSection"
        :project-count="projects.length"
        :github-disabled="!selectedProject || projectBusy"
        @navigate="handleNavigate"
      />

      <main>
        <AppHeader
          v-model:search-query="searchQuery"
          :eyebrow="currentViewMeta.eyebrow"
          :title="currentViewMeta.title"
          :subtitle="currentViewMeta.subtitle"
          :search-placeholder="currentViewMeta.searchPlaceholder"
          :show-operations="showOperations"
          :active-operation-count="activeOperationCount"
          :scanning="scanning"
          :busy="projectBusy"
          @toggle-operations="toggleOperations"
          @stop-scan="stopScan"
          @choose-scan-root="chooseScanRoot"
          @choose-project="chooseProjectFromHeader"
        />

        <div class="global-feedback" aria-live="polite">
          <ScanProgressBanner v-if="scanning && scanProgress" :progress="scanProgress" />
          <p v-else-if="scanning" class="scan-progress-pending">扫描准备中…</p>
          <p v-if="error" class="error-banner" role="alert">{{ error }}</p>
          <p v-if="notice" class="notice-banner" role="status">{{ notice }}</p>
        </div>

        <div class="view-viewport" :key="activeSection">
          <section v-if="activeSection === 'overview'" class="view-panel overview-view">
            <EnvironmentHero
              :loading="loading"
              :readiness="readiness"
              @refresh="refreshEnvironment"
            />
            <EnvironmentStatusGrid :environment="environment" :loading="loading" />
            <ProjectRoadmap />
          </section>

          <section v-else-if="activeSection === 'projects'" class="view-panel projects-view">
            <div class="projects-layout">
              <article class="project-panel project-catalog-pane">
                <ProjectCatalog
                  :projects="projects"
                  :visible-projects="filteredProjects"
                  :loading="projectsLoading"
                  :error="projectsError"
                  :selected-id="selectedProject?.id"
                  :busy="projectBusy"
                  :save-tags="updateTags"
                  @rescan="rescan"
                  @retry="refreshProjects"
                  @select="selectProject"
                  @favorite="toggleFavorite"
                />
              </article>

              <article class="project-panel project-detail-pane">
                <div v-if="!selectedProject" class="detail-empty">
                  <span class="detail-empty-mark">R</span>
                  <div>
                    <span class="section-label">WORKSPACE</span>
                    <h2>选择项目</h2>
                    <p>从左侧项目列表选择仓库，这里会显示 Git 状态、Diff、分支和提交历史。</p>
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
                <div v-else class="git-detail">
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
              </article>
            </div>
          </section>

          <section v-else-if="activeSection === 'github'" class="view-panel github-view">
            <div v-if="!selectedProject" class="detail-empty">
              <span class="detail-empty-mark">GH</span>
              <div>
                <span class="section-label">GITHUB WORKSPACE</span>
                <h2>先选择一个项目</h2>
                <p>GitHub 工作区需要一个已加载的本地仓库作为上下文。</p>
                <button class="secondary-action" @click="navigateWorkspace('projects')">打开项目</button>
              </div>
            </div>
            <template v-else>
              <div class="workspace-context">
                <div>
                  <span class="section-label">当前仓库</span>
                  <strong>{{ selectedProject.name }}</strong>
                  <small>{{ selectedProject.path }}</small>
                </div>
                <button class="small-action" @click="navigateWorkspace('projects')">项目详情</button>
              </div>
              <div v-if="projectDetailLoading" class="detail-loading" aria-busy="true">
                <span class="skeleton-line medium"></span>
                <span v-for="index in 6" :key="index" class="skeleton-row"></span>
              </div>
              <div v-else-if="!gitStatus" class="detail-empty">
                <span class="detail-empty-mark">!</span>
                <div>
                  <span class="section-label">GITHUB ERROR</span>
                  <h2>项目上下文未就绪</h2>
                  <p>本地 Git 状态读取失败，无法打开 GitHub 工作区。</p>
                  <button class="secondary-action" @click="selectProject(selectedProject)">重新加载</button>
                </div>
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
                @view-pull-request="viewPullRequest"
                @close-pull-request="clearPullRequest"
                @comment-issue="commentGithubIssue"
                @close-issue="closeGithubIssue"
                @refresh="refreshGithub"
                @dispatch-workflow="dispatchGithubWorkflow"
                @view-run="viewGithubRun"
                @rerun-run="rerunGithubWorkflow"
                @cancel-run="cancelGithubWorkflow"
                @download-artifacts="downloadGithubRunArtifacts"
                @download-release="downloadGithubReleaseItem"
                @notice="notice = $event"
              />
            </template>
          </section>

          <section v-else class="view-panel operations-view">
            <OperationsPanel
              :operations="operations"
              :scanning="scanning"
              @stop-scan="stopScan"
              @clear="clearOperations"
            />
          </section>
        </div>
      </main>
    </div>
  </div>
</template>
