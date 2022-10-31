import { createApp } from "vue";
import App from "./service/App.vue";
import router from "./service/router";
import store from "./service/store";
import vuetify from "./service/plugins/vuetify";
import { loadFonts } from "./service/plugins/webfontloader";

loadFonts();

createApp(App).use(router).use(store).use(vuetify).mount("#app");
