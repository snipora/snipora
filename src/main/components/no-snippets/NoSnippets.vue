<script setup lang="ts">
import {Empty, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle} from "@/components/ui/empty";
import {SniporaLogo} from "@/components/icons";
</script>

<template>
  <Empty v-bind="$attrs">
    <EmptyHeader>
      <EmptyMedia variant="icon">
        <SniporaLogo />
      </EmptyMedia>
      <EmptyTitle>
        {{ $t('component.no-snippets.title') }}
      </EmptyTitle>
      <EmptyDescription>
        {{ $t('component.no-snippets.description') }}
      </EmptyDescription>
    </EmptyHeader>
  </Empty>
</template>
