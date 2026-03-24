<script setup lang="ts">
import {AppSidebar} from "@/main/sidebar";
import {SidebarProvider} from "@/components/ui/sidebar";
import {provide, ref} from "vue";
import {INJECTION_KEY_MAIN_VIEW, ViewState, VIEW_TO_COMPONENT} from "@/main/views";

const viewState = ref<ViewState>({
  id: "all-snippets",
});

provide(INJECTION_KEY_MAIN_VIEW, viewState);
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <main>
      <component :is="VIEW_TO_COMPONENT[viewState.id]" v-bind="viewState" />
    </main>
  </SidebarProvider>
</template>
