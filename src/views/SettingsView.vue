<template>
  <div class="settings-view">
    <h2>设置</h2>
    
    <div class="settings-container">
      <!-- GitHub账户设置 -->
      <div class="settings-section">
        <h3>GitHub账户</h3>
        
        <div v-if="isAuthenticated" class="github-account-info">
          <div class="account-header">
            <img :src="githubUser?.avatar_url" :alt="githubUser?.login" class="avatar" />
            <div class="account-details">
              <h4>{{ githubUser?.name || githubUser?.login }}</h4>
              <p>{{ githubUser?.login }}</p>
              <p v-if="githubUser?.email">{{ githubUser?.email }}</p>
            </div>
          </div>
          
          <div class="account-actions">
            <n-button type="error" @click="handleLogout">
              <template #icon>
                <n-icon>
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18">
                    <path fill="currentColor" d="M17 7l-1.41 1.41L18.17 11H8v2h10.17l-2.58 2.58L17 17l5-5zM4 5h8V3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8v-2H4V5z"/>
                  </svg>
                </n-icon>
              </template>
              登出GitHub
            </n-button>
          </div>
        </div>
        
        <div v-else class="github-login-prompt">
          <p>您尚未连接到GitHub账户</p>
          <n-button type="primary" @click="handleGitHubLogin">
            <template #icon>
              <n-icon>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" width="18" height="18">
                  <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/>
                </svg>
              </n-icon>
            </template>
            登录GitHub
          </n-button>
        </div>
      </div>
      
      <!-- 服务器配置同步设置 -->
      <div class="settings-section" v-if="isAuthenticated">
        <h3>服务器配置同步</h3>
        
        <div class="setting-item">
          <div>
            <h4>保存服务器配置到GitHub Gist</h4>
            <p>将当前连接的服务器配置保存到您的GitHub账户，以便在其他设备上同步</p>
          </div>
          <n-button type="primary" @click="saveServersToGist" :loading="savingToGist">
            保存到Gist
          </n-button>
        </div>
        
        <div class="setting-item">
          <div>
            <h4>从GitHub Gist加载服务器配置</h4>
            <p>从您的GitHub Gist中加载之前保存的服务器配置</p>
          </div>
          <n-button @click="showLoadModal" :loading="loadingFromGist">
            从Gist加载
          </n-button>
        </div>
        
        <div v-if="gistUrl" class="setting-item">
          <div>
            <h4>配置已保存</h4>
            <p>您的服务器配置已保存到以下Gist：</p>
            <a :href="gistUrl" target="_blank">{{ gistUrl }}</a>
          </div>
        </div>
      </div>
      
      <!-- 其他设置选项 -->
      <div class="settings-section">
        <h3>应用设置</h3>
        <div class="setting-item">
          <label>主题</label>
          <n-select v-model:value="theme" :options="themeOptions" />
        </div>
        <div class="setting-item">
          <label>自动检查更新</label>
          <n-switch v-model:value="autoUpdate" />
        </div>
      </div>
    </div>
    
    <!-- 从Gist加载服务器配置的模态框 -->
    <n-modal v-model:show="showLoadModalFlag" preset="dialog" title="从GitHub Gist加载服务器配置">
      <div style="padding: 20px;">
        <n-form>
          <n-form-item label="Gist URL">
            <n-input 
              v-model:value="gistUrlInput" 
              placeholder="请输入GitHub Gist URL"
            />
          </n-form-item>
          <div style="display: flex; justify-content: flex-end; gap: 10px;">
            <n-button @click="showLoadModalFlag = false">取消</n-button>
            <n-button type="primary" @click="loadServersFromGist" :loading="loadingFromGist">
              加载
            </n-button>
          </div>
        </n-form>
      </div>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { useMessage } from 'naive-ui'
import { useRouter } from 'vue-router'
import { getIsAuthenticated, getUser } from '../stores/authStore'
import { openGitHubLogin, logout } from '../services/githubAuthService'
import { saveServersToGist as saveServersToGistStore, loadServersFromGist as loadServersFromGistStore } from '../stores/serverStore'

const message = useMessage()
const router = useRouter()

// 认证状态 - 使用计算属性实现响应式
const isAuthenticated = computed(() => getIsAuthenticated())
const githubUser = computed(() => getUser())

// 设置选项
const theme = ref('light')
const autoUpdate = ref(true)
const themeOptions = [
  { label: '浅色', value: 'light' },
  { label: '深色', value: 'dark' },
  { label: '自动', value: 'auto' }
]

// Gist操作状态
const savingToGist = ref(false)
const loadingFromGist = ref(false)
const gistUrl = ref('')
const gistUrlInput = ref('')
const showLoadModalFlag = ref(false)



// 处理GitHub登录
const handleGitHubLogin = async () => {
  try {
    await openGitHubLogin()
    message.info('正在打开GitHub登录页面...')
  } catch (error) {
    console.error('创建GitHub登录URL失败:', error)
    message.error('创建GitHub登录URL失败')
  }
}

// 处理登出
const handleLogout = async () => {
  try {
    await logout()
    message.success('已成功登出GitHub账户')
  } catch (error) {
    console.error('登出失败:', error)
    message.error('登出失败')
  }
}

// 保存服务器配置到Gist
const saveServersToGist = async () => {
  if (!isAuthenticated.value) {
    message.warning('请先登录GitHub账户')
    return
  }
  
  try {
    savingToGist.value = true
    gistUrl.value = await saveServersToGistStore()
    message.success('服务器配置已成功保存到GitHub Gist')
  } catch (error) {
    console.error('保存到Gist失败:', error)
    message.error('保存到Gist失败: ' + (error as Error).message)
  } finally {
    savingToGist.value = false
  }
}

// 显示加载模态框
const showLoadModal = () => {
  if (!isAuthenticated.value) {
    message.warning('请先登录GitHub账户')
    return
  }
  showLoadModalFlag.value = true
}

// 从Gist加载服务器配置
const loadServersFromGist = async () => {
  if (!isAuthenticated.value) {
    message.warning('请先登录GitHub账户')
    return
  }
  
  if (!gistUrlInput.value) {
    message.warning('请输入Gist URL')
    return
  }
  
  try {
    loadingFromGist.value = true
    await loadServersFromGistStore(gistUrlInput.value)
    message.success('服务器配置已成功从GitHub Gist加载')
    showLoadModalFlag.value = false
    gistUrlInput.value = ''
  } catch (error) {
    console.error('从Gist加载失败:', error)
    message.error('从Gist加载失败: ' + (error as Error).message)
  } finally {
    loadingFromGist.value = false
  }
}

// 组件挂载时初始化
onMounted(() => {
  // 不再强制重定向到登录页面，允许未登录用户访问设置页面
})
</script>

<style scoped>
.settings-view {
  padding: 20px;
  background-color: #ffffff;
  border-radius: 4px;
  overflow: hidden;
}

.settings-view > h2 {
  margin-top: 0;
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 1px solid #e0e0e0;
}

.settings-container {
  max-width: 800px;
}

.settings-section {
  margin-bottom: 30px;
  padding: 20px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  background-color: #fafafa;
}

.settings-section h3 {
  margin-top: 0;
  margin-bottom: 15px;
  color: #333;
}

.github-account-info {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 15px;
  background-color: white;
  border-radius: 6px;
  border: 1px solid #e0e0e0;
}

.account-header {
  display: flex;
  align-items: center;
  gap: 15px;
}

.avatar {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  border: 2px solid #e0e0e0;
}

.account-details h4 {
  margin: 0 0 5px 0;
  font-size: 18px;
  color: #333;
}

.account-details p {
  margin: 3px 0;
  color: #666;
  font-size: 14px;
}

.account-actions {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 10px;
}

.github-login-prompt {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 15px;
  padding: 30px;
  text-align: center;
  background-color: white;
  border-radius: 6px;
  border: 1px solid #e0e0e0;
}

.github-login-prompt p {
  margin: 0;
  color: #666;
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
  padding: 10px 0;
  border-bottom: 1px solid #eee;
}

.setting-item:last-child {
  margin-bottom: 0;
  border-bottom: none;
}

.setting-item label {
  font-weight: 500;
  color: #333;
}

.setting-item h4 {
  margin: 0 0 5px 0;
  color: #333;
}

.setting-item p {
  margin: 0;
  color: #666;
  font-size: 14px;
}

@media (max-width: 768px) {
  .github-account-info {
    flex-direction: column;
    gap: 15px;
    text-align: center;
  }
  
  .account-actions {
    width: 100%;
    align-items: center;
  }
  
  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 10px;
  }
}
</style>