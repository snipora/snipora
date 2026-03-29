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
import {Input} from "@/components/ui/input";
import {Textarea} from "@/components/ui/textarea";
import {Spinner} from "@/components/ui/spinner";
import {invokeCreateSnippet} from "@/api/commands/snippets";
import {TagsInputWithCompletion} from "@/components/ui2/tags-input-with-completion";

const isOpen = ref(false);
const isSubmitting = ref(false);

const label = ref("");
const snippet = ref("");
const tags = ref<string[]>([]);

function resetForm() {
  label.value = "";
  snippet.value = "";
  tags.value = [];
}

async function handleSubmit() {
  if (!label.value.trim() || !snippet.value.trim()) {
    return;
  }

  isSubmitting.value = true;

  try {
    await invokeCreateSnippet({
      label: label.value.trim(),
      snippet: snippet.value.trim(),
      tags: tags.value,
    });
    isOpen.value = false;
    resetForm();
  } finally {
    isSubmitting.value = false;
  }
}
</script>

<template>
  <Dialog v-model:open="isOpen">
    <DialogTrigger>
      <slot />
    </DialogTrigger>
    <DialogScrollContent class="max-w-2xl">
      <DialogHeader>
        <DialogTitle>
          {{ $t("dialogs.new-snippet.dialog-title") }}
        </DialogTitle>
        <DialogDescription>
          {{ $t("dialogs.new-snippet.dialog-description") }}
        </DialogDescription>
      </DialogHeader>
      <form @submit.prevent="handleSubmit" class="grid gap-4">
        <Input
            v-model="label"
            required
            :placeholder="$t('dialogs.new-snippet.form.label-placeholder')"
        />
        <Textarea
            v-model="snippet"
            class="max-h-96 font-mono placeholder:font-sans"
            required
            :placeholder="$t('dialogs.new-snippet.form.snippet-placeholder')"
        />
        <TagsInputWithCompletion
            v-model="tags"
            :placeholder="$t('dialogs.new-snippet.form.tags-placeholder')"
        />
      </form>
      <DialogFooter>
        <Button
            type="button"
            variant="outline"
            @click="isOpen = false"
        >
          {{ $t("dialogs.new-snippet.form.cancel") }}
        </Button>
        <Button
          type="submit"
          :disabled="isSubmitting || !label.trim() || !snippet.trim()"
          @click="handleSubmit"
        >
          <Spinner v-if="isSubmitting" class="mr-2" />
          {{ $t("dialogs.new-snippet.form.create") }}
        </Button>
      </DialogFooter>
    </DialogScrollContent>
  </Dialog>
</template>
