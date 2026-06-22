import {Ref, ref} from "vue";
import {watchDebounced} from "@vueuse/core";
import {invokeSearchSnippets} from "@/api/commands";
import {SnippetDto} from "@/api/dto.ts";

// todo: add error handling and returning
export function useSearchedSnippets(queryRef: Ref<string>) {
  const matches = ref<SnippetDto[]>([]);
  const isSearching = ref(false);

  const { pause, resume } = watchDebounced(queryRef, async (query) => {
    if (!query.length) {
      matches.value = [];
      isSearching.value = false;
      return;
    }

    isSearching.value = true;
    try {
      const result = await invokeSearchSnippets(query);
      if (query !== queryRef.value) return;
      matches.value = result;
    } catch (e) {
      console.error(e);
    } finally {
      if (query === queryRef.value) {
        isSearching.value = false;
      }
    }
  }, { debounce: 50, immediate: true });

  return { matches, isSearching, pause, resume };
}
