<script setup lang="ts">
import {stringToColor} from "@/lib/coloring.ts";

defineProps<{
  tag: string
}>()
</script>

<template>
  <div
      class="flex h-5 items-center rounded-md py-0.5 px-2 text-sm shadow select-none capitalize text-white"
      :style="{ backgroundColor: stringToColor(tag) }"
  >
    <slot>
      {{ tag }}
    </slot>
  </div>
</template>
