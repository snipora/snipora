<script setup lang="ts">
import {useSmartPopupHeight} from "@/popup/composables/useSmartPopupHeight.ts";
import {usePopupEscapeListener} from "@/popup/composables/usePopupEscapeListener.ts";
import {computed, ref, useTemplateRef, watch} from "vue";
import {useSearchedSnippets} from "@/popup/composables/useSearchedSnippets.ts";
import {useRecentSnippets} from "@/composables/data/useRecentSnippets.ts";
import {invokeInsertSnippet, invokePopupHide} from "@/api/commands";
import {useTauriEventListener} from "@/composables/useTauriEventListener.ts";
import {ComboboxRoot, ComboboxInput, ComboboxContent, ComboboxItem} from "reka-ui";
import {LucideSearch} from "@lucide/vue";
import {Spinner} from "@/components/ui/spinner";
import {useColorMode} from "@/composables/useColorMode.ts";

useColorMode();
useSmartPopupHeight();
usePopupEscapeListener();

const queryInputEl = useTemplateRef("queryInputEl");
const searchTerm = ref("");
const { matches: searchedSnippets, isSearching } = useSearchedSnippets(searchTerm);
const { recentSnippets } = useRecentSnippets();
const displayedSnippets = computed(() => searchTerm.value.length ? searchedSnippets.value : recentSnippets.value?.slice(0, 5));

watch(displayedSnippets, () => {
  document.documentElement.scrollTo({ top: 0, behavior: "instant" });
})

async function handleSelect(snippetId: string) {
  await invokeInsertSnippet(snippetId);
  searchTerm.value = "";
  await invokePopupHide();
}

useTauriEventListener("popup:prepare", () => {
  searchTerm.value = "";
});
useTauriEventListener("popup:focus-input", () => {
  queryInputEl.value?.$el.focus();
});
</script>

<template>
  <ComboboxRoot
      :default-open="true"
      :open="true"
      @update:open=""
      :ignore-filter="true"
      :reset-search-term-on-blur="false"
      :reset_search-term-on-select="false"
      class="bg-popover text-popover-foreground space-y-1 size-full overflow-clip rounded-md p-1"
  >
    <div class="sticky top-1 z-10 bg-popover flex h-9 items-center gap-2 border-b px-3 rounded-lg overflow-clip shadow-xs">
      <Spinner v-if="isSearching" class="size-4 shrink-0 opacity-50" />
      <LucideSearch v-else class="size-4 shrink-0 opacity-50" />
      <ComboboxInput
          ref="queryInputEl"
          v-model.trim="searchTerm"
          class="flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-50 placeholder:text-muted-foreground"
          :placeholder="$t('popup.input.placeholder')"
          @keydown.tab.prevent=""
          @keydown.shift.tab.prevent=""
      />
      <div v-if="searchedSnippets.length" class="grid place-items-center text-sm opacity-50">
        {{ searchedSnippets.length }}
      </div>
    </div>
    <ComboboxContent class="scroll-py-1 overflow-x-hidden">
      <ComboboxItem
          v-for="snippet in displayedSnippets"
          :key="snippet.id"
          :value="null"
          class="data-highlighted:bg-accent data-highlighted:text-accent-foreground relative flex cursor-default items-center rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none"
          @select.prevent="handleSelect(snippet.id)"
      >
        <div class="size-full">
          <h3 class="select-none text-lg font-semibold tracking-tight">
            {{ snippet.label }}
          </h3>
          <pre class="font-mono bg-secondary text-secondary-foreground px-2 py-1 rounded-md inset-shadow-xs shadow-xs overflow-hidden line-clamp-3">{{ snippet.snippet }}</pre>
        </div>
      </ComboboxItem>
    </ComboboxContent>
  </ComboboxRoot>
</template>
