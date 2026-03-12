import {createI18n} from "vue-i18n";
import messages from "@intlify/unplugin-vue-i18n/messages";
console.log(messages);

export default createI18n({
  locale: "en-US",
  messages,
});
