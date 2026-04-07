import {Ref, ref} from "vue";
import {watchDebounced} from "@vueuse/core";
import {invokeSearchSnippets} from "@/api/commands";
import {SnippetDto} from "@/api/dto.ts";

export function useSearchedSnippets(queryRef: Ref<string>) {
  const matches = ref<SnippetDto[]>([]);
  const isSearching = ref(false);

  watchDebounced(queryRef, async (query) => {
    if (!query.length) {
      matches.value = [];
      isSearching.value = false;
      return;
    } else {
      isSearching.value = true;
      let result: SnippetDto[];
      try {
        result = await invokeSearchSnippets(query);
      } catch (e) {
        isSearching.value = false;
        console.error(e);
        throw e;
      }
      if (query !== queryRef.value) return;
      matches.value = result;
      isSearching.value = false;
    }
  }, { debounce: 300, immediate: true });

  return { matches, isSearching };
}
