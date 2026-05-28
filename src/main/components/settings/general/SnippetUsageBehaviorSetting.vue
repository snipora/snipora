<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { useLocalSettings } from "@/composables/useLocalSettings.js";
import {LucideClipboardCopy, LucideClipboardPaste, LucideKeyboard} from "@lucide/vue";

const snippetUsageBehavior = useLocalSettings("general.snippetUsageBehavior");
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t('setting.general.snippet-usage-behavior.label') }}
      </FieldTitle>
      <FieldDescription>
        {{ $t('setting.general.snippet-usage-behavior.description') }}
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
            {{ $t('setting.general.snippet-usage-behavior.copy-to-clipboard.label') }}
          </template>
          <template #description>
            {{ $t('setting.general.snippet-usage-behavior.copy-to-clipboard.description') }}
          </template>
        </SelectItem>
        <SelectItem value="simulate-paste" disabled>
          <template #default>
            <LucideClipboardPaste />
            {{ $t('setting.general.snippet-usage-behavior.simulate-paste.label') }}
          </template>
          <template #description>
            {{ $t('setting.general.snippet-usage-behavior.simulate-paste.description') }}
          </template>
        </SelectItem>
        <SelectItem value="natural-typing" disabled>
          <template #default>
            <LucideKeyboard />
            {{ $t('setting.general.snippet-usage-behavior.natural-typing.label') }}
          </template>
          <template #description>
            {{ $t('setting.general.snippet-usage-behavior.natural-typing.description') }}
          </template>
        </SelectItem>
      </SelectContent>
    </Select>
  </Field>
</template>
