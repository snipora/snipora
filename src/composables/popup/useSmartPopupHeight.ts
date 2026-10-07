import {createSharedComposable, useElementSize, watchThrottled} from "@vueuse/core";
import {invokePopupAdjustHeight} from "@/api/commands";


console.info(`devicePixelRatio: ${window.devicePixelRatio}`);


export const useSmartPopupHeight = createSharedComposable(() => {
  const { height: bodyHeight } = useElementSize(document.body);

  watchThrottled(bodyHeight, async (bodyHeight) => {
    const devicePixelRatio = window.devicePixelRatio ?? 1;
    const preferredPhysicalHeight = bodyHeight * devicePixelRatio;
    await invokePopupAdjustHeight(preferredPhysicalHeight);
  }, { throttle: 50 });
});
