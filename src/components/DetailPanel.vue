<script setup lang="ts">
import type { GithubPullRequestDetail } from "../api";
import type { DetailTarget } from "../workspace";
import GithubIssueDetailPanel from "./GithubIssueDetailPanel.vue";
import GithubPullRequestDetailPanel from "./GithubPullRequestDetailPanel.vue";
import GithubReleaseDetailPanel from "./GithubReleaseDetailPanel.vue";
import GithubRunDetailPanel from "./GithubRunDetailPanel.vue";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;

const props = defineProps<{
  target: DetailTarget;
  path: string;
  busy: boolean;
  pullRequestDetail: GithubPullRequestDetail | null;
  runLog: string;
  runAction: RunAction;
}>();

const emit = defineEmits<{
  close: [];
  refresh: [];
  notice: [message: string];
  "rerun-run": [databaseId: number];
  "cancel-run": [databaseId: number];
  "download-artifacts": [databaseId: number];
}>();

function title() {
  if (!props.target) return "详情";
  if (props.target.kind === "pull-request") return `Pull Request #${props.target.item.number}`;
  if (props.target.kind === "issue") return `Issue #${props.target.item.number}`;
  if (props.target.kind === "release") return `Release ${props.target.item.tagName}`;
  return `Actions Run #${props.target.item.databaseId}`;
}
</script>

<template>
  <aside v-if="target" class="detail-panel" aria-label="详情面板">
    <div class="detail-panel-heading">
      <div>
        <span class="section-label">DETAIL PANEL</span>
        <h2>{{ title() }}</h2>
      </div>
      <button class="small-action" aria-label="关闭详情面板" @click="emit('close')">关闭</button>
    </div>
    <div class="detail-panel-scroll">
      <GithubPullRequestDetailPanel
        v-if="target.kind === 'pull-request'"
        :pull-request="target.item"
        :detail="pullRequestDetail"
        embedded
      />
      <GithubIssueDetailPanel
        v-else-if="target.kind === 'issue'"
        :path="path"
        :issue="target.item"
        :busy="busy"
        :run-action="runAction"
        @refresh="emit('refresh')"
        @notice="emit('notice', $event)"
      />
      <GithubReleaseDetailPanel
        v-else-if="target.kind === 'release'"
        :path="path"
        :release="target.item"
        :busy="busy"
        :run-action="runAction"
        @refresh="emit('refresh')"
        @notice="emit('notice', $event)"
      />
      <GithubRunDetailPanel
        v-else
        :run="target.item"
        :log="runLog"
        :busy="busy"
        @rerun-run="emit('rerun-run', $event)"
        @cancel-run="emit('cancel-run', $event)"
        @download-artifacts="emit('download-artifacts', $event)"
      />
    </div>
  </aside>
</template>
