<script setup lang="ts">
import {useUntaggedSnippets} from "@/composables/data/useUntaggedSnippets.ts";
import { SnippetList, SnippetListEntry} from "@/main/components/snippet-list";

const { untaggedSnippets } = useUntaggedSnippets();
</script>

<template>
  <SnippetList>
    <SnippetListEntry v-for="snippet in untaggedSnippets" :key="snippet.id" :snippet="snippet" />
  </SnippetList>
</template>
