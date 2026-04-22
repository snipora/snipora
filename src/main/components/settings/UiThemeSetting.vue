<script setup lang="ts">
import {Field, FieldContent, FieldDescription, FieldLabel, FieldTitle} from "@/components/ui/field";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { useColorMode } from "@/composables/useColorMode";
import { LucideMoon, LucideSun, LucideSunMoon } from "@lucide/vue";

const { colorTheme, systemTheme } = useColorMode();
</script>

<template>
  <Field orientation="vertical">
    <FieldTitle>
      {{ $t("settings.appearance.uiTheme.label") }}
    </FieldTitle>
    <FieldDescription>
      {{ $t("settings.appearance.uiTheme.description") }}
    </FieldDescription>
    <RadioGroup class="grid-cols-3" v-model="colorTheme">
      <FieldLabel class="bg-background text-foreground cursor-pointer" :class="systemTheme">
        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle class="text-center">
              <LucideSunMoon class="size-5" />
              {{ $t("settings.appearance.uiTheme.options.system") }}
            </FieldTitle>
          </FieldContent>
          <RadioGroupItem value="system" />
        </Field>
      </FieldLabel>

      <FieldLabel class="bg-background text-foreground cursor-pointer light">
        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle class="text-center">
              <LucideSun class="size-5" />
              {{ $t("settings.appearance.uiTheme.options.light") }}
            </FieldTitle>
          </FieldContent>
          <RadioGroupItem value="light" />
        </Field>
      </FieldLabel>

      <FieldLabel class="bg-background text-foreground cursor-pointer dark">
        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle class="text-center">
              <LucideMoon class="size-5" />
              {{ $t("settings.appearance.uiTheme.options.dark") }}
            </FieldTitle>
          </FieldContent>
          <RadioGroupItem value="dark" />
        </Field>
      </FieldLabel>
    </RadioGroup>
  </Field>
</template>
