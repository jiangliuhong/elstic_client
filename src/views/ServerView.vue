<template>
  <div class="server-view">

    
    <div class="server-layout">
      <!-- 左侧服务器列表 -->
      <div class="server-list-panel">
        <div class="server-controls-container">
          <div class="controls-left">
            <n-input 
              v-model:value="filterText" 
              placeholder="搜索服务器名称" 
              clearable
              style="width: 100%;"
            />
          </div>
          <div class="controls-right">
            <n-button @click="refreshServers" title="刷新" style="margin-right: 8px;">
              <template #icon>
                <n-icon>
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16">
                    <path fill="currentColor" d="M17.65 6.35A7.958 7.958 0 0 0 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0 1 12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
                  </svg>
                </n-icon>
              </template>
            </n-button>
            <n-button type="primary" @click="addServer" title="添加服务器">
              <template #icon>
                <n-icon>
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16">
                    <path fill="currentColor" d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
                  </svg>
                </n-icon>
              </template>
            </n-button>
          </div>
        </div>
        <div class="server-list">
          <div 
            class="server-item" 
            v-for="server in filteredServers" 
            :key="server.id"
            :class="{ active: selectedServer?.id === server.id }"
            @click="selectServer(server)"
          >
            <div class="server-info">
              <div class="server-name">{{ server.name }}</div>
              <div class="server-url">{{ server.url }}</div>
            </div>
            <div class="server-status">
              <span 
                v-if="server.connected" 
                class="status-indicator connected"
                title="已连接"
              >●</span>
              <span 
                v-else-if="server.connectionError" 
                class="status-indicator error"
                title="连接失败"
              >●</span>
              <span 
                v-else 
                class="status-indicator disconnected"
                title="未连接"
              >●</span>
            </div>
            <div class="server-actions">
              <n-button 
                size="small" 
                :type="server.connected ? 'success' : 'primary'" 
                :disabled="server.connected"
                @click.stop="connectToServer(server.id)"
              >
                {{ server.connected ? '已连接' : '连接' }}
              </n-button>
              <n-button size="small" type="error" @click.stop="removeServer(server.id)">删除</n-button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 右侧服务器详情/编辑 -->
      <div class="server-detail-panel">
        <div v-if="hasServerData()" class="server-detail">
          <h3>{{ getFormTitle() }}</h3>
          <n-form ref="formRef" :model="formModel" :rules="rules" :disabled="mode === 'view'">
            <n-form-item label="服务器名称" path="name" :show-require-mark="true">
              <n-input 
                v-model:value="formModel.name" 
                placeholder="请输入服务器名称"
              />
            </n-form-item>
            <n-form-item label="服务器地址" path="url" :show-require-mark="true">
              <n-input 
                v-model:value="formModel.url" 
                placeholder="请输入服务器地址"
              />
            </n-form-item>
            <n-form-item label="用户名" path="username">
              <n-input 
                v-model:value="formModel.username"
                placeholder="请输入用户名"
              />
            </n-form-item>
            <n-form-item label="密码" path="password">
              <n-input 
                v-model:value="formModel.password"
                type="password"
                placeholder="请输入密码"
              />
            </n-form-item>
            <div class="form-actions" v-if="mode !== 'view'">
              <n-button type="primary" @click="handleSubmit">{{ isEditing ? '更新' : '保存' }}</n-button>
              <n-button @click="cancelForm">取消</n-button>
            </div>
            <div class="form-actions" v-else>
              <n-button @click="editServer(selectedServer!)">编辑</n-button>
            </div>
          </n-form>
        </div>
        <div v-else class="no-selection">
          <p>请选择一个服务器查看详情</p>
          <p>或点击"添加服务器"按钮添加新服务器</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useMessage } from 'naive-ui'
import type { FormInst, FormRules } from 'naive-ui'
import { addConnectedServer, setActiveServer } from '../stores/serverStore'
import { useRouter } from 'vue-router'

interface Server {
  id: string
  name: string
  url: string
  username?: string
  password?: string
  connected?: boolean
  connectionError?: string
}

const servers = ref<Server[]>([
  { id: '1', name: '本地服务器', url: 'http://139.159.160.236:9200', username: 'elastic', password: '!Imprt^Gk', connected: false }
])

const selectedServer = ref<Server | null>(null)
const mode = ref<'view' | 'form'>('view') // 'view' or 'form'
const isEditing = ref(false)
const formRef = ref<FormInst | null>(null)
const filterText = ref('') // 添加过滤文本

// 统一的表单模型
const formModel = ref({
  id: '',
  name: '',
  url: '',
  username: '',
  password: ''
})

const message = useMessage()
const router = useRouter()

// 过滤后的服务器列表
const filteredServers = computed(() => {
  if (!filterText.value) {
    return servers.value
  }
  const filter = filterText.value.toLowerCase()
  return servers.value.filter(server => 
    server.name.toLowerCase().includes(filter)
  )
})

const rules: FormRules = {
  name: {
    required: true,
    message: '请输入服务器名称',
    trigger: ['input', 'blur']
  },
  url: {
    required: true,
    message: '请输入服务器地址',
    trigger: ['input', 'blur']
  }
}

const hasServerData = () => {
  return selectedServer.value || mode.value === 'form'
}

const getFormTitle = () => {
  if (mode.value === 'form') {
    return isEditing.value ? '编辑服务器' : '添加新服务器'
  }
  return '服务器信息'
}

const addServer = () => {
  mode.value = 'form'
  isEditing.value = false
  formModel.value = {
    id: '',
    name: '',
    url: '',
    username: '',
    password: ''
  }
}

const editServer = (server: Server) => {
  mode.value = 'form'
  isEditing.value = true
  // 保持选中的服务器不变
  formModel.value = {
    id: server.id,
    name: server.name,
    url: server.url,
    username: server.username || '',
    password: server.password || ''
  }
}

const handleSubmit = () => {
  formRef.value?.validate((errors) => {
    if (!errors) {
      if (isEditing.value) {
        // 更新服务器
        const index = servers.value.findIndex(s => s.id === formModel.value.id)
        if (index !== -1) {
          servers.value[index] = {
            id: formModel.value.id,
            name: formModel.value.name.trim(),
            url: formModel.value.url.trim(),
            username: formModel.value.username || undefined,
            password: formModel.value.password || undefined
          }
          selectedServer.value = servers.value[index]
          message.success('服务器更新成功')
        }
      } else {
        // 添加新服务器
        const newServer: Server = {
          id: Date.now().toString(),
          name: formModel.value.name.trim(),
          url: formModel.value.url.trim(),
          username: formModel.value.username || undefined,
          password: formModel.value.password || undefined
        }
        servers.value.push(newServer)
        selectedServer.value = newServer
        message.success('服务器添加成功')
      }
      mode.value = 'view'
    } else {
      message.error('请填写必填字段')
    }
  })
}

const cancelForm = () => {
  mode.value = 'view'
  // 保持选中的服务器不变
}

const removeServer = (id: string) => {
  servers.value = servers.value.filter(server => server.id !== id)
  if (selectedServer.value?.id === id) {
    selectedServer.value = null
  }
}

const selectServer = (server: Server) => {
  selectedServer.value = server
  mode.value = 'view'
  // 在查看模式下，更新formModel以显示选中的服务器信息
  formModel.value = {
    id: server.id,
    name: server.name,
    url: server.url,
    username: server.username || '',
    password: server.password || ''
  }
}

const connectToServer = async (id: string) => {
  const server = servers.value.find(s => s.id === id)
  if (!server) {
    message.error('未找到服务器')
    return
  }
  
  // 显示连接中状态
  server.connected = false
  server.connectionError = undefined
  message.info(`正在连接到 ${server.name}...`)
  
  try {
    // 模拟连接过程（实际应用中这里应该是真实的API调用）
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    // 模拟连接成功
    server.connected = true
    server.connectionError = undefined
    
    // 添加到已连接服务器列表
    addConnectedServer(server)
    
    // 设置为活动服务器
    setActiveServer(server)
    
    message.success(`成功连接到 ${server.name}`)
    
    // 跳转到数据浏览页
    router.push('/data')
  } catch (error) {
    server.connected = false
    server.connectionError = '连接失败'
    message.error(`连接到 ${server.name} 失败: ${error}`)
  }
}

const refreshServers = () => {
  console.log('刷新服务器列表')
  // 这里可以实现刷新逻辑
}

// 组件挂载时可以加载服务器列表
onMounted(() => {
  console.log('服务器管理页面已加载')
})
</script>

<style scoped>
.server-view {
  padding: 0;
  background-color: #ffffff;
  border-radius: 4px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.server-view > h2 {
  padding: 16px;
  margin: 0;
  border-bottom: 1px solid #e0e0e0;
  background-color: #fafafa;
  font-size: 18px;
}

.server-controls {
  padding: 16px;
  margin: 0;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid #e0e0e0;
  background-color: #fafafa;
}

.controls-left {
  display: flex;
  align-items: center;
}

.controls-right {
  display: flex;
  align-items: center;
}

.server-layout {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.server-list-panel {
  width: 300px;
  border-right: 1px solid #e0e0e0;
  display: flex;
  flex-direction: column;
}

.server-detail-panel {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  background-color: #fafafa;
}

.server-detail {
  background-color: #ffffff;
  padding: 16px;
  border-radius: 4px;
  border: 1px solid #e0e0e0;
}

.server-detail h3 {
  margin-top: 0;
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid #e0e0e0;
}

.no-selection {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #666666;
}

.btn {
  padding: 8px 16px;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  background-color: #f5f5f5;
  color: #333333;
}

.btn:hover {
  background-color: #e0e0e0;
}

.btn-primary {
  background-color: #4a4a4a;
  color: white;
  border-color: #3a3a3a;
}

.btn-primary:hover {
  background-color: #3a3a3a;
}

.btn-secondary {
  background-color: #f5f5f5;
  color: #333333;
  border-color: #d0d0d0;
}

.btn-secondary:hover {
  background-color: #e0e0e0;
}

.btn-danger {
  background-color: #f5f5f5;
  color: #cc0000;
  border-color: #d0d0d0;
}

.btn-danger:hover {
  background-color: #ffe0e0;
}

.btn-small {
  padding: 4px 8px;
  font-size: 12px;
}

.server-list {
  flex: 1;
  overflow-y: auto;
  border: none;
  border-radius: 0;
  background-color: #ffffff;
}

.server-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #e0e0e0;
  background-color: #ffffff;
  cursor: pointer;
}

.server-item:hover {
  background-color: #f0f0f0;
}

.server-item.active {
  background-color: #e3f2fd;
  border-left: 3px solid #2196f3;
}

.server-info {
  flex: 1;
}

.server-name {
  font-weight: 500;
  margin-bottom: 2px;
  color: #333333;
}

.server-url {
  color: #666666;
  font-size: 13px;
}

.server-status {
  margin-right: 10px;
}

.status-indicator {
  font-size: 16px;
}

.status-indicator.connected {
  color: #4caf50;
}

.status-indicator.disconnected {
  color: #9e9e9e;
}

.status-indicator.error {
  color: #f44336;
}

.server-actions {
  display: flex;
  gap: 8px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 4px;
  font-weight: 500;
  color: #333333;
}

.form-group input {
  width: 100%;
  padding: 8px;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 14px;
  background-color: #ffffff;
}

.form-group input:focus {
  outline: none;
  border-color: #4a4a4a;
  box-shadow: 0 0 0 2px rgba(74, 74, 74, 0.2);
}

.server-controls-container {
  display: flex;
  padding: 16px;
  border-bottom: 1px solid #e0e0e0;
  background-color: #fafafa;
  position: sticky;
  top: 0;
  z-index: 10;
}

.controls-left {
  flex: 1;
  margin-right: 10px;
}

.controls-right {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.form-actions {
  display: flex;
  gap: 10px;
}

/* 优化横向滚动条问题 */
.server-view {
  overflow-x: hidden;
}

.server-layout {
  overflow-x: hidden;
}

.server-list-panel {
  overflow-x: hidden;
}

.server-item {
  min-width: 0;
}

.server-info {
  min-width: 0;
}

.server-name,
.server-url {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
</style>