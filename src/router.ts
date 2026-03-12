import {createRouter, createWebHashHistory} from "vue-router";
import { routes, handleHotUpdate } from "vue-router/auto-routes";

const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});
export default router;

if (import.meta.hot) {
  handleHotUpdate(router);
}
