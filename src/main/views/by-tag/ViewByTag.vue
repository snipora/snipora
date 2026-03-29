<script setup lang="ts">
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";
import {computed} from "vue";
import { SnippetList, SnippetListEntry} from "@/main/components/snippet-list";

const props = defineProps<{
  tag: string
}>();

const { snippets: allSnippets } = useAllSnippets();
const snippets = computed(
    () => allSnippets.value
        ?.filter(s => s.tags.includes(props.tag))
)
</script>

<template>
  <SnippetList>
    <SnippetListEntry v-for="snippet in snippets" :key="snippet.id" :snippet="snippet" />
  </SnippetList>
</template>
