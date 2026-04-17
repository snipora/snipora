<script setup lang="ts">
import {cn} from "@/lib/utils.ts";
import {HTMLAttributes} from "vue";

defineProps<{
  class?: HTMLAttributes["class"]
}>();
</script>

<template>
  <main :class="cn('h-svh grid place-items-center', $props.class)">
    <slot />
  </main>
</template>
