import {ref} from "vue";

export function useAsyncAction<P extends unknown[], R, E = Error>(
    action: (...args: P) => Promise<R>,
    options?: {
      onSuccess?: (result: R) => void,
      onError?: (error: E) => void,
    },
) {
  const isRunning = ref(false);
  const lastResult = ref<R | undefined>();
  const lastError = ref<E | undefined>();

  async function invoke(...args: P): Promise<R | undefined> {
    lastResult.value = undefined;
    lastError.value = undefined;
    isRunning.value = true;
    try {
      const result = await action(...args);
      lastResult.value = result;
      options?.onSuccess?.(result);
      return result;
    } catch (error) {
      lastError.value = error as E;
      options?.onError?.(error as E);
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
    isRunning,
    lastResult,
    lastError,
  };
}
