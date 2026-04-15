<script setup lang="ts">
import {ref, watch} from "vue";
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
import {invokeUpdateSnippet} from "@/api/commands/snippets";
import {TagsInputWithCompletion} from "@/main/components/tags-input-with-completion";
import {useAsyncAction} from "@/composables/useAsyncAction.ts";
import {Alert, AlertDescription, AlertTitle} from "@/components/ui/alert";
import {LucideCircleAlert} from "@lucide/vue";
import {SnippetDto} from "@/api/dto.ts";

const props = defineProps<{
  snippet: SnippetDto
}>();

const isOpen = ref(false);

const label = ref("");
const snippet = ref("");
const tags = ref<string[]>([]);

watch(() => props.snippet, (newSnippet) => {
  label.value = newSnippet.label;
  snippet.value = newSnippet.snippet;
  tags.value = [...newSnippet.tags];
}, {immediate: true});


const { invoke: handleSubmit, isRunning: isSubmitting, lastError } = useAsyncAction(async () => {
  if (!label.value.trim() || !snippet.value.trim()) {
    return;
  }

  await invokeUpdateSnippet({
    id: props.snippet.id,
    label: label.value.trim(),
    snippet: snippet.value.trim(),
    tags: tags.value,
  });
  isOpen.value = false;
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
          {{ $t("dialogs.edit-snippet.dialog-title") }}
        </DialogTitle>
        <DialogDescription>
          {{ $t("dialogs.edit-snippet.dialog-description") }}
        </DialogDescription>
      </DialogHeader>
      <form @submit.prevent="handleSubmit" class="grid gap-4">
        <Input
            v-model="label"
            required
            autofocus
            :placeholder="$t('dialogs.edit-snippet.form.label-placeholder')"
        />
        <Textarea
            v-model="snippet"
            class="max-h-96 font-mono placeholder:font-sans"
            required
            :placeholder="$t('dialogs.edit-snippet.form.snippet-placeholder')"
        />
        <TagsInputWithCompletion
            v-model="tags"
            :placeholder="$t('dialogs.edit-snippet.form.tags-placeholder')"
        />
      </form>
      <Alert v-if="lastError" variant="destructive">
        <LucideCircleAlert />
        <AlertTitle>
          {{ $t('dialogs.edit-snippet.error.title') }}
        </AlertTitle>
        <AlertDescription>
          {{ lastError.message }}
        </AlertDescription>
      </Alert>
      <DialogFooter>
        <Button
            type="button"
            variant="outline"
            @click="isOpen = false"
        >
          {{ $t("dialogs.edit-snippet.form.cancel") }}
        </Button>
        <Button
          type="submit"
          :disabled="isSubmitting || !label.trim() || !snippet.trim()"
          @click="handleSubmit"
        >
          <Spinner v-if="isSubmitting" class="mr-2" />
          {{ $t("dialogs.edit-snippet.form.save") }}
        </Button>
      </DialogFooter>
    </DialogScrollContent>
  </Dialog>
</template>