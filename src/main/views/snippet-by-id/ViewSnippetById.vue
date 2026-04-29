<script setup lang="ts">
import {useSnippetById} from "@/composables/data/useSnippetById.ts";
import {CenteredLayout, DefaultLayout} from "@/main/layouts";
import {SomethingWentWrong} from "@/main/components/something-went-wrong";
import {Spinner} from "@/components/ui/spinner";
import {SnippetListEntry} from "@/main/components/snippet-list";

const props = defineProps<{
  snippetId: string
}>();

const {snippet} = useSnippetById(() => props.snippetId);
</script>

<template>
  <CenteredLayout v-if="snippet === undefined">
    <Spinner class="size-8" />
  </CenteredLayout>
  <DefaultLayout v-else-if="snippet">
    <SnippetListEntry :snippet="snippet" class="border-none shadow-none p-0" />
  </DefaultLayout>
  <DefaultLayout v-else>
    <SomethingWentWrong />
  </DefaultLayout>
</template>
