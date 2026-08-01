<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { computed, onMounted, onUnmounted, ref } from "vue";
import AppHeader from "./components/AppHeader.vue";
import AppSidebar from "./components/AppSidebar.vue";
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

const error = ref("");
const notice = ref("");
const projectBusy = ref(false);
const searchQuery = ref("");
const activeSection = ref<"overview" | "projects">("overview");
const mainContent = ref<HTMLElement | null>(null);
const showOperations = ref(false);
const { operations, activeOperationCount, beginOperation, finishOperation, clearOperations } =
  useOperations();
const {
  environment, projects, selectedProject, loading, scanning, scanProgress,
  refreshEnvironment, refreshProjects, chooseProject, chooseScanRoot,
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

function navigateWorkspace(section: "overview" | "projects") {
  activeSection.value = section;
  if (section === "overview") {
    mainContent.value?.scrollTo({ top: 0, behavior: "smooth" });
  } else {
    document.getElementById("project-workspace")
      ?.scrollIntoView({ behavior: "smooth", block: "start" });
  }
}

async function navigateGithubWorkspace() {
  if (!selectedProject.value) return;
  navigateWorkspace("projects");
  await selectDetailTab("github");
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

async function selectProject(project: Project) {
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
  stopScanProgress = await listen<ScanProgress>("scan-progress", (event) => {
    scanProgress.value = event.payload;
  });
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
        @navigate="navigateWorkspace"
        @github="navigateGithubWorkspace"
        @operations="showOperations = true; navigateWorkspace('overview')"
      />

      <main ref="mainContent">
      <AppHeader
        v-model:search-query="searchQuery"
        :show-operations="showOperations"
        :active-operation-count="activeOperationCount"
        :scanning="scanning"
        :busy="projectBusy"
        @toggle-operations="showOperations = !showOperations"
        @stop-scan="stopScan"
        @choose-scan-root="chooseScanRoot"
        @choose-project="chooseProject"
      />

      <EnvironmentHero
        :loading="loading"
        :readiness="readiness"
        @refresh="refreshEnvironment"
      />

      <ScanProgressBanner v-if="scanning && scanProgress" :progress="scanProgress" />

      <OperationsPanel
        v-if="showOperations"
        :operations="operations"
        :scanning="scanning"
        @stop-scan="stopScan"
        @clear="clearOperations"
      />

      <p v-if="error" class="error-banner" role="alert">{{ error }}</p>
      <p v-if="notice" class="notice-banner" role="status">{{ notice }}</p>

      <EnvironmentStatusGrid :environment="environment" />

      <section id="project-workspace" class="workspace-grid">
        <article class="project-panel">
          <ProjectCatalog
            :projects="projects"
            :visible-projects="filteredProjects"
            :selected-id="selectedProject?.id"
            :busy="projectBusy"
            :save-tags="updateTags"
            @rescan="rescan"
            @select="selectProject"
            @favorite="toggleFavorite"
          />

          <div v-if="selectedProject && gitStatus" class="git-detail">
            <GitDetailControls
              v-model:commit-message="commitMessage"
              v-model:new-branch="newBranch"
              :project="selectedProject"
              :status="gitStatus"
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
              @select-tab="selectDetailTab"
              @create-branch="createNewBranch"
              @checkout-branch="checkoutBranch"
              @remove-branch="removeBranch"
            />

            <GithubWorkspacePanel
              v-if="detailTab === 'github'"
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
              @dispatch-workflow="dispatchGithubWorkflow"
              @view-run="viewGithubRun"
              @rerun-run="rerunGithubWorkflow"
              @cancel-run="cancelGithubWorkflow"
              @download-artifacts="downloadGithubRunArtifacts"
              @download-release="downloadGithubReleaseItem"
              @notice="notice = $event"
            />
          </div>
        </article>

        <ProjectRoadmap />
      </section>
      </main>
    </div>
  </div>
</template>
