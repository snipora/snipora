<script setup lang="ts">
import {computed, HTMLAttributes, ref} from "vue";
import {cn} from "@/lib/utils.ts";
import {Button} from "@/components/ui/button";
import {LucideClipboardCheck, LucideClipboardCopy, LucideFoldVertical, LucideUnfoldVertical} from "@lucide/vue";
import {useClipboard} from "@vueuse/core";

const props = defineProps<{
  class?: HTMLAttributes["class"]
  snippet: string
}>();

const canExpand = computed(() => props.snippet.split(/\r\n|\r|\n/).length >= 5);
const isExpanded = ref<boolean>(false);

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
    <div class="absolute top-0 right-0 flex flex-row-reverse bg-secondary opacity-0 group-focus-within/snippet-card:opacity-100 group-hover/snippet-card:opacity-100 transition-opacity">
      <Button variant="ghost" size="icon-sm" class="cursor-pointer" @click="handleCopy">
        <LucideClipboardCheck v-if="recentlyCopied" />
        <LucideClipboardCopy v-else />
      </Button>
      <Button v-if="canExpand" variant="ghost" size="icon-sm" class="cursor-pointer" @click="isExpanded = !isExpanded">
        <LucideFoldVertical v-if="isExpanded" />
        <LucideUnfoldVertical v-else />
      </Button>
    </div>
    <pre
        :data-expanded="isExpanded"
        :class="cn('font-mono bg-secondary text-secondary-foreground px-2 py-1 rounded-md inset-shadow-xs overflow-x-scroll! line-clamp-5 data-[expanded=true]:line-clamp-none', props.class)"
    >{{ snippet }}</pre>
  </div>
</template>
