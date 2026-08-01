<script setup lang="ts">
import { reactive, watch } from "vue";

import type { GithubIssueEdit } from "../api";

const props = defineProps<{ title: string; body: string; busy: boolean }>();
const emit = defineEmits<{ save: [edit: GithubIssueEdit] }>();
type TextEditKey = Exclude<keyof GithubIssueEdit, "removeMilestone" | "removeType" | "removeParent">;

const textFields = [
  { key: "addAssignees", label: "添加负责人" }, { key: "removeAssignees", label: "移除负责人" },
  { key: "addLabels", label: "添加标签" }, { key: "removeLabels", label: "移除标签" },
  { key: "addProjects", label: "添加项目" }, { key: "removeProjects", label: "移除项目" },
  { key: "milestone", label: "设置里程碑" }, { key: "issueType", label: "设置 Issue 类型" },
  { key: "parent", label: "设置父 Issue" }, { key: "addSubIssues", label: "添加子 Issue" },
  { key: "removeSubIssues", label: "移除子 Issue" }, { key: "addBlockedBy", label: "添加被阻塞关系" },
  { key: "removeBlockedBy", label: "移除被阻塞关系" }, { key: "addBlocking", label: "添加阻塞关系" },
  { key: "removeBlocking", label: "移除阻塞关系" },
] as const satisfies readonly { key: TextEditKey; label: string }[];

const form = reactive<GithubIssueEdit>({
  title: null, body: null,
  addAssignees: null, removeAssignees: null, addLabels: null, removeLabels: null,
  addProjects: null, removeProjects: null, milestone: null, removeMilestone: false,
  issueType: null, removeType: false, parent: null, removeParent: false,
  addSubIssues: null, removeSubIssues: null, addBlockedBy: null, removeBlockedBy: null,
  addBlocking: null, removeBlocking: null,
});

watch(
  () => [props.title, props.body] as const,
  ([title, body]) => Object.assign(form, {
    title, body,
    addAssignees: null, removeAssignees: null, addLabels: null, removeLabels: null,
    addProjects: null, removeProjects: null, milestone: null, removeMilestone: false,
    issueType: null, removeType: false, parent: null, removeParent: false,
    addSubIssues: null, removeSubIssues: null, addBlockedBy: null, removeBlockedBy: null,
    addBlocking: null, removeBlocking: null,
  }),
  { immediate: true },
);

function removalSelected(key: TextEditKey) {
  if (key === "milestone") return form.removeMilestone;
  if (key === "issueType") return form.removeType;
  return key === "parent" && form.removeParent;
}
</script>

<template>
  <details class="github-issue-editor">
    <summary>编辑 Issue</summary>
    <div class="github-issue-edit-grid">
      <label>标题<input v-model="form.title" data-edit-field aria-label="Issue 标题" :disabled="busy" /></label>
      <label class="wide">正文<textarea v-model="form.body" data-edit-field aria-label="Issue 正文" :disabled="busy"></textarea></label>
      <label v-for="field in textFields" :key="field.key">
        {{ field.label }}
        <input
          v-model="form[field.key]" data-edit-field :aria-label="field.label" placeholder="逗号分隔"
          :disabled="busy || removalSelected(field.key)"
        />
      </label>
      <label class="toggle"><input v-model="form.removeMilestone" data-edit-field type="checkbox" :disabled="busy || Boolean(form.milestone)" />移除里程碑</label>
      <label class="toggle"><input v-model="form.removeType" data-edit-field type="checkbox" :disabled="busy || Boolean(form.issueType)" />移除 Issue 类型</label>
      <label class="toggle"><input v-model="form.removeParent" data-edit-field type="checkbox" :disabled="busy || Boolean(form.parent)" />移除父 Issue</label>
    </div>
    <button data-action="save-issue-edit" class="small-action commit-action" :disabled="busy" @click="emit('save', { ...form })">
      保存 Issue 修改
    </button>
  </details>
</template>
