import {createSharedComposable} from "@vueuse/core";
import {computed, onBeforeMount, ref} from "vue";
import {
  disable as disableAutostart,
  enable as enableAutostart,
  isEnabled as checkIsAutostartEnabled
} from "@tauri-apps/plugin-autostart";


export const useAutostart = createSharedComposable(() => {
  const isAutostartEnabled = ref(false);

  onBeforeMount(async () => {
    isAutostartEnabled.value = await checkIsAutostartEnabled();
  });

  async function changeAutostart(enable: boolean): Promise<void> {
    if (enable) {
      console.info("enabling autostart");
      await enableAutostart();
      isAutostartEnabled.value = true;
    } else {
      console.info("disabling autostart");
      await disableAutostart();
      isAutostartEnabled.value = false;
    }
  }

  return computed({
    get: () => isAutostartEnabled.value,
    set: (value) => changeAutostart(value!),
  });
});
