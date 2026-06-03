<script setup lang="ts">
import {computed, HTMLAttributes} from "vue";
import {TagsList} from "@/components/ui2/tags-list";
import TagsListTag from "@/components/ui2/tags-list/TagsListTag.vue";
import sortOn from "sort-on";

const props = defineProps<{
  class?: HTMLAttributes["class"]
  tags: string[] | undefined
}>();

const sortedTags = computed(() => {
  return sortOn(props.tags ?? [], "");
});
</script>

<template>
  <TagsList
      v-if="sortedTags"
      data-slot="snippet-card-tags-list"
      :class="props.class"
  >
    <TagsListTag
        v-for="tag in sortedTags"
        :key="tag"
        :tag="tag"
    />
  </TagsList>
</template>
