<script setup lang="ts">
import type { GithubOverview, GithubPullRequest, GithubPullRequestDetail } from "../api";
import GithubConfigurationPanel from "./GithubConfigurationPanel.vue";
import GithubEnvironmentsPanel from "./GithubEnvironmentsPanel.vue";
import GithubIssueWorkspace from "./GithubIssueWorkspace.vue";
import GithubPullRequestDetailPanel from "./GithubPullRequestDetailPanel.vue";
import GithubPullRequestSection from "./GithubPullRequestSection.vue";
import GithubReleaseWorkspace from "./GithubReleaseWorkspace.vue";
import GithubResourcesPanel from "./GithubResourcesPanel.vue";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;

defineProps<{
  path: string;
  busy: boolean;
  overview: GithubOverview | null;
  selectedPullRequest: GithubPullRequest | null;
  pullRequestDetail: GithubPullRequestDetail | null;
  runLog: string;
  runAction: RunAction;
}>();

const githubTitle = defineModel<string>("githubTitle", { required: true });
const githubBody = defineModel<string>("githubBody", { required: true });
const githubComment = defineModel<string>("githubComment", { required: true });
const releaseTag = defineModel<string>("releaseTag", { required: true });
const releaseTitle = defineModel<string>("releaseTitle", { required: true });
const releaseNotes = defineModel<string>("releaseNotes", { required: true });
const cloneReference = defineModel<string>("cloneReference", { required: true });
const repositoryName = defineModel<string>("repositoryName", { required: true });
const repositoryVisibility = defineModel<"public" | "private" | "internal">(
  "repositoryVisibility",
  { required: true },
);
const repositoryDescription = defineModel<string>("repositoryDescription", {
  required: true,
});

const emit = defineEmits<{
  clone: [];
  "create-repository": [];
  fork: [];
  sync: [];
  "create-item": [kind: "pr" | "issue"];
  "create-release": [];
  review: [number: number, action: "approve" | "comment" | "request-changes"];
  merge: [number: number];
  "view-pull-request": [pullRequest: GithubPullRequest];
  "close-pull-request": [];
  "comment-issue": [number: number];
  "close-issue": [number: number];
  refresh: [];
  "dispatch-workflow": [workflowId: number];
  "view-run": [databaseId: number];
  "rerun-run": [databaseId: number];
  "cancel-run": [databaseId: number];
  "download-artifacts": [databaseId: number];
  "download-release": [tag: string];
  notice: [message: string];
}>();
</script>

<template>
  <div v-if="!overview" class="github-panel">
    <span class="section-label">GITHUB WORKSPACE</span>
    <h3>为当前本地项目连接 GitHub</h3>
    <div class="github-repository-tools standalone">
      <input v-model="cloneReference" placeholder="owner/repo（用于克隆）" aria-label="克隆仓库" />
      <button class="small-action" :disabled="busy" @click="emit('clone')">克隆</button>
      <input v-model="repositoryName" placeholder="新仓库名称" aria-label="新仓库名称" />
      <select v-model="repositoryVisibility" aria-label="仓库可见性">
        <option value="private">Private</option>
        <option value="public">Public</option>
        <option value="internal">Internal</option>
      </select>
      <input
        v-model="repositoryDescription"
        placeholder="描述（可选）"
        aria-label="仓库描述"
      />
      <button
        class="small-action commit-action"
        :disabled="busy"
        @click="emit('create-repository')"
      >
        创建仓库并推送
      </button>
    </div>
  </div>

  <div v-else class="github-panel">
    <div class="github-repo">
      <div>
        <span class="section-label">GITHUB REPOSITORY</span>
        <h3>{{ overview.repository.nameWithOwner }}</h3>
        <p>{{ overview.repository.description ?? "暂无描述" }}</p>
      </div>
      <div class="github-counts">
        <button class="small-action" :disabled="busy" @click="emit('fork')">Fork</button>
        <button class="small-action" :disabled="busy" @click="emit('sync')">Sync</button>
        <span>★ {{ overview.repository.stargazerCount }}</span>
        <span>⑂ {{ overview.repository.forkCount }}</span>
      </div>
    </div>
    <div class="github-compose">
      <input v-model="githubTitle" placeholder="标题" aria-label="GitHub 标题" />
      <textarea v-model="githubBody" placeholder="正文（可选）" aria-label="GitHub 正文"></textarea>
      <button
        class="small-action commit-action"
        :disabled="busy"
        @click="emit('create-item', 'pr')"
      >
        创建 PR
      </button>
      <button class="small-action" :disabled="busy" @click="emit('create-item', 'issue')">
        创建 Issue
      </button>
    </div>
    <div class="github-repository-tools">
      <input v-model="cloneReference" placeholder="owner/repo 或 GitHub URL" aria-label="克隆仓库" />
      <button class="small-action" :disabled="busy" @click="emit('clone')">克隆</button>
      <input v-model="repositoryName" placeholder="新仓库名称" aria-label="新仓库名称" />
      <select v-model="repositoryVisibility" aria-label="仓库可见性">
        <option value="private">Private</option>
        <option value="public">Public</option>
        <option value="internal">Internal</option>
      </select>
      <input
        v-model="repositoryDescription"
        placeholder="描述（可选）"
        aria-label="仓库描述"
      />
      <button
        class="small-action commit-action"
        :disabled="busy"
        @click="emit('create-repository')"
      >
        从当前项目创建
      </button>
    </div>
    <div class="github-release-compose">
      <input v-model="releaseTag" placeholder="Release Tag" aria-label="Release Tag" />
      <input
        v-model="releaseTitle"
        placeholder="Release 标题（可选）"
        aria-label="Release 标题"
      />
      <textarea
        v-model="releaseNotes"
        placeholder="Release Notes（可选）"
        aria-label="Release Notes"
      ></textarea>
      <button
        class="small-action commit-action"
        :disabled="busy"
        @click="emit('create-release')"
      >
        创建 Release
      </button>
    </div>
    <GithubConfigurationPanel
      :key="path"
      :path="path"
      :busy="busy"
      :run-action="runAction"
      @notice="emit('notice', $event)"
    />
    <GithubEnvironmentsPanel
      :key="`environments-${path}`"
      :path="path"
      :busy="busy"
      :run-action="runAction"
      @notice="emit('notice', $event)"
    />
    <GithubResourcesPanel
      :key="`resources-${path}`"
      :path="path"
      :busy="busy"
      :run-action="runAction"
      @notice="emit('notice', $event)"
    />
    <input
      v-model="githubComment"
      class="github-comment"
      placeholder="Review 或 Issue 评论"
      aria-label="GitHub 评论"
    />
    <div class="github-columns">
      <GithubPullRequestSection
        :pull-requests="overview.pullRequests"
        :busy="busy"
        @view="emit('view-pull-request', $event)"
        @review="(number, action) => emit('review', number, action)"
        @merge="emit('merge', $event)"
      />
      <GithubIssueWorkspace
        :path="path" :issues="overview.issues" :busy="busy" :run-action="runAction"
        @comment="emit('comment-issue', $event)" @close="emit('close-issue', $event)"
        @refresh="emit('refresh')" @notice="emit('notice', $event)"
      />
      <section>
        <h4>Workflows · {{ overview.workflows.length }}</h4>
        <div v-for="workflow in overview.workflows.slice(0, 4)" :key="workflow.id" class="github-row">
          <span>{{ workflow.state }}</span>
          <strong>{{ workflow.name }}</strong>
          <div class="github-row-actions">
            <small :title="workflow.path">{{ workflow.path }}</small>
            <button :disabled="busy" @click="emit('dispatch-workflow', workflow.id)">运行</button>
          </div>
        </div>
      </section>
      <section>
        <h4>Actions · {{ overview.runs.length }}</h4>
        <div v-for="run in overview.runs.slice(0, 4)" :key="run.databaseId" class="github-row">
          <span>{{ run.conclusion ?? run.status }}</span>
          <strong>{{ run.displayTitle }}</strong>
          <div class="github-row-actions">
            <button :disabled="busy" @click="emit('view-run', run.databaseId)">日志</button>
            <button :disabled="busy" @click="emit('rerun-run', run.databaseId)">重跑</button>
            <button
              :disabled="busy || (run.status !== 'in_progress' && run.status !== 'queued')"
              @click="emit('cancel-run', run.databaseId)"
            >
              取消
            </button>
            <button :disabled="busy" @click="emit('download-artifacts', run.databaseId)">
              Artifact
            </button>
          </div>
        </div>
      </section>
      <GithubReleaseWorkspace
        :path="path" :releases="overview.releases" :busy="busy" :run-action="runAction"
        @download="emit('download-release', $event)" @refresh="emit('refresh')"
        @notice="emit('notice', $event)"
      />
    </div>
    <GithubPullRequestDetailPanel
      v-if="selectedPullRequest"
      :pull-request="selectedPullRequest"
      :detail="pullRequestDetail"
      @close="emit('close-pull-request')"
    />
    <pre v-if="runLog" class="github-log">{{ runLog }}</pre>
  </div>
</template>
