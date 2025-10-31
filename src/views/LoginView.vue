<template>
  <div class="login-page">
    <div class="login-content">
      <h2>GitHub认证</h2>
      <p v-if="message">{{ message }}</p>
      <n-spin v-if="loading" size="large" />
      <n-button v-else @click="startGitHubAuth" type="primary" size="large">
        使用GitHub登录
      </n-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { listen } from '@tauri-apps/api/event'
import { openUrl } from '@tauri-apps/plugin-opener'
import { invoke } from '@tauri-apps/api/core'
import { getIsAuthenticated, setAuth } from '../stores/authStore'

const router = useRouter()
const message = ref('点击下方按钮开始GitHub认证')
const loading = ref(false)
let unlisten: any = null

onMounted(async () => {
  // 检查是否已经认证
  if (getIsAuthenticated()) {
    router.replace('/server')
    return
  }
  
  // 监听OAuth成功事件
  unlisten = await listen('oauth-success', (event: any) => {
    message.value = '认证成功！正在跳转...'
    loading.value = false
    
    // 如果事件包含用户信息，直接更新认证状态
    if (event.payload && event.payload.access_token && event.payload.user) {
      setAuth(event.payload.access_token, {
        id: event.payload.user.id,
        login: event.payload.user.login,
        avatar_url: event.payload.user.avatar_url,
        name: event.payload.user.name,
        email: event.payload.user.email
      })
    }
    
    // 等待一小段时间确保认证状态已更新
    setTimeout(() => {
      // 验证认证状态
      if (getIsAuthenticated()) {
        router.replace('/server')
      } else {
        message.value = '认证状态无效，请重试'
        loading.value = false
      }
    }, 500)
  })
})

onUnmounted(() => {
  // 组件卸载时取消监听
  if (unlisten) {
    unlisten()
  }
})

const startGitHubAuth = async () => {
  try {
    loading.value = true
    message.value = '正在启动认证流程...'
    
    // 获取GitHub授权URL
    const authUrl = await invoke<string>('start_github_auth')
    
    message.value = '请在浏览器中完成GitHub授权...'
    
    // 在默认浏览器中打开授权页面
    await openUrl(authUrl)
    
  } catch (error) {
    console.error('启动GitHub认证失败:', error)
    message.value = '启动认证失败，请重试'
    loading.value = false
  }
}
</script>

<style scoped>
.login-page {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background-color: #f5f5f5;
}

.login-content {
  text-align: center;
  padding: 40px;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  min-width: 350px;
}

.login-content h2 {
  margin-top: 0;
  margin-bottom: 20px;
  color: #333;
}

.login-content p {
  margin-bottom: 30px;
  color: #666;
}
</style>