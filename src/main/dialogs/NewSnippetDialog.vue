<script setup lang="ts">
import {computed, ref} from "vue";
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
import {Input} from "@/components/ui/input";
import {Textarea} from "@/components/ui/textarea";
import {Spinner} from "@/components/ui/spinner";
import {invokeCreateSnippet} from "@/api/commands/snippets";
import {TagsInputWithCompletion} from "@/main/components/tags-input-with-completion";
import {defineShortcuts} from "@/composables/defineShortcut.ts";
import {useAsyncAction} from "@/composables/useAsyncAction.ts";
import {Alert, AlertDescription, AlertTitle} from "@/components/ui/alert";
import {LucideCircleAlert, LucideStickyNotePlus, LucideX} from "@lucide/vue";

const isOpen = ref(false);

const label = ref("");
const snippet = ref("");
const tags = ref<string[]>([]);
const isSubmittable = computed(() => !isSubmitting.value && !!label.value.trim().length && !!snippet.value.trim().length);

function resetForm() {
  label.value = "";
  snippet.value = "";
  tags.value = [];
}

const { invoke: handleSubmit, isRunning: isSubmitting, lastError } = useAsyncAction(async () => {
  if (!label.value.trim() || !snippet.value.trim()) {
    return;
  }

  await invokeCreateSnippet({
    label: label.value.trim(),
    snippet: snippet.value.trim(),
    tags: tags.value,
  });
  isOpen.value = false;
  resetForm();
});

defineShortcuts({
  ctrl_n: () => {
    isOpen.value = true;
  },
  ctrl_s: {
    handler: handleSubmit,
    whenever: [isOpen, isSubmittable],
    usingInput: true,
  },
  ctrl_enter: {
    handler: handleSubmit,
    whenever: [isOpen, isSubmittable],
    usingInput: true,
  },
});
</script>

<template>
  <Dialog v-model:open="isOpen">
    <DialogTrigger as-child>
      <slot />
    </DialogTrigger>
    <DialogScrollContent class="max-w-2xl">
      <DialogHeader>
        <DialogTitle>
          {{ $t('dialog.new-snippet.dialog-title') }}
        </DialogTitle>
        <DialogDescription>
          {{ $t('dialog.new-snippet.dialog-description') }}
        </DialogDescription>
      </DialogHeader>
      <form @submit.prevent="handleSubmit" class="grid gap-4">
        <Input
            v-model.trim="label"
            required
            autofocus
            :placeholder="$t('dialog.new-snippet.form.label-placeholder')"
        />
        <Textarea
            v-model="snippet"
            class="max-h-96 font-mono placeholder:font-sans"
            required
            :placeholder="$t('dialog.new-snippet.form.snippet-placeholder')"
        />
        <TagsInputWithCompletion
            v-model="tags"
            :placeholder="$t('dialog.new-snippet.form.tags-placeholder')"
        />
      </form>
      <Alert v-if="lastError" variant="destructive">
        <LucideCircleAlert />
        <AlertTitle>
          {{ $t('dialog.new-snippet.error.title') }}
        </AlertTitle>
        <AlertDescription>
          {{ lastError.message }}
        </AlertDescription>
      </Alert>
      <DialogFooter>
        <Button variant="outline" @click="isOpen = false">
          <LucideX />
          {{ $t('dialog.action.cancel') }}
        </Button>
        <Button :disabled="!isSubmittable" @click="handleSubmit">
          <Spinner v-if="isSubmitting" />
          <LucideStickyNotePlus v-else />
          {{ $t('dialog.action.create') }}
        </Button>
      </DialogFooter>
    </DialogScrollContent>
  </Dialog>
</template>
