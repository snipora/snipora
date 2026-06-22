import {createSharedComposable, onKeyDown} from "@vueuse/core";
import {invokePopupHide} from "@/api/commands";


export const usePopupEscapeListener = createSharedComposable(() => {
  onKeyDown("Escape", async () => {
    await invokePopupHide();
  });
});
