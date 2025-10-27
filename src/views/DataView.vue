<template>
  <div class="data-view">
    <div v-if="connectedServers.length === 0" class="no-server-connected">
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
    <div v-else>
      <n-tabs type="line" animated @update:value="handleTabChange">
        <n-tab-pane name="status" tab="状态">
          <StatusTab 
            ref="statusTabRef"
            :server="activeServer"
            :loading="loading"
            :cluster-info="clusterInfo"
            :indices-info="indicesInfo"
            :shard-columns="shardColumns"
            :shard-data="shardData"
            :on-update-cluster-info="updateClusterInfo"
            :on-update-indices-info="updateIndicesInfo"
            :on-update-shard-data="updateShardData"
            :on-update-loading="updateLoading"
          />
        </n-tab-pane>
        
        <n-tab-pane name="browse" tab="数据浏览">
          <BrowseTab
            :index-options="indexOptions"
            :selected-index="selectedIndex"
            :selected-data="selectedData"
            :data-columns="dataColumns"
            :pagination="pagination"
            @update:selectedIndex="selectedIndex = $event"
            @refresh="refreshData"
          />
        </n-tab-pane>
        
        <n-tab-pane name="query" tab="查询">
          <QueryTab
            v-model:queryTab="queryTab"
            :queryTabs="queryTabs"
            :fieldOptions="fieldOptions"
            :operatorOptions="operatorOptions"
            :resultColumns="resultColumns"
            :resultPagination="resultPagination"
            @addQueryTab="addQueryTab"
            @closeQueryTab="handleQueryTabClose"
            @queryTypeChange="handleQueryTypeChange"
            @runQuery="runQuery"
            @saveQuery="saveQuery"
          />
        </n-tab-pane>
      </n-tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { NTabs, NTabPane, NResult, NButton } from 'naive-ui'
import { useRouter } from 'vue-router'
import { getConnectedServers, getActiveServer } from '../stores/serverStore'
import StatusTab from '../components/data/StatusTab.vue'
import BrowseTab from '../components/data/BrowseTab.vue'
import QueryTab from '../components/data/QueryTab.vue'

const router = useRouter()
const statusTabRef = ref<InstanceType<typeof StatusTab> | null>(null)

// 获取已连接的服务器和活动服务器
const connectedServers = computed(() => getConnectedServers())
const activeServer = computed(() => {
  const server = getActiveServer()
  return server || {
    id: '',
    name: '',
    url: '',
    username: '',
    password: '',
    connected: false
  }
})

// 状态tab相关数据
const loading = ref({
  cluster: false,
  indices: false,
  shards: false
})

const clusterInfo = ref({
  name: 'elasticsearch_cluster',
  nodes: 3,
  status: 'green',
  version: 'N/A'
})

const indicesInfo = ref({
  count: 12,
  documents: '1,245,678',
  size: '2.4 GB'
})

const shardColumns = ref([
  { title: '索引', key: 'index' },
  { title: '分片', key: 'shard' },
  { title: '主/副本', key: 'prirep' },
  { title: '类型', key: 'type' },
  { title: '状态', key: 'status' },
  { title: '节点', key: 'node' }
])

const shardData = ref([
  { index: 'user_data', shard: '0', prirep: 'p', type: 'primary', status: 'STARTED', node: 'node-1' },
  { index: 'user_data', shard: '1', prirep: 'r', type: 'replica', status: 'STARTED', node: 'node-2' },
  { index: 'product_info', shard: '0', prirep: 'p', type: 'primary', status: 'STARTED', node: 'node-3' }
])

// 数据浏览tab相关数据
const indexOptions = ref([
  { label: 'user_data', value: 'user_data' },
  { label: 'product_info', value: 'product_info' },
  { label: 'order_history', value: 'order_history' }
])

const selectedIndex = ref<string | null>(null)
const selectedData = ref<any[]>([])

const dataColumns = ref([
  { title: 'ID', key: 'id' },
  { title: 'Name', key: 'name' },
  { title: 'Type', key: 'type' },
  { title: 'Created', key: 'created' }
])

const pagination = ref({
  page: 1,
  pageSize: 10,
  showSizePicker: true,
  pageSizes: [10, 20, 50]
})

// 查询tab相关数据
const queryTab = ref<string>('query-1')
const queryTabs = ref([
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
])

const fieldOptions = ref([
  { label: 'ID', value: 'id' },
  { label: 'Name', value: 'name' },
  { label: 'Type', value: 'type' }
])

const operatorOptions = ref([
  { label: '等于', value: 'equals' },
  { label: '包含', value: 'contains' },
  { label: '大于', value: 'greater' },
  { label: '小于', value: 'less' }
])

const resultColumns = ref([
  { title: 'ID', key: 'id' },
  { title: 'Name', key: 'name' },
  { title: 'Score', key: 'score' }
])

const resultPagination = ref({
  page: 1,
  pageSize: 10
})

// 更新集群信息
const updateClusterInfo = (info: any) => {
  clusterInfo.value = { ...clusterInfo.value, ...info }
}

// 更新索引信息
const updateIndicesInfo = (info: any) => {
  indicesInfo.value = { ...indicesInfo.value, ...info }
}

// 更新分片数据
const updateShardData = (data: any[]) => {
  shardData.value = data
}

// 更新加载状态
const updateLoading = (newLoading: any) => {
  loading.value = { ...loading.value, ...newLoading }
}

// 处理tab切换
const handleTabChange = (value: string) => {
  console.log('切换到tab:', value)
  // 如果切换到状态tab，刷新数据
  if (value === 'status' && statusTabRef.value) {
    statusTabRef.value.refreshData()
  }
}

// 刷新数据
const refreshData = () => {
  // 模拟加载数据
  selectedData.value = [
    { id: 1, name: 'Item 1', type: 'Type A', created: '2023-01-01' },
    { id: 2, name: 'Item 2', type: 'Type B', created: '2023-01-02' }
  ]
}

// 添加查询tab
const addQueryTab = () => {
  const newId = `query-${Date.now()}`
  queryTabs.value.push({
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
  queryTab.value = newId
}

// 关闭查询tab
const handleQueryTabClose = (name: string) => {
  if (queryTabs.value.length > 1) {
    const index = queryTabs.value.findIndex(tab => tab.id === name)
    if (index !== -1) {
      queryTabs.value.splice(index, 1)
      // 如果关闭的是当前激活的tab，切换到第一个tab
      if (queryTab.value === name) {
        queryTab.value = queryTabs.value[0].id
      }
    }
  }
}

// 查询类型切换
const handleQueryTypeChange = (tabId: string, type: string) => {
  const tab = queryTabs.value.find(tab => tab.id === tabId)
  if (tab) {
    tab.type = type
  }
}

// 执行查询
const runQuery = (tabId: string) => {
  const tab = queryTabs.value.find(tab => tab.id === tabId)
  if (tab) {
    // 模拟查询结果
    tab.result = [
      { id: '1', name: 'Result 1', score: 0.95 },
      { id: '2', name: 'Result 2', score: 0.87 }
    ] as any
  }
}

// 保存查询
const saveQuery = (tabId: string) => {
  const tab = queryTabs.value.find(tab => tab.id === tabId)
  if (tab) {
    // 这里可以实现保存查询的逻辑
    console.log('保存查询:', tab)
  }
}

// 跳转到服务器管理页面
const goToServerPage = () => {
  router.push('/server')
}

// 组件挂载时加载数据
onMounted(() => {
  console.log('数据浏览页面已加载')
})
</script>

<style scoped>
.data-view {
  /* padding: 16px; */
  background-color: #ffffff;
  border-radius: 4px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.data-view > h2 {
  margin: 0 0 16px 0;
  padding: 16px 16px 0 16px;
  font-size: 18px;
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

.n-card {
  margin-bottom: 16px;
}
</style>