<script setup lang="ts">
import {HTMLAttributes} from "vue";
import {TagsList} from "@/components/ui2/tags-list";
import TagsListTag from "@/components/ui2/tags-list/TagsListTag.vue";

const props = defineProps<{
  class?: HTMLAttributes["class"]
  tags: string[] | undefined
}>();
</script>

<template>
  <TagsList
      v-if="tags"
      data-slot="snippet-card-tags-list"
      :class="props.class"
  >
    <TagsListTag
        v-for="tag in tags"
        :key="tag"
        :tag="tag"
    />
  </TagsList>
</template>
