<script setup lang="ts">
import type { GithubOverview } from "../api";

defineProps<{
  overview: GithubOverview;
  busy: boolean;
  runLog: string;
}>();

const emit = defineEmits<{
  "dispatch-workflow": [workflowId: number];
  "view-run": [databaseId: number];
  "rerun-run": [databaseId: number];
  "cancel-run": [databaseId: number];
  "download-artifacts": [databaseId: number];
}>();
</script>

<template>
  <section class="github-section-view github-actions-workspace" aria-label="GitHub Actions">
    <div class="github-section-heading">
      <div>
        <span class="section-label">AUTOMATION</span>
        <h4>Workflows & Actions</h4>
      </div>
      <span class="github-section-count">{{ overview.runs.length }} runs</span>
    </div>
    <div class="github-columns github-actions-columns">
      <section>
        <h4>Workflows · {{ overview.workflows.length }}</h4>
        <div v-for="workflow in overview.workflows" :key="workflow.id" class="github-row">
          <span>{{ workflow.state }}</span>
          <strong>{{ workflow.name }}</strong>
          <div class="github-row-actions">
            <small :title="workflow.path">{{ workflow.path }}</small>
            <button :disabled="busy" @click="emit('dispatch-workflow', workflow.id)">运行</button>
          </div>
        </div>
        <p v-if="!overview.workflows.length" class="clean-state">暂无 Workflow。</p>
      </section>
      <section>
        <h4>Actions · {{ overview.runs.length }}</h4>
        <div v-for="run in overview.runs" :key="run.databaseId" class="github-row">
          <span>{{ run.conclusion ?? run.status }}</span>
          <strong>{{ run.displayTitle }}</strong>
          <div class="github-row-actions">
            <button :disabled="busy" @click="emit('view-run', run.databaseId)">日志</button>
            <button :disabled="busy" @click="emit('rerun-run', run.databaseId)">重跑</button>
            <button
              :disabled="busy || (run.status !== 'in_progress' && run.status !== 'queued')"
              @click="emit('cancel-run', run.databaseId)"
            >
              取消
            </button>
            <button :disabled="busy" @click="emit('download-artifacts', run.databaseId)">Artifact</button>
          </div>
        </div>
        <p v-if="!overview.runs.length" class="clean-state">暂无 Actions Run。</p>
      </section>
    </div>
    <pre v-if="runLog" class="github-log">{{ runLog }}</pre>
  </section>
</template>
