import {createSharedComposable} from "@vueuse/core";
import {defineShortcuts} from "@/composables/interaction";
import {Window} from "@tauri-apps/api/window";

export const useCloseWindowShortcut = createSharedComposable(() => {
    defineShortcuts({
        ctrl_q: async () => {
            await Window.getCurrent().close()
        },
    });
});
