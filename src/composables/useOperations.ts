import { computed, ref } from "vue";

type OperationState = "running" | "success" | "failed";

export interface OperationRecord {
  id: number;
  label: string;
  state: OperationState;
  startedAt: number;
  finishedAt?: number;
  error?: string;
}

export function useOperations() {
  const operations = ref<OperationRecord[]>([]);
  let nextOperationId = 1;

  const activeOperationCount = computed(
    () => operations.value.filter((operation) => operation.state === "running").length,
  );

  function beginOperation(label: string) {
    const operation: OperationRecord = {
      id: nextOperationId++,
      label,
      state: "running",
      startedAt: Date.now(),
    };
    operations.value.unshift(operation);
    operations.value = operations.value.slice(0, 50);
    return operation.id;
  }

  function finishOperation(
    id: number,
    state: Exclude<OperationState, "running">,
    cause?: unknown,
  ) {
    const operation = operations.value.find((item) => item.id === id);
    if (!operation) return;
    operation.state = state;
    operation.finishedAt = Date.now();
    operation.error = state === "failed" ? String(cause ?? "未知错误") : undefined;
  }

  function clearOperations() {
    operations.value = [];
  }

  return {
    operations,
    activeOperationCount,
    beginOperation,
    finishOperation,
    clearOperations,
  };
}
