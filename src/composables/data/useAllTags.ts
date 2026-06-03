import {createSharedComposable} from "@vueuse/core";
import {computed} from "vue";
import {useAllSnippets} from "./useAllSnippets.ts";
import sortOn from "sort-on";

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
    return sortOn([...tagCounts.value.keys()], "");
  });

  return {
    tags,
    tagCounts,
  };
});
