<script lang="ts" setup>
import {Icon, IconNode, LucideProps} from "@lucide/vue";

defineProps</* @vue-ignore */ LucideProps>();

const tagOffIcon: IconNode = [
    ["path", { d: "m14.172 4.172-1.586-1.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l1.586 1.586" }],
    ["path", { d: "M2 22 22 2" }],
    ["path", { d: "m7 17 4.29 4.29a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42L17 7" }],
    ["circle", { cx: 7.5, cy: 7.5, r: .5, fill: "currentColor" }],
];
</script>

<template>
  <Icon :iconNode="tagOffIcon" v-bind="$props" name="tag-off" />
</template>
