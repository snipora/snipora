import {invoke} from "@tauri-apps/api/core";
import {LocalSettingsDto} from "@/api/dto.ts";

export async function invokeFetchLocalSettings(): Promise<LocalSettingsDto> {
  return invoke<LocalSettingsDto>("fetch_local_settings");
}

export async function invokeUpdateLocalSettings(updatedSettings: DeepPartial<LocalSettingsDto>): Promise<void> {
  return invoke<void>("update_local_settings", { updatedSettings });
}
