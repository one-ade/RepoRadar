<script setup lang="ts">
export type WorkspaceSection = "overview" | "projects" | "github" | "operations";

defineProps<{
  activeSection: WorkspaceSection;
  projectCount: number;
  githubDisabled: boolean;
}>();

const emit = defineEmits<{
  navigate: [section: WorkspaceSection];
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
        :aria-current="activeSection === 'overview' ? 'page' : undefined"
        @click="emit('navigate', 'overview')"
      >
        <span class="nav-icon">⌁</span>
        仪表盘
      </button>
      <button
        :class="['nav-item', activeSection === 'projects' && 'active']"
        :aria-current="activeSection === 'projects' ? 'page' : undefined"
        @click="emit('navigate', 'projects')"
      >
        <span class="nav-icon">◇</span>
        项目
        <span class="count-badge">{{ projectCount }}</span>
      </button>
      <button
        :class="['nav-item', activeSection === 'github' && 'active']"
        :disabled="githubDisabled"
        :aria-current="activeSection === 'github' ? 'page' : undefined"
        @click="emit('navigate', 'github')"
      >
        <span class="nav-icon">⑂</span>
        GitHub
      </button>
      <button
        :class="['nav-item', activeSection === 'operations' && 'active']"
        :aria-current="activeSection === 'operations' ? 'page' : undefined"
        @click="emit('navigate', 'operations')"
      >
        <span class="nav-icon">↯</span>
        操作中心
      </button>
    </nav>

    <button class="settings" disabled>设置</button>
  </aside>
</template>
