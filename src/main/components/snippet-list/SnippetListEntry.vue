<script setup lang="ts">
import {SnippetDto} from "@/api/dto.ts";
import {LucideCopy, LucidePencil, LucideTrash} from "@lucide/vue";
import {
  SnippetCard, SnippetCardAction,
  SnippetCardActions,
  SnippetCardHeader,
  SnippetCardSnippet,
  SnippetCardTags
} from "@/main/components/snippet-card";
import {invokeCreateSnippet, invokeDeleteSnippet} from "@/api/commands";
import {useAsyncAction} from "@/composables/useAsyncAction.ts";
import {Spinner} from "@/components/ui/spinner";

const props = defineProps<{
  snippet: SnippetDto
}>();

const { invoke: handleDuplicate, isRunning: isDuplicating } = useAsyncAction(async () => {
  await invokeCreateSnippet({
    label: props.snippet.label,
    snippet: props.snippet.snippet,
    tags: props.snippet.tags,
  });
});

const { invoke: handleDelete, isRunning: isDeleting } = useAsyncAction(async () => {
  await invokeDeleteSnippet(props.snippet.id);
});
</script>

<template>
  <SnippetCard>
    <SnippetCardHeader :label="snippet.label" />
    <SnippetCardActions>
      <SnippetCardAction>
        <LucidePencil />
      </SnippetCardAction>
      <SnippetCardAction :disabled="isDuplicating" @click="handleDuplicate">
        <Spinner v-if="isDuplicating" />
        <LucideCopy v-else />
      </SnippetCardAction>
      <!-- todo: dialog for confirmation -->
      <SnippetCardAction class="hover:text-destructive" :disabled="isDeleting" @click="handleDelete">
        <Spinner v-if="isDeleting" />
        <LucideTrash v-else />
      </SnippetCardAction>
    </SnippetCardActions>
    <SnippetCardTags :tags="snippet.tags" />
    <SnippetCardSnippet :snippet="snippet.snippet" show-copy />
  </SnippetCard>
</template>
