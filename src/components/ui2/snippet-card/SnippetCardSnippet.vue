<script setup lang="ts">
import {HTMLAttributes} from "vue";
import {cn} from "@/lib/utils.ts";
import {Button} from "@/components/ui/button";
import {LucideClipboardCheck, LucideClipboardCopy} from "@lucide/vue";
import {useClipboard} from "@vueuse/core";

const props = defineProps<{
  class?: HTMLAttributes["class"]
  snippet: string
  showCopy?: boolean
}>();

const { copy: copyToClipboard, copied: recentlyCopied } = useClipboard({
  source: () => props.snippet
});

async function handleCopy() {
  await copyToClipboard();
}
</script>

<template>
  <div
      data-slot="snippet-card-snippet"
      class="relative overflow-hidden"
  >
    <Button v-if="showCopy" variant="ghost" size="icon-sm" class="bg-secondary opacity-0 group-hover/snippet-card:opacity-100 transition-opacity absolute top-0 right-0 cursor-pointer" @click="handleCopy">
      <LucideClipboardCheck v-if="recentlyCopied" />
      <LucideClipboardCopy v-else />
    </Button>
    <pre :class="cn('font-mono bg-secondary text-secondary-foreground px-2 py-1 rounded-md inset-shadow-xs overflow-x-scroll', props.class)">{{ snippet }}</pre>
  </div>
</template>
