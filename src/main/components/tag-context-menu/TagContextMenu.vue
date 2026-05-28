<script setup lang="ts">
import {ref} from "vue";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger
} from "@/components/ui/context-menu";
import {Tag} from "@/api/dto.ts";
import {LucideEye, LucideTextCursorInput, LucideTrash} from "@lucide/vue";
import TagRenameDialog from "@/main/components/tag-context-menu/TagRenameDialog.vue";
import TagConfirmDeleteDialog from "@/main/components/tag-context-menu/TagConfirmDeleteDialog.vue";
import {useViewState} from "@/main/views/useViewState.ts";

defineProps<{
  tag: Tag
}>();

const { setViewState } = useViewState();

const renameDialogOpen = ref(false);
const deleteDialogOpen = ref(false);
</script>

<template>
  <ContextMenu>
    <ContextMenuTrigger>
      <slot />
    </ContextMenuTrigger>
    <ContextMenuContent>
      <ContextMenuItem @click="setViewState({ id: 'snippets-by-tag', tag })">
        <LucideEye />
        {{ $t('context-menu.tag.view') }}
      </ContextMenuItem>
      <ContextMenuItem @click="renameDialogOpen = true">
        <LucideTextCursorInput />
        {{ $t('context-menu.tag.rename') }}
      </ContextMenuItem>
      <ContextMenuSeparator />
      <ContextMenuItem variant="destructive" @click="deleteDialogOpen = true">
        <LucideTrash />
        {{ $t('context-menu.tag.delete') }}
      </ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>

  <TagRenameDialog :tag="tag" v-model:open="renameDialogOpen" />
  <TagConfirmDeleteDialog :tag="tag" v-model:open="deleteDialogOpen" />
</template>
