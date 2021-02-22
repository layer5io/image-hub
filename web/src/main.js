import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify';
import router from './router'

Vue.config.productionTip = false

Vue.prototype.$BASE_URL = ""
// Vue.prototype.$BASE_URL = "http://0.0.0.0:9091"

new Vue({
  vuetify,
  router,
  render: h => h(App)
}).$mount('#app')
