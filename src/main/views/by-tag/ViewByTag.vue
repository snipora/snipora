<script setup lang="ts">
import {SnippetCard} from "@/components/ui2/snippet-card";
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";
import {computed} from "vue";

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
  <div class="grid gap-6">
    <SnippetCard v-for="snippet in snippets" :key="snippet.id" :snippet="snippet" />
  </div>
</template>
