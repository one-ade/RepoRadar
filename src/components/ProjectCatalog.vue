<script setup lang="ts">
import type { Project } from "../api";

defineProps<{
  projects: Project[];
  visibleProjects: Project[];
  selectedId?: number;
  busy: boolean;
}>();

const emit = defineEmits<{
  rescan: [];
  select: [project: Project];
  favorite: [project: Project];
}>();
</script>

<template>
  <div class="project-heading">
    <div>
      <span class="section-label">项目雷达</span>
      <h2>
        {{
          projects.length
            ? `${visibleProjects.length}/${projects.length} 个项目`
            : "还没有项目"
        }}
      </h2>
    </div>
    <button
      v-if="projects.length"
      class="text-button"
      :disabled="busy"
      @click="emit('rescan')"
    >
      重新扫描
    </button>
  </div>

  <div v-if="visibleProjects.length" class="project-list">
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
        </span>
        <span class="tracked-label">已跟踪</span>
      </button>
      <button
        :class="['favorite-toggle', project.favorite && 'favorite']"
        :aria-label="project.favorite ? '取消收藏' : '收藏项目'"
        :disabled="busy"
        @click.stop="emit('favorite', project)"
      >
        {{ project.favorite ? "★" : "☆" }}
      </button>
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
</template>
