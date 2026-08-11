<script setup lang="ts">
import { ref } from "vue";
import type { GithubOverview, GithubPullRequest, GithubPullRequestDetail } from "../api";
import type { GithubComposer, GithubSection } from "../workspace";
import GithubActionsWorkspace from "./GithubActionsWorkspace.vue";
import GithubIssueWorkspace from "./GithubIssueWorkspace.vue";
import GithubPullRequestSection from "./GithubPullRequestSection.vue";
import GithubReleaseWorkspace from "./GithubReleaseWorkspace.vue";
import GithubToolsPanel from "./GithubToolsPanel.vue";
import GithubWorkspaceNav from "./GithubWorkspaceNav.vue";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;

const props = withDefaults(defineProps<{
  path: string;
  busy: boolean;
  overview: GithubOverview | null;
  section?: GithubSection;
  selectedPullRequest: GithubPullRequest | null;
  pullRequestDetail: GithubPullRequestDetail | null;
  runLog: string;
  runAction: RunAction;
}>(), { section: "pull-requests" as GithubSection });

const composer = ref<GithubComposer>("none");

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
  "view-issue": [issue: import("../api").GithubIssue];
  "view-release": [release: import("../api").GithubRelease];
  "comment-issue": [number: number];
  "close-issue": [number: number];
  refresh: [];
  "dispatch-workflow": [workflowId: number];
  "view-run": [databaseId: number];
  "rerun-run": [databaseId: number];
  "cancel-run": [databaseId: number];
  "download-artifacts": [databaseId: number];
  "download-release": [tag: string];
  "select-section": [section: GithubSection];
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
    <GithubWorkspaceNav :section="props.section" @select="emit('select-section', $event)" />
    <div class="github-create-actions" aria-label="GitHub 创建操作">
      <button class="small-action" :disabled="busy" @click="composer = 'pull-request'">新建 PR</button>
      <button class="small-action" :disabled="busy" @click="composer = 'issue'">新建 Issue</button>
      <button class="small-action" :disabled="busy" @click="composer = 'release'">新建 Release</button>
      <button class="small-action" :disabled="busy" @click="composer = 'clone'">克隆仓库</button>
      <button class="small-action" :disabled="busy" @click="composer = 'repository'">创建仓库</button>
    </div>

    <section v-if="composer !== 'none'" class="github-composer-card">
      <div class="github-section-heading">
        <div>
          <span class="section-label">COMPOSER</span>
          <h4>{{ composer === 'pull-request' ? '创建 Pull Request' : composer === 'issue' ? '创建 Issue' : composer === 'release' ? '创建 Release' : composer === 'clone' ? '克隆仓库' : '创建 GitHub 仓库' }}</h4>
        </div>
        <button class="small-action" @click="composer = 'none'">取消</button>
      </div>
      <div v-if="composer === 'pull-request' || composer === 'issue'" class="github-compose">
        <input v-model="githubTitle" placeholder="标题" aria-label="GitHub 标题" />
        <textarea v-model="githubBody" placeholder="正文（可选）" aria-label="GitHub 正文"></textarea>
        <button
          class="small-action commit-action"
          :disabled="busy"
          @click="emit('create-item', composer === 'pull-request' ? 'pr' : 'issue'); composer = 'none'"
        >
          创建 {{ composer === 'pull-request' ? 'PR' : 'Issue' }}
        </button>
      </div>
      <div v-else-if="composer === 'release'" class="github-release-compose">
        <input v-model="releaseTag" placeholder="Release Tag" aria-label="Release Tag" />
        <input v-model="releaseTitle" placeholder="Release 标题（可选）" aria-label="Release 标题" />
        <textarea v-model="releaseNotes" placeholder="Release Notes（可选）" aria-label="Release Notes"></textarea>
        <button class="small-action commit-action" :disabled="busy" @click="emit('create-release'); composer = 'none'">创建 Release</button>
      </div>
      <div v-else class="github-repository-tools">
        <template v-if="composer === 'clone'">
          <input v-model="cloneReference" placeholder="owner/repo 或 GitHub URL" aria-label="克隆仓库" />
          <button class="small-action commit-action" :disabled="busy" @click="emit('clone'); composer = 'none'">克隆</button>
        </template>
        <template v-else>
          <input v-model="repositoryName" placeholder="新仓库名称" aria-label="新仓库名称" />
          <select v-model="repositoryVisibility" aria-label="仓库可见性">
            <option value="private">Private</option>
            <option value="public">Public</option>
            <option value="internal">Internal</option>
          </select>
          <input v-model="repositoryDescription" placeholder="描述（可选）" aria-label="仓库描述" />
          <button class="small-action commit-action" :disabled="busy" @click="emit('create-repository'); composer = 'none'">创建并推送</button>
        </template>
      </div>
    </section>

    <section v-if="props.section === 'pull-requests'" class="github-section-view">
      <div class="github-section-heading">
        <div><span class="section-label">COLLABORATION</span><h4>Pull Requests · {{ overview.pullRequests.length }}</h4></div>
      </div>
      <input v-model="githubComment" class="github-comment" placeholder="Review 评论" aria-label="GitHub 评论" />
      <GithubPullRequestSection
        :pull-requests="overview.pullRequests"
        :busy="busy"
        :show-heading="false"
        @view="emit('view-pull-request', $event)"
        @review="(number, action) => emit('review', number, action)"
        @merge="emit('merge', $event)"
      />
    </section>

    <section v-else-if="props.section === 'issues'" class="github-section-view">
      <input v-model="githubComment" class="github-comment" placeholder="Issue 评论" aria-label="GitHub 评论" />
      <GithubIssueWorkspace
        :path="path" :issues="overview.issues" :busy="busy" :run-action="runAction" :show-heading="false"
        @view="emit('view-issue', $event)"
        @comment="emit('comment-issue', $event)" @close="emit('close-issue', $event)"
      />
    </section>

    <GithubActionsWorkspace
      v-else-if="props.section === 'actions'"
      :overview="overview"
      :busy="busy"
      :run-log="runLog"
      @dispatch-workflow="emit('dispatch-workflow', $event)"
      @view-run="emit('view-run', $event)"
      @rerun-run="emit('rerun-run', $event)"
      @cancel-run="emit('cancel-run', $event)"
      @download-artifacts="emit('download-artifacts', $event)"
    />

    <GithubReleaseWorkspace
      v-else-if="props.section === 'releases'"
      :path="path" :releases="overview.releases" :busy="busy" :run-action="runAction" :show-heading="false"
      @view="emit('view-release', $event)"
      @download="emit('download-release', $event)"
    />

    <GithubToolsPanel
      v-else
      :path="path"
      :busy="busy"
      :run-action="runAction"
      @notice="emit('notice', $event)"
    />
  </div>
</template>
