<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { useUpdater } from "@/composables/updater";
import {Spinner} from "@/components/ui/spinner";
import {LucideRefreshCw} from "@lucide/vue";

const { checkForUpdate, isChecking, canCheck } = useUpdater();
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t('setting.updates.manual-check.label') }}
      </FieldTitle>
      <FieldDescription>
        {{ $t('setting.updates.manual-check.description') }}
      </FieldDescription>
    </FieldContent>
    <Button
      variant="outline"
      :disabled="!canCheck"
      @click="checkForUpdate"
    >
      <Spinner v-if="isChecking" />
      <LucideRefreshCw v-else />
      {{ $t('setting.updates.manual-check.button') }}
    </Button>
  </Field>
</template>
