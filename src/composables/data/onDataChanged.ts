import {useTauriEventListener} from "@/composables/primitives";

export function onDataChanged(cb: () => void) {
  useTauriEventListener("data-changed", (_) => cb());
}
