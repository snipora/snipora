<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { Switch } from "@/components/ui/switch";
import { useLocalSettings } from "@/composables/useLocalSettings";

const autoCheckForUpdates = useLocalSettings("general.autoCheckForUpdates");
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t("settings.updates.auto-check-updates.label") }}
      </FieldTitle>
      <FieldDescription>
        {{ $t("settings.updates.auto-check-updates.description") }}
      </FieldDescription>
    </FieldContent>
    <Switch
        disabled
        v-model="autoCheckForUpdates"
        class="cursor-pointer"
    />
  </Field>
</template>
