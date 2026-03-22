<script setup lang="ts">
import {SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuItem} from "@/components/ui/sidebar";
import {LucideDiamondPlus, LucideHash} from "lucide-vue-next";
import {LucideTagOff} from "@/components/icons";
</script>

<template>
  <SidebarHeader>
    <SidebarMenu>
      <SidebarMenuItem>
        <SidebarMenuButton>
          <LucideDiamondPlus />
          {{ $t('sidebar.new-snippet.label') }}
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton>
          <LucideHash />
          {{ $t('sidebar.all-snippets.label') }}
        </SidebarMenuButton>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton>
          <LucideTagOff />
          {{ $t('sidebar.untagged.label') }}
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  </SidebarHeader>
</template>
