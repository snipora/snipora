import {invoke} from "@tauri-apps/api/core";

export async function invokeSetLocale(locale: string): Promise<void> {
  return invoke<void>("set_locale", { locale });
}
