<script setup lang="ts">
import {AppSidebar} from "@/main/sidebar";
import {SidebarProvider} from "@/components/ui/sidebar";
import {provide, useTemplateRef, watch} from "vue";
import {INJECTION_KEY_MAIN_VIEW, ViewState, VIEW_TO_COMPONENT} from "@/main/views";
import {ScrollArea} from "@/components/ui/scroll-area";
import {useLocalStorage} from "@vueuse/core";
import {useTauriEventListener} from "@/composables/primitives";
import {useColorMode} from "@/composables/settings";
import { Toaster } from "@/components/ui/sonner";
import { useUpdaterToasts, useUpdaterAutoChecker } from "@/composables/updater";

useColorMode();
useUpdaterToasts();
useUpdaterAutoChecker();

const viewState = useLocalStorage<ViewState>('view-state', () => ({
  id: "all-snippets",
}), { listenToStorageChanges: false });

provide(INJECTION_KEY_MAIN_VIEW, viewState);

useTauriEventListener<ViewState>("main:set-view-state", (event) => {
  viewState.value = event.payload;
});

const scrollAreaRef = useTemplateRef("scroll-area");

watch(viewState, () => {
  scrollAreaRef.value?.scrollTo({ top: 0, left: 0, behavior: "instant" });
}, { deep: false });
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <ScrollArea ref="scroll-area" class="h-svh w-full">
      <component :is="VIEW_TO_COMPONENT[viewState.id]" v-bind="viewState" />
    </ScrollArea>
  </SidebarProvider>
  <Toaster rich-colors />
</template>
