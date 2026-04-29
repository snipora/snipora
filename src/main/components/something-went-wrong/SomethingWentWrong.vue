<script setup lang="ts">
import {Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle} from "@/components/ui/empty";
import {LucideCircleAlert} from "@lucide/vue";
</script>

<template>
  <Empty v-bind="$attrs">
    <EmptyHeader>
      <EmptyMedia variant="icon">
        <LucideCircleAlert />
      </EmptyMedia>
      <EmptyTitle>
        {{ $t('components.something-went-wrong.title') }}
      </EmptyTitle>
      <EmptyDescription>
        {{ $t('components.something-went-wrong.description') }}
      </EmptyDescription>
    </EmptyHeader>
  </Empty>
</template>