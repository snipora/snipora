import { computedAsync, createSharedComposable } from "@vueuse/core";
import { invokeLogsLocation } from "@/api/commands/info.ts";

export const useLogsLocation = createSharedComposable(() => {
  return computedAsync(invokeLogsLocation);
});
