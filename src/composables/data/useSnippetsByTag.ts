import {computed, type MaybeRefOrGetter, toValue} from "vue";
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";

export function useSnippetsByTag(tagName: MaybeRefOrGetter<string>) {
  const {snippets: allSnippets} = useAllSnippets();

  const snippets = computed(() => {
    if (allSnippets.value === undefined) return undefined;
    const t = toValue(tagName);
    return allSnippets.value.filter(s => s.tags.includes(t));
  });

  return {
    snippets,
  };
}
