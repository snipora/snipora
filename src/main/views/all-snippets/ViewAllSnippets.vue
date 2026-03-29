<script setup lang="ts">
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";
import { SnippetList, SnippetListEntry} from "@/main/components/snippet-list";

const { snippets } = useAllSnippets();
</script>

<template>
  <SnippetList>
    <SnippetListEntry v-for="snippet in snippets" :key="snippet.id" :snippet="snippet" />
  </SnippetList>
</template>
