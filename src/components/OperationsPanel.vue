<script setup lang="ts">
import type { OperationRecord } from "../composables/useOperations";

defineProps<{
  operations: OperationRecord[];
  scanning: boolean;
}>();

const emit = defineEmits<{
  "stop-scan": [];
  clear: [];
}>();

function operationTime(timestamp: number) {
  return new Date(timestamp).toLocaleTimeString();
}
</script>

<template>
  <section class="operations-panel" aria-live="polite">
    <div class="operations-heading">
      <div>
        <span class="section-label">OPERATIONS</span>
        <h2>操作记录</h2>
      </div>
      <div class="operations-actions">
        <button
          v-if="scanning"
          class="text-button cancel-text"
          @click="emit('stop-scan')"
        >
          取消扫描
        </button>
        <button class="text-button" :disabled="!operations.length" @click="emit('clear')">
          清空
        </button>
      </div>
    </div>
    <div v-if="operations.length" class="operation-list">
      <div v-for="operation in operations.slice(0, 12)" :key="operation.id" class="operation-row">
        <span :class="['operation-state', operation.state]"></span>
        <div>
          <strong>{{ operation.label }}</strong>
          <small>
            {{ operationTime(operation.startedAt) }}
            <template v-if="operation.finishedAt">
              ·
              {{
                operation.state === "success"
                  ? "完成"
                  : operation.state === "failed"
                    ? "失败"
                    : "进行中"
              }}
            </template>
            <template v-else> · 进行中</template>
          </small>
        </div>
        <small v-if="operation.error" class="operation-error">{{ operation.error }}</small>
      </div>
    </div>
    <p v-else class="clean-state">暂无操作记录</p>
    <p class="operation-note">仅记录动作名称与结果，不保存命令参数、Token 或仓库内容。</p>
  </section>
</template>
