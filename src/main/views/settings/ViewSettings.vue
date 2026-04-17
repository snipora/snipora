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
import {LucideClipboardCopy, LucideMoon, LucidePencilLine, LucideSun, LucideSunMoon} from "@lucide/vue";
import {invokeSetTrayIcon} from "@/api/commands";
import {useAutostart} from "@/composables/useAutostart.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select";
import {DefaultLayout} from "@/main/layouts";

const colorMode = useColorMode();
const systemColor = colorMode.system;
const storeColor = colorMode.store;

const autostartEnabled = useAutostart();
</script>

<template>
  <DefaultLayout class="flex flex-col gap-8">
    <FieldSet>
      <FieldLegend variant="legend">
        {{ $t("settings.general.title") }}
      </FieldLegend>

      <FieldGroup class="flex flex-col gap-6">
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
              :model-value="autostartEnabled"
              @update:model-value="v => autostartEnabled = v"
              class="cursor-pointer"
          />
        </Field>

        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle>
              {{ $t("settings.general.shortcut.label") }}
            </FieldTitle>
            <FieldDescription>
              {{ $t("settings.general.shortcut.description") }}
            </FieldDescription>
          </FieldContent>
          <Input
              disabled
              :placeholder="$t('settings.general.shortcut.placeholder')"
              class="w-48"
          />
        </Field>

        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle>
              {{ $t('settings.general.use-behavior.label') }}
            </FieldTitle>
            <FieldDescription>
              {{ $t('settings.general.use-behavior.description') }}
            </FieldDescription>
          </FieldContent>
          <Select default-value="clipboard">
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="clipboard">
                <template #default>
                  <LucideClipboardCopy />
                  {{ $t('settings.general.use-behavior.clipboard.label') }}
                </template>
                <template #description>
                  {{ $t('settings.general.use-behavior.clipboard.description') }}
                </template>
              </SelectItem>
              <SelectItem value="paste" disabled>
                <template #default>
                  <LucidePencilLine />
                  {{ $t('settings.general.use-behavior.paste.label') }}
                </template>
                <template #description>
                  {{ $t('settings.general.use-behavior.paste.description') }}
                </template>
              </SelectItem>
            </SelectContent>
          </Select>
        </Field>
      </FieldGroup>
    </FieldSet>

    <FieldSeparator />

    <FieldSet>
      <FieldLegend variant="legend">
        {{ $t("settings.appearance.title") }}
      </FieldLegend>

      <FieldGroup class="flex flex-col gap-6">
        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle>
              {{ $t("settings.appearance.show-tag-counts.label") }}
            </FieldTitle>
            <FieldDescription>
              {{ $t("settings.appearance.show-tag-counts.description") }}
            </FieldDescription>
          </FieldContent>
          <Switch
              :model-value="true"
              disabled
              class="cursor-pointer"
          />
        </Field>

        <Field orientation="vertical">
          <FieldTitle>
            {{ $t("settings.appearance.trayTheme.label") }}
          </FieldTitle>
          <FieldDescription>
            {{ $t("settings.appearance.trayTheme.description") }}
          </FieldDescription>
          <RadioGroup
              class="grid-cols-3"
              default-value="app"
              @update:model-value="d => invokeSetTrayIcon(d as 'app')"
          >
            <FieldLabel class="cursor-pointer">
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

        <Field orientation="vertical">
          <FieldTitle>
            {{ $t("settings.appearance.uiTheme.label") }}
          </FieldTitle>
          <FieldDescription>
            {{ $t("settings.appearance.uiTheme.description") }}
          </FieldDescription>
          <RadioGroup class="grid-cols-3" v-model="storeColor">
            <FieldLabel class="bg-background text-foreground cursor-pointer" :class="systemColor">
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
      </FieldGroup>
    </FieldSet>
  </DefaultLayout>
</template>
