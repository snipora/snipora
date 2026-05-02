import {createSharedComposable} from "@vueuse/core";
import {computed} from "vue";
import {useRuntimeInfo} from "@/composables/useRuntimeInfo.ts";

export const useOsType = createSharedComposable(() => {
  const runtimeInfo = useRuntimeInfo();
  const osType = computed(() => runtimeInfo.value?.os);
  return {
    osType,
    isWindows: computed(() => osType.value === "windows"),
    isLinux: computed(() => osType.value === "linux"),
    isMacOS: computed(() => osType.value === "macos"),
  };
});
