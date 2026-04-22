<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { useLocalSettings } from "@/composables/useLocalSettings.ts";
import { LucideClipboardCopy, LucidePencilLine } from "@lucide/vue";

const snippetUsageBehavior = useLocalSettings("popup.snippetUsageBehavior");
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t('settings.general.use-behavior.label') }}
      </FieldTitle>
      <FieldDescription>
        {{ $t('settings.general.use-behavior.description') }}
      </FieldDescription>
    </FieldContent>
    <Select v-model="snippetUsageBehavior">
      <SelectTrigger>
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="copy-to-clipboard">
          <template #default>
            <LucideClipboardCopy />
            {{ $t('settings.general.use-behavior.clipboard.label') }}
          </template>
          <template #description>
            {{ $t('settings.general.use-behavior.clipboard.description') }}
          </template>
        </SelectItem>
        <SelectItem value="simulate-paste" disabled>
          <template #default>
            <LucidePencilLine />
            {{ $t('settings.general.use-behavior.paste.label') }}
          </template>
          <template #description>
            {{ $t('settings.general.use-behavior.paste.description') }}
          </template>
        </SelectItem>
      </SelectContent>
    </Select>
  </Field>
</template>
