<script setup lang="ts">
import {usePopupEscapeListener, useSmartPopupHeight} from "@/composables/popup";
import {computed, ref, useTemplateRef, watch} from "vue";
import {useAllTags, useRecentSnippets, useSearchedSnippets} from "@/composables/data";
import {invokePopupHide, invokeUseSnippet} from "@/api/commands";
import {useTauriEventListener} from "@/composables/primitives";
import {ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxRoot, useFilter} from "reka-ui";
import {LucideSearch, LucideTag} from "@lucide/vue";
import {Spinner} from "@/components/ui/spinner";
import {useColorMode, useLocalSettings} from "@/composables/settings";
import {useTriggerCompletion} from "@/composables/interaction";
import {stringToColor} from "@/lib/coloring.ts";

useColorMode();
useSmartPopupHeight();
usePopupEscapeListener();

const { tags: allTags, tagCounts } = useAllTags();
const showTagCounts = useLocalSettings("appearance.showTagCounts");

const searchTerm = ref("");
const queryInputEl = useTemplateRef("queryInputEl");
const inputRef = computed(() => queryInputEl.value?.$el as HTMLInputElement);

const { matches: searchedSnippets, isSearching, pause: pauseSearch, resume: resumeSearch } = useSearchedSnippets(searchTerm);
const { recentSnippets } = useRecentSnippets();
const displayedSnippets = computed(() => searchTerm.value.length ? searchedSnippets.value : recentSnippets.value?.slice(0, 5));

const { isCompleting, searchText: completionText, replace: complete } = useTriggerCompletion({ input: inputRef, triggers: ["@"] });

const { contains } = useFilter({ sensitivity: "base" });
const completionTags = computed(() =>
    !completionText.value.length
        ? allTags.value
        : allTags.value?.filter(tag => contains(tag, completionText.value))
);

watch(displayedSnippets, () => {
  document.documentElement.scrollTo({ top: 0, behavior: "instant" });
});

watch(isCompleting, (completing) => {
  if (completing) {
    pauseSearch();
  } else {
    resumeSearch();
  }
});

function handleComplete(tagName: string) {
  complete(`@${tagName}`);
}

async function handleSelect(snippetId: string) {
  await invokePopupHide();
  await invokeUseSnippet(snippetId);
  searchTerm.value = "";
}

useTauriEventListener("popup:prepare", () => {
  searchTerm.value = "";
});
useTauriEventListener("popup:focus-input", () => {
  inputRef.value?.focus();
});
</script>

<template>
  <ComboboxRoot
      :default-open="true"
      :open="true"
      @update:open=""
      :ignore-filter="true"
      :reset-search-term-on-blur="false"
      :reset-search-term-on-select="false"
      class="bg-popover text-popover-foreground border space-y-1 size-full overflow-clip rounded-md p-1"
  >
    <div class="sticky top-1 z-10 bg-popover flex h-9 items-center gap-2 border-b px-3 rounded-lg overflow-clip shadow-xs">
      <Spinner v-if="isSearching" class="size-4 shrink-0 opacity-50" />
      <LucideSearch v-else class="size-4 shrink-0 opacity-50" />
      <ComboboxInput
          ref="queryInputEl"
          v-model="searchTerm"
          class="flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50 placeholder:text-muted-foreground"
          :placeholder="$t('popup.input.placeholder')"
          @keydown.tab.prevent=""
          @keydown.shift.tab.prevent=""
      />
      <div v-if="searchTerm" class="grid place-items-center text-sm opacity-50">
        {{ searchedSnippets.length }}
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
            v-for="snippet in displayedSnippets"
            :key="snippet.id"
            :value="null"
            class="data-highlighted:bg-accent data-highlighted:text-accent-foreground relative cursor-pointer items-center rounded-sm px-2 py-1.5 text-sm outline-hidden data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none"
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
</template>
