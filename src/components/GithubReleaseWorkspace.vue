<script setup lang="ts">
import type { GithubRelease } from "../api";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;
defineProps<{
  path: string;
  releases: readonly GithubRelease[];
  busy: boolean;
  runAction: RunAction;
  showHeading?: boolean;
}>();
const emit = defineEmits<{
  view: [release: GithubRelease];
  download: [tag: string];
}>();
</script>

<template>
  <section class="github-release-workspace">
    <h4 v-if="showHeading !== false">Releases · {{ releases.length }}</h4>
    <div v-for="release in releases" :key="release.tagName" class="github-row">
      <span>{{ release.tagName }}</span><strong>{{ release.name ?? release.tagName }}</strong>
      <div class="github-row-actions">
        <small>{{ release.isLatest ? "Latest" : release.isDraft ? "Draft" : release.isPrerelease ? "Prerelease" : "Release" }}</small>
        <button data-action="view-release" :disabled="busy" @click="emit('view', release)">详情</button>
        <button :disabled="busy" @click="emit('download', release.tagName)">下载</button>
      </div>
    </div>
  </section>
</template>
