import {createSharedComposable, useElementSize, watchThrottled} from "@vueuse/core";
import {invokePopupAdjustHeight} from "@/api/commands";


export const useSmartPopupHeight = createSharedComposable(() => {
  const { height: bodyHeight } = useElementSize(document.body);

  watchThrottled(bodyHeight, async (preferredHeight) => {
    await invokePopupAdjustHeight(preferredHeight);
  }, { throttle: 50 });
});
