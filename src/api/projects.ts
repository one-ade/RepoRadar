import { invoke } from "@tauri-apps/api/core";

import type { PathInspection, Project, ScanSummary } from "./types";

export function listProjects(): Promise<Project[]> {
  return invoke<Project[]>("list_projects");
}

export function inspectProjectPath(path: string): Promise<PathInspection> {
  return invoke<PathInspection>("inspect_project_path", { path });
}

export function addProject(path: string, initialize: boolean): Promise<Project> {
  return invoke<Project>("add_project", { path, initialize });
}

export function addScanRoot(path: string, maxDepth = 8): Promise<void> {
  return invoke<void>("add_scan_root", { path, maxDepth });
}

export function scanProjects(): Promise<ScanSummary> {
  return invoke<ScanSummary>("scan_projects");
}

export function cancelScan(): Promise<void> {
  return invoke<void>("cancel_scan");
}

export function setProjectFavorite(id: number, favorite: boolean): Promise<void> {
  return invoke<void>("set_project_favorite", { id, favorite });
}
