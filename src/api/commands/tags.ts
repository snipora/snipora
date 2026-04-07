import {Tag} from "@/api/dto.ts";
import {invoke} from "@tauri-apps/api/core";

/**
 * return a list of all tags available
 */
export async function invokeGetAllTags(): Promise<Tag[]> {
  return invoke<Tag[]>("get_all_tags");
}

/**
 * delete a tag globally and from all snippets that use it
 */
export async function invokeDeleteTag(tag: string): Promise<void> {
  return invoke("delete_tag", {tag});
}
