import {computedAsync, createSharedComposable} from "@vueuse/core";
import {invokeOsType} from "@/api/commands/info.ts";
import {computed} from "vue";

export const useOsType = createSharedComposable(() => {
  const osType = computedAsync(invokeOsType)
  return {
    osType,
    isWindows: computed(() => osType.value === "windows"),
    isLinux: computed(() => osType.value === "linux"),
    isMacOS: computed(() => osType.value === "macos"),
    isIOS: computed(() => osType.value === "ios"),
    isAndroid: computed(() => osType.value === "android"),
  };
});
