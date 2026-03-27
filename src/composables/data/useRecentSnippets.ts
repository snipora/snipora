import {createSharedComposable} from "@vueuse/core";
import {computed} from "vue";
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";

export const useRecentSnippets = createSharedComposable(() => {
  const {snippets} = useAllSnippets();

  const recentSnippets = computed(() => {
    if (!snippets.value) {
      return null;
    }
    return [...snippets.value]
      .filter((s) => s.last_used_at !== null)
      .sort((a, b) => (b.last_used_at ?? 0) - (a.last_used_at ?? 0));
  });

  return {
    recentSnippets,
  };
});
