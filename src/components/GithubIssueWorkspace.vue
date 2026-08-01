<script setup lang="ts">
import { ref, shallowRef, watch } from "vue";

import { editIssue, getIssueDetail, type GithubIssue, type GithubIssueDetail, type GithubIssueEdit } from "../api";
import GithubDetailGroups from "./GithubDetailGroups.vue";
import GithubIssueEditor from "./GithubIssueEditor.vue";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;
const props = defineProps<{ path: string; issues: readonly GithubIssue[]; busy: boolean; runAction: RunAction }>();
const emit = defineEmits<{
  comment: [number: number]; close: [number: number]; refresh: []; notice: [message: string];
}>();
const selectedIssue = ref<GithubIssue | null>(null);
const detail = shallowRef<GithubIssueDetail | null>(null);
let requestId = 0;

const fieldGroups = [
  { label: "概览", names: ["number", "title", "state", "stateReason", "body", "url", "author", "createdAt", "updatedAt", "closed", "closedAt", "isPinned"] },
  { label: "关系", names: ["assignees", "labels", "milestone", "issueType", "parent", "subIssues", "subIssuesSummary", "blockedBy", "blocking", "closedByPullRequestsReferences"] },
  { label: "项目与活动", names: ["projectCards", "projectItems", "comments", "reactionGroups", "id"] },
] as const;

function clearIssue() {
  requestId += 1;
  selectedIssue.value = null;
  detail.value = null;
}

async function viewIssue(issue: GithubIssue) {
  const currentRequest = ++requestId;
  selectedIssue.value = issue;
  detail.value = null;
  await props.runAction(async () => {
    const loaded = await getIssueDetail(props.path, issue.number);
    if (currentRequest === requestId) detail.value = loaded;
  }, `加载 Issue #${issue.number}`);
}

function stringField(name: string) {
  const value = detail.value?.fields.find((field) => field.name === name)?.value;
  return typeof value === "string" ? value : "";
}

async function saveIssue(edit: GithubIssueEdit) {
  if (!selectedIssue.value) return;
  const number = selectedIssue.value.number;
  await props.runAction(async () => {
    await editIssue(props.path, number, edit);
    detail.value = await getIssueDetail(props.path, number);
    emit("refresh");
    emit("notice", `Issue #${number} 已更新`);
  }, `更新 Issue #${number}`);
}

watch(() => props.path, clearIssue);
</script>

<template>
  <section class="github-issue-workspace">
    <h4>Issues · {{ issues.length }}</h4>
    <div v-for="item in issues.slice(0, 5)" :key="item.number" class="github-row">
      <span>#{{ item.number }}</span><strong>{{ item.title }}</strong>
      <div class="github-row-actions">
        <button data-action="view-issue" :disabled="busy" @click="viewIssue(item)">详情</button>
        <button :disabled="busy" @click="emit('comment', item.number)">评论</button>
        <button :disabled="busy" @click="emit('close', item.number)">关闭</button>
      </div>
    </div>

    <div v-if="selectedIssue" class="github-pr-detail" aria-label="Issue 详情">
      <header>
        <div><span class="section-label">ISSUE #{{ selectedIssue.number }}</span><h4>{{ selectedIssue.title }}</h4></div>
        <button class="github-pr-detail-close" aria-label="关闭 Issue 详情" @click="clearIssue">×</button>
      </header>
      <p v-if="!detail" class="github-pr-detail-loading">正在加载完整详情…</p>
      <template v-else>
        <GithubDetailGroups :detail="detail" :groups="fieldGroups" />
        <GithubIssueEditor :title="stringField('title')" :body="stringField('body')" :busy="busy" @save="saveIssue" />
      </template>
    </div>
  </section>
</template>
