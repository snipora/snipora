<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { Switch } from "@/components/ui/switch";
import { useLocalSettings } from "@/composables/settings";

const autoCheckForUpdates = useLocalSettings("general.autoCheckForUpdates");
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t('setting.updates.auto-check-updates.label') }}
      </FieldTitle>
      <FieldDescription>
        {{ $t('setting.updates.auto-check-updates.description') }}
      </FieldDescription>
    </FieldContent>
    <Switch v-model="autoCheckForUpdates" />
  </Field>
</template>
