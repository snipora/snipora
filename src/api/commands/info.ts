import {RuntimeInfo} from "@/api/dto.ts";
import {invoke} from "@tauri-apps/api/core";

export async function invokeRuntimeInfo(): Promise<RuntimeInfo> {
  return await invoke<RuntimeInfo>("runtime_info");
}
