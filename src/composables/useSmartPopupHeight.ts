import {createSharedComposable, useElementSize, watchDebounced} from "@vueuse/core";
import {invokePopupAdjustHeight} from "@/api/commands";


export const useSmartPopupHeight = createSharedComposable(() => {
  const { height: documentHeight } = useElementSize(document.documentElement);

  watchDebounced(documentHeight, async (preferredHeight) => {
    await invokePopupAdjustHeight(preferredHeight);
  }, { debounce: 50 });
});
