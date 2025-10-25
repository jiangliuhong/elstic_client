import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'
import ServerView from '../views/ServerView.vue'
import DataView from '../views/DataView.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/server'
  },
  {
    path: '/server',
    name: 'Server',
    component: ServerView
  },
  {
    path: '/data',
    name: 'Data',
    component: DataView
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router