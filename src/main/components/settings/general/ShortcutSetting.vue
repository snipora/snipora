<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t('setting.general.shortcut.label') }}
      </FieldTitle>
      <FieldDescription>
        {{ $t('setting.general.shortcut.description') }}
      </FieldDescription>
    </FieldContent>
    <Input
        disabled
        :placeholder="$t('setting.general.shortcut.placeholder')"
        class="w-48"
    />
  </Field>
</template>
