import "@/index.css";
import "@/forward-console.ts";
import {_initLocalSettings} from "@/composables/useLocalSettings.ts";
import {App, createApp} from "vue";
import i18n from "@/i18n";
import MainApp from "./MainApp.vue";

await _initLocalSettings();

createApp(MainApp)
    .use(i18n)
    .use({
      install: (app: App) => {
        app.config.errorHandler = (err, instance, info) => {
          console.error(err, instance, info);
        }
      },
    })
    .mount("#app");

if (import.meta.env.DEV) {
  document.addEventListener("keydown", (event) => {
    if ((event.ctrlKey && event.key === "r") || (event.key === "F5")) {
      event.preventDefault();
      window.location.reload();
    }
  });
}
