<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import {getVersion as getAppVersion} from "@tauri-apps/api/app";
import {computedAsync} from "@vueuse/core";

const appVersion = computedAsync(getAppVersion);
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t("settings.info.version.label") }}
      </FieldTitle>
      <FieldDescription>
        {{ $t("settings.info.version.description") }}
      </FieldDescription>
    </FieldContent>
    <span class="text-sm text-muted-foreground">
      {{ appVersion }}
    </span>
  </Field>
</template>
