import {createSharedComposable} from "@vueuse/core";
import {computed} from "vue";
import {useAllSnippets} from "./useAllSnippets.ts";

export const useAllTags = createSharedComposable(() => {
  const {snippets} = useAllSnippets();

  const tagCounts = computed(() => {
    if (snippets.value === undefined) return undefined;
    const counts = new Map<string, number>();
    for (const snippet of snippets.value) {
      for (const tag of snippet.tags) {
        counts.set(tag, (counts.get(tag) ?? 0) + 1);
      }
    }
    return counts;
  });

  const tags = computed(() => {
    if (tagCounts.value === undefined) return undefined;
    return Array.from(tagCounts.value.keys())
        .sort((a, b) => a.localeCompare(b));
  });

  return {
    tags,
    tagCounts,
  };
});
