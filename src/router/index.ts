import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'
import ServerView from '../views/ServerView.vue'
import DataView from '../views/DataView.vue'
import SettingsView from '../views/SettingsView.vue'
import LoginView from '../views/LoginView.vue'

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
  },
  {
    path: '/settings',
    name: 'Settings',
    component: SettingsView
  },
  {
    path: '/login',
    name: 'Login',
    component: LoginView
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

// 路由守卫：简化认证检查
router.beforeEach((to, from, next) => {
  // 从localStorage检查认证状态
  const isAuthenticated = localStorage.getItem('github_access_token') !== null
  
  if (to.path === '/login' && isAuthenticated) {
    // 如果是登录页面但已认证，重定向到主页
    next('/server')
  } else {
    // 否则正常导航，移除其他路由的登录限制
    next()
  }
})

export default router