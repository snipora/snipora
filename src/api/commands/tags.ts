import {Tag} from "@/api/dto.ts";
import {invoke} from "@tauri-apps/api/core";

/**
 * return a list of all tags available
 */
export async function invokeGetAllTags(): Promise<Tag[]> {
  return invoke<Tag[]>("get_all_snippets");
}
