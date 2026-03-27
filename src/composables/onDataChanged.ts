import {useTauriEventListener} from "@/composables/useTauriEventListener.ts";

export function onDataChanged(cb: () => void) {
  useTauriEventListener("data-changed", (_) => cb());
}
