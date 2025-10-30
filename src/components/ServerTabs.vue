<template>
  <div class="server-tabs">
    <template v-if="serverTabs.length > 0">
      <n-tabs 
        v-model:value="activeTabId" 
        type="card" 
        closable 
        tab-style="min-width: 120px;"
        @close="handleTabClose"
        @update:value="handleTabChange"
      >
        <n-tab-pane 
          v-for="serverTab in serverTabs" 
          :key="serverTab.id"
          :name="serverTab.id" 
          :tab="serverTab.name"
        >
          <template #tab>
            <div class="tab-header">
              <span class="server-name">{{ serverTab.name }}</span>
              <span 
                v-if="serverTab.connected" 
                class="status-indicator connected"
                title="已连接"
              >●</span>
              <span 
                v-else 
                class="status-indicator disconnected"
                title="连接断开"
              >●</span>
            </div>
          </template>
          
          <!-- 服务器数据内容 -->
          <div class="server-content">
            <n-tabs type="line" animated>
              <n-tab-pane name="status" tab="状态">
                <StatusTab 
                  :ref="el => statusTabRefs[serverTab.id] = el"
                  :server="serverTab.server"
                  :loading="serverTab.loading"
                  :cluster-info="serverTab.clusterInfo"
                  :indices-info="serverTab.indicesInfo"
                  :shard-columns="serverTab.shardColumns"
                  :shard-data="serverTab.shardData"
                  :on-update-cluster-info="(info) => updateServerData(serverTab.id, 'clusterInfo', info)"
                  :on-update-indices-info="(info) => updateServerData(serverTab.id, 'indicesInfo', info)"
                  :on-update-shard-data="(data) => updateServerData(serverTab.id, 'shardData', data)"
                  :on-update-loading="(loading) => updateServerData(serverTab.id, 'loading', loading)"
                />
              </n-tab-pane>
              
              <n-tab-pane name="browse" tab="数据浏览">
                <BrowseTab
                  :index-options="serverTab.indexOptions"
                  :selected-index="serverTab.selectedIndex"
                  :selected-data="serverTab.selectedData"
                  :data-columns="serverTab.dataColumns"
                  :pagination="serverTab.pagination"
                  @update:selectedIndex="(index) => updateServerData(serverTab.id, 'selectedIndex', index)"
                  @refresh="() => refreshServerData(serverTab.id)"
                />
              </n-tab-pane>
              
              <n-tab-pane name="query" tab="查询">
                <QueryTab
                  v-model:queryTab="serverTab.queryTab"
                  :queryTabs="serverTab.queryTabs"
                  :fieldOptions="serverTab.fieldOptions"
                  :operatorOptions="serverTab.operatorOptions"
                  :resultColumns="serverTab.resultColumns"
                  :resultPagination="serverTab.resultPagination"
                  @addQueryTab="() => addQueryTab(serverTab.id)"
                  @closeQueryTab="(name) => handleQueryTabClose(serverTab.id, name)"
                  @queryTypeChange="(tabId, type) => handleQueryTypeChange(serverTab.id, tabId, type)"
                  @runQuery="(tabId) => runQuery(serverTab.id, tabId)"
                  @saveQuery="(tabId) => saveQuery(serverTab.id, tabId)"
                />
              </n-tab-pane>
            </n-tabs>
          </div>
        </n-tab-pane>
      </n-tabs>
    </template>
    
    <!-- 无服务器连接时的提示 -->
    <div v-else class="no-server-connected">
      <div class="center-container">
        <n-result
          status="404"
          title="未连接服务器"
          description="请先在服务器管理页面连接到ElasticSearch服务器"
        >
          <template #footer>
            <n-button @click="goToServerPage">前往服务器管理页面</n-button>
          </template>
        </n-result>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { NTabs, NTabPane, NResult, NButton } from 'naive-ui'
import { useRouter } from 'vue-router'
import { getConnectedServers, removeConnectedServer } from '../stores/serverStore'
import StatusTab from './data/StatusTab.vue'
import BrowseTab from './data/BrowseTab.vue'
import QueryTab from './data/QueryTab.vue'

interface Server {
  id: string
  name: string
  url: string
  username?: string
  password?: string
  connected?: boolean
  connectionError?: string
}

interface ServerTabData {
  id: string
  name: string
  server: Server
  connected: boolean
  
  // 状态tab数据
  loading: {
    cluster: boolean
    indices: boolean
    shards: boolean
  }
  clusterInfo: {
    name: string
    nodes: number
    status: string
    version: string
  }
  indicesInfo: {
    count: number
    documents: string
    size: string
  }
  shardColumns: Array<{ title: string; key: string }>
  shardData: Array<any>
  
  // 数据浏览tab数据
  indexOptions: Array<{ label: string; value: string }>
  selectedIndex: string | null
  selectedData: Array<any>
  dataColumns: Array<{ title: string; key: string }>
  pagination: {
    page: number
    pageSize: number
    showSizePicker: boolean
    pageSizes: number[]
  }
  
  // 查询tab数据
  queryTab: string
  queryTabs: Array<{
    id: string
    name: string
    type: string
    basicForm: {
      field: string
      operator: string
      value: string
    }
    advancedForm: {
      dsl: string
    }
    result: Array<any>
  }>
  fieldOptions: Array<{ label: string; value: string }>
  operatorOptions: Array<{ label: string; value: string }>
  resultColumns: Array<{ title: string; key: string }>
  resultPagination: {
    page: number
    pageSize: number
  }
}

const router = useRouter()
const statusTabRefs = ref<Record<string, any>>({})

// 活动页签ID
const activeTabId = ref<string>('')

// 服务器页签数据
const serverTabs = ref<ServerTabData[]>([])

// 获取已连接的服务器
const connectedServers = computed(() => getConnectedServers())

// 监听已连接服务器变化，同步页签
watch(connectedServers, (servers) => {
  nextTick(() => {
    syncTabsWithServers()
  })
}, { immediate: true, deep: true })

// 同步页签与服务器连接状态
const syncTabsWithServers = () => {
  const currentServerIds = new Set(serverTabs.value.map(tab => tab.id))
  const connectedServerIds = new Set(connectedServers.value.map(server => server.id))
  
  // 移除已断开连接的服务器页签
  serverTabs.value = serverTabs.value.filter(tab => connectedServerIds.has(tab.id))
  
  // 添加新连接的服务器页签
  connectedServers.value.forEach(server => {
    if (!currentServerIds.has(server.id)) {
      addServerTab(server)
    }
  })
  
  // 如果没有活动页签，设置第一个为活动页签
  if (serverTabs.value.length > 0 && !activeTabId.value) {
    activeTabId.value = serverTabs.value[0].id
  }
}

// 添加服务器页签
const addServerTab = (server: Server) => {
  const newTab: ServerTabData = {
    id: server.id,
    name: server.name,
    server: server,
    connected: server.connected || false,
    
    // 初始化状态tab数据
    loading: {
      cluster: false,
      indices: false,
      shards: false
    },
    clusterInfo: {
      name: 'elasticsearch_cluster',
      nodes: 3,
      status: 'green',
      version: 'N/A'
    },
    indicesInfo: {
      count: 12,
      documents: '1,245,678',
      size: '2.4 GB'
    },
    shardColumns: [
      { title: '索引', key: 'index' },
      { title: '分片', key: 'shard' },
      { title: '主/副本', key: 'prirep' },
      { title: '类型', key: 'type' },
      { title: '状态', key: 'status' },
      { title: '节点', key: 'node' }
    ],
    shardData: [
      { index: 'user_data', shard: '0', prirep: 'p', type: 'primary', status: 'STARTED', node: 'node-1' },
      { index: 'user_data', shard: '1', prirep: 'r', type: 'replica', status: 'STARTED', node: 'node-2' },
      { index: 'product_info', shard: '0', prirep: 'p', type: 'primary', status: 'STARTED', node: 'node-3' }
    ],
    
    // 初始化数据浏览tab数据
    indexOptions: [
      { label: 'user_data', value: 'user_data' },
      { label: 'product_info', value: 'product_info' },
      { label: 'order_history', value: 'order_history' }
    ],
    selectedIndex: null,
    selectedData: [],
    dataColumns: [
      { title: 'ID', key: 'id' },
      { title: 'Name', key: 'name' },
      { title: 'Type', key: 'type' },
      { title: 'Created', key: 'created' }
    ],
    pagination: {
      page: 1,
      pageSize: 10,
      showSizePicker: true,
      pageSizes: [10, 20, 50]
    },
    
    // 初始化查询tab数据
    queryTab: 'query-1',
    queryTabs: [
      {
        id: 'query-1',
        name: '基本查询',
        type: 'basic',
        basicForm: {
          field: '',
          operator: '',
          value: ''
        },
        advancedForm: {
          dsl: ''
        },
        result: []
      }
    ],
    fieldOptions: [
      { label: 'ID', value: 'id' },
      { label: 'Name', value: 'name' },
      { label: 'Type', value: 'type' }
    ],
    operatorOptions: [
      { label: '等于', value: 'equals' },
      { label: '包含', value: 'contains' },
      { label: '大于', value: 'greater' },
      { label: '小于', value: 'less' }
    ],
    resultColumns: [
      { title: 'ID', key: 'id' },
      { title: 'Name', key: 'name' },
      { title: 'Score', key: 'score' }
    ],
    resultPagination: {
      page: 1,
      pageSize: 10
    }
  }
  
  serverTabs.value.push(newTab)
  
  // 如果这是第一个页签，设置为活动页签
  if (serverTabs.value.length === 1) {
    activeTabId.value = newTab.id
  }
}

// 处理页签关闭
const handleTabClose = (tabId: string) => {
  // 从已连接服务器列表中移除
  removeConnectedServer(tabId)
  
  // 使用 nextTick 确保 DOM 更新完成后再处理活动页签
  nextTick(() => {
    // 如果关闭的是当前活动页签，切换到下一个页签
    if (activeTabId.value === tabId) {
      const remainingTabs = serverTabs.value.filter(tab => tab.id !== tabId)
      if (remainingTabs.length > 0) {
        const currentIndex = serverTabs.value.findIndex(tab => tab.id === tabId)
        // 切换到下一个页签，如果没有下一个，则切换到上一个
        const nextIndex = currentIndex < remainingTabs.length ? currentIndex : Math.max(0, currentIndex - 1)
        activeTabId.value = remainingTabs[nextIndex].id
      } else {
        activeTabId.value = ''
      }
    }
  })
}

// 处理页签切换
const handleTabChange = (tabId: string) => {
  activeTabId.value = tabId
  
  // 使用 nextTick 确保 DOM 更新完成后再刷新数据
  nextTick(() => {
    const statusTabRef = statusTabRefs.value[tabId]
    if (statusTabRef) {
      statusTabRef.refreshData()
    }
  })
}

// 更新服务器数据
const updateServerData = (serverId: string, dataType: string, data: any) => {
  const tab = serverTabs.value.find(t => t.id === serverId)
  if (tab) {
    (tab as any)[dataType] = data
  }
}

// 刷新服务器数据
const refreshServerData = (serverId: string) => {
  const tab = serverTabs.value.find(t => t.id === serverId)
  if (tab) {
    // 模拟加载数据
    tab.selectedData = [
      { id: 1, name: 'Item 1', type: 'Type A', created: '2023-01-01' },
      { id: 2, name: 'Item 2', type: 'Type B', created: '2023-01-02' }
    ]
  }
}

// 添加查询tab
const addQueryTab = (serverId: string) => {
  const tab = serverTabs.value.find(t => t.id === serverId)
  if (tab) {
    const newId = `query-${Date.now()}`
    tab.queryTabs.push({
      id: newId,
      name: '新查询',
      type: 'basic',
      basicForm: {
        field: '',
        operator: '',
        value: ''
      },
      advancedForm: {
        dsl: ''
      },
      result: []
    })
    tab.queryTab = newId
  }
}

// 关闭查询tab
const handleQueryTabClose = (serverId: string, name: string) => {
  const tab = serverTabs.value.find(t => t.id === serverId)
  if (tab && tab.queryTabs.length > 1) {
    const index = tab.queryTabs.findIndex(queryTab => queryTab.id === name)
    if (index !== -1) {
      tab.queryTabs.splice(index, 1)
      // 如果关闭的是当前激活的tab，切换到第一个tab
      if (tab.queryTab === name) {
        tab.queryTab = tab.queryTabs[0].id
      }
    }
  }
}

// 查询类型切换
const handleQueryTypeChange = (serverId: string, tabId: string, type: string) => {
  const tab = serverTabs.value.find(t => t.id === serverId)
  if (tab) {
    const queryTab = tab.queryTabs.find(qt => qt.id === tabId)
    if (queryTab) {
      queryTab.type = type
    }
  }
}

// 执行查询
const runQuery = (serverId: string, tabId: string) => {
  const tab = serverTabs.value.find(t => t.id === serverId)
  if (tab) {
    const queryTab = tab.queryTabs.find(qt => qt.id === tabId)
    if (queryTab) {
      // 模拟查询结果
      queryTab.result = [
        { id: '1', name: 'Result 1', score: 0.95 },
        { id: '2', name: 'Result 2', score: 0.87 }
      ]
    }
  }
}

// 保存查询
const saveQuery = (serverId: string, tabId: string) => {
  const tab = serverTabs.value.find(t => t.id === serverId)
  if (tab) {
    const queryTab = tab.queryTabs.find(qt => qt.id === tabId)
    if (queryTab) {
      // 这里可以实现保存查询的逻辑
      console.log('保存查询:', queryTab)
    }
  }
}

// 跳转到服务器管理页面
const goToServerPage = () => {
  router.push('/server')
}
</script>

<style scoped>
.server-tabs {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.tab-header {
  display: flex;
  align-items: center;
  gap: 6px;
}

.server-name {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.status-indicator {
  font-size: 12px;
  flex-shrink: 0;
}

.status-indicator.connected {
  color: #4caf50;
}

.status-indicator.disconnected {
  color: #9e9e9e;
}

.server-content {
  height: calc(100% - 40px);
  overflow: hidden;
  padding: 0 12px;
}

.no-server-connected {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
}

.center-container {
  width: 100%;
  max-width: 500px;
  text-align: center;
}

/* 确保页签内容高度正确 */
:deep(.n-tabs-nav) {
  margin-bottom: 0;
}

:deep(.n-tabs-content) {
  height: calc(100% - 46px);
}

:deep(.n-tab-pane) {
  height: 100%;
  overflow: hidden;
}
</style>