<script setup lang="ts">
import {useAllSnippets} from "@/composables/data/useAllSnippets.ts";
import { SnippetList, SnippetListEntry} from "@/main/components/snippet-list";
import {CenteredLayout, DefaultLayout} from "@/main/layouts";
import {NoSnippets} from "@/main/components/no-snippets";
import {Spinner} from "@/components/ui/spinner";

const { snippets } = useAllSnippets();
</script>

<template>
  <CenteredLayout v-if="snippets === undefined">
    <Spinner class="size-8" />
  </CenteredLayout>
  <DefaultLayout v-else-if="snippets.length">
    <SnippetList>
      <SnippetListEntry
          v-for="snippet in snippets"
          :key="snippet.id"
          :snippet="snippet"
      />
    </SnippetList>
  </DefaultLayout>
  <CenteredLayout v-else>
    <NoSnippets />
  </CenteredLayout>
</template>
