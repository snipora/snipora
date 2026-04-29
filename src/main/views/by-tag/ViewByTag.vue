<script setup lang="ts">
import { SnippetList, SnippetListEntry} from "@/main/components/snippet-list";
import {DefaultLayout} from "@/main/layouts";
import {useSnippetsByTag} from "@/composables/data/useSnippetsByTag.ts";

const props = defineProps<{
  tag: string
}>();

const { snippets } = useSnippetsByTag(() => props.tag);
</script>

<template>
  <DefaultLayout>
    <SnippetList>
      <SnippetListEntry v-for="snippet in snippets" :key="snippet.id" :snippet="snippet" />
    </SnippetList>
  </DefaultLayout>
</template>
