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
  AutoCheckForUpdatesSetting,
  InfoAppVersion,
  InfoRuntimeInfo,
} from "@/main/components/settings";
</script>

<template>
  <DefaultLayout class="flex flex-col gap-8">
    <FieldSet>
      <FieldLegend variant="legend">
        {{ $t("settings.general.title") }}
      </FieldLegend>
      <FieldGroup class="flex flex-col gap-6">
        <ShortcutSetting />
        <SnippetUsageBehaviorSetting />
        <AutostartSetting />
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
    <FieldSeparator />
    <FieldSet>
      <FieldLegend variant="legend">
        {{ $t('settings.updates.title') }}
      </FieldLegend>
      <FieldGroup class="flex flex-col gap-6">
        <AutoCheckForUpdatesSetting />
      </FieldGroup>
    </FieldSet>
    <FieldSeparator />
    <FieldSet>
      <FieldLegend variant="legend">
        {{ $t("settings.info.title") }}
      </FieldLegend>
      <FieldGroup class="flex flex-col gap-6">
        <InfoAppVersion />
        <InfoRuntimeInfo />
      </FieldGroup>
    </FieldSet>
  </DefaultLayout>
</template>
