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
  InfoLogsLocation,
  QuitSniporaButton,
} from "@/main/components/settings";
import {
  LucideChevronDown, LucideChevronUp,
  LucideNotebookText,
  LucidePalette,
  LucideRefreshCw,
  LucideSlidersHorizontal
} from "@lucide/vue";
import { useLocalStorage } from "@vueuse/core";
import { Button } from "@/components/ui/button";

const infoOpen = useLocalStorage("setting:info-open", import.meta.env.DEV, {
  listenToStorageChanges: false,
});
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
      <FieldLegend variant="legend" :data-open="infoOpen" class="data-[open=false]:mb-0">
        <LucideNotebookText />
        {{ $t('setting.info.title') }}
        <Button variant="ghost" size="icon-sm" class="ml-auto" @click="infoOpen = !infoOpen">
          <LucideChevronUp v-if="infoOpen" />
          <LucideChevronDown v-else />
        </Button>
      </FieldLegend>
      <FieldGroup v-if="infoOpen" class="flex flex-col gap-6">
        <InfoAppVersion />
        <InfoRuntimeInfo />
        <InfoLogsLocation />
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
