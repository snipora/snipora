<script setup lang="ts">
import {SidebarFooter, SidebarMenu, SidebarMenuButton, SidebarMenuItem} from "@/components/ui/sidebar";
import {LucideSettings} from "@lucide/vue";
import {useViewState} from "@/main/views/useViewState.ts";

const { viewState, setViewState } = useViewState();
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
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  </SidebarFooter>
</template>
