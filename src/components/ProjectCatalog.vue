<script setup lang="ts">
import { ref } from "vue";
import type { Project } from "../api";

const props = defineProps<{
  projects: Project[];
  visibleProjects: Project[];
  loading: boolean;
  error?: string;
  selectedId?: number;
  busy: boolean;
  saveTags: (project: Project, tags: string[]) => Promise<boolean>;
  searchQuery?: string;
  scanning?: boolean;
}>();

const editingId = ref<number>();
const tagDraft = ref("");
const collapsed = ref(false);

function editTags(project: Project) {
  editingId.value = project.id;
  tagDraft.value = project.tags.join(", ");
}

function cancelTagEdit() {
  editingId.value = undefined;
}

async function submitTags(project: Project) {
  const tags = tagDraft.value
    .split(/[,，]/)
    .map((tag) => tag.trim())
    .filter(Boolean);
  if (await props.saveTags(project, tags)) cancelTagEdit();
}

const emit = defineEmits<{
  "update:searchQuery": [value: string];
  add: [];
  scan: [];
  "stop-scan": [];
  rescan: [];
  retry: [];
  select: [project: Project];
  favorite: [project: Project];
}>();
</script>

<template>
  <aside :class="['project-rail', collapsed && 'collapsed']" aria-label="项目栏">
  <div class="project-rail-topline">
    <div>
      <span class="section-label">PROJECT RAIL</span>
      <h2>仓库</h2>
    </div>
    <div class="project-rail-topline-actions">
      <span class="project-count">{{ projects.length }}</span>
      <button
        class="project-rail-collapse"
        type="button"
        :aria-expanded="!collapsed"
        aria-label="折叠项目栏"
        @click="collapsed = !collapsed"
      >
        {{ collapsed ? "展开" : "收起" }}
      </button>
    </div>
  </div>

  <label class="project-search">
    <span class="sr-only">搜索项目、路径或标签</span>
    <span class="project-search-icon">⌕</span>
    <input
      :value="searchQuery"
      placeholder="搜索项目、路径或标签…"
      @input="emit('update:searchQuery', ($event.target as HTMLInputElement).value)"
    />
  </label>

  <div class="project-rail-actions">
    <button class="primary-action" :disabled="busy" @click="emit('add')">＋ 添加</button>
    <button
      v-if="scanning"
      class="secondary-action cancel-action"
      :disabled="busy"
      @click="emit('stop-scan')"
    >
      取消扫描
    </button>
    <button v-else class="secondary-action" :disabled="busy" @click="emit('scan')">扫描目录</button>
  </div>

  <div class="project-heading">
    <div>
      <span class="section-label">项目雷达</span>
      <h2>
        {{ loading ? "正在加载项目" : error ? "项目加载失败" : projects.length ? `${visibleProjects.length}/${projects.length} 个项目` : "还没有项目" }}
      </h2>
    </div>
    <button
      v-if="projects.length && !loading && !error"
      class="text-button"
      :disabled="busy"
      @click="emit('rescan')"
    >
      重新扫描
    </button>
  </div>

  <div v-if="error" class="project-error-content" role="alert">
    <span class="detail-empty-mark">!</span>
    <div>
      <h2>项目加载失败</h2>
      <p>项目暂时不可用，请稍后重试。</p>
      <small class="project-error-message">{{ error }}</small>
      <button class="secondary-action retry-action" :disabled="busy || loading" @click="emit('retry')">
        重试加载
      </button>
    </div>
  </div>

  <div v-else-if="loading" class="project-skeleton" aria-label="正在加载项目" aria-busy="true">
    <span v-for="index in 4" :key="index" class="skeleton-row"></span>
  </div>

  <div v-else-if="visibleProjects.length" class="project-list">
    <div
      v-for="project in visibleProjects"
      :key="project.id"
      :class="['project-row', selectedId === project.id && 'selected']"
    >
      <button class="project-select" :disabled="busy" @click="emit('select', project)">
        <span class="repo-mark">R</span>
        <span class="project-copy">
          <strong>{{ project.name }}</strong>
          <small>{{ project.path }}</small>
          <span v-if="project.tags.length" class="project-tags">
            <span v-for="tag in project.tags" :key="tag" class="project-tag">{{ tag }}</span>
          </span>
        </span>
        <span class="tracked-label">已跟踪</span>
      </button>
      <span class="project-actions">
        <button
          class="small-action tag-edit"
          :aria-label="`编辑 ${project.name} 标签`"
          :disabled="busy"
          @click="editTags(project)"
        >
          标签
        </button>
        <button
          :class="['favorite-toggle', project.favorite && 'favorite']"
          :aria-label="project.favorite ? '取消收藏' : '收藏项目'"
          :disabled="busy"
          @click.stop="emit('favorite', project)"
        >
          {{ project.favorite ? "★" : "☆" }}
        </button>
      </span>
      <form v-if="editingId === project.id" class="tag-editor" @submit.prevent="submitTags(project)">
        <input
          v-model="tagDraft"
          aria-label="项目标签"
          maxlength="167"
          placeholder="rust, frontend, client"
          :disabled="busy"
        />
        <button class="small-action commit-action" :disabled="busy" type="submit">保存</button>
        <button class="small-action tag-cancel" :disabled="busy" type="button" @click="cancelTagEdit">
          取消
        </button>
        <small class="tag-hint">最多 8 个标签，每个不超过 20 个字符，以逗号分隔</small>
      </form>
    </div>
  </div>

  <div v-else class="empty-content">
    <div class="radar">
      <span></span>
      <span></span>
      <i></i>
    </div>
    <div>
      <h2>让雷达发现你的仓库</h2>
      <p>添加一个项目，或选择常用开发目录进行扫描。RepoRadar 不会跟随符号链接。</p>
    </div>
  </div>
  </aside>
</template>
