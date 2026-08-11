<script setup lang="ts">
import type { GithubSection } from "../workspace";

defineProps<{ section?: GithubSection }>();

const emit = defineEmits<{
  select: [section: GithubSection];
}>();

const items: ReadonlyArray<readonly [GithubSection, string]> = [
  ["pull-requests", "Pull Requests"],
  ["issues", "Issues"],
  ["actions", "Actions"],
  ["releases", "Releases"],
  ["tools", "Tools"],
];
</script>

<template>
  <nav class="github-workspace-nav" aria-label="GitHub 工作区">
    <button
      v-for="item in items"
      :key="item[0]"
      :class="['github-workspace-nav-item', section === item[0] && 'active']"
      :aria-current="section === item[0] ? 'page' : undefined"
      @click="emit('select', item[0])"
    >
      {{ item[1] }}
    </button>
  </nav>
</template>
