import "@/index.css";
import "@/forward-console";
import "@/prevent-context-menu";
import {_initLocalSettings} from "@/composables/settings";
import {App, createApp} from "vue";
import i18n from "@/i18n";
import PopupApp from "./PopupApp.vue";

await _initLocalSettings();

createApp(PopupApp)
    .use(i18n)
    .use({
      install: (app: App) => {
        app.config.errorHandler = (err, instance, info) => {
          console.error(err, instance, info);
        }
      },
    })
    .mount("#app");
