<script setup lang="ts">
import type { GithubPullRequest, GithubPullRequestDetail } from "../api";
import GithubDetailGroups from "./GithubDetailGroups.vue";

defineProps<{
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
    <GithubDetailGroups v-else :detail="detail" :groups="fieldGroups" />
  </section>
</template>
