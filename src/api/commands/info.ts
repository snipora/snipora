import {OsType} from "@/api/dto.ts";
import {invoke} from "@tauri-apps/api/core";

export async function invokeOsType(): Promise<OsType> {
  return await invoke<OsType>("os_type");
}
