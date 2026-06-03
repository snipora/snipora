import {createSharedComposable} from "@vueuse/core";
import {computed} from "vue";
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";
import sortOn from "sort-on";

export const useRecentSnippets = createSharedComposable(() => {
  const {snippets} = useAllSnippets();

  const recentSnippets = computed(() => {
    if (snippets.value === undefined) return undefined;

    return sortOn(
      snippets.value.filter((s) => s.lastUsedAt !== null),
      "-lastUsedAt",
    );
  });

  return {
    recentSnippets,
  };
});
