<script setup lang="ts">
defineProps<{
  loading: boolean;
  readiness: number;
}>();

defineEmits<{ refresh: [] }>();
</script>

<template>
  <section class="hero-card">
    <div>
      <span class="section-label">环境检查</span>
      <h2>{{ loading ? "正在检测开发环境…" : `${readiness}/3 项基础能力已就绪` }}</h2>
      <p>RepoRadar 复用系统 Git 和 GitHub CLI，不接管或保存你的认证凭据。</p>
    </div>
    <button class="refresh-button" :disabled="loading" @click="$emit('refresh')">
      {{ loading ? "检测中" : "重新检测" }}
    </button>
  </section>
</template>
