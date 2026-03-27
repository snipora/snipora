<script setup lang="ts">
import {SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuItem} from "@/components/ui/sidebar";
import {LucideDiamondPlus, LucideHash} from "@lucide/vue";
import {LucideTagOff} from "@/components/icons";
import {useViewState} from "@/main/views/useViewState.ts";
import NewSnippetDialog from "@/main/dialogs/NewSnippetDialog.vue";

const { viewState, setViewState } = useViewState();
</script>

<template>
  <SidebarHeader>
    <SidebarMenu>
      <NewSnippetDialog>
        <SidebarMenuItem>
            <SidebarMenuButton>
              <LucideDiamondPlus />
              {{ $t('sidebar.new-snippet.label') }}
            </SidebarMenuButton>
        </SidebarMenuItem>
      </NewSnippetDialog>
      <SidebarMenuItem>
        <SidebarMenuButton
            :is-active="viewState.id === 'all-snippets'"
            @click="setViewState({ id: 'all-snippets' })"
        >
          <LucideHash />
          {{ $t('sidebar.all-snippets.label') }}
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton
            :is-active="viewState.id === 'untagged-snippets'"
            @click="setViewState({ id: 'untagged-snippets' })"
        >
          <LucideTagOff />
          {{ $t('sidebar.untagged.label') }}
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  </SidebarHeader>
</template>
