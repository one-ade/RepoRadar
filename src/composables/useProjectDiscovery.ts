import { confirm } from "@tauri-apps/plugin-dialog";
import { ref } from "vue";

import {
  addProject,
  addScanRoot,
  cancelScan,
  getEnvironment,
  inspectProjectPath,
  listProjects,
  scanProjects,
  setProjectFavorite,
  setProjectTags,
  type EnvironmentStatus,
  type Project,
  type ScanProgress,
} from "../api";

type RunAction = (action: () => Promise<void>, label?: string) => Promise<void>;
type ChooseDirectory = (title: string) => Promise<string | null>;

export function useProjectDiscovery(
  runAction: RunAction,
  chooseDirectory: ChooseDirectory,
  notify: (message: string) => void,
  reportError: (message: string) => void,
) {
  const environment = ref<EnvironmentStatus | null>(null);
  const projects = ref<Project[]>([]);
  const selectedProject = ref<Project | null>(null);
  const loading = ref(true);
  const scanning = ref(false);
  const scanProgress = ref<ScanProgress | null>(null);

  async function refreshEnvironment() {
    loading.value = true;
    reportError("");
    try {
      environment.value = await getEnvironment();
    } catch (cause) {
      reportError(String(cause));
    } finally {
      loading.value = false;
    }
  }

  async function refreshProjects() {
    projects.value = await listProjects();
  }

  async function chooseProject() {
    const path = await chooseDirectory("选择 Git 项目");
    if (!path) return;
    await runAction(async () => {
      const inspection = await inspectProjectPath(path);
      let initialize = false;
      if (!inspection.repositoryKind) {
        initialize = await confirm(
          `${inspection.name} 不是 Git 项目，是否在该目录执行 git init？`,
          { title: "初始化 Git 项目", kind: "warning" },
        );
        if (!initialize) return;
      }
      const project = await addProject(path, initialize);
      await refreshProjects();
      notify(`已添加 ${project.name}`);
    });
  }

  async function runScan() {
    scanning.value = true;
    scanProgress.value = null;
    try {
      const summary = await scanProjects();
      projects.value = summary.projects;
      notify(
        summary.cancelled
          ? `扫描已取消：已发现 ${summary.found} 个项目`
          : `扫描完成：发现 ${summary.found} 个项目，跳过 ${summary.skipped} 个无权限目录`,
      );
    } finally {
      scanning.value = false;
    }
  }

  async function chooseScanRoot() {
    const path = await chooseDirectory("选择项目扫描目录");
    if (!path) return;
    await runAction(async () => {
      await addScanRoot(path);
      await runScan();
    });
  }

  async function rescan() {
    await runAction(runScan);
  }

  async function stopScan() {
    try {
      await cancelScan();
    } catch (cause) {
      reportError(String(cause));
    }
  }

  async function toggleFavorite(project: Project) {
    await runAction(async () => {
      await setProjectFavorite(project.id, !project.favorite);
      project.favorite = !project.favorite;
    });
  }

  async function updateTags(project: Project, tags: string[]) {
    let saved = false;
    await runAction(async () => {
      const updated = await setProjectTags(project.id, tags);
      projects.value = projects.value.map((item) => (item.id === updated.id ? updated : item));
      if (selectedProject.value?.id === updated.id) selectedProject.value = updated;
      saved = true;
    }, "更新项目标签");
    return saved;
  }

  return {
    environment,
    projects,
    selectedProject,
    loading,
    scanning,
    scanProgress,
    refreshEnvironment,
    refreshProjects,
    chooseProject,
    chooseScanRoot,
    rescan,
    stopScan,
    toggleFavorite,
    updateTags,
  };
}
