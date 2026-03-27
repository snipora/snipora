import {createSharedComposable} from "@vueuse/core";
import {computed} from "vue";
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";

export const useUntaggedSnippets = createSharedComposable(() => {
  const {snippets} = useAllSnippets();

  const untaggedSnippets = computed(() => {
    if (!snippets.value) {
      return null;
    }
    return snippets.value.filter((s) => s.tags.length === 0);
  });

  return {
    untaggedSnippets,
  };
});
