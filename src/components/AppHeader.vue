<script setup lang="ts">
import type { GitStatus, Project } from "../api";
import type { GlobalSection, RepositoryView } from "../workspace";

defineProps<{
  project: Project | null;
  status: GitStatus | null;
  globalSection: GlobalSection;
  repositoryView: RepositoryView;
  activityOpen: boolean;
  activeOperationCount: number;
  scanning: boolean;
  busy: boolean;
}>();

const emit = defineEmits<{
  "toggle-operations": [];
  "stop-scan": [];
  "choose-scan-root": [];
  "choose-project": [];
}>();
</script>

<template>
  <header class="workspace-header">
    <div class="workspace-heading">
      <p class="eyebrow">
        {{ globalSection === 'diagnostics' ? 'ENVIRONMENT' : globalSection === 'activity' ? 'ACTIVITY' : 'REPOSITORY WORKSPACE' }}
      </p>
      <div class="workspace-title-row">
        <h1>{{ project ? project.name : globalSection === 'diagnostics' ? '环境诊断' : '仓库工作台' }}</h1>
        <span v-if="project" class="workspace-view-badge">
          {{ repositoryView === 'github' ? 'GitHub' : repositoryView === 'branches' ? 'Branches' : repositoryView === 'history' ? 'History' : 'Changes' }}
        </span>
      </div>
      <p class="subtitle">
        {{ project ? project.path : globalSection === 'diagnostics' ? '检查本地 Git、GitHub CLI 与数据库就绪状态。' : '选择一个仓库开始处理 Git 与 GitHub 工作流。' }}
      </p>
      <div v-if="project && status" class="workspace-meta" aria-label="当前仓库状态">
        <span class="branch-pill">⎇ {{ status.branch }}</span>
        <span>{{ status.files.length }} 个变更</span>
        <span v-if="status.ahead">↑ {{ status.ahead }}</span>
        <span v-if="status.behind">↓ {{ status.behind }}</span>
      </div>
    </div>
    <div class="header-actions">
      <button
        class="secondary-action"
        :class="activityOpen && 'active-action'"
        :aria-expanded="activityOpen"
        aria-controls="activity-tray"
        @click="emit('toggle-operations')"
      >
        活动{{ activeOperationCount ? ` · ${activeOperationCount}` : "" }}
      </button>
      <button
        v-if="scanning"
        class="secondary-action cancel-action"
        @click="emit('stop-scan')"
      >
        取消扫描
      </button>
      <button
        v-else
        class="secondary-action"
        :disabled="busy"
        @click="emit('choose-scan-root')"
      >
        扫描目录
      </button>
      <button class="primary-action" :disabled="busy" @click="emit('choose-project')">
        <span>＋</span>
        添加项目
      </button>
    </div>
  </header>
</template>
