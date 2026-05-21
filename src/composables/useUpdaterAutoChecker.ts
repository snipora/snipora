import { watch } from "vue";
import { createSharedComposable, useIntervalFn } from "@vueuse/core";
import { useUpdater } from "@/composables/useUpdater";
import { useLocalSettings } from "@/composables/useLocalSettings";

const UPDATE_CHECK_INTERVAL = 6/*h*/ * 60/*min*/ * 60/*sec*/ * 1000/*ms*/;  // = every 6h

export const useUpdaterAutoChecker = createSharedComposable(() => {
  const { checkForUpdate } = useUpdater();
  const autoCheckForUpdates = useLocalSettings("general.autoCheckForUpdates");

  const { pause: pauseAutoChecks, resume: resumeAutoChecks } = useIntervalFn(checkForUpdate, UPDATE_CHECK_INTERVAL, {
    immediate: autoCheckForUpdates.value,
    immediateCallback: true,
  });

  watch(autoCheckForUpdates, (shouldCheck) => {
    if (shouldCheck) {
      resumeAutoChecks();
    } else {
      pauseAutoChecks();
    }
  });
});
