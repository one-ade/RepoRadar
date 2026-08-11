<script setup lang="ts">
import type { GitBranch, GitCommit, GitStatus, Project } from "../api";
import type { RepositoryView } from "../workspace";

export type DetailTab = RepositoryView;
const tabs: ReadonlyArray<readonly [DetailTab, string]> = [
  ["changes", "Changes"],
  ["branches", "分支"],
  ["history", "提交历史"],
  ["github", "GitHub"],
];

defineProps<{
  project: Project;
  status: GitStatus;
  diff: string;
  branches: GitBranch[];
  history: GitCommit[];
  tab: DetailTab;
  busy: boolean;
}>();

const commitMessage = defineModel<string>("commitMessage", { required: true });
const newBranch = defineModel<string>("newBranch", { required: true });
const emit = defineEmits<{
  stage: [];
  unstage: [];
  fetch: [];
  pull: [];
  push: [];
  commit: [];
  "select-tab": [tab: DetailTab];
  "create-branch": [];
  "checkout-branch": [branch: string];
  "remove-branch": [branch: string];
}>();
</script>

<template>
  <section class="git-workspace" :aria-busy="busy">
    <header class="git-workspace-context">
      <div>
        <span class="section-label">LOCAL REPOSITORY</span>
        <h3>{{ project.name }}</h3>
        <p class="workspace-path">{{ project.path }}</p>
      </div>
      <span class="branch-pill">⎇ {{ status.branch }}</span>
    </header>

    <div class="git-metrics">
      <span>{{ status.files.length }} 个变更</span>
      <span v-if="status.ahead">↑ {{ status.ahead }}</span>
      <span v-if="status.behind">↓ {{ status.behind }}</span>
      <span v-if="!status.ahead && !status.behind">已同步</span>
    </div>

    <div class="git-actions" aria-label="Git 高频操作">
      <button class="small-action" :disabled="busy" @click="emit('stage')">暂存全部</button>
      <button class="small-action" :disabled="busy" @click="emit('unstage')">取消暂存</button>
      <button class="small-action" :disabled="busy" @click="emit('fetch')">Fetch</button>
      <button class="small-action" :disabled="busy" @click="emit('pull')">Pull</button>
      <button class="small-action" :disabled="busy" @click="emit('push')">Push</button>
      <input
        v-model="commitMessage"
        aria-label="提交信息"
        placeholder="提交信息"
        :disabled="busy"
        @keyup.enter="!busy && emit('commit')"
      />
      <button class="small-action commit-action" :disabled="busy" @click="emit('commit')">
        提交
      </button>
    </div>

    <div class="detail-tabs" role="tablist" aria-label="仓库工作区">
      <button
        v-for="item in tabs"
        :key="item[0]"
        :class="['detail-tab', tab === item[0] && 'active']"
        role="tab"
        :aria-selected="tab === item[0]"
        @click="emit('select-tab', item[0])"
      >
        {{ item[1] }}
      </button>
    </div>

    <div v-if="tab === 'changes'" class="git-view-panel changes-panel">
      <div v-if="status.files.length" class="file-list">
        <div v-for="file in status.files" :key="file.path" class="file-row">
          <span :class="['file-status', file.indexStatus === '?' && 'untracked']">
            {{ file.indexStatus === "?" ? "未跟踪" : `${file.indexStatus}${file.worktreeStatus}` }}
          </span>
          <span>{{ file.path }}</span>
        </div>
      </div>
      <p v-else class="clean-state">工作区干净，没有未提交文件。</p>
      <div class="diff-panel">
        <pre v-if="diff">{{ diff }}</pre>
        <p v-else class="clean-state">没有未提交 Diff</p>
      </div>
    </div>

    <div v-else-if="tab === 'branches'" class="git-view-panel branch-panel">
      <div class="branch-create">
        <input
          v-model="newBranch"
          placeholder="新分支名"
          aria-label="新分支名"
          :disabled="busy"
          @keyup.enter="!busy && emit('create-branch')"
        />
        <button class="small-action commit-action" :disabled="busy" @click="emit('create-branch')">
          创建并切换
        </button>
      </div>
      <div class="branch-list">
        <div v-for="branch in branches" :key="branch.name" class="branch-row">
          <button class="branch-name" :disabled="branch.current || busy" @click="emit('checkout-branch', branch.name)">
            <span>{{ branch.current ? "●" : "○" }}</span>
            {{ branch.name }}
          </button>
          <small>{{ branch.upstream ?? "本地" }}</small>
          <button class="branch-delete" :disabled="branch.current || busy" @click="emit('remove-branch', branch.name)">
            删除
          </button>
        </div>
      </div>
      <p v-if="!branches.length" class="clean-state">暂无本地分支。</p>
    </div>

    <div v-else-if="tab === 'history'" class="git-view-panel history-panel">
      <div v-for="commit in history" :key="commit.hash" class="history-row">
        <span class="commit-hash">{{ commit.hash.slice(0, 7) }}</span>
        <div>
          <strong>{{ commit.subject }}</strong>
          <small>{{ commit.author }} · {{ commit.date }}</small>
        </div>
      </div>
      <p v-if="!history.length" class="clean-state">暂无提交记录。</p>
    </div>
  </section>
</template>
