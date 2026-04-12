<script setup lang="ts">
import {Button} from "@/components/ui/button";
import type { ButtonProps } from "@/components/ui/button/Button.vue";

defineProps<ButtonProps>();
</script>

<template>
  <Button size="icon-sm" variant="outline" v-bind="$attrs">
    <slot />
  </Button>
</template>
