import { createApp } from "vue";
import App from "./App.vue";

// 禁止全局右键菜单
document.addEventListener("contextmenu", (e) => {
  e.preventDefault();
});

createApp(App).mount("#app");
