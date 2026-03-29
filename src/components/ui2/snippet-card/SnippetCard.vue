<script setup lang="ts">
import {HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";

const props = defineProps<{
  class?: HTMLAttributes["class"]
}>();
</script>

<template>
  <div
      data-slot="snippet-card"
      :class="cn('group/snippet-card border border-border shadow-sm rounded-lg p-2 grid grid-cols-[1fr_auto] gap-2 *:col-span-2 [&:has([data-slot=snippet-card-actions])_[data-slot=snippet-card-header]]:col-span-1 *:data-[slot=snippet-card-actions]:col-span-1', props.class)"
  >
    <slot />
  </div>
</template>
