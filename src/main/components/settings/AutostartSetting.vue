<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { Switch } from "@/components/ui/switch";
import { useAutostart } from "@/composables/useAutostart";

const autostartEnabled = useAutostart();
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t("settings.general.startup.label") }}
      </FieldTitle>
      <FieldDescription>
        {{ $t("settings.general.startup.description") }}
      </FieldDescription>
    </FieldContent>
    <Switch
        v-model="autostartEnabled"
        class="cursor-pointer"
    />
  </Field>
</template>
