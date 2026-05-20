<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t("settings.general.shortcut.label") }}
      </FieldTitle>
      <FieldDescription>
        {{ $t("settings.general.shortcut.description") }}
      </FieldDescription>
    </FieldContent>
    <Input
        disabled
        :placeholder="$t('settings.general.shortcut.placeholder')"
        class="w-48"
    />
  </Field>
</template>
