<script setup lang="ts">
import {Sidebar} from "@/components/ui/sidebar";
import AppSidebarHeader from "@/main/sidebar/AppSidebarHeader.vue";
import AppSidebarContent from "@/main/sidebar/AppSidebarContent.vue";
import AppSidebarFooter from "@/main/sidebar/AppSidebarFooter.vue";
</script>

<template>
  <Sidebar collapsible="none" class="h-svh">
    <AppSidebarHeader />
    <AppSidebarContent />
    <AppSidebarFooter />
  </Sidebar>
</template>
