<template>
  <div class="callback-page">
    <div class="callback-content">
      <h2>GitHub认证中...</h2>
      <n-spin size="large" />
      <p v-if="message">{{ message }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useMessage } from 'naive-ui'
import { listen } from '@tauri-apps/api/event'
import { checkAuthStatus, getCurrentUser } from '../stores/authStore'

import { UnlistenFn } from '@tauri-apps/api/event'

const route = useRoute()
const router = useRouter()
const message = ref('正在处理GitHub认证...')
let unlisten: UnlistenFn | null = null

onMounted(async () => {
  try {
    message.value = '请在浏览器中完成GitHub登录...'
    
    // 监听OAuth成功事件
    unlisten = await listen('oauth-success', () => {
      message.value = '认证成功！正在跳转...'
      console.log('收到OAuth成功事件，准备跳转到 /server')
      
      // 等待一小段时间确保认证状态已更新
      setTimeout(() => {
        // 验证认证状态
        if (checkAuthStatus()) {
          console.log('认证状态验证成功，执行路由跳转')
          router.replace('/server').catch(err => {
            console.error('路由跳转失败:', err)
            message.value = '跳转失败，请手动导航到服务器页面'
          })
        } else {
          console.error('认证状态验证失败')
          message.value = '认证状态无效，请重试'
        }
      }, 1000)
    })

    // 也检查当前URL是否包含code参数（兼容直接重定向的场景）
    const code = route.query.code as string
    if (code) {
      await processGitHubCode(code)
    }
  } catch (error) {
    console.error('认证过程中出现错误:', error)
    message.value = '认证过程中出现错误'
  }
})

onUnmounted(() => {
  // 组件卸载时取消监听
  if (unlisten) {
    unlisten()
  }
})

const processGitHubCode = async (code: string) => {
  try {
    console.log('Processing GitHub code:', code)
    // 处理GitHub回调
    const success = await handleGitHubCallback(code)
    console.log('Handle callback result:', success)
    
    if (success) {
      message.value = '认证成功！正在跳转...'
      console.log('认证成功，准备跳转到 /server')
      // 立即跳转到主页
      router.replace('/server').catch(err => {
        console.error('路由跳转失败:', err)
        message.value = '跳转失败，请手动导航到服务器页面'
      })
    } else {
      message.value = '认证失败，请重试'
    }
  } catch (error) {
    console.error('认证过程中出现错误:', error)
    message.value = '认证过程中出现错误: ' + error
  }
}
</script>

<style scoped>
.callback-page {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background-color: #f5f5f5;
}

.callback-content {
  text-align: center;
  padding: 30px;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.callback-content h2 {
  margin-top: 0;
  color: #333;
}

.callback-content p {
  margin-top: 20px;
  color: #666;
}
</style>