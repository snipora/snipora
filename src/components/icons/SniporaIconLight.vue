<script lang="ts" setup>
import {Icon, IconNode, LucideProps} from "@lucide/vue";

defineProps</* @vue-ignore */ LucideProps>();

const sniporaIconLight: IconNode = [
    ["path", { d: "m15 10 2 1h5v-2h-5z" }],
    ["path", { d: "m15 16 2 1h5v-2h-5z" }],
    ["path", { d: "m15 20v2h-13v-20h9l4 4" }],
    ["path", { d: "m6 10h5" }],
    ["path", { d: "m6 14h5" }],
    ["path", { d: "m6 18h5" }],
];
</script>

<template>
  <Icon :iconNode="sniporaIconLight" v-bind="$props" name="snipora-light" stroke="#000" />
</template>
