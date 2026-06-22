<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Field, FieldContent, FieldDescription, FieldTitle } from "@/components/ui/field";
import { exit } from "@tauri-apps/plugin-process";
import { LucideDoorOpen } from "@lucide/vue";

function handleQuit() {
  exit(0);
}
</script>

<template>
  <Field orientation="horizontal">
    <FieldContent>
      <FieldTitle>
        {{ $t('setting.quit.label') }}
      </FieldTitle>
      <FieldDescription>
        {{ $t('setting.quit.description') }}
      </FieldDescription>
    </FieldContent>
    <Button variant="destructive" @click="handleQuit">
      <LucideDoorOpen />
      {{ $t('setting.quit.label') }}
    </Button>
  </Field>
</template>
