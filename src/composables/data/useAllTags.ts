import {createSharedComposable, useThrottleFn} from "@vueuse/core";
import {onMounted, ref} from "vue";
import {invokeGetAllTags} from "@/api/commands";
import {onDataChanged} from "@/composables/onDataChanged.ts";
import {Tag} from "@/api/dto.ts";

export const useAllTags = createSharedComposable(() => {
  const tags = ref<Tag[] | null>(null);

  async function fetchTags() {
    tags.value = await invokeGetAllTags();
  }
  const fetchThrottled = useThrottleFn(fetchTags, 100);

  onMounted(fetchTags);
  onDataChanged(fetchThrottled);

  return {
    tags,
  };
});
