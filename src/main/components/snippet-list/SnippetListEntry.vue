<script setup lang="ts">
import {SnippetDto} from "@/api/dto.ts";
import {LucideCopy, LucidePencil, LucideTrash} from "@lucide/vue";
import {
  SnippetCard,
  SnippetCardAction,
  SnippetCardActions,
  SnippetCardHeader,
  SnippetCardSnippet,
  SnippetCardTags
} from "@/main/components/snippet-card";

import {Spinner} from "@/components/ui/spinner";
import {invokeCreateSnippet} from "@/api/commands";
import {useAsyncAction} from "@/composables/useAsyncAction.ts";
import EditSnippetDialog from "@/main/dialogs/EditSnippetDialog.vue";
import DeleteSnippetDialog from "@/main/dialogs/DeleteSnippetDialog.vue";

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
</script>

<template>
  <SnippetCard>
    <SnippetCardHeader :label="snippet.label" />
    <SnippetCardActions>
      <EditSnippetDialog :snippet="snippet">
        <SnippetCardAction>
          <LucidePencil />
        </SnippetCardAction>
      </EditSnippetDialog>
      <SnippetCardAction :disabled="isDuplicating" @click="handleDuplicate">
        <Spinner v-if="isDuplicating" />
        <LucideCopy v-else />
      </SnippetCardAction>
      <DeleteSnippetDialog :snippet="snippet">
        <SnippetCardAction class="hover:text-destructive">
          <LucideTrash />
        </SnippetCardAction>
      </DeleteSnippetDialog>
    </SnippetCardActions>
    <SnippetCardTags :tags="snippet.tags" />
    <SnippetCardSnippet :snippet="snippet.snippet" />
  </SnippetCard>
</template>
