<template>
  <div class="status-content">
    <n-grid :cols="2" :x-gap="12" :y-gap="12">
      <n-gi>
        <n-card title="集群信息">
          <n-skeleton v-if="loading.cluster" :repeat="3" />
          <div v-else>
            <n-descriptions label-placement="left" :column="1">
              <n-descriptions-item label="集群名称">
                {{ clusterInfo.name || 'N/A' }}
              </n-descriptions-item>
              <n-descriptions-item label="节点数量">
                {{ clusterInfo.nodes || 'N/A' }}
              </n-descriptions-item>
              <n-descriptions-item label="状态">
                <n-tag :type="clusterInfo.status === 'green' ? 'success' : clusterInfo.status === 'yellow' ? 'warning' : 'error'">
                  {{ clusterInfo.status || 'N/A' }}
                </n-tag>
              </n-descriptions-item>
              <n-descriptions-item label="版本">
                {{ clusterInfo.version || 'N/A' }}
              </n-descriptions-item>
            </n-descriptions>
          </div>
        </n-card>
      </n-gi>
      <n-gi>
        <n-card title="索引信息">
          <n-skeleton v-if="loading.indices" :repeat="3" />
          <div v-else>
            <n-descriptions label-placement="left" :column="1">
              <n-descriptions-item label="索引数量">
                {{ indicesInfo.count || 'N/A' }}
              </n-descriptions-item>
              <n-descriptions-item label="文档总数">
                {{ indicesInfo.documents || 'N/A' }}
              </n-descriptions-item>
              <n-descriptions-item label="存储大小">
                {{ indicesInfo.size || 'N/A' }}
              </n-descriptions-item>
            </n-descriptions>
          </div>
        </n-card>
      </n-gi>
    </n-grid>
    
    <n-card title="分片信息" class="shard-card">
      <n-skeleton v-if="loading.shards" :repeat="2" />
      <div v-else>
        <n-data-table
          :columns="shardColumns"
          :data="shardData"
          :bordered="true"
          :single-line="false"
        />
      </div>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { 
  NCard, 
  NSkeleton, 
  NDescriptions, 
  NDescriptionsItem, 
  NTag, 
  NGrid, 
  NGi, 
  NDataTable 
} from 'naive-ui'
import ElasticsearchService from '../../services/elasticsearchService'

// 定义服务器接口
interface Server {
  id: string
  name: string
  url: string
  username?: string
  password?: string
  connected?: boolean
}

// 定义集群信息接口
interface ClusterInfo {
  name: string
  nodes: number
  status: string
  version: string
}

// 定义索引信息接口
interface IndicesInfo {
  count: number
  documents: string
  size: string
}

// 定义分片数据接口
interface ShardData {
  index: string
  shard: string
  prirep: string
  type: string
  status: string
  node: string
}

// 定义props
const props = defineProps<{
  server: Server
  loading: {
    cluster: boolean
    indices: boolean
    shards: boolean
  }
  clusterInfo: ClusterInfo
  indicesInfo: IndicesInfo
  shardColumns: Array<any>
  shardData: ShardData[]
  onUpdateClusterInfo: (info: ClusterInfo) => void
  onUpdateIndicesInfo: (info: IndicesInfo) => void
  onUpdateShardData: (data: ShardData[]) => void
  onUpdateLoading: (loading: any) => void
}>()

// 创建Elasticsearch服务实例
const createEsService = () => {
  const config = {
    url: props.server.url,
    username: props.server.username,
    password: props.server.password
  }

  return new ElasticsearchService(config)
}

// 获取集群信息
const fetchClusterInfo = async () => {
  try {
    props.onUpdateLoading({ ...props.loading, cluster: true })
    const service = createEsService()
    
    // 获取集群健康状态
    const healthResponse = await service.getClusterHealth()
    
    // 获取集群信息
    const infoResponse = await service.getClusterInfo()
    
    // 获取节点信息
    const nodesResponse = await service.getNodesInfo()
    const nodeCount = Object.keys(nodesResponse.nodes || {}).length
    
    props.onUpdateClusterInfo({
      name: healthResponse.cluster_name || 'elasticsearch_cluster',
      nodes: nodeCount,
      status: healthResponse.status,
      version: infoResponse.version?.number || 'N/A'
    })
  } catch (error: any) {
    console.error('获取集群信息失败:', error)
    // 显示错误信息
    props.onUpdateClusterInfo({
      name: '获取失败',
      nodes: 0,
      status: 'red',
      version: 'N/A'
    })
  } finally {
    props.onUpdateLoading({ ...props.loading, cluster: false })
  }
}

// 获取索引信息
const fetchIndicesInfo = async () => {
  try {
    props.onUpdateLoading({ ...props.loading, indices: true })
    const service = createEsService()
    
    // 获取索引统计信息
    const statsResponse = await service.getIndicesStats()
    
    // 计算索引数量和文档总数
    const indices = statsResponse.indices || {}
    const indexCount = Object.keys(indices).length
    let totalDocs = 0
    let totalSize = 0
    
    for (const indexName in indices) {
      const index = indices[indexName]
      totalDocs += index.primaries?.docs?.count || 0
      totalSize += index.primaries?.store?.size_in_bytes || 0
    }
    
    // 格式化大小
    const formatBytes = (bytes: number) => {
      if (bytes === 0) return '0 Bytes'
      const k = 1024
      const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
      const i = Math.floor(Math.log(bytes) / Math.log(k))
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
    }
    
    props.onUpdateIndicesInfo({
      count: indexCount,
      documents: totalDocs.toLocaleString(),
      size: formatBytes(totalSize)
    })
  } catch (error: any) {
    console.error('获取索引信息失败:', error)
    // 显示错误信息
    props.onUpdateIndicesInfo({
      count: 0,
      documents: 'N/A',
      size: 'N/A'
    })
  } finally {
    props.onUpdateLoading({ ...props.loading, indices: false })
  }
}

// 获取分片信息
const fetchShardData = async () => {
  try {
    props.onUpdateLoading({ ...props.loading, shards: true })
    const service = createEsService()
    
    // 获取分片信息
    const shardResponse: any = await service.getShards()
    
    // 处理分片数据
    const shards = (shardResponse || []).map((shard: any) => ({
      index: shard.index,
      shard: shard.shard,
      prirep: shard.prirep,
      type: shard.prirep === 'p' ? 'primary' : 'replica',
      status: shard.state,
      node: shard.node
    }))
    
    props.onUpdateShardData(shards)
  } catch (error: any) {
    console.error('获取分片信息失败:', error)
    // 显示空数据
    props.onUpdateShardData([])
  } finally {
    props.onUpdateLoading({ ...props.loading, shards: false })
  }
}

// 组件挂载时获取数据
onMounted(async () => {
  if (props.server && props.server.connected) {
    await Promise.all([
      fetchClusterInfo(),
      fetchIndicesInfo(),
      fetchShardData()
    ])
  }
})

// 导出刷新函数，供父组件调用
defineExpose({
  refreshData: async () => {
    if (props.server && props.server.connected) {
      await Promise.all([
        fetchClusterInfo(),
        fetchIndicesInfo(),
        fetchShardData()
      ])
    }
  }
})
</script>

<style scoped>
.status-content {
  padding: 16px 0;
}

.shard-card {
  margin-top: 16px;
}
</style>