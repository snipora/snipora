import {useUnsubscribable} from "@/composables/primitives";
import {EventCallback, EventName, listen} from "@tauri-apps/api/event";

export function useTauriEventListener<T = void>(event: EventName, handler: EventCallback<T>): void {
  useUnsubscribable(() => listen<T>(event, handler));
}
