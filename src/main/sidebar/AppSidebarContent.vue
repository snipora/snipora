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

const allTags = computedAsync(() => invokeGetAllTags());
</script>

<template>
  <SidebarContent>
    <SidebarGroup>
      <SidebarGroupContent>
        <SidebarMenu>
          <SidebarMenuItem v-for="tag in allTags" :key="tag">
            <SidebarMenuButton class="capitalize" :is-active="false">
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
