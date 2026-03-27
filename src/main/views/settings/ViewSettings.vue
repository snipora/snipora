<script setup lang="ts">
import {useColorMode} from "@vueuse/core";
import {
  Field,
  FieldContent,
  FieldDescription,
  FieldLabel,
  FieldLegend,
  FieldSet,
  FieldSeparator,
  FieldTitle,
} from "@/components/ui/field";
import { FieldGroup } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Switch } from "@/components/ui/switch";
import {
  SniporaIconDark,
  SniporaIconLight,
  SniporaLogo,
} from "@/components/icons";
import {LucideMoon, LucideSun, LucideSunMoon} from "lucide-vue-next";
import {invokeSetTrayIcon} from "@/api/commands";
import {useAutostart} from "@/composables/useAutostart.ts";

const colorMode = useColorMode();
const systemColor = colorMode.system;
const storeColor = colorMode.store;

const autostartEnabled = useAutostart();
</script>

<template>
  <div class="flex flex-col gap-8">
    <FieldSet>
      <FieldLegend variant="legend">
        {{ $t("settings.general.title") }}
      </FieldLegend>

      <FieldGroup class="flex flex-col gap-6">
        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle>{{ $t("settings.general.startup.label") }}</FieldTitle>
            <FieldDescription>
              {{ $t("settings.general.startup.description") }}
            </FieldDescription>
          </FieldContent>
          <Switch :model-value="autostartEnabled" @update:model-value="e => autostartEnabled = e" />
        </Field>

        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle>{{ $t("settings.general.shortcut.label") }}</FieldTitle>
          </FieldContent>
          <Input
            :placeholder="$t('settings.general.shortcut.placeholder')"
            class="w-48"
          />
        </Field>
      </FieldGroup>
    </FieldSet>

    <FieldSeparator />

    <FieldSet>
      <FieldLegend variant="legend">
        {{ $t("settings.appearance.title") }}
      </FieldLegend>

      <FieldGroup class="flex flex-col gap-6">
        <Field orientation="vertical">
          <FieldTitle>{{ $t("settings.appearance.trayTheme.label") }}</FieldTitle>
          <FieldDescription>
            {{ $t("settings.appearance.trayTheme.description") }}
          </FieldDescription>
          <RadioGroup
              class="grid-cols-3"
              default-value="logo"
              @update:model-value="d => invokeSetTrayIcon(d as 'app')"
          >
            <FieldLabel>
              <Field orientation="horizontal">
                <FieldContent>
                  <div class="size-10 grid place-items-center rounded-md border bg-background text-foreground">
                    <SniporaLogo class="size-6" />
                  </div>
                  <FieldTitle class="text-center">
                    {{ $t("settings.appearance.trayTheme.options.logo") }}
                  </FieldTitle>
                </FieldContent>
                <RadioGroupItem value="app" />
              </Field>
            </FieldLabel>

            <FieldLabel>
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

            <FieldLabel>
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

        <Field orientation="vertical">
          <FieldTitle>{{ $t("settings.appearance.uiTheme.label") }}</FieldTitle>
          <FieldDescription>
            {{ $t("settings.appearance.uiTheme.description") }}
          </FieldDescription>
          <RadioGroup class="grid-cols-3" v-model="storeColor">
            <FieldLabel class="bg-background text-foreground" :class="systemColor">
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

            <FieldLabel class="bg-background text-foreground light">
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

            <FieldLabel class="bg-background text-foreground dark">
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
      </FieldGroup>
    </FieldSet>
  </div>
</template>
