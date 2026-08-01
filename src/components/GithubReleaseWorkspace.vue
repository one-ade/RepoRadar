<script setup lang="ts">
import { confirm, open } from "@tauri-apps/plugin-dialog";
import { reactive, ref, shallowRef, watch } from "vue";

import {
  editGithubRelease,
  getReleaseDetail,
  uploadGithubReleaseAssets,
  type GithubRelease,
  type GithubReleaseDetail,
  type GithubReleaseEdit,
} from "../api";
import GithubDetailGroups from "./GithubDetailGroups.vue";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;
const props = defineProps<{
  path: string;
  releases: readonly GithubRelease[];
  busy: boolean;
  runAction: RunAction;
}>();
const emit = defineEmits<{
  download: [tag: string];
  refresh: [];
  notice: [message: string];
}>();
const selectedRelease = ref<GithubRelease | null>(null);
const detail = shallowRef<GithubReleaseDetail | null>(null);
const assetPaths = ref<string[]>([]);
const assetLabels = ref<string[]>([]);
const clobber = ref(false);
let requestId = 0;

const fieldGroups = [
  { label: "概览", names: ["tagName", "name", "body", "url", "author", "isDraft", "isPrerelease", "isImmutable", "createdAt", "publishedAt"] },
  { label: "构建与下载", names: ["targetCommitish", "assets", "tarballUrl", "zipballUrl", "uploadUrl"] },
  { label: "标识", names: ["databaseId", "id", "apiUrl"] },
] as const;

function emptyEdit(): GithubReleaseEdit {
  return {
    tag: null, title: null, notes: null, notesFile: null, discussionCategory: null,
    target: null, draft: null, latest: null, prerelease: null, verifyTag: false,
  };
}

const form = reactive<GithubReleaseEdit>(emptyEdit());

function clearRelease() {
  requestId += 1;
  selectedRelease.value = null;
  detail.value = null;
  assetPaths.value = [];
  assetLabels.value = [];
  clobber.value = false;
  Object.assign(form, emptyEdit());
}

async function viewRelease(release: GithubRelease) {
  const currentRequest = ++requestId;
  selectedRelease.value = release;
  detail.value = null;
  Object.assign(form, emptyEdit());
  await props.runAction(async () => {
    const loaded = await getReleaseDetail(props.path, release.tagName);
    if (currentRequest === requestId) detail.value = loaded;
  }, `加载 Release ${release.tagName}`);
}

async function chooseNotesFile() {
  const selection = await open({ multiple: false, directory: false, title: "选择 Release 说明文件" });
  if (typeof selection === "string") {
    form.notes = null;
    form.notesFile = selection;
  }
}

async function saveRelease() {
  if (!selectedRelease.value) return;
  const currentTag = selectedRelease.value.tagName;
  const nextTag = form.tag?.trim() || currentTag;
  await props.runAction(async () => {
    await editGithubRelease(props.path, currentTag, { ...form });
    detail.value = await getReleaseDetail(props.path, nextTag);
    selectedRelease.value = { ...selectedRelease.value!, tagName: nextTag };
    Object.assign(form, emptyEdit());
    emit("refresh");
    emit("notice", `Release ${nextTag} 已更新`);
  }, `更新 Release ${currentTag}`);
}

async function chooseAssets() {
  const selection = await open({ multiple: true, directory: false, title: "选择 Release 资源" });
  const paths = typeof selection === "string" ? [selection] : selection;
  if (paths?.length) {
    assetPaths.value = paths;
    assetLabels.value = paths.map(() => "");
  }
}

async function uploadAssets() {
  if (!selectedRelease.value || !assetPaths.value.length) return;
  if (clobber.value && !(await confirm("同名资源会先被删除；上传失败时原资源无法恢复。确定继续？", {
    title: "覆盖 Release 资源", kind: "warning",
  }))) return;
  const tag = selectedRelease.value.tagName;
  const files = assetPaths.value.map((path, index) => {
    const label = assetLabels.value[index]?.trim();
    return label ? `${path}#${label}` : path;
  });
  await props.runAction(async () => {
    await uploadGithubReleaseAssets(props.path, tag, files, clobber.value);
    detail.value = await getReleaseDetail(props.path, tag);
    assetPaths.value = [];
    assetLabels.value = [];
    clobber.value = false;
    emit("refresh");
    emit("notice", `Release ${tag} 资源已上传`);
  }, `上传 Release ${tag} 资源`);
}

watch(() => props.path, clearRelease);
</script>

<template>
  <section class="github-release-workspace">
    <h4>Releases · {{ releases.length }}</h4>
    <div v-for="release in releases.slice(0, 5)" :key="release.tagName" class="github-row">
      <span>{{ release.tagName }}</span><strong>{{ release.name ?? release.tagName }}</strong>
      <div class="github-row-actions">
        <small>{{ release.isLatest ? "Latest" : release.isDraft ? "Draft" : release.isPrerelease ? "Prerelease" : "Release" }}</small>
        <button data-action="view-release" :disabled="busy" @click="viewRelease(release)">详情</button>
        <button :disabled="busy" @click="emit('download', release.tagName)">下载</button>
      </div>
    </div>

    <div v-if="selectedRelease" class="github-pr-detail" aria-label="Release 详情">
      <header>
        <div><span class="section-label">RELEASE {{ selectedRelease.tagName }}</span><h4>{{ selectedRelease.name ?? selectedRelease.tagName }}</h4></div>
        <button class="github-pr-detail-close" aria-label="关闭 Release 详情" @click="clearRelease">×</button>
      </header>
      <p v-if="!detail" class="github-pr-detail-loading">正在加载完整详情…</p>
      <template v-else>
        <GithubDetailGroups :detail="detail" :groups="fieldGroups" />
        <details class="github-detail-editor">
          <summary>编辑 Release</summary>
          <div class="github-edit-grid">
            <label>新 Tag<input v-model="form.tag" data-release-edit-field aria-label="新 Release Tag" :disabled="busy" /></label>
            <label>标题<input v-model="form.title" data-release-edit-field aria-label="新 Release 标题" :disabled="busy" /></label>
            <label class="wide">说明<textarea v-model="form.notes" data-release-edit-field aria-label="新 Release 说明" :disabled="busy || Boolean(form.notesFile)" /></label>
            <label>说明文件<input v-model="form.notesFile" data-release-edit-field aria-label="Release 说明文件" :disabled="busy || Boolean(form.notes)" /></label>
            <button class="small-action field-action" :disabled="busy || Boolean(form.notes)" @click="chooseNotesFile">选择说明文件</button>
            <label>讨论分类<input v-model="form.discussionCategory" data-release-edit-field aria-label="Release 讨论分类" :disabled="busy" /></label>
            <label>目标分支<input v-model="form.target" data-release-edit-field aria-label="Release 目标分支" :disabled="busy" /></label>
            <label v-for="field in ([['draft', '草稿状态'], ['latest', 'Latest 状态'], ['prerelease', '预发布状态']] as const)" :key="field[0]">
              {{ field[1] }}
              <select v-model="form[field[0]]" data-release-edit-field :aria-label="field[1]" :disabled="busy">
                <option :value="null">不修改</option><option :value="true">是</option><option :value="false">否</option>
              </select>
            </label>
            <label class="toggle"><input v-model="form.verifyTag" data-release-edit-field type="checkbox" aria-label="验证 Release Tag" :disabled="busy" />验证 Git Tag</label>
          </div>
          <button data-action="save-release-edit" class="small-action commit-action" :disabled="busy" @click="saveRelease">保存 Release 修改</button>
        </details>

        <details class="github-detail-editor">
          <summary>上传资源</summary>
          <button data-action="choose-release-assets" class="small-action" :disabled="busy" @click="chooseAssets">选择资源文件</button>
          <div v-if="assetPaths.length" class="github-release-assets">
            <label v-for="(path, index) in assetPaths" :key="path">
              <span :title="path">{{ path }}</span>
              <input v-model="assetLabels[index]" :aria-label="`资源显示名称 ${index + 1}`" placeholder="显示名称（可选）" :disabled="busy" />
            </label>
            <label class="toggle"><input v-model="clobber" type="checkbox" aria-label="覆盖同名资源" :disabled="busy" />覆盖同名资源</label>
          </div>
          <button data-action="upload-release-assets" class="small-action commit-action" :disabled="busy || !assetPaths.length" @click="uploadAssets">上传资源</button>
        </details>
      </template>
    </div>
  </section>
</template>
