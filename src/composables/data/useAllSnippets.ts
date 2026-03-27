import {createSharedComposable, useThrottleFn} from "@vueuse/core";
import {onMounted, ref} from "vue";
import {invokeGetAllSnippets} from "@/api/commands";
import {onDataChanged} from "@/composables/onDataChanged.ts";
import {SnippetDto} from "@/api/dto.ts";

export const useAllSnippets = createSharedComposable(() => {
  const snippets = ref<SnippetDto[] | null>(null);

  async function fetchSnippets() {
    snippets.value = await invokeGetAllSnippets();
  }
  const fetchThrottled = useThrottleFn(fetchSnippets, 100);

  onMounted(fetchSnippets);
  onDataChanged(fetchThrottled);

  return {
    snippets,
  };
});
