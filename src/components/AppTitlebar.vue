<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";

function minimizeWindow() {
  void getCurrentWindow().minimize();
}

function toggleMaximizeWindow() {
  void getCurrentWindow().toggleMaximize();
}

function closeWindow() {
  void getCurrentWindow().close();
}
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-brand" data-tauri-drag-region>
      <span class="brand-mark">R</span>
      <strong>RepoRadar</strong>
    </div>
    <div class="window-controls">
      <button type="button" title="最小化" aria-label="最小化" @click.stop="minimizeWindow">−</button>
      <button type="button" title="最大化" aria-label="最大化" @click.stop="toggleMaximizeWindow">
        □
      </button>
      <button
        type="button"
        class="close-control"
        title="关闭"
        aria-label="关闭"
        @click.stop="closeWindow"
      >
        ×
      </button>
    </div>
  </div>
</template>
