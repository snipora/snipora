import {computedAsync, createSharedComposable} from "@vueuse/core";
import {invokeRuntimeInfo} from "@/api/commands/info.ts";

export const useRuntimeInfo = createSharedComposable(() => {
  return computedAsync(invokeRuntimeInfo);
});
