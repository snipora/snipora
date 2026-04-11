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
import {invokeDeleteSnippet} from "@/api/commands";

const props = defineProps<{
  snippet: SnippetDto
}>();

function handleEdit() {

}

function handleDuplicate() {

}

async function handleDelete() {
  await invokeDeleteSnippet(props.snippet.id);
}
</script>

<template>
  <SnippetCard>
    <SnippetCardHeader :label="snippet.label" />
    <SnippetCardActions>
      <SnippetCardAction @click="handleEdit">
        <LucidePencil />
      </SnippetCardAction>
      <SnippetCardAction @click="handleDuplicate">
        <LucideCopy />
      </SnippetCardAction>
      <!-- todo: dialog for confirmation -->
      <SnippetCardAction class="hover:text-destructive" @click="handleDelete">
        <LucideTrash />
      </SnippetCardAction>
    </SnippetCardActions>
    <SnippetCardTags :tags="snippet.tags" />
    <SnippetCardSnippet :snippet="snippet.snippet" show-copy />
  </SnippetCard>
</template>
