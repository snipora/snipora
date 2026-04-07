import {invoke} from "@tauri-apps/api/core";

/**
 * delete a tag globally and from all snippets that use it
 */
export async function invokeDeleteTag(tag: string): Promise<void> {
  return invoke("delete_tag", {tag});
}
