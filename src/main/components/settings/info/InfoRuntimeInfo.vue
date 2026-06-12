<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import {useRuntimeInfo} from "@/composables/platform";

const runtimeInfo = useRuntimeInfo();
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t('setting.info.runtime.label') }}
      </FieldTitle>
      <FieldDescription>
        {{ $t('setting.info.runtime.description') }}
      </FieldDescription>
    </FieldContent>
    <span v-if="runtimeInfo" class="text-sm text-muted-foreground">
      {{ runtimeInfo.os }}-{{ runtimeInfo.arch }}-{{ runtimeInfo.bundleType }}
    </span>
  </Field>
</template>
