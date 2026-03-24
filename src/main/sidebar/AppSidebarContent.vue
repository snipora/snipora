<script setup lang="ts">
import {stringToColor} from "@/lib/coloring.ts";
import {LucideTag} from "lucide-vue-next";
import {
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem
} from "@/components/ui/sidebar";
import {computedAsync} from "@vueuse/core";
import {invokeGetAllTags} from "@/api/commands";
import {useViewState} from "@/main/views/useViewState.ts";

const allTags = computedAsync(() => invokeGetAllTags());
const { viewState, setViewState } = useViewState();
</script>

<template>
  <SidebarContent>
    <SidebarGroup>
      <SidebarGroupContent>
        <SidebarMenu>
          <SidebarMenuItem v-for="tag in allTags" :key="tag">
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
