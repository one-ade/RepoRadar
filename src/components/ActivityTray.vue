<script setup lang="ts">
import type { ScanProgress } from "../api";
import type { OperationRecord } from "../composables/useOperations";
import OperationsPanel from "./OperationsPanel.vue";
import ScanProgressBanner from "./ScanProgressBanner.vue";

defineProps<{
  operations: OperationRecord[];
  scanning: boolean;
  progress: ScanProgress | null;
}>();

const emit = defineEmits<{
  close: [];
  "stop-scan": [];
  clear: [];
}>();
</script>

<template>
  <aside id="activity-tray" class="activity-tray" aria-label="活动记录" aria-live="polite">
    <div class="activity-tray-heading">
      <div>
        <span class="section-label">ACTIVITY TRAY</span>
        <h2>活动</h2>
      </div>
      <button class="small-action" aria-label="关闭活动面板" @click="emit('close')">关闭</button>
    </div>
    <ScanProgressBanner v-if="scanning && progress" :progress="progress" />
    <p v-else-if="scanning" class="scan-progress-pending">扫描准备中…</p>
    <OperationsPanel
      :operations="operations"
      :scanning="scanning"
      @stop-scan="emit('stop-scan')"
      @clear="emit('clear')"
    />
  </aside>
</template>
