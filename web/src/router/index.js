import Vue from 'vue'
import VueRouter from 'vue-router'

import Hello from '../views/Hello.vue'
import Auth from '../views/Auth.vue'
import Signup from '../views/Signup.vue'
import Upgrade from '../views/Upgrade.vue'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'Hello',
    component: Hello
  },{
    path: '/login',
    name: 'Auth',
    component: Auth
  },{
    path: '/signup',
    name: 'Signup',
    component: Signup
  },{
    path: '/upgrade',
    name: 'Upgrade',
    component: Upgrade
  }
]

const router = new VueRouter({
  // mode: 'history',
  // base: process.env.BASE_URL,
  routes
})

export default router
