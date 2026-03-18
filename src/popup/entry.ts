import "@/index.css";
import "@/forward-console.ts";
import { createApp } from "vue";
import i18n from "@/i18n";
import PopupApp from "./PopupApp.vue";

createApp(PopupApp)
    .use(i18n)
    .mount("#app");
