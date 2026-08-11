<script setup lang="ts">
import type { GithubRun } from "../api";

defineProps<{
  run: GithubRun;
  log: string;
  busy: boolean;
}>();

const emit = defineEmits<{
  "rerun-run": [databaseId: number];
  "cancel-run": [databaseId: number];
  "download-artifacts": [databaseId: number];
}>();
</script>

<template>
  <section class="detail-object-content" aria-label="Actions Run 详情" :aria-busy="busy">
    <div class="detail-object-summary">
      <span class="section-label">ACTIONS RUN #{{ run.databaseId }}</span>
      <h3>{{ run.displayTitle }}</h3>
      <p>{{ run.workflowName }} · {{ run.status }} · {{ run.conclusion ?? "进行中" }}</p>
    </div>
    <div class="detail-action-row">
      <button class="small-action" :disabled="busy" @click="emit('rerun-run', run.databaseId)">重跑</button>
      <button class="small-action" :disabled="busy || (run.status !== 'in_progress' && run.status !== 'queued')" @click="emit('cancel-run', run.databaseId)">取消</button>
      <button class="small-action" :disabled="busy" @click="emit('download-artifacts', run.databaseId)">下载 Artifact</button>
    </div>
    <pre v-if="log" class="github-log">{{ log }}</pre>
    <p v-else class="github-pr-detail-loading">正在加载运行日志…</p>
  </section>
</template>
