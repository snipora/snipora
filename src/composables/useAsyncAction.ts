import {ref, shallowReadonly, shallowRef} from "vue";

export function useAsyncAction<P extends unknown[], R, E extends Error>(
    action: (...args: P) => Promise<R>,
    options?: {
      ignoreReRuns?: boolean
      onSuccess?: (result: R) => void,
      onError?: (error: E) => void,
    },
) {
  const { ignoreReRuns = true, onSuccess, onError } = options ?? {};

  const isRunning = shallowRef(false);
  const lastResult = ref<R | undefined>();
  const lastError = ref<E | undefined>();

  async function invoke(...args: P): Promise<R | undefined> {
    if (ignoreReRuns && isRunning.value) return;
    lastResult.value = undefined;
    lastError.value = undefined;
    isRunning.value = true;
    try {
      const result = await action(...args);
      lastResult.value = result;
      onSuccess?.(result);
      return result;
    } catch (e) {
      const error = e instanceof Error ? e : new Error(`${e}`);
      lastError.value = error as E;
      onError?.(error as E);
      return undefined;
    } finally {
      isRunning.value = false;
    }
  }

  function reset() {
    lastResult.value = undefined;
    lastError.value = undefined;
  }

  return {
    invoke,
    reset,
    isRunning: shallowReadonly(isRunning),
    lastResult: shallowReadonly(lastResult),
    lastError: shallowReadonly(lastError),
  };
}
