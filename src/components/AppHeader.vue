<script setup lang="ts">
defineProps<{
  eyebrow: string;
  title: string;
  subtitle: string;
  searchPlaceholder: string;
  searchQuery: string;
  showOperations: boolean;
  activeOperationCount: number;
  scanning: boolean;
  busy: boolean;
}>();

const emit = defineEmits<{
  "update:searchQuery": [value: string];
  "toggle-operations": [];
  "stop-scan": [];
  "choose-scan-root": [];
  "choose-project": [];
}>();
</script>

<template>
  <header>
    <div>
      <p class="eyebrow">{{ eyebrow }}</p>
      <h1>{{ title }}</h1>
      <p class="subtitle">{{ subtitle }}</p>
    </div>
    <input
      :value="searchQuery"
      class="search-input"
      :placeholder="searchPlaceholder"
      aria-label="搜索项目"
      @input="emit('update:searchQuery', ($event.target as HTMLInputElement).value)"
    />
    <div class="header-actions">
      <button
        class="secondary-action"
        :aria-expanded="showOperations"
        @click="emit('toggle-operations')"
      >
        操作中心{{ activeOperationCount ? ` · ${activeOperationCount}` : "" }}
      </button>
      <button
        v-if="scanning"
        class="secondary-action cancel-action"
        @click="emit('stop-scan')"
      >
        取消扫描
      </button>
      <button
        v-else
        class="secondary-action"
        :disabled="busy"
        @click="emit('choose-scan-root')"
      >
        扫描目录
      </button>
      <button class="primary-action" :disabled="busy" @click="emit('choose-project')">
        <span>＋</span>
        添加项目
      </button>
    </div>
  </header>
</template>
