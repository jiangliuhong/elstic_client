<template>
  <div class="server-view">
    <h2>服务器管理</h2>
    <div class="server-controls">
      <n-button type="primary" @click="addServer">添加服务器</n-button>
      <n-button @click="refreshServers">刷新</n-button>
    </div>
    
    <div class="server-layout">
      <!-- 左侧服务器列表 -->
      <div class="server-list-panel">
        <div class="server-list">
          <div 
            class="server-item" 
            v-for="server in servers" 
            :key="server.id"
            :class="{ active: selectedServer?.id === server.id }"
            @click="selectServer(server)"
          >
            <div class="server-info">
              <div class="server-name">{{ server.name }}</div>
              <div class="server-url">{{ server.url }}</div>
            </div>
            <div class="server-actions">
              <n-button size="small" @click.stop="connectToServer(server.id)">连接</n-button>
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
import { ref, onMounted } from 'vue'
import { useMessage } from 'naive-ui'
import type { FormInst, FormRules } from 'naive-ui'

interface Server {
  id: string
  name: string
  url: string
  username?: string
  password?: string
}

const servers = ref<Server[]>([
  { id: '1', name: '本地服务器', url: 'http://localhost:9200', username: 'admin', password: 'password' }
])

const selectedServer = ref<Server | null>(null)
const mode = ref<'view' | 'form'>('view') // 'view' or 'form'
const isEditing = ref(false)
const formRef = ref<FormInst | null>(null)

// 统一的表单模型
const formModel = ref({
  id: '',
  name: '',
  url: '',
  username: '',
  password: ''
})

const message = useMessage()

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

const getCurrentModel = () => {
  if (mode.value === 'form') {
    return formModel.value
  }
  return selectedServer.value || {
    id: '',
    name: '',
    url: '',
    username: '',
    password: ''
  }
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

const connectToServer = (id: string) => {
  console.log('连接到服务器:', id)
  // 这里可以实现实际的连接逻辑
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
  height: 100%;
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
  gap: 10px;
  border-bottom: 1px solid #e0e0e0;
  background-color: #fafafa;
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

.form-actions {
  display: flex;
  gap: 10px;
}
</style>