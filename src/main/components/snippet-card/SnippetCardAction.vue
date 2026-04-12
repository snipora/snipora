<script setup lang="ts">
import {Button} from "@/components/ui/button";
import type { ButtonProps } from "@/components/ui/button/Button.vue";

defineProps<ButtonProps>();
</script>

<template>
  <Button v-bind="$props" size="icon-sm" variant="outline">
    <slot />
  </Button>
</template>
