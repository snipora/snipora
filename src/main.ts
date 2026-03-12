import "@/index.css";
import { createApp } from "vue";
import router from "@/router.ts"
import i18n from "@/i18n";
import AppWrapper from "@/AppWrapper.vue";

try {
  createApp(AppWrapper)
      .use(router)
      .use(i18n)
      .mount("#app");
} catch (error) {
  console.error(error);
}
