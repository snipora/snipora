<script setup lang="ts">
import {useUntaggedSnippets} from "@/composables/data/useUntaggedSnippets.ts";
import { SnippetList, SnippetListEntry} from "@/main/components/snippet-list";
import {NoSnippets} from "@/main/components/no-snippets";
import {Spinner} from "@/components/ui/spinner";
import {CenteredLayout, DefaultLayout} from "@/main/layouts";

const { untaggedSnippets } = useUntaggedSnippets();
</script>

<template>
  <CenteredLayout v-if="untaggedSnippets === undefined">
    <Spinner class="size-8" />
  </CenteredLayout>
  <DefaultLayout v-else-if="untaggedSnippets.length">
    <SnippetList>
      <SnippetListEntry v-for="snippet in untaggedSnippets" :key="snippet.id" :snippet="snippet" />
    </SnippetList>
  </DefaultLayout>
  <CenteredLayout v-else>
    <NoSnippets />
  </CenteredLayout>
</template>
