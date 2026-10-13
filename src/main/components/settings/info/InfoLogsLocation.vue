<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { useLogsLocation } from "@/composables/platform/useLogsLocation.ts";
import { Input } from "@/components/ui/input";

const logsLocation = useLogsLocation();
</script>

<template>
  <Field>
    <FieldContent>
      <FieldTitle>
        {{ $t('setting.info.logs-location.label') }}
      </FieldTitle>
      <FieldDescription>
        {{ $t('setting.info.logs-location.description') }}
      </FieldDescription>
    </FieldContent>
    <Input readonly class="select-all" :model-value="logsLocation" />
  </Field>
</template>
