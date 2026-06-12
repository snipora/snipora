<script setup lang="ts">
import {SidebarFooter, SidebarMenu, SidebarMenuButton, SidebarMenuItem} from "@/components/ui/sidebar";
import {LucideSettings} from "@lucide/vue";
import {useViewState} from "@/main/views/useViewState.ts";
import {defineShortcuts} from "@/composables/interaction";
import {Shortcut} from "@/components/ui2/shortcut";

const { viewState, setViewState } = useViewState();

defineShortcuts({
  ctrl_alt_s: () => {
    setViewState({ id: 'settings' });
  },
});
</script>

<template>
  <SidebarFooter>
    <SidebarMenu>
      <SidebarMenuItem>
        <SidebarMenuButton
            :is-active="viewState.id === 'settings'"
            @click="setViewState({ id: 'settings' })"
        >
          <LucideSettings />
          {{ $t('sidebar.settings.label') }}
          <Shortcut shortcut="ctrl_alt_s" class="ml-auto" />
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  </SidebarFooter>
</template>
