<script setup lang="ts">
import {stringToColor} from "@/lib/coloring.ts";
import {LucideTag} from "@lucide/vue";
import {
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem
} from "@/components/ui/sidebar";
import {useViewState} from "@/main/views/useViewState.ts";
import {useAllTags} from "@/composables/data/useAllTags.ts";

const { tags } = useAllTags();
const { viewState, setViewState } = useViewState();
</script>

<template>
  <SidebarContent>
    <SidebarGroup>
      <SidebarGroupContent>
        <SidebarMenu>
          <SidebarMenuItem v-for="tag in tags" :key="tag">
            <SidebarMenuButton
                class="capitalize"
                :is-active="viewState.id === 'snippets-by-tag' && tag === viewState.tag"
                @click="setViewState({ id: 'snippets-by-tag', tag })"
            >
              <LucideTag :style="{ fill: stringToColor(`${tag}`) }" />
              <span>
                {{ tag }}
              </span>
              <!--<span class="ml-auto text-muted-foreground">{{ snippetCount }}</span>-->
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  </SidebarContent>
</template>
