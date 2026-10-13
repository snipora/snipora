import { createSharedComposable } from "@vueuse/core";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { readonly, ref } from "vue";
import { useUnsubscribable } from "@/composables/primitives";

export const useIsWindowFocused = createSharedComposable(() => {
  const window = WebviewWindow.getCurrent();
  const isWindowFocused = ref<boolean>(false);

  useUnsubscribable(() => window.onFocusChanged((event) => {
    isWindowFocused.value = event.payload;
  }));

  return readonly(isWindowFocused);
});
