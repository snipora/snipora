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
  ManualCheckButton,
  InfoAppVersion,
  InfoRuntimeInfo,
  QuitSniporaButton,
} from "@/main/components/settings";
import {LucideNotebookText, LucidePalette, LucideRefreshCw, LucideSlidersHorizontal} from "@lucide/vue";
</script>

<template>
  <DefaultLayout class="flex flex-col gap-8">
    <FieldSet>
      <FieldLegend variant="legend">
        <LucideSlidersHorizontal />
        {{ $t('setting.general.title') }}
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
        <LucidePalette />
        {{ $t('setting.appearance.title') }}
      </FieldLegend>
      <FieldGroup class="flex flex-col gap-6">
        <ShowTagCountsSetting />
        <UiThemeSetting />
        <TrayThemeSetting />
      </FieldGroup>
    </FieldSet>
    <FieldSeparator />
    <FieldSet>
      <FieldLegend variant="legend">
        <LucideRefreshCw />
        {{ $t('setting.updates.title') }}
      </FieldLegend>
      <FieldGroup class="flex flex-col gap-6">
        <AutoCheckForUpdatesSetting />
        <ManualCheckButton />
      </FieldGroup>
    </FieldSet>
    <FieldSeparator />
    <FieldSet>
      <FieldLegend variant="legend">
        <LucideNotebookText />
        {{ $t('setting.info.title') }}
      </FieldLegend>
      <FieldGroup class="flex flex-col gap-6">
        <InfoAppVersion />
        <InfoRuntimeInfo />
      </FieldGroup>
    </FieldSet>
    <FieldSeparator />
    <FieldSet>
      <FieldGroup class="flex flex-col gap-6">
        <QuitSniporaButton />
      </FieldGroup>
    </FieldSet>
  </DefaultLayout>
</template>
