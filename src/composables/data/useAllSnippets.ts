import {createSharedComposable, useThrottleFn} from "@vueuse/core";
import {onMounted, ref} from "vue";
import {invokeGetAllSnippets} from "@/api/commands";
import {onDataChanged} from "@/composables/data";
import {SnippetDto} from "@/api/dto.ts";
import sortOn from "sort-on";

export const useAllSnippets = createSharedComposable(() => {
  const snippets = ref<SnippetDto[] | undefined>(undefined);

  async function fetchSnippets() {
    snippets.value = sortOn(await invokeGetAllSnippets(), "-updatedAt");
  }
  const fetchThrottled = useThrottleFn(fetchSnippets, 100);

  onMounted(fetchSnippets);
  onDataChanged(fetchThrottled);

  return {
    snippets,
  };
});
