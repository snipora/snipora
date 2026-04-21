<script setup lang="ts">
import {AppSidebar} from "@/main/sidebar";
import {SidebarProvider} from "@/components/ui/sidebar";
import {provide} from "vue";
import {INJECTION_KEY_MAIN_VIEW, ViewState, VIEW_TO_COMPONENT} from "@/main/views";
import {ScrollArea} from "@/components/ui/scroll-area";
import {useLocalStorage} from "@vueuse/core";
import {useTauriEventListener} from "@/composables/useTauriEventListener.ts";
import {useColorMode} from "@/composables/useColorMode.ts";

useColorMode();

const viewState = useLocalStorage<ViewState>('view-state', () => ({
  id: "all-snippets",
}), { listenToStorageChanges: false });

provide(INJECTION_KEY_MAIN_VIEW, viewState);

useTauriEventListener("main:show-settings", () => {
  viewState.value = { id: "settings" };
});
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <ScrollArea class="h-svh w-full">
      <component :is="VIEW_TO_COMPONENT[viewState.id]" v-bind="viewState" />
    </ScrollArea>
  </SidebarProvider>
</template>
