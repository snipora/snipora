import { createSharedComposable } from "@vueuse/core";
import { computed, readonly, ref, toRaw } from "vue";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { invokeRuntimeInfo } from "@/api/commands/info";

export type UpdateStatus =
  | "idle"
  | "checking"
  | "available"
  | "downloading"
  | "ready"
  | "installing"
  | "installed";

export const useUpdater = createSharedComposable(() => {
  const status = ref<UpdateStatus>("idle");
  const update = ref<Update | null>(null);
  const downloadedBytes = ref(0);
  const totalBytes = ref(0);
  const error = ref<string | null>(null);

  const isChecking = computed(() => status.value === "checking");
  const isDownloading = computed(() => status.value === "downloading");
  const isInstalling = computed(() => status.value === "installing");

  const progress = computed(() => {
    if (totalBytes.value === 0) return NaN;
    return (downloadedBytes.value / totalBytes.value) * 100;
  });

  async function checkForUpdate(): Promise<Update | null> {
    if (isChecking.value) return null;
    console.info("[updater] Checking for update...");
    status.value = "checking";
    error.value = null;
    try {
      const runtimeInfo = await invokeRuntimeInfo();
      const result = await check({
        target: `${runtimeInfo.os}-${runtimeInfo.arch}-${runtimeInfo.bundleType}`,
      });
      update.value = result;
      status.value = result !== null ? "available" : "idle";
      console.info("[updater] Check result:", result ? `${result.version} available` : "no update");
      return result;
    } catch (e) {
      status.value = "idle";
      error.value = e instanceof Error ? e.message : String(e);
      console.error("[updater] Check failed:", error.value);
      return null;
    }
  }

  async function downloadUpdate(): Promise<void> {
    if (!update.value || status.value !== "available") return;
    console.info("[updater] Downloading update...");
    status.value = "downloading";
    downloadedBytes.value = 0;
    totalBytes.value = 0;
    error.value = null;
    try {
      // ! Required work around. Otherwise, we get an TypeError
      const downloadFn = toRaw(update.value).download;
      await downloadFn((event) => {
        switch (event.event) {
          case "Started":
            totalBytes.value = event.data.contentLength ?? 0;
            console.info("[updater] Download started, total bytes:", totalBytes.value);
            break;
          case "Progress":
            downloadedBytes.value += event.data.chunkLength;
            break;
        }
      });
      status.value = "ready";
      console.info("[updater] Download complete");
    } catch (e) {
      status.value = "available";
      error.value = e instanceof Error ? e.message : String(e);
      console.error("[updater] Download failed:", error.value);
    }
  }

  async function installUpdate(): Promise<void> {
    if (!update.value || status.value !== "ready") return;
    console.info("[updater] Installing update...");
    status.value = "installing";
    error.value = null;
    try {
      // ! Required work around. Otherwise, we get an TypeError
      const installFn = toRaw(update.value).install;
      await installFn();
      status.value = "installed";
      console.info("[updater] Update installed");
    } catch (e) {
      status.value = "ready";
      error.value = e instanceof Error ? e.message : String(e);
      console.error("[updater] Install failed:", error.value);
      throw e;
    }
  }

  async function relaunchApp(): Promise<void> {
    console.info("[updater] Relaunching app...");
    await relaunch();
  }

  function reset(): void {
    console.info("[updater] Reset to idle");
    status.value = "idle";
    update.value = null;
    downloadedBytes.value = 0;
    totalBytes.value = 0;
    error.value = null;
  }

  return {
    status: readonly(status),
    update: readonly(update),
    downloadedBytes: readonly(downloadedBytes),
    totalBytes: readonly(totalBytes),
    progress: readonly(progress),
    error: readonly(error),
    isChecking,
    isDownloading,
    isInstalling,
    checkForUpdate,
    downloadUpdate,
    installUpdate,
    relaunchApp,
    reset,
  };
});
