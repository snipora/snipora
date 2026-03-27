<script setup lang="ts">
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";
import {SnippetCard} from "@/components/ui2/snippet-card";

const { snippets } = useAllSnippets();
</script>

<template>
  <div class="grid gap-6">
    <SnippetCard v-for="snippet in snippets" :key="snippet.id" :snippet="snippet" />
  </div>
</template>
