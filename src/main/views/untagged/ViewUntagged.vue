<script setup lang="ts">
import {useUntaggedSnippets} from "@/composables/data/useUntaggedSnippets.ts";
import {SnippetCard} from "@/components/ui2/snippet-card";

const { untaggedSnippets } = useUntaggedSnippets();
</script>

<template>
  <div class="grid gap-6">
    <SnippetCard v-for="snippet in untaggedSnippets" :key="snippet.id" :snippet="snippet" />
  </div>
</template>
