<script setup lang="ts">
import {Kbd} from "@/components/ui/kbd";
import {computed, HTMLAttributes} from "vue";
import {parseShortcut} from "@/components/ui2/shortcut/utils.ts";
import {useOsType} from "@/composables/useOsType.ts";
import {cn} from "@/lib/utils.ts";
import {LucideArrowBigUp, LucideChevronUp, LucideCommand, LucideOption} from "@lucide/vue";

const props = defineProps<{
  shortcut: string
  class?: HTMLAttributes["class"]
}>();

const { isMacOS } = useOsType();
const parsed = computed(() => parseShortcut(props.shortcut));
</script>

<template>
  <Kbd v-if="parsed" :class="cn('gap-0.5', parsed.type === 'chained' && 'tracking-widest', props.class)">
    <template v-if="parsed.type === 'chained'">
      {{ parsed.keys.join("-") }}
    </template>
    <template v-else-if="parsed.type === 'combined'">
      <template v-if="parsed.ctrlKey">
        <LucideCommand v-if="isMacOS" class="size-[1em]" />
        <LucideChevronUp v-else class="h-[1em] w-[.8em] -mt-[0.5em]" />
      </template>
      <LucideArrowBigUp v-if="parsed.shiftKey" class="size-[1em]" />
      <template v-if="parsed.altKey">
        <LucideOption v-if="isMacOS" class="size-[1em]" />
        <span v-else class="mr-0.5">Alt</span>
      </template>
      <span class="uppercase">
        {{ parsed.key }}
      </span>
    </template>
  </Kbd>
</template>
