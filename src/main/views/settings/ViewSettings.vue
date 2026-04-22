<script setup lang="ts">
import { FieldGroup } from "@/components/ui/field";
import { FieldSet, FieldLegend, FieldSeparator } from "@/components/ui/field";
import { DefaultLayout } from "@/main/layouts";
import {
  AutostartSetting,
  ShortcutSetting,
  SnippetUsageBehaviorSetting,
  ShowTagCountsSetting,
  TrayThemeSetting,
  UiThemeSetting,
} from "@/main/components/settings";
</script>

<template>
  <DefaultLayout class="flex flex-col gap-8">
    <FieldSet>
      <FieldLegend variant="legend">
        {{ $t("settings.general.title") }}
      </FieldLegend>

      <FieldGroup class="flex flex-col gap-6">
        <AutostartSetting />
        <ShortcutSetting />
        <SnippetUsageBehaviorSetting />
      </FieldGroup>
    </FieldSet>

    <FieldSeparator />

    <FieldSet>
      <FieldLegend variant="legend">
        {{ $t("settings.appearance.title") }}
      </FieldLegend>

      <FieldGroup class="flex flex-col gap-6">
        <ShowTagCountsSetting />
        <TrayThemeSetting />
        <UiThemeSetting />
      </FieldGroup>
    </FieldSet>
  </DefaultLayout>
</template>
