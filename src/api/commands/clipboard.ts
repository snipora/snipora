import {SnippetDto} from "@/api/dto.ts";
import {invoke} from "@tauri-apps/api/core";

/**
 * inserts a snippet into the clipboard
 * @param snippetId id of the snippet to use
 */
export async function invokeInsertSnippet(snippetId: SnippetDto["id"]): Promise<void> {
  return invoke<void>("insert_snippet", { snippetId });
}
