import {createSharedComposable, useElementSize, watchThrottled} from "@vueuse/core";
import {invokePopupAdjustHeight} from "@/api/commands";


export const useSmartPopupHeight = createSharedComposable(() => {
  const { height: documentHeight } = useElementSize(document.body);

  let lastPhysicalHeight = -1;

  watchThrottled(documentHeight, async () => {
    const rect = document.body.getBoundingClientRect();
    const physical = Math.ceil(rect.height * window.devicePixelRatio);
    if (physical <= 0 || physical === lastPhysicalHeight) return;
    lastPhysicalHeight = physical;
    await invokePopupAdjustHeight(physical);
  }, { throttle: 50 });
});