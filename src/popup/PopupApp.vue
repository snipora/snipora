<script setup lang="ts">
import {useSmartPopupHeight} from "@/popup/composables/useSmartPopupHeight.ts";
import {usePopupEscapeListener} from "@/popup/composables/usePopupEscapeListener.ts";
import {useColorMode} from "@vueuse/core";

useColorMode({ writeDefaults: false });
useSmartPopupHeight();
usePopupEscapeListener();
</script>

<template>
  <div class="h-20 bg-background rounded-lg border border-border grid place-items-center">
    Popup App {{ $t('hello-world') }}
  </div>
</template>
