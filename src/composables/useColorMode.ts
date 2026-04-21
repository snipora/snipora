import {useLocalSettings} from "@/composables/useLocalSettings.ts";
import {createSharedComposable, usePreferredDark} from "@vueuse/core";
import {computed, watch} from "vue";

export const useColorMode = createSharedComposable(() => {
  const colorTheme = useLocalSettings("ui.theme");

  const preferredDark = usePreferredDark();
  const systemTheme = computed(() => preferredDark.value ? "dark" : "light");
  const state = computed(() => colorTheme.value === "system" ? systemTheme.value : colorTheme.value);

  watch(state, (cls) => {
    const el = window.document.querySelector("html");
    if (!el) return;

    if (cls === "light") {
      el.classList.add("light");
      el.classList.remove("dark");
    } else {
      el.classList.add("dark");
      el.classList.remove("light");
    }
  }, { immediate: true });

  return { colorTheme, systemTheme, state };
});
