import {useLocalSettings} from "@/composables/settings";
import {createSharedComposable, usePreferredDark} from "@vueuse/core";
import {computed, watch} from "vue";

export const useColorMode = createSharedComposable(() => {
  const colorTheme = useLocalSettings("appearance.uiTheme");

  const preferredDark = usePreferredDark();
  const systemTheme = computed(() => preferredDark.value ? "dark" : "light");
  const resolvedTheme = computed(() => colorTheme.value === "system" ? systemTheme.value : colorTheme.value);

  watch(resolvedTheme, (theme) => {
    const el = window.document.querySelector("html");
    if (!el) return;

    if (theme === "light") {
      el.classList.add("light");
      el.classList.remove("dark");
    } else {
      el.classList.add("dark");
      el.classList.remove("light");
    }
  }, { immediate: true });

  return { colorTheme, systemTheme };
});
