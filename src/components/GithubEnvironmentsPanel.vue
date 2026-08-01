<script setup lang="ts">
import { confirm } from "@tauri-apps/plugin-dialog";
import { ref, shallowRef, watch } from "vue";

import {
  deleteGithubEnvironment,
  deleteGithubEnvironmentSecret,
  deleteGithubEnvironmentVariable,
  getGithubEnvironmentConfiguration,
  getGithubEnvironments,
  saveGithubEnvironment,
  setGithubEnvironmentSecret,
  setGithubEnvironmentVariable,
  type GithubEnvironment,
  type GithubEnvironmentConfiguration,
} from "../api";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;
const props = defineProps<{ path: string; busy: boolean; runAction: RunAction }>();
const emit = defineEmits<{ notice: [message: string] }>();
const environments = shallowRef<GithubEnvironment[] | null>(null);
const selectedEnvironment = ref("");
const configuration = shallowRef<GithubEnvironmentConfiguration | null>(null);
const environmentName = ref("");
const variableName = ref("");
const variableValue = ref("");
const secretName = ref("");
const secretValue = ref("");
let requestId = 0;

function reset() {
  requestId += 1;
  environments.value = null;
  selectedEnvironment.value = "";
  configuration.value = null;
}

async function loadEnvironments() {
  await props.runAction(async () => {
    environments.value = await getGithubEnvironments(props.path);
  }, "加载部署环境");
}

async function viewEnvironment(name: string) {
  const currentRequest = ++requestId;
  selectedEnvironment.value = name;
  configuration.value = null;
  await props.runAction(async () => {
    const loaded = await getGithubEnvironmentConfiguration(props.path, name);
    if (currentRequest === requestId) configuration.value = loaded;
  }, `加载部署环境 ${name}`);
}

async function refreshConfiguration() {
  configuration.value = await getGithubEnvironmentConfiguration(
    props.path,
    selectedEnvironment.value,
  );
}

async function createEnvironment() {
  const name = environmentName.value.trim();
  if (!name) return;
  await props.runAction(async () => {
    await saveGithubEnvironment(props.path, name);
    environmentName.value = "";
    environments.value = await getGithubEnvironments(props.path);
    emit("notice", `部署环境 ${name} 已保存`);
  }, `保存部署环境 ${name}`);
}

async function removeEnvironment(name: string) {
  if (!(await confirm(`确定删除部署环境 ${name}？`, { title: "删除部署环境", kind: "warning" }))) return;
  await props.runAction(async () => {
    await deleteGithubEnvironment(props.path, name);
    if (selectedEnvironment.value === name) {
      selectedEnvironment.value = "";
      configuration.value = null;
    }
    environments.value = await getGithubEnvironments(props.path);
    emit("notice", `部署环境 ${name} 已删除`);
  }, `删除部署环境 ${name}`);
}

async function saveVariable() {
  if (!selectedEnvironment.value) return;
  const name = variableName.value;
  await props.runAction(async () => {
    await setGithubEnvironmentVariable(props.path, selectedEnvironment.value, name, variableValue.value);
    variableName.value = "";
    variableValue.value = "";
    await refreshConfiguration();
    emit("notice", `环境变量 ${name} 已保存`);
  }, `保存环境变量 ${name}`);
}

async function saveSecret() {
  if (!selectedEnvironment.value) return;
  const name = secretName.value;
  const value = secretValue.value;
  secretValue.value = "";
  await props.runAction(async () => {
    await setGithubEnvironmentSecret(props.path, selectedEnvironment.value, name, value);
    secretName.value = "";
    await refreshConfiguration();
    emit("notice", `环境密钥 ${name} 已保存`);
  }, `保存环境密钥 ${name}`);
}

async function removeScoped(kind: "variable" | "secret", name: string) {
  const label = kind === "variable" ? "变量" : "密钥";
  if (!(await confirm(`确定删除环境${label} ${name}？`, { title: `删除环境${label}`, kind: "warning" }))) return;
  await props.runAction(async () => {
    const remove = kind === "variable" ? deleteGithubEnvironmentVariable : deleteGithubEnvironmentSecret;
    await remove(props.path, selectedEnvironment.value, name);
    await refreshConfiguration();
    emit("notice", `环境${label} ${name} 已删除`);
  }, `删除环境${label} ${name}`);
}

watch(() => props.path, reset);
</script>

<template>
  <details class="github-configuration github-environments">
    <summary><span>部署环境</span><small>Environment · Variable · Secret</small></summary>
    <button data-action="load-environments" class="small-action" :disabled="busy" @click="loadEnvironments">
      {{ environments ? "刷新部署环境" : "加载部署环境" }}
    </button>
    <template v-if="environments">
      <div class="github-config-compose">
        <input v-model="environmentName" aria-label="新部署环境名称" placeholder="环境名称" />
        <button data-action="save-environment" class="small-action commit-action" :disabled="busy || !environmentName.trim()" @click="createEnvironment">创建 / 更新环境</button>
      </div>
      <div class="github-columns">
        <section>
          <h4>Environments · {{ environments.length }}</h4>
          <div v-for="environment in environments" :key="environment.id" class="github-row">
            <span>{{ environment.protectionRules.length }} rules</span><strong>{{ environment.name }}</strong>
            <div class="github-row-actions">
              <small>{{ environment.updatedAt }}</small>
              <button data-action="view-environment" :disabled="busy" @click="viewEnvironment(environment.name)">配置</button>
              <button :disabled="busy" @click="removeEnvironment(environment.name)">删除</button>
            </div>
          </div>
        </section>
        <section v-if="selectedEnvironment">
          <h4>{{ selectedEnvironment }}</h4>
          <p v-if="!configuration">正在加载环境配置…</p>
          <template v-else>
            <div class="github-config-compose compact">
              <input v-model="variableName" aria-label="环境变量名" placeholder="变量名" />
              <input v-model="variableValue" aria-label="环境变量值" placeholder="变量值" />
              <button class="small-action" :disabled="busy" @click="saveVariable">保存变量</button>
              <input v-model="secretName" aria-label="环境密钥名" placeholder="密钥名" />
              <input v-model="secretValue" aria-label="环境密钥值" type="password" placeholder="密钥值" autocomplete="new-password" />
              <button data-action="save-environment-secret" class="small-action" :disabled="busy" @click="saveSecret">保存密钥</button>
            </div>
            <div class="github-environment-values">
              <div v-for="variable in configuration.variables" :key="`variable-${variable.name}`" class="github-row">
                <span>Variable</span><strong>{{ variable.name }}</strong><div class="github-row-actions"><small>{{ variable.value }}</small><button :disabled="busy" @click="removeScoped('variable', variable.name)">删除</button></div>
              </div>
              <div v-for="secret in configuration.secrets" :key="`secret-${secret.name}`" class="github-row">
                <span>Secret</span><strong>{{ secret.name }}</strong><div class="github-row-actions"><small>值不可见</small><button :disabled="busy" @click="removeScoped('secret', secret.name)">删除</button></div>
              </div>
            </div>
          </template>
        </section>
      </div>
    </template>
  </details>
</template>
