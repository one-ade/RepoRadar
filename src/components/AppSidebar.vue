<script setup lang="ts">
import type { GlobalSection } from "../workspace";

defineProps<{
  activeSection: GlobalSection;
  activeOperationCount: number;
}>();

const emit = defineEmits<{
  navigate: [section: GlobalSection];
}>();
</script>

<template>
  <aside class="sidebar global-rail">
    <div class="brand">
      <span class="brand-mark">R</span>
      <div>
        <strong>RepoRadar</strong>
        <small>Git workspace</small>
      </div>
    </div>

    <nav aria-label="主导航">
      <button
        :class="['nav-item', activeSection === 'repositories' && 'active']"
        :aria-current="activeSection === 'repositories' ? 'page' : undefined"
        @click="emit('navigate', 'repositories')"
      >
        <span class="nav-icon">⌁</span>
        <span class="nav-label">仓库</span>
      </button>
      <button
        :class="['nav-item', activeSection === 'activity' && 'active']"
        :aria-current="activeSection === 'activity' ? 'page' : undefined"
        @click="emit('navigate', 'activity')"
      >
        <span class="nav-icon">↯</span>
        <span class="nav-label">活动</span>
        <span v-if="activeOperationCount" class="count-badge">{{ activeOperationCount }}</span>
      </button>
      <button
        :class="['nav-item', activeSection === 'diagnostics' && 'active']"
        :aria-current="activeSection === 'diagnostics' ? 'page' : undefined"
        @click="emit('navigate', 'diagnostics')"
      >
        <span class="nav-icon">◌</span>
        <span class="nav-label">诊断</span>
      </button>
    </nav>

    <button class="settings" disabled>设置</button>
  </aside>
</template>
