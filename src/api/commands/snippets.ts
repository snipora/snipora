import {SnippetDto} from "@/api/dto.ts";
import {invoke} from "@tauri-apps/api/core";


/**
 * creates a new snippet
 * @param snippet new snippet data
 */
export async function invokeCreateSnippet(
    snippet: Pick<SnippetDto, "label" | "snippet" | "tags">,
): Promise<SnippetDto["id"]> {
  return invoke<SnippetDto["id"]>("create_snippet", snippet);
}

/**
 * update an existing snippet
 * @param snippet updated snippet data
 */
export async function invokeUpdateSnippet(
    snippet: Pick<SnippetDto, "id" | "label" | "snippet" | "tags">,
): Promise<void> {
  return invoke<void>("update_snippet", snippet);
}

/**
 * deletes an existing snippet
 * @param snippetId id of the snippet to delete
 */
export async function invokeDeleteSnippet(snippetId: SnippetDto["id"]): Promise<void> {
  return invoke<void>("delete_snippet", { snippetId });
}

/**
 * returns all snippets
 */
export async function invokeGetAllSnippets(): Promise<SnippetDto[]> {
  return invoke<SnippetDto[]>("get_all_snippets");
}

/**
 * get up to `limit` of recently used snippets
 * @param limit maximum number of snippets
 */
export async function invokeGetRecentSnippets(limit: number): Promise<SnippetDto[]> {
  return invoke<SnippetDto[]>("get_recent_snippets", { limit });
}

/**
 * returns all snippets that have no tag
 */
export async function invokeGetAllUntaggedSnippets(): Promise<SnippetDto[]> {
  return invoke<SnippetDto[]>("get_untagged_snippets");
}
