<script setup lang="ts">
import {SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuItem} from "@/components/ui/sidebar";
import {LucideDiamondPlus, LucideHash} from "@lucide/vue";
import {LucideTagOff} from "@/components/icons";
import {useViewState} from "@/main/views/useViewState.ts";
import NewSnippetDialog from "@/main/dialogs/NewSnippetDialog.vue";
import {useUntaggedSnippets} from "@/composables/data/useUntaggedSnippets.ts";
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";
import {useLocalSettings} from "@/composables/useLocalSettings.ts";

const { viewState, setViewState } = useViewState();

const showTagCounts = useLocalSettings("ui.showTagCounts");

const { snippets: allSnippets } = useAllSnippets();
const { untaggedSnippets } = useUntaggedSnippets();
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
          <span>
            {{ $t('sidebar.all-snippets.label') }}
          </span>
          <span v-if="showTagCounts" class="ml-auto text-xs text-muted-foreground">
            {{ allSnippets?.length }}
          </span>
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem v-if="untaggedSnippets?.length">
        <SidebarMenuButton
            :is-active="viewState.id === 'untagged-snippets'"
            @click="setViewState({ id: 'untagged-snippets' })"
        >
          <LucideTagOff />
          <span>
            {{ $t('sidebar.untagged.label') }}
          </span>
          <span v-if="showTagCounts" class="ml-auto text-xs text-muted-foreground">
            {{ untaggedSnippets.length }}
          </span>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  </SidebarHeader>
</template>
