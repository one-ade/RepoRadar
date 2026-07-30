import { invoke } from "@tauri-apps/api/core";

import type { EnvironmentStatus } from "./types";

export function getEnvironment(): Promise<EnvironmentStatus> {
  return invoke<EnvironmentStatus>("get_environment");
}
