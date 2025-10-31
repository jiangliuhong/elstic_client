<template>
  <div class="title-bar">
    <div class="title-bar-left">
      <div class="app-title">ElasticSearch</div>
    </div>
    <div class="title-bar-right">
      <!-- GitHub登录状态显示 -->
      <div v-if="isAuthenticated" class="github-user-info" @click="handleUserClick">
        <img 
          v-if="githubUser?.avatar_url" 
          :src="githubUser.avatar_url" 
          :alt="githubUser?.login"
          class="user-avatar"
        />
        <span class="username">{{ githubUser?.login }}</span>
      </div>
      
      <!-- GitHub登录按钮 -->
      <button 
        v-if="!isAuthenticated" 
        class="github-login-btn"
        @click="handleGitHubLogin"
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/>
        </svg>
        <span>登录GitHub</span>
      </button>
      
      <button class="window-control minimize">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <path d="M0 6h12v1H0z" fill="currentColor"></path>
        </svg>
      </button>
      <button class="window-control maximize">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <path
            d="M1 1h10v10H1V1zm1 1v8h8V2H2z"
            fill="currentColor"
            stroke="currentColor"
            stroke-width="0.5"
          ></path>
        </svg>
      </button>
      <button class="window-control close">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <path
            d="M6 4.586L10.293.293l1.414 1.414L7.414 6l4.293 4.293-1.414 1.414L6 7.414l-4.293 4.293-1.414-1.414L4.586 6 .293 1.707 1.707.293 6 4.586z"
            fill="currentColor"
          ></path>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { getIsAuthenticated, getUser, isAuthenticated as authState } from '../stores/authStore'
import { createGitHubAuthUrl } from '../services/githubAuthService'
import { useMessage } from 'naive-ui'

const isAuthenticated = ref(false)
const githubUser = ref(null)
const message = useMessage()

// 检查认证状态
const checkAuthStatus = () => {
  isAuthenticated.value = getIsAuthenticated()
  if (isAuthenticated.value) {
    githubUser.value = getUser()
  }
}

// 处理GitHub登录
const handleGitHubLogin = async () => {
  try {
    // 创建GitHub授权URL并打开
    const authUrl = await createGitHubAuthUrl()
    window.open(authUrl, '_blank')
  } catch (error) {
    console.error('创建GitHub登录URL失败:', error)
    message.error('创建GitHub登录URL失败')
  }
}

// 处理用户点击（可以显示用户菜单）
const handleUserClick = () => {
  console.log('用户信息被点击')
}

// 监听认证状态变化
watch(authState, () => {
  checkAuthStatus()
})

// 组件挂载时检查认证状态
onMounted(() => {
  checkAuthStatus()
})
</script>

<style scoped>
.title-bar {
  height: 100px;
  background-color: #333;
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 16px;
  -webkit-app-region: drag;
}

.title-bar-left {
  font-size: 24px;
  font-weight: bold;
}

.title-bar-right {
  display: flex;
  gap: 16px;
  -webkit-app-region: no-drag;
  align-items: center;
}

.github-login-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  background-color: #24292e;
  color: white;
  border: 1px solid rgba(27, 31, 35, 0.15);
  border-radius: 6px;
  padding: 6px 12px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.github-login-btn:hover {
  background-color: #2c3238;
}

.github-user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 6px;
  transition: background-color 0.2s;
}

.github-user-info:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.user-avatar {
  width: 24px;
  height: 24px;
  border-radius: 50%;
}

.username {
  font-size: 14px;
  font-weight: 500;
}

.window-control {
  width: 40px;
  height: 30px;
  background: none;
  border: none;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.window-control:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.window-control.close:hover {
  background-color: #e81123;
}
</style>