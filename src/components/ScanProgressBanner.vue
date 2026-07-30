<script setup lang="ts">
import type { ScanProgress } from "../api";

defineProps<{ progress: ScanProgress }>();
</script>

<template>
  <section class="scan-progress" aria-live="polite">
    <div class="progress-heading">
      <span>扫描中 · {{ progress.scanned }} 个目录</span>
      <span>{{ progress.currentPath }}</span>
    </div>
    <div class="progress-track"><span></span></div>
  </section>
</template>
