<script setup lang="ts">
import { computed } from "vue";

import type {
  GithubDetailField,
  GithubDetailValue,
  GithubPullRequest,
  GithubPullRequestDetail,
} from "../api";

const props = defineProps<{
  pullRequest: GithubPullRequest;
  detail: GithubPullRequestDetail | null;
}>();

defineEmits<{ close: [] }>();

const fieldGroups = [
  {
    label: "概览",
    names: [
      "number", "title", "state", "body", "url", "author", "createdAt", "updatedAt",
      "closed", "closedAt",
    ],
  },
  {
    label: "分支与变更",
    names: [
      "baseRefName", "baseRefOid", "headRefName", "headRefOid", "headRepository",
      "headRepositoryOwner", "isCrossRepository", "additions", "deletions", "changedFiles",
      "files", "commits",
    ],
  },
  {
    label: "评审与合并",
    names: [
      "isDraft", "reviewDecision", "reviewRequests", "reviews", "latestReviews", "comments",
      "mergeable", "mergeStateStatus", "mergeCommit", "potentialMergeCommit", "mergedAt",
      "mergedBy", "autoMergeRequest", "maintainerCanModify",
    ],
  },
  {
    label: "关联与元数据",
    names: [
      "id", "fullDatabaseId", "assignees", "labels", "milestone", "closingIssuesReferences",
      "projectCards", "projectItems", "reactionGroups", "statusCheckRollup",
    ],
  },
] as const;

const groupedFields = computed(() => {
  const remaining = new Map(props.detail?.fields.map((field) => [field.name, field]) ?? []);
  const groups: Array<{ label: string; fields: GithubDetailField[] }> = [];

  for (const group of fieldGroups) {
    const fields = group.names.flatMap((name) => {
      const field = remaining.get(name);
      if (!field) return [];
      remaining.delete(name);
      return [field];
    });
    if (fields.length) groups.push({ label: group.label, fields });
  }
  if (remaining.size) groups.push({ label: "其他", fields: [...remaining.values()] });
  return groups;
});

function isComplex(value: GithubDetailValue) {
  return value !== null && typeof value === "object";
}

function scalarText(value: GithubDetailValue) {
  if (value === null || value === "") return "—";
  if (typeof value === "boolean") return value ? "是" : "否";
  return typeof value === "object" ? "" : String(value);
}

function complexSummary(value: GithubDetailValue) {
  return Array.isArray(value) ? `${value.length} 项` : "展开 JSON";
}

function formattedJson(value: GithubDetailValue) {
  return JSON.stringify(value, null, 2) ?? "";
}
</script>

<template>
  <section class="github-pr-detail" aria-label="Pull Request 详情">
    <header>
      <div>
        <span class="section-label">PULL REQUEST #{{ pullRequest.number }}</span>
        <h4>{{ pullRequest.title }}</h4>
        <p>{{ pullRequest.headRefName }} → {{ pullRequest.baseRefName }}</p>
      </div>
      <button class="github-pr-detail-close" aria-label="关闭 Pull Request 详情" @click="$emit('close')">
        ×
      </button>
    </header>

    <p v-if="!detail" class="github-pr-detail-loading">正在加载完整详情…</p>
    <div v-else class="github-pr-detail-groups">
      <section v-for="group in groupedFields" :key="group.label">
        <h5>{{ group.label }}</h5>
        <dl>
          <div v-for="field in group.fields" :key="field.name" class="github-pr-detail-field">
            <dt>{{ field.name }}</dt>
            <dd>
              <details v-if="isComplex(field.value)">
                <summary>{{ complexSummary(field.value) }}</summary>
                <pre>{{ formattedJson(field.value) }}</pre>
              </details>
              <span v-else>{{ scalarText(field.value) }}</span>
            </dd>
          </div>
        </dl>
      </section>
    </div>
  </section>
</template>
