import { h, watch } from "vue";
import { toast, type ToastClasses } from "vue-sonner";
import { Progress } from "@/components/ui/progress";
import { useI18n } from "vue-i18n";
import { createSharedComposable } from "@vueuse/core";
import { useUpdater } from "@/composables/updater";

const CLASSES: ToastClasses = {
  content: "w-full",
  description: "w-full whitespace-pre-line",
};

export const useUpdaterToasts = createSharedComposable(() => {
  const { t } = useI18n();
  const updater = useUpdater();

  let toastId: string | number | undefined;

  function showProgressToast() {
    toastId = toast.loading(t('updater.downloading.title'), {
      id: toastId,
      description: () => {
        const pct = updater.progress.value;
        return !isNaN(pct) ? h(Progress, { modelValue: pct, class: "w-full" }) : null;
      },
      action: undefined,
      duration: Infinity,
      dismissible: false,
      closeButton: false,
      classes: CLASSES,
    });
  }

  watch(() => updater.status.value, (status, prevStatus) => {
    switch (status) {
      case "idle": {
        if (prevStatus === 'checking' && updater.error.value) {
          toastId = toast.error(t('updater.checking-failed.title'), {
            id: toastId,
            description: t('updater.checking-failed.description', { error: updater.error.value }),
            action: undefined,
            duration: Infinity,
            dismissible: true,
            closeButton: true,
            classes: CLASSES,
          });
          return;
        }
        break;
      }
      case "available": {
        if (prevStatus === "downloading" && updater.error.value) {
          toastId = toast.error(t('updater.download-failed.title'), {
            id: toastId,
            description: t('updater.download-failed.description', { error: updater.error.value }),
            action: undefined,
            duration: Infinity,
            dismissible: true,
            closeButton: true,
            classes: CLASSES,
          });
          return;
        }
        if (!updater.update.value) return;
        toastId = toast.info(
          t('updater.update-available.title', { version: updater.update.value.version }),
          {
            id: toastId,
            description: t('updater.update-available.description', {
              currentVersion: updater.update.value.currentVersion,
            }),
            action: {
              label: t('updater.update-available.action'),
              onClick: () => updater.downloadUpdate(),
            },
            duration: Infinity,
            dismissible: true,
            closeButton: true,
            classes: CLASSES,
          },
        );
        break;
      }
      case "downloading":
        showProgressToast();
        break;
      case "ready": {
        if (prevStatus === "installing") {
          toastId = toast.error(t('updater.install-failed.title'), {
            id: toastId,
            description: t('updater.install-failed.description', { error: updater.error.value }),
            action: undefined,
            duration: Infinity,
            dismissible: true,
            closeButton: true,
            classes: CLASSES,
          });
          return;
        }
        if (!updater.update.value) return;
        toastId = toast.info(t('updater.ready-to-install.title'), {
          id: toastId,
          description: t('updater.ready-to-install.description', {
            version: updater.update.value.version,
          }),
          action: {
            label: t('updater.ready-to-install.action'),
            onClick: () => updater.installUpdate(),
          },
          duration: Infinity,
          dismissible: false,
          closeButton: false,
          classes: CLASSES,
        });
        break;
      }
      case "installed": {
        if (!updater.update.value) return;
        toastId = toast.info(t('updater.installed.title'), {
          id: toastId,
          description: t('updater.installed.description', {
            version: updater.update.value.version,
          }),
          action: {
            label: t('updater.installed.action'),
            onClick: () => updater.relaunchApp(),
          },
          duration: Infinity,
          dismissible: true,
          closeButton: true,
          classes: CLASSES,
        });
        break;
      }
    }
  });

  watch(() => updater.progress.value, () => {
    if (updater.isDownloading.value) {
      showProgressToast();
    }
  });

  return {
    checkForUpdate: updater.checkForUpdate,
  };
});
