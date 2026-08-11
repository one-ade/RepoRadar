<script setup lang="ts">
import type { GithubPullRequest } from "../api";

defineProps<{
  pullRequests: readonly GithubPullRequest[];
  busy: boolean;
  showHeading?: boolean;
}>();

const emit = defineEmits<{
  view: [pullRequest: GithubPullRequest];
  review: [number: number, action: "approve" | "comment" | "request-changes"];
  merge: [number: number];
}>();
</script>

<template>
  <section>
    <h4 v-if="showHeading !== false">Pull Requests · {{ pullRequests.length }}</h4>
    <div v-for="item in pullRequests.slice(0, 5)" :key="item.number" class="github-row">
      <span>#{{ item.number }}</span>
      <strong>{{ item.title }}</strong>
      <div class="github-row-actions">
        <button data-action="view-pr" :disabled="busy" @click="emit('view', item)">详情</button>
        <button :disabled="busy" @click="emit('review', item.number, 'comment')">评论</button>
        <button :disabled="busy" @click="emit('review', item.number, 'approve')">批准</button>
        <button
          data-action="request-changes"
          :disabled="busy"
          @click="emit('review', item.number, 'request-changes')"
        >
          请求修改
        </button>
        <button :disabled="busy || item.isDraft" @click="emit('merge', item.number)">合并</button>
      </div>
    </div>
  </section>
</template>
