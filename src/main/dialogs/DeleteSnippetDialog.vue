<script setup lang="ts">
import {ref} from "vue";
import {
  Dialog,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogScrollContent,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {Spinner} from "@/components/ui/spinner";
import {invokeDeleteSnippet} from "@/api/commands/snippets";
import {useAsyncAction} from "@/composables/useAsyncAction.ts";
import {LucideTrash, LucideX} from "@lucide/vue";
import {SnippetDto} from "@/api/dto.ts";

const props = defineProps<{
  snippet: SnippetDto
}>();

const isOpen = ref(false);

const { invoke: handleDelete, isRunning: isDeleting } = useAsyncAction(async () => {
  await invokeDeleteSnippet(props.snippet.id);
  isOpen.value = false;
});
</script>

<template>
  <Dialog v-model:open="isOpen">
    <DialogTrigger as-child>
      <slot />
    </DialogTrigger>
    <DialogScrollContent>
      <DialogHeader>
        <DialogTitle>{{ $t('dialog.delete-snippet.dialog-title') }}</DialogTitle>
        <DialogDescription>{{ $t('dialog.delete-snippet.dialog-description') }}</DialogDescription>
      </DialogHeader>
      <DialogFooter>
        <Button variant="outline" @click="isOpen = false">
          <LucideX />
          {{ $t('dialog.action.cancel') }}
        </Button>
        <Button variant="destructive" :disabled="isDeleting" @click="handleDelete">
          <Spinner v-if="isDeleting" />
          <LucideTrash v-else />
          {{ $t('dialog.action.delete') }}
        </Button>
      </DialogFooter>
    </DialogScrollContent>
  </Dialog>
</template>
