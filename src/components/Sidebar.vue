<template>
  <div class="sidebar">
    <div class="menu-top">
      <div 
        class="menu-item" 
        :class="{ active: activeMenu === 'server' }"
        @click="setActiveMenu('server')"
      >
        <div class="menu-icon">🖥️</div>
        <div class="menu-text">服务器</div>
      </div>
      <div 
        class="menu-item" 
        :class="{ active: activeMenu === 'data' }"
        @click="setActiveMenu('data')"
      >
        <div class="menu-icon">📊</div>
        <div class="menu-text">数据浏览</div>
      </div>
    </div>
    <div class="menu-bottom">
      <div 
        class="menu-item" 
        :class="{ active: activeMenu === 'settings' }"
        @click="setActiveMenu('settings')"
      >
        <div class="menu-icon">⚙️</div>
        <div class="menu-text">设置</div>
      </div>
      <div 
        class="menu-item" 
        :class="{ active: activeMenu === 'github' }"
        @click="openGitHub"
      >
        <div class="menu-icon">🐙</div>
        <div class="menu-text">GITHUB</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { openGitHubLogin } from '../services/githubAuthService'

// 定义事件发射器
const emit = defineEmits(['menu-change'])

const activeMenu = ref('server')
const router = useRouter()
const route = useRoute()

// 根据当前路由设置活动菜单
const updateActiveMenuFromRoute = () => {
  switch (route.path) {
    case '/server':
      activeMenu.value = 'server'
      break
    case '/data':
      activeMenu.value = 'data'
      break
    default:
      activeMenu.value = 'server'
      break
  }
  emit('menu-change', activeMenu.value)
}

const setActiveMenu = (menu: string) => {
  activeMenu.value = menu
  // 发送事件给父组件
  emit('menu-change', menu)
  
  // 导航到相应路由
  switch (menu) {
    case 'server':
      router.push('/server')
      break
    case 'data':
      router.push('/data')
      break
    case 'settings':
      router.push('/settings')
      break
  }
}

const openGitHub = async () => {
  try {
    // 打开GitHub登录页面
    await openGitHubLogin()
  } catch (error) {
    console.error('打开GitHub登录页面失败:', error)
    // 可以在这里添加用户提示
  }
}

// 监听路由变化
watch(() => route.path, () => {
  updateActiveMenuFromRoute()
})

// 组件挂载时根据当前路由设置菜单
onMounted(() => {
  updateActiveMenuFromRoute()
})
</script>

<style scoped>
.sidebar {
  width: 60px;
  height: 100%;
  background-color: #ffffff;
  color: #333333;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  border-right: 1px solid #e0e0e0;
}

.menu-top {
  display: flex;
  flex-direction: column;
}

.menu-bottom {
  display: flex;
  flex-direction: column;
}

.menu-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 0;
  cursor: pointer;
  transition: background-color 0.2s;
  color: #666666;
}

.menu-item:hover {
  background-color: #f5f5f5;
}

.menu-item.active {
  background-color: #e0e0e0;
  color: #333333;
  font-weight: 500;
}

.menu-icon {
  font-size: 18px;
  margin-bottom: 4px;
}

.menu-text {
  font-size: 12px;
}
</style>