<script setup lang="ts">
import {useVModel} from "@vueuse/core";
import {Popover, PopoverAnchor, PopoverContent, PopoverTrigger} from "@/components/ui/popover";
import {computed, ref, watch} from "vue";
import {ListboxContent, ListboxFilter, ListboxItem, ListboxItemIndicator, ListboxRoot, useFilter} from "reka-ui";
import {
  TagsInput,
  TagsInputInput,
  TagsInputItem,
  TagsInputItemDelete,
  TagsInputItemText
} from "@/components/ui/tags-input";
import {stringToColor} from "@/lib/coloring.ts";
import {Button} from "@/components/ui/button";
import {LucideChevronDown, LucideCheck, LucideTag} from "@lucide/vue";
import {useAllTags} from "@/composables/data/useAllTags.ts";

const props = defineProps<{
  defaultValue?: string[]
  modelValue?: string[]
  placeholder?: string
}>();

const emits = defineEmits<{
  (e: "update:modelValue", payload: string[]): void
}>();

const modelValue = useVModel(props, "modelValue", emits, {
  passive: true,
  defaultValue: props.defaultValue,
});

const { tags: knownTags } = useAllTags();
const popoverOpen = ref(false);
const searchTerm = ref("");

const { contains } = useFilter({ sensitivity: "base" });

const filteredTags = computed(() =>
  searchTerm.value === ""
    ? knownTags.value
    : knownTags.value?.filter(tag => contains(tag, searchTerm.value))
);

watch(searchTerm, (s) => {
  if (s) {
    popoverOpen.value = true;
  }
});
</script>

<template>
  <Popover v-model:open="popoverOpen">
    <ListboxRoot
        v-model="modelValue"
        highlight-on-hover
        multiple
    >
      <PopoverAnchor as-child>
        <TagsInput v-model="modelValue" class="w-full">
          <TagsInputItem v-for="tag in modelValue" :key="tag" :value="tag" class="capitalize" :style="{ backgroundColor: stringToColor(tag) }">
            <TagsInputItemText />
            <TagsInputItemDelete />
          </TagsInputItem>

          <ListboxFilter v-model="searchTerm" as-child>
            <TagsInputInput :placeholder="placeholder" @keydown.enter="searchTerm = ''" @keydown.down="popoverOpen = true" />
          </ListboxFilter>

          <PopoverTrigger as-child>
            <Button size="icon-sm" variant="ghost" class="order-last self-start ml-auto size-5">
              <LucideChevronDown />
            </Button>
          </PopoverTrigger>
        </TagsInput>
      </PopoverAnchor>
      <PopoverContent class="p-1 w-(--reka-popper-anchor-width)" @open-auto-focus.prevent>
        <ListboxContent class="max-h-80 scroll-py-1 overflow-x-hidden empty:p-1" tabindex="0">
          <span v-if="!filteredTags?.length" class="text-muted-foreground">
            {{ $t('components.tags-input-with-completion.no-tags') }}
          </span>
          <ListboxItem
              v-for="tag in filteredTags"
              :key="tag"
              :value="tag"
              class="data-highlighted:bg-accent data-highlighted:text-accent-foreground [&_svg:not([class*=\'text-\'])]:text-muted-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*=\'size-\'])]:size-4"
              @select="searchTerm = ''"
          >
            <LucideTag :style="{ fill: stringToColor(`${tag}`) }" />
            <span class="capitalize">
              {{ tag }}
            </span>
            <ListboxItemIndicator class="ml-auto inline-flex items-center justify-center">
              <LucideCheck />
            </ListboxItemIndicator>
          </ListboxItem>
        </ListboxContent>
      </PopoverContent>
    </ListboxRoot>
  </Popover>
</template>
