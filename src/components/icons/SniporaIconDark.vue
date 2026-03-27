<script lang="ts" setup>
import {LucideProps} from "@lucide/vue";

defineProps</* @vue-ignore */ LucideProps>();
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="#fff"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    v-bind="$props"
  >
    <path d="m15 10 2 1h5v-2h-5z" />
    <path d="m15 16 2 1h5v-2h-5z" />
    <path d="m15 20v2h-13v-20h9l4 4" />
    <path d="m6 10h5" />
    <path d="m6 14h5" />
    <path d="m6 18h5" />
  </svg>
</template>
