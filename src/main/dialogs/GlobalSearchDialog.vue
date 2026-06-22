<script setup lang="ts">
import {computed, ref, useTemplateRef, watch} from "vue";
import {Dialog, DialogScrollContent, DialogTrigger} from "@/components/ui/dialog";
import {defineShortcuts, useTriggerCompletion} from "@/composables/interaction";
import {ComboboxRoot, ComboboxContent, ComboboxInput, ComboboxItem, ComboboxEmpty, useFilter} from "reka-ui";
import {useSearchedSnippets} from "@/composables/data";
import {useViewState} from "@/main/views/useViewState.ts";
import {useAllTags} from "@/composables/data";
import {useLocalSettings} from "@/composables/settings";
import {Spinner} from "@/components/ui/spinner";
import {LucideSearch, LucideTag} from "@lucide/vue";
import {stringToColor} from "@/lib/coloring.ts";
import {whenever} from "@vueuse/core";
import {logicNot} from "@vueuse/math";

const { tags: allTags, tagCounts } = useAllTags();
const showTagCounts = useLocalSettings("appearance.showTagCounts");

const { setViewState } = useViewState();

const open = ref(false);

const searchTerm = ref("");
const queryInputEl = useTemplateRef("queryInputEl");
const inputRef = computed(() => queryInputEl.value?.$el as HTMLInputElement);

const { matches: matchedSnippets, isSearching, pause: pauseSearch, resume: resumeSearch } = useSearchedSnippets(searchTerm);

const { isCompleting, searchText: completionText, replace: complete } = useTriggerCompletion({ input: inputRef, triggers: ["@"] });

const { contains } = useFilter({ sensitivity: "base" });
const completionTags = computed(() =>
    !completionText.value.length
        ? allTags.value
        : allTags.value?.filter(tag => contains(tag, completionText.value))
);

whenever(logicNot(open), () => {
  searchTerm.value = ""
});

watch(matchedSnippets, () => {
  document.documentElement.scrollTo({ top: 0, behavior: "instant" });
});

watch(isCompleting, (completing) => {
  if (completing) {
    pauseSearch();
  } else {
    resumeSearch();
  }
});

defineShortcuts({
  ctrl_f: () => {
    open.value = true;
  },
  ctrl_k: () => {
    open.value = true;
  },
});

function handleComplete(tagName: string) {
  complete(`@${tagName}`);
}

async function handleSelect(snippetId: string) {
  setViewState({ id: "snippet-by-id", snippetId });
  open.value = false;
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger as-child>
      <slot />
    </DialogTrigger>
    <DialogScrollContent aria-describedby="undefined" class="p-0">
      <ComboboxRoot
          :default-open="true"
          :open="true"
          @update:open=""
          :ignore-filter="true"
          :reset-search-term-on-blur="false"
          :reset-search-term-on-select="false"
          @keydown.esc="open = false"
          class="bg-popover text-popover-foreground space-y-1 size-full overflow-hidden rounded-lg p-1"
      >
        <div
            class="sticky top-0 z-10 bg-popover flex h-9 items-center gap-2 px-3 rounded-lg overflow-clip"
            :class="{ 'border-b': searchTerm.length }"
        >
          <Spinner v-if="isSearching" class="size-4 shrink-0 opacity-50" />
          <LucideSearch v-else class="size-4 shrink-0 opacity-50" />
          <ComboboxInput
              ref="queryInputEl"
              v-model="searchTerm"
              class="flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-hidden placeholder:text-muted-foreground"
              :placeholder="$t('popup.input.placeholder')"
          />
          <div v-if="matchedSnippets.length" class="grid place-items-center text-sm opacity-50">
            {{ matchedSnippets.length }}
          </div>
        </div>
        <ComboboxContent class="scroll-py-1 overflow-x-hidden">
          <template v-if="isCompleting">
            <ComboboxItem
                v-for="tag in completionTags"
                :key="tag"
                :value="null"
                class="data-highlighted:bg-accent data-highlighted:text-accent-foreground flex items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0 capitalize cursor-pointer"
                @select.prevent="handleComplete(tag)"
                @mousedown.prevent
            >
              <LucideTag :style="{ fill: stringToColor(tag) }" />
              <span>
                {{ tag }}
              </span>
              <span v-if="showTagCounts" class="ml-auto text-xs text-muted-foreground">
                {{ tagCounts?.get(tag) }}
              </span>
            </ComboboxItem>
            <ComboboxEmpty class="text-center text-sm text-muted-foreground">
              No Tags
            </ComboboxEmpty>
          </template>
          <template v-else>
            <ComboboxItem
                v-for="snippet in matchedSnippets"
                :key="snippet.id"
                :value="null"
                class="data-highlighted:bg-accent data-highlighted:text-accent-foreground relative cursor-pointer items-center rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none"
                @select.prevent="handleSelect(snippet.id)"
            >
              <h3 class="text-lg font-semibold tracking-tight">
                {{ snippet.label }}
              </h3>
              <pre class="font-mono bg-secondary text-secondary-foreground px-2 py-1 rounded-md inset-shadow-xs shadow-xs overflow-hidden line-clamp-3">{{ snippet.snippet }}</pre>
            </ComboboxItem>
            <ComboboxEmpty v-if="searchTerm && !isSearching" class="text-center text-sm text-muted-foreground">
              No Snippets Found.
            </ComboboxEmpty>
          </template>
        </ComboboxContent>
      </ComboboxRoot>
    </DialogScrollContent>
  </Dialog>
</template>
