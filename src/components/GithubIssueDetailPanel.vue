<script setup lang="ts">
import { ref, shallowRef, watch } from "vue";

import {
  editIssue,
  getIssueDetail,
  type GithubIssue,
  type GithubIssueDetail,
  type GithubIssueEdit,
} from "../api";
import GithubDetailGroups from "./GithubDetailGroups.vue";
import GithubIssueEditor from "./GithubIssueEditor.vue";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;

const props = defineProps<{
  path: string;
  issue: GithubIssue;
  busy: boolean;
  runAction: RunAction;
}>();

const emit = defineEmits<{
  refresh: [];
  notice: [message: string];
}>();

const detail = shallowRef<GithubIssueDetail | null>(null);
const detailError = ref("");
let requestId = 0;

const fieldGroups = [
  { label: "概览", names: ["number", "title", "state", "stateReason", "body", "url", "author", "createdAt", "updatedAt", "closed", "closedAt", "isPinned"] },
  { label: "关系", names: ["assignees", "labels", "milestone", "issueType", "parent", "subIssues", "subIssuesSummary", "blockedBy", "blocking", "closedByPullRequestsReferences"] },
  { label: "项目与活动", names: ["projectCards", "projectItems", "comments", "reactionGroups", "id"] },
] as const;

async function loadIssue() {
  const currentRequest = ++requestId;
  detail.value = null;
  detailError.value = "";
  try {
    await props.runAction(async () => {
      const loaded = await getIssueDetail(props.path, props.issue.number);
      if (currentRequest === requestId) detail.value = loaded;
    }, `加载 Issue #${props.issue.number}`);
  } catch {
    if (currentRequest === requestId) detailError.value = "完整详情加载失败，请重试。";
    return;
  }
  if (currentRequest === requestId && !detail.value) {
    detailError.value = "完整详情加载失败，请重试。";
  }
}

function stringField(name: string) {
  const value = detail.value?.fields.find((field) => field.name === name)?.value;
  return typeof value === "string" ? value : "";
}

async function saveIssue(edit: GithubIssueEdit) {
  await props.runAction(async () => {
    await editIssue(props.path, props.issue.number, edit);
    await loadIssue();
    emit("refresh");
    emit("notice", `Issue #${props.issue.number} 已更新`);
  }, `更新 Issue #${props.issue.number}`);
}

watch(() => [props.path, props.issue.number] as const, loadIssue, { immediate: true });
</script>

<template>
  <section class="detail-object-content" aria-label="Issue 详情" :aria-busy="(!detail && !detailError) || busy">
    <div class="detail-object-summary">
      <span class="section-label">ISSUE #{{ issue.number }}</span>
      <h3>{{ issue.title }}</h3>
      <p>{{ issue.state }} · {{ issue.author?.login ?? "未知作者" }}</p>
    </div>
    <template v-if="detailError">
      <p class="github-pr-detail-error" role="alert">{{ detailError }}</p>
      <button data-action="retry-issue-detail" class="small-action retry-action" :disabled="busy" @click="loadIssue">
        重试加载
      </button>
    </template>
    <p v-else-if="!detail" class="github-pr-detail-loading">正在加载完整详情…</p>
    <template v-else>
      <GithubDetailGroups :detail="detail" :groups="fieldGroups" />
      <GithubIssueEditor :title="stringField('title')" :body="stringField('body')" :busy="busy" @save="saveIssue" />
    </template>
  </section>
</template>
