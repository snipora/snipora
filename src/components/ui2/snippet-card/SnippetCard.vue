<script setup lang="ts">
import {SnippetDto} from "@/api/dto.ts";
import {Button} from "@/components/ui/button";
import {LucideCopy, LucideCopyCheck} from "@lucide/vue";
import {useClipboard} from "@vueuse/core";
import {HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";
import {TagsList} from "@/components/ui2/tags-list";
import TagsListTag from "@/components/ui2/tags-list/TagsListTag.vue";

const props = withDefaults(defineProps<{
  class?: HTMLAttributes["class"]
  snippet: SnippetDto
  showCopy?: boolean
}>(), {
  showCopy: true
});

const { copy: copyToClipboard, copied: recentlyCopied } = useClipboard({
  source: () => props.snippet.snippet
});

async function handleCopy() {
  await copyToClipboard();
}
</script>

<template>
  <div :class="cn('grid gap-1', props.class)">
    <h3 class="select-none leading-none text-lg font-semibold tracking-tight">
      {{ snippet.label }}
    </h3>
    <TagsList v-if="snippet.tags">
      <TagsListTag
          v-for="tag in snippet.tags"
          :key="tag"
          :tag="tag"
      />
    </TagsList>
    <div class="relative">
      <Button v-if="showCopy" variant="ghost" size="icon-sm" class="absolute top-0 right-0 cursor-pointer" @click="handleCopy">
        <LucideCopyCheck v-if="recentlyCopied" />
        <LucideCopy v-else />
      </Button>
      <pre class="font-mono bg-secondary text-secondary-foreground px-2 py-1 rounded-md inset-shadow-xs">{{ snippet.snippet }}</pre>
    </div>
  </div>
</template>
