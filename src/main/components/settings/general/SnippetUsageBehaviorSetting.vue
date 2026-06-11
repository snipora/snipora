<script setup lang="ts">
import { Field, FieldContent, FieldDescription, FieldLabel, FieldTitle } from "@/components/ui/field";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useLocalSettings } from "@/composables/useLocalSettings.js";
import { LucideCircleAlert, LucideCircleQuestionMark, LucideClipboardCopy, LucideClipboardPaste, LucideKeyboard } from "@lucide/vue";

const snippetUsageBehavior = useLocalSettings("general.snippetUsageBehavior");
</script>

<template>
  <Field orientation="vertical">
    <FieldTitle>
      {{ $t('setting.general.snippet-usage-behavior.label') }}
    </FieldTitle>
    <FieldDescription>
      {{ $t('setting.general.snippet-usage-behavior.description') }}
    </FieldDescription>
    <RadioGroup v-model="snippetUsageBehavior">
      <FieldLabel class="bg-background text-foreground cursor-pointer">
        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle>
              <LucideClipboardCopy class="size-4" />
              {{ $t('setting.general.snippet-usage-behavior.copy-to-clipboard.label') }}
            </FieldTitle>
            <FieldDescription>
              {{ $t('setting.general.snippet-usage-behavior.copy-to-clipboard.description') }}
              <Tooltip>
                <TooltipTrigger as-child>
                  <LucideCircleQuestionMark class="inline-block size-4 align-text-bottom" />
                </TooltipTrigger>
                <TooltipContent>
                  {{ $t('setting.general.snippet-usage-behavior.copy-to-clipboard.tooltip') }}
                </TooltipContent>
              </Tooltip>
            </FieldDescription>
          </FieldContent>
          <RadioGroupItem value="copy-to-clipboard" />
        </Field>
      </FieldLabel>

      <FieldLabel class="bg-background text-foreground cursor-pointer opacity-50 pointer-events-none">
        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle>
              <LucideClipboardPaste class="size-4" />
              {{ $t('setting.general.snippet-usage-behavior.simulate-paste.label') }}
            </FieldTitle>
            <FieldDescription>
              {{ $t('setting.general.snippet-usage-behavior.simulate-paste.description') }}
            </FieldDescription>
          </FieldContent>
          <RadioGroupItem value="simulate-paste" disabled />
        </Field>
      </FieldLabel>

      <FieldLabel class="bg-background text-foreground cursor-pointer">
        <Field orientation="horizontal">
          <FieldContent>
            <FieldTitle>
              <LucideKeyboard class="size-4" />
              {{ $t('setting.general.snippet-usage-behavior.natural-typing.label') }}
            </FieldTitle>
            <FieldDescription>
              {{ $t('setting.general.snippet-usage-behavior.natural-typing.description') }}
              <Tooltip>
                <TooltipTrigger as-child>
                  <LucideCircleAlert class="inline-block size-4 align-text-bottom" />
                </TooltipTrigger>
                <TooltipContent>
                  {{ $t('setting.general.snippet-usage-behavior.natural-typing.tooltip') }}
                </TooltipContent>
              </Tooltip>
            </FieldDescription>
          </FieldContent>
          <RadioGroupItem value="natural-typing" />
        </Field>
      </FieldLabel>
    </RadioGroup>
  </Field>
</template>
