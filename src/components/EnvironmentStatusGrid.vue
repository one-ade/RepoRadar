<script setup lang="ts">
import type { EnvironmentStatus, ToolStatus } from "../api";

defineProps<{
  environment: EnvironmentStatus | null;
  loading?: boolean;
}>();

function toolLabel(tool: ToolStatus | undefined, auth = false) {
  if (!tool?.installed) return "未检测到";
  if (auth && !tool.authenticated) return "需要登录";
  return tool.version ?? "已就绪";
}
</script>

<template>
  <section class="status-grid" aria-label="环境状态" :aria-busy="loading">
    <template v-if="loading">
      <article v-for="index in 3" :key="index" class="status-card status-card-skeleton">
        <span class="skeleton-icon"></span>
        <span class="skeleton-line short"></span>
        <span class="skeleton-line medium"></span>
        <span class="skeleton-line long"></span>
      </article>
    </template>
    <template v-else>
    <article class="status-card">
      <div class="status-top">
        <span class="tool-logo git-logo">git</span>
        <span :class="['status-dot', environment?.git.installed && 'ready']"></span>
      </div>
      <p>本地版本控制</p>
      <h3>Git</h3>
      <span class="status-value">{{ toolLabel(environment?.git) }}</span>
    </article>

    <article class="status-card">
      <div class="status-top">
        <span class="tool-logo gh-logo">GH</span>
        <span :class="['status-dot', environment?.gh.authenticated && 'ready']"></span>
      </div>
      <p>GitHub 平台能力</p>
      <h3>GitHub CLI</h3>
      <span class="status-value">{{ toolLabel(environment?.gh, true) }}</span>
      <div v-if="environment?.githubHosts.length" class="host-list">
        <span v-for="host in environment.githubHosts" :key="`${host.host}:${host.login}`">
          {{ host.host }} / {{ host.login }}
        </span>
      </div>
      <small v-else class="host-empty">未发现已登录 Host</small>
    </article>

    <article class="status-card">
      <div class="status-top">
        <span class="tool-logo db-logo">DB</span>
        <span :class="['status-dot', environment?.databaseReady && 'ready']"></span>
      </div>
      <p>本地数据存储</p>
      <h3>SQLite</h3>
      <span class="status-value">
        {{ environment?.databaseReady ? "数据库已就绪" : "等待初始化" }}
      </span>
    </article>
    </template>
  </section>
</template>
