import {createSharedComposable} from "@vueuse/core";
import {computed} from "vue";
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";

export const useRecentSnippets = createSharedComposable(() => {
  const {snippets} = useAllSnippets();

  const recentSnippets = computed(() => {
    if (snippets.value === undefined) return undefined;

    return [...snippets.value]
      .filter((s) => s.lastUsedAt !== null)
      .sort((a, b) => (b.lastUsedAt ?? 0) - (a.lastUsedAt ?? 0));
  });

  return {
    recentSnippets,
  };
});
