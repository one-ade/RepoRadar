<script setup lang="ts">
import GithubAdvancedPanel from "./GithubAdvancedPanel.vue";
import GithubConfigurationPanel from "./GithubConfigurationPanel.vue";
import GithubEnvironmentsPanel from "./GithubEnvironmentsPanel.vue";
import GithubResourcesPanel from "./GithubResourcesPanel.vue";
import GithubSearchPanel from "./GithubSearchPanel.vue";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;

defineProps<{
  path: string;
  busy: boolean;
  runAction: RunAction;
}>();

const emit = defineEmits<{
  notice: [message: string];
}>();
</script>

<template>
  <section class="github-section-view github-tools-panel" aria-label="GitHub Tools">
    <div class="github-section-heading">
      <div>
        <span class="section-label">REPOSITORY TOOLS</span>
        <h4>配置与资源</h4>
      </div>
      <span class="github-section-count">按需加载</span>
    </div>
    <GithubConfigurationPanel :key="path" :path="path" :busy="busy" :run-action="runAction" @notice="emit('notice', $event)" />
    <GithubEnvironmentsPanel :key="`environments-${path}`" :path="path" :busy="busy" :run-action="runAction" @notice="emit('notice', $event)" />
    <GithubResourcesPanel :key="`resources-${path}`" :path="path" :busy="busy" :run-action="runAction" @notice="emit('notice', $event)" />
    <GithubSearchPanel :key="`search-${path}`" :path="path" :busy="busy" :run-action="runAction" />
    <GithubAdvancedPanel :key="`advanced-${path}`" :path="path" :busy="busy" :run-action="runAction" />
  </section>
</template>
