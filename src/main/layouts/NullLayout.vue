<script setup lang="ts">
defineProps<{}>();
</script>

<template>
  <main v-bind="$attrs">
    <slot />
  </main>
</template>
