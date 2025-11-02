<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import Sidebar from './components/Sidebar.vue'
import { NMessageProvider } from 'naive-ui'
import { RouterView, useRouter, useRoute } from 'vue-router'
import { getIsAuthenticated, setAuth } from './stores/authStore'
import { listen } from '@tauri-apps/api/event'

const activeMenu = ref('server')
const router = useRouter()
const route = useRoute()
let unlisten: any = null

const handleMenuChange = (menu: string) => {
  activeMenu.value = menu
}

onMounted(async () => {
  // 全局监听OAuth成功事件
  unlisten = await listen('oauth-success', (event: any) => {
    console.log('收到全局OAuth成功事件:', event)
    
    // 如果事件包含用户信息，直接更新认证状态
    if (event.payload && event.payload.access_token && event.payload.user) {
      console.log('更新认证状态:', event.payload)
      setAuth(event.payload.access_token, {
        id: event.payload.user.id,
        login: event.payload.user.login,
        avatar_url: event.payload.user.avatar_url,
        name: event.payload.user.name,
        email: event.payload.user.email
      })
      console.log('认证状态已更新，当前状态:', getIsAuthenticated())
    }
  })
})

onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})
</script>

<template>
  <n-message-provider>
    <div class="app">
      <div class="main-container">
        <Sidebar @menu-change="handleMenuChange" />
        <RouterView style="width: 100%;height: 100%;" />
      </div>
    </div>
  </n-message-provider>
</template>

<style scoped>
body{
  margin: 0 auto;
}
.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: #f5f5f5;
}

.main-container {
  display: flex;
  flex: 1;
  overflow: hidden;
  height: 100vh;
}
</style>