<script setup lang="ts">
import {ref} from "vue";
import {Dialog, DialogScrollContent, DialogTrigger} from "@/components/ui/dialog";
import {defineShortcuts} from "@/composables/defineShortcut.ts";
import {ComboboxRoot, ComboboxContent, ComboboxInput, ComboboxItem} from "reka-ui";
import {useSearchedSnippets} from "@/popup/composables/useSearchedSnippets.ts";
import {useViewState} from "@/main/views/useViewState.ts";
import {Spinner} from "@/components/ui/spinner";
import {LucideSearch} from "@lucide/vue";
import {whenever} from "@vueuse/core";
import {logicNot} from "@vueuse/math";

const open = ref(false);

defineShortcuts({
  "ctrl_k": () => {
    open.value = true;
  },
});

const { setViewState } = useViewState();

const searchTerm = ref("");

whenever(logicNot(open), () => searchTerm.value = "");

const { matches: searchedSnippets, isSearching } = useSearchedSnippets(searchTerm);

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
          @keydown.esc="open = false"
          class="bg-popover text-popover-foreground space-y-1 size-full overflow-clip rounded-lg p-1"
      >
        <div
            class="sticky top-0 z-10 bg-popover flex h-9 items-center gap-2 px-3 rounded-lg overflow-clip"
            :class="{ 'border-b': searchedSnippets.length }"
        >
          <Spinner v-if="isSearching" class="size-4 shrink-0 opacity-50" />
          <LucideSearch v-else class="size-4 shrink-0 opacity-50" />
          <ComboboxInput
              v-model.trim="searchTerm"
              class="flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-hidden placeholder:text-muted-foreground"
              :placeholder="$t('popup.input.placeholder')"
          />
          <div v-if="searchedSnippets.length" class="grid place-items-center text-sm opacity-50">
            {{ searchedSnippets.length }}
          </div>
        </div>
        <ComboboxContent v-if="searchedSnippets.length" class="scroll-py-1 overflow-x-hidden">
          <ComboboxItem
              v-for="snippet in searchedSnippets"
              :key="snippet.id"
              :value="null"
              class="data-highlighted:bg-accent data-highlighted:text-accent-foreground relative flex cursor-default items-center rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-disabled:pointer-events-none data-disabled:opacity-50 [&_svg]:pointer-events-none"
              @select.prevent="handleSelect(snippet.id)"
          >
            <div class="size-full">
              <h3 class="select-none text-lg font-semibold tracking-tight">
                {{ snippet.label }}
              </h3>
              <pre class="font-mono bg-secondary text-secondary-foreground px-2 py-1 rounded-md inset-shadow-xs overflow-hidden line-clamp-3">{{ snippet.snippet }}</pre>
            </div>
          </ComboboxItem>
        </ComboboxContent>
      </ComboboxRoot>
    </DialogScrollContent>
  </Dialog>
</template>
