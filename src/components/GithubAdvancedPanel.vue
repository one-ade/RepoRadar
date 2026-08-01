<script setup lang="ts">
import { confirm } from "@tauri-apps/plugin-dialog";
import { ref, watch } from "vue";

import { runGithubApiRequest, runSafeGithubCommand } from "../api";

type ApiMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";
type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;
const props = defineProps<{ path: string; busy: boolean; runAction: RunAction }>();
const command = ref("repo-view");
const extraArgs = ref("");
const method = ref<ApiMethod>("GET");
const endpoint = ref("repos/{owner}/{repo}");
const fields = ref("");
const output = ref("");

async function runCommand() {
  const args = extraArgs.value.split(/\r?\n/).map((value) => value.trim()).filter(Boolean);
  await props.runAction(async () => {
    output.value = await runSafeGithubCommand(props.path, command.value, args);
  }, `运行只读 gh ${command.value}`);
}

function parseFields() {
  return fields.value.split(/\r?\n/).map((line) => line.trim()).filter(Boolean).map((line) => {
    const separator = line.indexOf("=");
    return separator < 0
      ? { key: line, value: "" }
      : { key: line.slice(0, separator), value: line.slice(separator + 1) };
  });
}

async function runApi() {
  if (method.value !== "GET" && !(await confirm(
    `${method.value} ${endpoint.value} 可能修改 GitHub 数据，确定继续？`,
    { title: "执行 GitHub API 写请求", kind: "warning" },
  ))) return;
  await props.runAction(async () => {
    output.value = await runGithubApiRequest(
      props.path, method.value, endpoint.value, parseFields(),
    );
  }, `GitHub API ${method.value}`);
}

watch(() => props.path, () => { output.value = ""; });
</script>

<template>
  <details class="github-configuration github-advanced">
    <summary><span>高级 GitHub 工具</span><small>只读 gh · API Request Builder</small></summary>
    <div class="github-advanced-grid">
      <section>
        <h4>安全只读 gh</h4>
        <select v-model="command" aria-label="只读 gh 命令" :disabled="busy">
          <option value="repo-view">repo view</option><option value="pr-list">pr list</option>
          <option value="issue-list">issue list</option><option value="run-list">run list</option>
          <option value="workflow-list">workflow list</option><option value="release-list">release list</option>
          <option value="label-list">label list</option><option value="variable-list">variable list</option>
          <option value="secret-list">secret list</option><option value="ruleset-list">ruleset list</option>
        </select>
        <textarea v-model="extraArgs" aria-label="只读 gh 参数" placeholder="额外参数，每行一个；不经过 shell" :disabled="busy"></textarea>
        <button data-action="run-safe-gh" class="small-action" :disabled="busy" @click="runCommand">运行只读命令</button>
      </section>
      <section>
        <h4>gh api</h4>
        <div class="github-api-endpoint">
          <select v-model="method" aria-label="GitHub API 方法" :disabled="busy"><option>GET</option><option>POST</option><option>PUT</option><option>PATCH</option><option>DELETE</option></select>
          <input v-model="endpoint" aria-label="GitHub API Endpoint" placeholder="相对 API 路径" :disabled="busy" />
        </div>
        <textarea v-model="fields" aria-label="GitHub API 字段" placeholder="key=value，每行一个；值按字面量传递" :disabled="busy"></textarea>
        <button data-action="run-gh-api" class="small-action commit-action" :disabled="busy || !endpoint.trim()" @click="runApi">执行 API 请求</button>
      </section>
    </div>
    <pre v-if="output" class="github-resource-json">{{ output }}</pre>
  </details>
</template>
