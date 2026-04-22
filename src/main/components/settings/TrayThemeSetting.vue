<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldLabel, FieldTitle } from "@/components/ui/field";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { SniporaIconDark, SniporaIconLight, SniporaLogo } from "@/components/icons";
import { useLocalSettings } from "@/composables/useLocalSettings";

const trayTheme = useLocalSettings("tray.iconTheme");
</script>

<template>
  <Field orientation="vertical">
    <FieldTitle>
      {{ $t("settings.appearance.trayTheme.label") }}
    </FieldTitle>
    <FieldDescription>
      {{ $t("settings.appearance.trayTheme.description") }}
    </FieldDescription>
    <RadioGroup class="grid-cols-3" v-model="trayTheme">
      <FieldLabel class="cursor-pointer">
        <Field orientation="horizontal">
          <FieldContent>
            <div class="size-10 grid place-items-center rounded-md border bg-background text-foreground">
              <SniporaLogo class="size-6" />
            </div>
            <FieldTitle class="text-center">
              {{ $t("settings.appearance.trayTheme.options.app-icon") }}
            </FieldTitle>
          </FieldContent>
          <RadioGroupItem value="app-icon" />
        </Field>
      </FieldLabel>

      <FieldLabel class="cursor-pointer">
        <Field orientation="horizontal">
          <FieldContent>
            <div class="light size-10 grid place-items-center rounded-md border bg-background text-foreground">
              <SniporaIconLight />
            </div>
            <FieldTitle class="text-center">
              {{ $t("settings.appearance.trayTheme.options.light") }}
            </FieldTitle>
          </FieldContent>
          <RadioGroupItem value="light" />
        </Field>
      </FieldLabel>

      <FieldLabel class="cursor-pointer">
        <Field orientation="horizontal">
          <FieldContent>
            <div class="dark size-10 grid place-items-center rounded-md border bg-background text-foreground">
              <SniporaIconDark />
            </div>
            <FieldTitle class="text-center">
              {{ $t("settings.appearance.trayTheme.options.dark") }}
            </FieldTitle>
          </FieldContent>
          <RadioGroupItem value="dark" />
        </Field>
      </FieldLabel>
    </RadioGroup>
  </Field>
</template>
