<script setup lang="ts">
import {cn} from "@/lib/utils.ts";
import {HTMLAttributes} from "vue";

defineProps<{
  class?: HTMLAttributes["class"]
}>();
</script>

<template>
  <main :class="cn('mx-auto max-w-3xl p-4', $props.class)">
    <slot />
  </main>
</template>
