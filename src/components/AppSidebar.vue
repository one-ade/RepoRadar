<script setup lang="ts">
defineProps<{
  activeSection: "overview" | "projects";
  projectCount: number;
  githubDisabled: boolean;
}>();

const emit = defineEmits<{
  navigate: [section: "overview" | "projects"];
  github: [];
  operations: [];
}>();
</script>

<template>
  <aside class="sidebar">
    <div class="brand">
      <span class="brand-mark">R</span>
      <div>
        <strong>RepoRadar</strong>
        <small>Git workspace</small>
      </div>
    </div>

    <nav aria-label="主导航">
      <button
        :class="['nav-item', activeSection === 'overview' && 'active']"
        :aria-pressed="activeSection === 'overview'"
        @click="emit('navigate', 'overview')"
      >
        <span class="nav-icon">⌁</span>
        仪表盘
      </button>
      <button
        :class="['nav-item', activeSection === 'projects' && 'active']"
        :aria-pressed="activeSection === 'projects'"
        @click="emit('navigate', 'projects')"
      >
        <span class="nav-icon">◇</span>
        项目
        <span class="count-badge">{{ projectCount }}</span>
      </button>
      <button class="nav-item" :disabled="githubDisabled" @click="emit('github')">
        <span class="nav-icon">⑂</span>
        GitHub
      </button>
      <button class="nav-item" @click="emit('operations')">
        <span class="nav-icon">↯</span>
        操作中心
      </button>
    </nav>

    <button class="settings" disabled>设置</button>
  </aside>
</template>
