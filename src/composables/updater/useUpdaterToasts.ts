import { h, watch } from "vue";
import { toast } from "vue-sonner";
import { Progress } from "@/components/ui/progress";
import { useI18n } from "vue-i18n";
import { createSharedComposable } from "@vueuse/core";
import { useUpdater } from "@/composables/updater";

export const useUpdaterToasts = createSharedComposable(() => {
  const { t } = useI18n();
  const updater = useUpdater();

  let toastId: string | number | undefined;

  function showProgressToast() {
    toastId = toast.loading(t('updater.downloading.title'), {
      id: toastId,
      duration: Infinity,
      dismissible: false,
      closeButton: false,
      description: () => {
        const pct = updater.progress.value;
        return !isNaN(pct) ? h(Progress, { modelValue: pct, class: "w-full" }) : null;
      },
    });
  }

  watch(() => updater.status.value, (status, prevStatus) => {
    switch (status) {
      case "idle": {
        if (prevStatus === 'checking' && updater.error.value) {
          toastId = toast.error(t('updater.checking-failed.title'), {
            id: toastId,
            description: t('updater.checking-failed.description', { error: updater.error.value }),
            classes: {
              description: "whitespace-pre-line",
            },
            duration: Infinity,
            dismissible: true,
            closeButton: true,
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
            classes: {
              description: "whitespace-pre-line",
            },
            duration: Infinity,
            dismissible: true,
            closeButton: true,
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
            duration: Infinity,
            dismissible: true,
            closeButton: true,
            action: {
              label: t('updater.update-available.action'),
              onClick: () => updater.downloadUpdate(),
            },
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
            classes: {
              description: "whitespace-pre-line",
            },
            duration: Infinity,
            dismissible: true,
            closeButton: true,
          });
          return;
        }
        if (!updater.update.value) return;
        toastId = toast.info(t('updater.ready-to-install.title'), {
          id: toastId,
          description: t('updater.ready-to-install.description', {
            version: updater.update.value.version,
          }),
          duration: Infinity,
          dismissible: false,
          closeButton: false,
          action: {
            label: t('updater.ready-to-install.action'),
            onClick: () => updater.installUpdate(),
          },
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
          duration: Infinity,
          dismissible: true,
          closeButton: true,
          action: {
            label: t('updater.installed.action'),
            onClick: () => updater.relaunchApp(),
          },
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
