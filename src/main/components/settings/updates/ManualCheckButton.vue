<script setup lang="ts">
import { computed } from "vue";
import { Button } from "@/components/ui/button";
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { useUpdater } from "@/composables/useUpdater";
import {Spinner} from "@/components/ui/spinner";

const updater = useUpdater();

const disabled = computed(() =>
  updater.isChecking.value || updater.isDownloading.value || updater.isInstalling.value,
);
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t("settings.updates.manual-check.label") }}
      </FieldTitle>
      <FieldDescription>
        {{ $t("settings.updates.manual-check.description") }}
      </FieldDescription>
    </FieldContent>
    <Button
      variant="outline"
      :disabled="disabled"
      @click="updater.checkForUpdate()"
    >
      <Spinner v-if="updater.isChecking.value" />
      {{ $t("settings.updates.manual-check.button") }}
    </Button>
  </Field>
</template>
