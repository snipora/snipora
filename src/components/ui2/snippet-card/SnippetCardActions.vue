<script setup lang="ts">
import {ButtonGroup} from "@/components/ui/button-group";
</script>

<template>
  <ButtonGroup
      data-slot="snippet-card-actions"
      class="opacity-0 group-hover/snippet-card:opacity-100 transition-opacity"
  >
    <slot />
  </ButtonGroup>
</template>
