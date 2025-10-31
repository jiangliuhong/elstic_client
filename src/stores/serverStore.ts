import { ref } from 'vue'
import { saveServerConfigToGist, getServerConfigFromGist } from '../services/githubGistService'

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

// 将服务器配置保存到GitHub Gist
const saveServersToGist = async (): Promise<string> => {
  try {
    const serversData = {
      servers: connectedServers.value,
      activeServerId: activeServer.value?.id
    }
    
    const gistUrl = await saveServerConfigToGist(serversData)
    return gistUrl
  } catch (error) {
    console.error('Failed to save servers to gist:', error)
    throw error
  }
}

// 从GitHub Gist加载服务器配置
const loadServersFromGist = async (gistUrl: string): Promise<void> => {
  try {
    const serversData = await getServerConfigFromGist(gistUrl)
    
    // 更新连接的服务器列表
    if (serversData.servers && Array.isArray(serversData.servers)) {
      connectedServers.value = serversData.servers
    }
    
    // 设置活动服务器
    if (serversData.activeServerId) {
      const server = connectedServers.value.find(s => s.id === serversData.activeServerId)
      if (server) {
        activeServer.value = server
      }
    }
  } catch (error) {
    console.error('Failed to load servers from gist:', error)
    throw error
  }
}

export {
  type Server,
  connectedServers,
  activeServer,
  addConnectedServer,
  removeConnectedServer,
  setActiveServer,
  getConnectedServers,
  getActiveServer,
  saveServersToGist,
  loadServersFromGist
}