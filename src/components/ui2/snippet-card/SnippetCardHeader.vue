<script setup lang="ts">
import {HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";

const props = defineProps<{
  class?: HTMLAttributes["class"]
  label: string
}>();
</script>

<template>
  <h3
      data-slot="snippet-card-header"
      :class="cn('select-none leading-none text-xl font-semibold tracking-tight', props.class)"
  >
    {{ label }}
  </h3>
</template>
