import {createSharedComposable, useElementSize, watchThrottled} from "@vueuse/core";
import {invokePopupAdjustHeight} from "@/api/commands";


export const useSmartPopupHeight = createSharedComposable(() => {
  const { height: documentHeight } = useElementSize(document.documentElement);

  watchThrottled(documentHeight, async (preferredHeight) => {
    await invokePopupAdjustHeight(preferredHeight);
  }, { throttle: 50 });
});
