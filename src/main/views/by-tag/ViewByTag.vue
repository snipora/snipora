<script setup lang="ts">
const props = defineProps<{
  tag: string
}>();
</script>

<template>
  By Tag {{ props.tag }}
</template>
