import {inject, readonly} from "vue";
import {INJECTION_KEY_MAIN_VIEW, ViewState} from "@/main/views/index.ts";

export function useViewState() {
  const viewState = inject(INJECTION_KEY_MAIN_VIEW)!;

  function setViewState(newView: ViewState): void {
    viewState.value = newView;
  }

  return {
    viewState: readonly(viewState),
    setViewState,
  };
}