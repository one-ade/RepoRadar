<script setup lang="ts">
import { ref, shallowRef, watch } from "vue";

import { searchGithub, type GithubDetailValue } from "../api";

type SearchKind = "code" | "commits" | "issues" | "prs" | "repos";
type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;
const props = defineProps<{ path: string; busy: boolean; runAction: RunAction }>();
const kind = ref<SearchKind>("repos");
const query = ref("");
const currentRepository = ref(false);
const results = shallowRef<GithubDetailValue[] | null>(null);

async function runSearch() {
  await props.runAction(async () => {
    results.value = await searchGithub(
      props.path,
      kind.value,
      query.value,
      currentRepository.value,
    );
  }, `GitHub ${kind.value} 搜索`);
}

function record(value: GithubDetailValue): Readonly<Record<string, GithubDetailValue>> {
  return value && !Array.isArray(value) && typeof value === "object"
    ? value as Readonly<Record<string, GithubDetailValue>>
    : {};
}

function resultTitle(value: GithubDetailValue) {
  const item = record(value);
  return [item.fullName, item.title, item.path, item.sha, item.id]
    .find((field) => typeof field === "string") as string | undefined ?? "GitHub 搜索结果";
}

function resultUrl(value: GithubDetailValue) {
  const url = record(value).url;
  return typeof url === "string" ? url : "";
}

watch(() => props.path, () => { results.value = null; });
</script>

<template>
  <details class="github-configuration github-search">
    <summary><span>GitHub 全局搜索</span><small>Code · Commits · Issues · PRs · Repositories</small></summary>
    <form class="github-search-form" @submit.prevent="runSearch">
      <select v-model="kind" aria-label="GitHub 搜索类型" :disabled="busy">
        <option value="repos">Repositories</option><option value="code">Code</option>
        <option value="commits">Commits</option><option value="issues">Issues</option>
        <option value="prs">Pull Requests</option>
      </select>
      <input v-model="query" aria-label="GitHub 搜索条件" placeholder="关键词与 GitHub qualifiers" :disabled="busy" />
      <label><input v-model="currentRepository" type="checkbox" aria-label="仅当前仓库" :disabled="busy" />仅当前仓库</label>
      <button data-action="search-github" class="small-action commit-action" :disabled="busy || !query.trim()">搜索</button>
    </form>
    <p v-if="results && !results.length">没有匹配结果</p>
    <div v-if="results?.length" class="github-search-results">
      <details v-for="(result, index) in results" :key="`${resultTitle(result)}-${index}`">
        <summary>
          <a v-if="resultUrl(result)" :href="resultUrl(result)" target="_blank" rel="noreferrer">{{ resultTitle(result) }}</a>
          <span v-else>{{ resultTitle(result) }}</span>
        </summary>
        <pre>{{ JSON.stringify(result, null, 2) }}</pre>
      </details>
    </div>
  </details>
</template>
