import {computed, MaybeRefOrGetter, toValue} from "vue";
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";

export function useSnippetById(snippetId: MaybeRefOrGetter<string>) {
  const {snippets} = useAllSnippets();

  const snippet = computed(() => {
    if (snippets.value === undefined) return undefined;
    const sId = toValue(snippetId);
    return snippets.value.find(s => s.id === sId) ?? null;
  });

  return {
    snippet,
  };
}
