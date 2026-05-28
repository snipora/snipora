import {invoke} from "@tauri-apps/api/core";

/**
 * Delete a tag globally and from all snippets that use it.
 */
export async function invokeDeleteTag(tag: string): Promise<void> {
  return invoke("delete_tag", { tag });
}

/**
 * Rename a tag. The new name must not already exist.
 */
export async function invokeRenameTag(tag: string, newName: string): Promise<void> {
  return invoke("rename_tag", { tag, newName });
}

/**
 * Merge all snippets from `sourceTag` into `targetTag`, then delete `sourceTag`.
 * The `targetTag` must already exist.
 */
export async function invokeMergeTag(sourceTag: string, targetTag: string): Promise<void> {
  return invoke("merge_tag", { sourceTag, targetTag });
}
