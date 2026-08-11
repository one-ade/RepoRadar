<script setup lang="ts">
import type { GithubIssue } from "../api";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;

defineProps<{
  path: string;
  issues: readonly GithubIssue[];
  busy: boolean;
  runAction: RunAction;
  showHeading?: boolean;
}>();

const emit = defineEmits<{
  view: [issue: GithubIssue];
  comment: [number: number];
  close: [number: number];
}>();
</script>

<template>
  <section class="github-issue-workspace">
    <h4 v-if="showHeading !== false">Issues · {{ issues.length }}</h4>
    <div v-for="item in issues" :key="item.number" class="github-row">
      <span>#{{ item.number }}</span><strong>{{ item.title }}</strong>
      <div class="github-row-actions">
        <button data-action="view-issue" :disabled="busy" @click="emit('view', item)">详情</button>
        <button :disabled="busy" @click="emit('comment', item.number)">评论</button>
        <button :disabled="busy" @click="emit('close', item.number)">关闭</button>
      </div>
    </div>
  </section>
</template>
