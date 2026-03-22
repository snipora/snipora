import {invoke} from "@tauri-apps/api/core";

export async function invokeSetTrayIcon(theme: "app" | "light" | "dark"): Promise<void> {
  return invoke<void>("set_tray_icon", { theme });
}
