import {SnippetDto} from "@/api/dto.ts";
import {invoke} from "@tauri-apps/api/core";

/**
 * use a snippet. e.g. copies snippet into the clipboard
 * @param snippetId id of the snippet to use
 */
export async function invokeUseSnippet(snippetId: SnippetDto["id"]): Promise<void> {
  return invoke<void>("use_snippet", { snippetId });
}
