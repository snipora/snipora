import {invoke} from "@tauri-apps/api/core";


/**
 * Hides the popup window
 */
export async function invokePopupHide(): Promise<void> {
  return await invoke<void>("popup_hide");
}


/**
 * Adjusts the window-height of the popup window with a preferred height.
 * The actual new height may differ.
 * @param preferredHeight preferred new height
 */
export async function invokePopupAdjustHeight(preferredHeight: number): Promise<void> {
  return await invoke<void>("popup_adjust_height", { preferredHeight });
}
