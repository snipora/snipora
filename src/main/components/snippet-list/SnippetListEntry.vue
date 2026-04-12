<script setup lang="ts">
import {ref} from "vue";
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
import {
  Dialog,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogScrollContent,
  DialogTitle,
} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {Spinner} from "@/components/ui/spinner";
import {invokeCreateSnippet, invokeDeleteSnippet} from "@/api/commands";
import {useAsyncAction} from "@/composables/useAsyncAction.ts";

const props = defineProps<{
  snippet: SnippetDto
}>();

const isDeleteDialogOpen = ref(false);

const { invoke: handleDuplicate, isRunning: isDuplicating } = useAsyncAction(async () => {
  await invokeCreateSnippet({
    label: props.snippet.label,
    snippet: props.snippet.snippet,
    tags: props.snippet.tags,
  });
});

const { invoke: handleDelete, isRunning: isDeleting } = useAsyncAction(async () => {
  await invokeDeleteSnippet(props.snippet.id);
  isDeleteDialogOpen.value = false;
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
      <SnippetCardAction class="hover:text-destructive" @click="isDeleteDialogOpen = true">
        <LucideTrash />
      </SnippetCardAction>
    </SnippetCardActions>
    <Dialog v-model:open="isDeleteDialogOpen">
      <DialogScrollContent>
        <DialogHeader>
          <DialogTitle>{{ $t("dialogs.delete-snippet.dialog-title") }}</DialogTitle>
          <DialogDescription>{{ $t("dialogs.delete-snippet.dialog-description") }}</DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="outline" @click="isDeleteDialogOpen = false">
            {{ $t("dialogs.delete-snippet.cancel") }}
          </Button>
          <Button variant="destructive" :disabled="isDeleting" @click="handleDelete">
            <Spinner v-if="isDeleting" />
            {{ $t("dialogs.delete-snippet.confirm") }}
          </Button>
        </DialogFooter>
      </DialogScrollContent>
    </Dialog>
    <SnippetCardTags :tags="snippet.tags" />
    <SnippetCardSnippet :snippet="snippet.snippet" show-copy />
  </SnippetCard>
</template>
