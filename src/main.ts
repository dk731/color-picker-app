import { createApp } from "vue";
import { createPinia } from "pinia";
import "./style.css";
import App from "./App.vue";
import router from "./router";
import gsap from "gsap";
import { MotionPathPlugin } from "gsap/MotionPathPlugin";

const app = createApp(App);
const pinia = createPinia();

gsap.registerPlugin(MotionPathPlugin);

app.use(pinia);
app.use(router);

app.mount("#app");
