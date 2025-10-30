import { ref } from 'vue'

interface Server {
  id: string
  name: string
  url: string
  username?: string
  password?: string
  connected?: boolean
  connectionError?: string
}

// 存储已连接的服务器
const connectedServers = ref<Server[]>([])

// 当前活动的服务器
const activeServer = ref<Server | null>(null)

// 添加连接的服务器
const addConnectedServer = (server: Server) => {
  // 检查是否已连接
  const existingIndex = connectedServers.value.findIndex(s => s.id === server.id)
  if (existingIndex !== -1) {
    // 更新现有连接
    connectedServers.value[existingIndex] = { ...server, connected: true }
  } else {
    // 添加新连接
    connectedServers.value.push({ ...server, connected: true })
  }
  
  // 如果没有活动服务器，设置第一个为活动服务器
  if (!activeServer.value && connectedServers.value.length > 0) {
    activeServer.value = connectedServers.value[0]
  }
}

// 移除连接的服务器
const removeConnectedServer = (serverId: string) => {
  connectedServers.value = connectedServers.value.filter(server => server.id !== serverId)
  
  // 如果移除的是活动服务器，设置下一个为活动服务器
  if (activeServer.value?.id === serverId) {
    activeServer.value = connectedServers.value.length > 0 ? connectedServers.value[0] : null
  }
}

// 设置活动服务器
const setActiveServer = (server: Server) => {
  activeServer.value = server
  
  // 确保服务器在已连接列表中
  const existingIndex = connectedServers.value.findIndex(s => s.id === server.id)
  if (existingIndex === -1) {
    connectedServers.value.push({ ...server, connected: true })
  } else {
    connectedServers.value[existingIndex] = { ...server, connected: true }
  }
}

// 获取连接的服务器
const getConnectedServers = () => {
  return connectedServers.value
}

// 获取活动服务器
const getActiveServer = () => {
  return activeServer.value
}

export {
  type Server,
  connectedServers,
  activeServer,
  addConnectedServer,
  removeConnectedServer,
  setActiveServer,
  getConnectedServers,
  getActiveServer
}