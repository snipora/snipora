import { onBeforeUnmount, watch } from "vue";

type UnsubscribeFnGetter = () => (undefined | null | UnsubscribeFn | Promise<UnsubscribeFn>)
type UnsubscribeFn = () => void;

export function useUnsubscribable(fnGetter: UnsubscribeFnGetter) {
  let unsubscribeFn: UnsubscribeFn | undefined | null = null;

  function cleanup(): void {
    unsubscribeFn?.();
    unsubscribeFn = null;
  }

  watch(fnGetter, async (fnOrPromise, _old, onCleanup) => {
    unsubscribeFn = (fnOrPromise instanceof Promise) ? await fnOrPromise : fnOrPromise;
    onCleanup(cleanup);
  }, { immediate: true });

  onBeforeUnmount(cleanup);
}
