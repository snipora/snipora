<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { Switch } from "@/components/ui/switch";
import { useLocalSettings } from "@/composables/useLocalSettings";

const showTagCounts = useLocalSettings("appearance.showTagCounts");
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t("setting.appearance.show-tag-counts.label") }}
      </FieldTitle>
      <FieldDescription>
        {{ $t("setting.appearance.show-tag-counts.description") }}
      </FieldDescription>
    </FieldContent>
    <Switch
        v-model="showTagCounts"
        class="cursor-pointer"
    />
  </Field>
</template>
