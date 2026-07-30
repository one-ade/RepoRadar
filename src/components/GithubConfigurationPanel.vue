<script setup lang="ts">
import { confirm } from "@tauri-apps/plugin-dialog";
import { ref } from "vue";
import {
  checkGithubRulesets,
  deleteGithubLabel,
  deleteGithubSecret,
  deleteGithubVariable,
  getGithubConfiguration,
  saveGithubLabel,
  setGithubSecret,
  setGithubVariable,
  type GithubConfiguration,
} from "../api";

interface Props {
  readonly path: string;
  readonly busy: boolean;
  readonly runAction: (action: () => Promise<void>, label?: string) => Promise<void>;
}

const props = defineProps<Props>();
const emit = defineEmits<{ notice: [message: string] }>();

const configuration = ref<GithubConfiguration | null>(null);
const rulesetReport = ref("");
const variableName = ref("");
const variableValue = ref("");
const secretName = ref("");
const secretValue = ref("");
const labelName = ref("");
const labelColor = ref("#7c3aed");
const labelDescription = ref("");

async function refreshConfiguration() {
  configuration.value = await getGithubConfiguration(props.path);
}

async function loadConfiguration() {
  await props.runAction(refreshConfiguration, "加载 GitHub 变量与密钥");
}

async function inspectRulesets() {
  await props.runAction(async () => {
    rulesetReport.value = await checkGithubRulesets(props.path);
  }, "检查 GitHub Ruleset");
}

async function saveVariable() {
  const name = variableName.value.trim();
  const value = variableValue.value;
  await props.runAction(async () => {
    await setGithubVariable(props.path, name, value);
    variableName.value = "";
    variableValue.value = "";
    await refreshConfiguration();
    emit("notice", "GitHub 变量已保存");
  }, `保存变量 ${name}`);
}

async function removeVariable(name: string) {
  const approved = await confirm(`删除变量 ${name}？`, { title: "删除 GitHub 变量", kind: "warning" });
  if (!approved) return;
  await props.runAction(async () => {
    await deleteGithubVariable(props.path, name);
    await refreshConfiguration();
    emit("notice", `GitHub 变量 ${name} 已删除`);
  }, `删除变量 ${name}`);
}

async function saveSecret() {
  const name = secretName.value.trim();
  const value = secretValue.value;
  secretValue.value = "";
  await props.runAction(async () => {
    await setGithubSecret(props.path, name, value);
    secretName.value = "";
    await refreshConfiguration();
    emit("notice", `GitHub 密钥 ${name} 已保存`);
  }, `保存密钥 ${name}`);
}

async function removeSecret(name: string) {
  const approved = await confirm(`删除密钥 ${name}？`, { title: "删除 GitHub 密钥", kind: "warning" });
  if (!approved) return;
  await props.runAction(async () => {
    await deleteGithubSecret(props.path, name);
    await refreshConfiguration();
    emit("notice", `GitHub 密钥 ${name} 已删除`);
  }, `删除密钥 ${name}`);
}

async function saveLabel() {
  const name = labelName.value.trim();
  const description = labelDescription.value.trim();
  await props.runAction(async () => {
    await saveGithubLabel(props.path, name, labelColor.value, description);
    labelName.value = "";
    labelDescription.value = "";
    await refreshConfiguration();
    emit("notice", "GitHub 标签已保存");
  }, `保存标签 ${name}`);
}

async function removeLabel(name: string) {
  const approved = await confirm(`删除标签 ${name}？`, { title: "删除 GitHub 标签", kind: "warning" });
  if (!approved) return;
  await props.runAction(async () => {
    await deleteGithubLabel(props.path, name);
    await refreshConfiguration();
    emit("notice", `GitHub 标签 ${name} 已删除`);
  }, `删除标签 ${name}`);
}
</script>

<template>
  <section class="github-configuration">
    <div class="github-configuration-heading">
      <h4>仓库变量、密钥与标签</h4>
      <div class="github-row-actions">
        <button :disabled="busy" @click="inspectRulesets">检查 Ruleset</button>
        <button
          data-testid="load-github-configuration"
          :disabled="busy"
          @click="loadConfiguration"
        >
          {{ configuration ? "刷新配置" : "加载配置" }}
        </button>
      </div>
    </div>

    <template v-if="configuration">
      <div class="github-config-compose">
        <input v-model="variableName" placeholder="变量名" aria-label="GitHub 变量名" />
        <input v-model="variableValue" placeholder="变量值" aria-label="GitHub 变量值" />
        <button class="small-action commit-action" :disabled="busy || !variableName || !variableValue" @click="saveVariable">
          保存变量
        </button>
      </div>

      <div class="github-config-compose">
        <input v-model="secretName" placeholder="密钥名" aria-label="GitHub 密钥名" />
        <input
          v-model="secretValue"
          type="password"
          autocomplete="new-password"
          placeholder="密钥值（不会回显）"
          aria-label="GitHub 密钥值"
        />
        <button class="small-action commit-action" :disabled="busy || !secretName || !secretValue" @click="saveSecret">
          保存密钥
        </button>
      </div>

      <div class="github-config-compose label-compose">
        <input v-model="labelName" placeholder="标签名" aria-label="GitHub 标签名" />
        <input v-model="labelColor" type="color" aria-label="GitHub 标签颜色" />
        <input v-model="labelDescription" placeholder="标签描述（可选）" aria-label="GitHub 标签描述" />
        <button class="small-action commit-action" :disabled="busy || !labelName" @click="saveLabel">
          保存标签
        </button>
      </div>

      <div class="github-columns">
        <section>
          <h4>Variables · {{ configuration.variables.length }}</h4>
          <div v-for="item in configuration.variables" :key="item.name" class="github-row">
            <span>{{ item.name }}</span>
            <strong>{{ item.value }}</strong>
            <div class="github-row-actions">
              <button :disabled="busy" @click="removeVariable(item.name)">删除</button>
            </div>
          </div>
          <p v-if="!configuration.variables.length" class="clean-state">暂无变量</p>
        </section>

        <section>
          <h4>Secrets · {{ configuration.secrets.length }}</h4>
          <div v-for="item in configuration.secrets" :key="item.name" class="github-row">
            <span>{{ item.name }}</span>
            <strong>值不可见</strong>
            <div class="github-row-actions">
              <button :disabled="busy" @click="removeSecret(item.name)">删除</button>
            </div>
          </div>
          <p v-if="!configuration.secrets.length" class="clean-state">暂无密钥</p>
        </section>

        <section>
          <h4>Labels · {{ configuration.labels.length }}</h4>
          <div v-for="item in configuration.labels" :key="item.name" class="github-row">
            <span
              class="label-swatch"
              :style="{ backgroundColor: `#${item.color ?? '7c3aed'}` }"
              aria-hidden="true"
            ></span>
            <strong :title="item.description ?? undefined">{{ item.name }}</strong>
            <div class="github-row-actions">
              <button :disabled="busy" @click="removeLabel(item.name)">删除</button>
            </div>
          </div>
          <p v-if="!configuration.labels.length" class="clean-state">暂无标签</p>
        </section>
      </div>
    </template>

    <p v-else>按需加载，避免权限不足影响仓库总览；密钥值不会显示或保存。</p>
    <pre v-if="rulesetReport" class="github-log">{{ rulesetReport }}</pre>
  </section>
</template>
