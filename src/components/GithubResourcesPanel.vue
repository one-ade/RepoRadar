<script setup lang="ts">
import { confirm } from "@tauri-apps/plugin-dialog";
import { ref, shallowRef, watch } from "vue";

import {
  deleteGithubCodespace,
  getGithubCodespaceLog,
  getGithubCodespaces,
  getGithubDiscussions,
  getGithubProjectItems,
  getGithubProjects,
  stopGithubCodespace,
  type GithubCodespace,
  type GithubDetailValue,
  type GithubDiscussion,
  type GithubProject,
} from "../api";
import { externalUrl } from "../externalUrl";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;
const props = defineProps<{ path: string; busy: boolean; runAction: RunAction }>();
const emit = defineEmits<{ notice: [message: string] }>();
const projects = shallowRef<GithubProject[] | null>(null);
const discussions = shallowRef<GithubDiscussion[] | null>(null);
const codespaces = shallowRef<GithubCodespace[] | null>(null);
const projectItems = shallowRef<GithubDetailValue | null>(null);
const projectQuery = ref("");
const codespaceLog = ref("");

async function loadProjects() {
  await props.runAction(async () => { projects.value = await getGithubProjects(props.path); }, "加载 GitHub Projects");
}

async function loadDiscussions() {
  await props.runAction(async () => { discussions.value = await getGithubDiscussions(props.path); }, "加载 GitHub Discussions");
}

async function loadCodespaces() {
  await props.runAction(async () => { codespaces.value = await getGithubCodespaces(props.path); }, "加载 GitHub Codespaces");
}

async function viewProjectItems(number: number) {
  await props.runAction(async () => {
    projectItems.value = await getGithubProjectItems(props.path, number, projectQuery.value);
  }, `加载 Project #${number} 项`);
}

async function viewCodespaceLog(name: string) {
  await props.runAction(async () => {
    codespaceLog.value = await getGithubCodespaceLog(props.path, name);
  }, `加载 Codespace ${name} 日志`);
}

async function stopCodespace(name: string) {
  await props.runAction(async () => {
    await stopGithubCodespace(props.path, name);
    codespaces.value = await getGithubCodespaces(props.path);
    emit("notice", `Codespace ${name} 已停止`);
  }, `停止 Codespace ${name}`);
}

async function removeCodespace(name: string) {
  const approved = await confirm(
    `确定永久删除 Codespace ${name}？未推送和未保存的内容会丢失。`,
    { title: "删除 Codespace", kind: "warning" },
  );
  if (!approved) return;
  await props.runAction(async () => {
    await deleteGithubCodespace(props.path, name, true);
    codespaces.value = await getGithubCodespaces(props.path);
    codespaceLog.value = "";
    emit("notice", `Codespace ${name} 已删除`);
  }, `删除 Codespace ${name}`);
}

watch(() => props.path, () => {
  projects.value = null;
  discussions.value = null;
  codespaces.value = null;
  projectItems.value = null;
  codespaceLog.value = "";
});
</script>

<template>
  <details class="github-configuration github-resources">
    <summary><span>GitHub Resources</span><small>Projects · Discussions · Codespaces</small></summary>
    <div class="github-columns">
      <section>
        <div class="github-resource-heading">
          <h4>Projects{{ projects ? ` · ${projects.length}` : "" }}</h4>
          <button data-action="load-projects" class="small-action" :disabled="busy" @click="loadProjects">{{ projects ? "刷新" : "加载" }}</button>
        </div>
        <input v-model="projectQuery" class="github-resource-filter" aria-label="Project 项过滤条件" placeholder="项过滤条件（可选）" />
        <div v-for="project in projects ?? []" :key="project.id" class="github-row">
          <span>#{{ project.number }}</span>
          <strong><a v-if="externalUrl(project.url)" :href="externalUrl(project.url)" target="_blank" rel="noreferrer">{{ project.title }}</a><span v-else>{{ project.title }}</span></strong>
          <div class="github-row-actions">
            <small>{{ project.items.totalCount }} items · {{ project.fields.totalCount }} fields</small>
            <button data-action="view-project-items" :disabled="busy" @click="viewProjectItems(project.number)">项目项</button>
          </div>
        </div>
        <pre v-if="projectItems" class="github-resource-json">{{ JSON.stringify(projectItems, null, 2) }}</pre>
      </section>

      <section>
        <div class="github-resource-heading">
          <h4>Discussions{{ discussions ? ` · ${discussions.length}` : "" }}</h4>
          <button data-action="load-discussions" class="small-action" :disabled="busy" @click="loadDiscussions">{{ discussions ? "刷新" : "加载" }}</button>
        </div>
        <div v-for="discussion in discussions ?? []" :key="discussion.id" class="github-row">
          <span>#{{ discussion.number }}</span>
          <strong><a v-if="externalUrl(discussion.url)" :href="externalUrl(discussion.url)" target="_blank" rel="noreferrer">{{ discussion.title }}</a><span v-else>{{ discussion.title }}</span></strong>
          <div class="github-row-actions"><small>{{ discussion.category.name }} · {{ discussion.comments.totalCount }} comments</small><span>{{ discussion.isAnswered ? "已回答" : "讨论中" }}</span></div>
        </div>
      </section>

      <section class="github-resource-wide">
        <div class="github-resource-heading">
          <h4>Codespaces{{ codespaces ? ` · ${codespaces.length}` : "" }}</h4>
          <button data-action="load-codespaces" class="small-action" :disabled="busy" @click="loadCodespaces">{{ codespaces ? "刷新" : "加载" }}</button>
        </div>
        <div v-for="codespace in codespaces ?? []" :key="codespace.name" class="github-row">
          <span>{{ codespace.state }}</span><strong>{{ codespace.name }}</strong>
          <div class="github-row-actions">
            <small>{{ codespace.machineName }}</small>
            <button data-action="codespace-log" :disabled="busy" @click="viewCodespaceLog(codespace.name)">日志</button>
            <button :disabled="busy || codespace.state !== 'Available'" @click="stopCodespace(codespace.name)">停止</button>
            <button data-action="delete-codespace" :disabled="busy" @click="removeCodespace(codespace.name)">删除</button>
          </div>
        </div>
        <pre v-if="codespaceLog" class="github-log">{{ codespaceLog }}</pre>
      </section>
    </div>
  </details>
</template>
