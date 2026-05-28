<script setup lang="ts">
import {
  Dialog,
  DialogContent,
  DialogDescription, DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from "@/components/ui/dialog";
import {Input} from "@/components/ui/input";
import {computed, ref} from "vue";
import {Button} from "@/components/ui/button";
import {useAllTags} from "@/composables/data/useAllTags.ts";
import {Tag} from "@/api/dto.ts";
import {useAsyncAction} from "@/composables/useAsyncAction.ts";
import {invokeMergeTag, invokeRenameTag} from "@/api/commands";
import {Field, FieldError} from "@/components/ui/field";
import {Spinner} from "@/components/ui/spinner";
import {LucideMerge, LucideTextCursor, LucideX} from "@lucide/vue";
import {useVModel, whenever} from "@vueuse/core";
import {defineShortcuts} from "@/composables/defineShortcut.ts";

const props = defineProps<{
  tag: Tag
  open?: boolean
}>();

const emit = defineEmits<{
  'update:open': [value: boolean]
}>();

const isOpen = useVModel(props, "open", emit, {
  defaultValue: false,
});

const { tags: allTags } = useAllTags();

const matchesExistingTag = computed(() => props.tag !== tagName.value && allTags.value?.includes(tagName.value));

const tagName = ref<Tag>("");

const tagRegex = /^[a-z0-9\-_]{1,32}$/;

const isValid = computed(() => {
  const normalized = tagName.value.trim().toLowerCase();
  return normalized.length > 0 && tagRegex.test(normalized);
});

whenever(isOpen, () => {
  tagName.value = props.tag;
});

const { invoke: renameOrMerge, isRunning: isRenaming, lastError: error } = useAsyncAction(async () => {
  if (matchesExistingTag.value) {
    await invokeMergeTag(props.tag, tagName.value);
  } else {
    await invokeRenameTag(props.tag, tagName.value);
  }
  isOpen.value = false;
});

defineShortcuts({
  ctrl_s: {
    handler: () => renameOrMerge(),
    whenever: [isOpen, isValid],
    usingInput: true,
  },
  ctrl_enter: {
    handler: () => renameOrMerge(),
    whenever: [isOpen, isValid],
    usingInput: true,
  }
})
</script>

<template>
  <Dialog v-model:open="isOpen">
    <DialogTrigger as-child>
      <slot />
    </DialogTrigger>
    <DialogContent>
      <DialogHeader>
        <DialogTitle>
          {{ $t('dialog.rename-tag.dialog-title') }}
        </DialogTitle>
        <DialogDescription>
          <template v-if="matchesExistingTag">
            {{ $t('dialog.rename-tag.merge-description', { tag: tagName }) }}
          </template>
          <template v-else>
            {{ $t('dialog.rename-tag.rename-description') }}
          </template>
        </DialogDescription>
      </DialogHeader>
      <Field>
        <Input
            v-model.trim="tagName"
            :disabled="isRenaming"
            class="capitalize placeholder:normal-case"
            :placeholder="$t('dialog.rename-tag.input-placeholder')"
        />
        <FieldError :errors="[error]" />
      </Field>
      <DialogFooter>
        <Button variant="secondary" @click="isOpen = false">
          <LucideX />
          {{ $t('dialog.action.cancel') }}
        </Button>
        <Button :disabled="!isValid || isRenaming" @click="renameOrMerge">
          <Spinner v-if="isRenaming" />
          <LucideTextCursor v-else-if="!matchesExistingTag" />
          <LucideMerge v-else />
          <template v-if="matchesExistingTag">
            {{ $t('dialog.rename-tag.merge') }}
          </template>
          <template v-else>
            {{ $t('dialog.rename-tag.rename') }}
          </template>
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
