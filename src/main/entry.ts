import "@/index.css";
import { createApp } from "vue";
import i18n from "@/i18n";
import MainApp from "./MainApp.vue";

createApp(MainApp)
    .use(i18n)
    .mount("#app");
