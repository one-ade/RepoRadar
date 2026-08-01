<script setup lang="ts">
import { computed } from "vue";

import type { GithubDetailField, GithubDetailValue, GithubPullRequestDetail } from "../api";

const props = defineProps<{
  detail: GithubPullRequestDetail;
  groups: readonly { label: string; names: readonly string[] }[];
}>();

const groupedFields = computed(() => {
  const remaining = new Map(props.detail.fields.map((field) => [field.name, field]));
  const groups: Array<{ label: string; fields: GithubDetailField[] }> = [];
  for (const group of props.groups) {
    const fields = group.names.flatMap((name) => {
      const field = remaining.get(name);
      if (!field) return [];
      remaining.delete(name);
      return [field];
    });
    if (fields.length) groups.push({ label: group.label, fields });
  }
  if (remaining.size) groups.push({ label: "其他", fields: [...remaining.values()] });
  return groups;
});

function isComplex(value: GithubDetailValue) {
  return value !== null && typeof value === "object";
}

function scalarText(value: GithubDetailValue) {
  if (value === null || value === "") return "—";
  if (typeof value === "boolean") return value ? "是" : "否";
  return typeof value === "object" ? "" : String(value);
}
</script>

<template>
  <div class="github-pr-detail-groups">
    <section v-for="group in groupedFields" :key="group.label">
      <h5>{{ group.label }}</h5>
      <dl>
        <div v-for="field in group.fields" :key="field.name" class="github-pr-detail-field">
          <dt>{{ field.name }}</dt>
          <dd>
            <details v-if="isComplex(field.value)">
              <summary>{{ Array.isArray(field.value) ? `${field.value.length} 项` : "展开 JSON" }}</summary>
              <pre>{{ JSON.stringify(field.value, null, 2) }}</pre>
            </details>
            <span v-else>{{ scalarText(field.value) }}</span>
          </dd>
        </div>
      </dl>
    </section>
  </div>
</template>
