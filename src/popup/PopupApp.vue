<script setup lang="ts">
import {useSmartPopupHeight} from "@/composables/useSmartPopupHeight.ts";
import {usePopupEscapeListener} from "@/composables/usePopupEscapeListener.ts";

useSmartPopupHeight();
usePopupEscapeListener();
</script>

<template>
  <div class="h-20 bg-background rounded-lg border border-border grid place-items-center">
    Popup App {{ $t('hello-world') }}
  </div>
</template>
