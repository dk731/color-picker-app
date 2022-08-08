import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      redirect: "main_app",
    },
    {
      path: "/main_app",
      name: "MainApp",
      component: () => import("../views/MainAppView.vue"),
    },
    {
      path: "/color_pick",
      name: "ColorPick",
      component: () => import("../views/ColorPickingView.vue"),
    },
  ],
});

export default router;
