<script setup lang="ts">
import {computed} from "vue";
import {Tag} from "@/api/dto.ts";
import {useAsyncAction} from "@/composables/useAsyncAction.ts";
import {invokeDeleteTag} from "@/api/commands";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from "@/components/ui/dialog";
import {Button} from "@/components/ui/button";
import {Spinner} from "@/components/ui/spinner";
import {LucideTrash, LucideX} from "@lucide/vue";

const props = defineProps<{
  tag: Tag
  open?: boolean
}>();

const emit = defineEmits<{
  'update:open': [value: boolean]
}>();

const open = computed({
  get: () => props.open ?? false,
  set: (value) => emit('update:open', value),
});

const { invoke: handleDelete, isRunning: isDeleting } = useAsyncAction(async () => {
  await invokeDeleteTag(props.tag);
  open.value = false;
});
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger as-child>
      <slot />
    </DialogTrigger>
    <DialogContent>
      <DialogHeader>
        <DialogTitle>
          {{ $t('dialogs.delete-tag.dialog-title') }}
        </DialogTitle>
        <DialogDescription>
          {{ $t('dialogs.delete-tag.dialog-description') }}
        </DialogDescription>
      </DialogHeader>
      <DialogFooter>
        <Button variant="outline" @click="open = false">
          <LucideX />
          {{ $t('dialogs.action.cancel') }}
        </Button>
        <Button variant="destructive" :disabled="isDeleting" @click="handleDelete">
          <Spinner v-if="isDeleting" />
          <LucideTrash v-else />
          {{ $t('dialogs.action.delete') }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
