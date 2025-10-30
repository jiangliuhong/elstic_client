<template>
  <div class="status-content">
    <!-- 上部分：集群和索引信息 (1/3高度) -->
    <div ref="infoSectionRef" class="info-section">
      <n-card title="集群和索引信息" class="info-card">
        <n-skeleton v-if="loading.cluster || loading.indices" :repeat="4" />
        <div v-else>
          <n-descriptions label-placement="left" bordered :column="4">
            <!-- 集群信息 -->
            <n-descriptions-item label="集群名称">
              {{ clusterInfo.name || "N/A" }}
            </n-descriptions-item>
            <n-descriptions-item label="节点数量">
              {{ clusterInfo.nodes || "N/A" }}
            </n-descriptions-item>
            <n-descriptions-item label="集群状态">
              <n-tag
                :type="
                  clusterInfo.status === 'green'
                    ? 'success'
                    : clusterInfo.status === 'yellow'
                    ? 'warning'
                    : 'error'
                "
              >
                {{ clusterInfo.status || "N/A" }}
              </n-tag>
            </n-descriptions-item>
            <n-descriptions-item label="版本">
              {{ clusterInfo.version || "N/A" }}
            </n-descriptions-item>

            <!-- 索引信息 -->
            <n-descriptions-item label="索引数量">
              {{ indicesInfo.count || "N/A" }}
            </n-descriptions-item>
            <n-descriptions-item label="文档总数">
              {{ indicesInfo.documents || "N/A" }}
            </n-descriptions-item>
            <n-descriptions-item label="存储大小">
              {{ indicesInfo.size || "N/A" }}
            </n-descriptions-item>
            <n-descriptions-item label="分片总数">
              {{ shardData.length || "N/A" }}
            </n-descriptions-item>
          </n-descriptions>
        </div>
      </n-card>
    </div>

    <!-- 下部分：分片信息 (自适应高度，可滚动) -->
    <div class="shard-section">
      <n-card title="分片信息" class="shard-card">
        <n-skeleton v-if="loading.shards" :repeat="2" />
        <div v-else class="shard-list-container">
          <n-scrollbar
            :style="{ 'max-height': scrollbarMaxHeight, 'height': scrollbarMaxHeight }"
            :bordered="true"
            :show-divider="true"
          >
            <n-list>
              <n-list-item v-for="[key, list] in processedShardData" :key="key">
                <template #prefix>
                  <n-button style="min-width: 100px;">{{ key }}</n-button>
                </template>
                <n-descriptions label-placement="left" :column="5" size="small">
                  <template v-for="(item, index) in list" :key="index">
                    <!-- 集群信息 -->
                    <n-descriptions-item label="分片">
                      {{ item.shard }}
                    </n-descriptions-item>
                    <n-descriptions-item label="主/副本">
                      {{ item.prirep }}
                    </n-descriptions-item>
                    <n-descriptions-item label="类型">
                      {{ item.type }}
                    </n-descriptions-item>
                    <n-descriptions-item label="状态">
                      {{ item.status }}
                    </n-descriptions-item>
                    <n-descriptions-item label="节点">
                      {{ item.node }}
                    </n-descriptions-item>
                  </template>
                </n-descriptions>
              </n-list-item>
            </n-list>
          </n-scrollbar>
        </div>
      </n-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { NList, NListItem, NScrollbar } from "naive-ui";
import {
  computed,
  defineProps,
  nextTick,
  onMounted,
  onUnmounted,
  ref,
} from "vue";
import ElasticsearchService from "../../services/elasticsearchService";

// 定义服务器接口
interface Server {
  id: string;
  name: string;
  url: string;
  username?: string;
  password?: string;
  connected?: boolean;
}

// 定义集群信息接口
interface ClusterInfo {
  name: string;
  nodes: number;
  status: string;
  version: string;
}

// 定义索引信息接口
interface IndicesInfo {
  count: number;
  documents: string;
  size: string;
}

// 定义分片数据接口
interface ShardData {
  index: string;
  shard: string;
  prirep: string;
  type: string;
  status: string;
  node: string;
}
// 定义props
const props = defineProps<{
  server: Server;
  loading: {
    cluster: boolean;
    indices: boolean;
    shards: boolean;
  };
  clusterInfo: ClusterInfo;
  indicesInfo: IndicesInfo;
  shardColumns: Array<any>;
  shardData: ShardData[];
  onUpdateClusterInfo: (info: ClusterInfo) => void;
  onUpdateIndicesInfo: (info: IndicesInfo) => void;
  onUpdateShardData: (data: ShardData[]) => void;
  onUpdateLoading: (loading: any) => void;
}>();

const processedShardData = computed(() => {
  const map: Map<string, ShardData[]> = new Map<string, ShardData[]>();
  props.shardData.forEach((item: ShardData) => {
    let sds = map.get(item.index);
    if (sds) {
      sds.push(item);
    } else {
      sds = [item];
      map.set(item.index, sds);
    }
  });
  
  // 对每个索引下的分片数据按分片号排序
  map.forEach((shards, index) => {
    shards.sort((a, b) => {
      // 按分片号排序，如果分片号相同则按主/副本排序（主分片在前）
      const shardA = parseInt(a.shard, 10);
      const shardB = parseInt(b.shard, 10);
      
      if (shardA !== shardB) {
        return shardA - shardB;
      }
      
      // 分片号相同时，主分片(p)排在副本分片(r)前面
      if (a.prirep === 'p' && b.prirep === 'r') return -1;
      if (a.prirep === 'r' && b.prirep === 'p') return 1;
      return 0;
    });
  });
  
  return map;
});

// 响应式高度计算
const scrollbarMaxHeight = ref("400px");
const infoSectionRef = ref<HTMLElement | null>(null);

const updateScrollbarHeight = () => {
  // 获取信息区域的实际高度
  const infoSectionHeight = infoSectionRef.value?.offsetHeight || 200;
  
  // 计算剩余可用高度，减去信息区域高度、padding、margin等
  const availableHeight = window.innerHeight - infoSectionHeight - 120; // 120px为其他元素的预估高度
  
  // 确保最小高度
  scrollbarMaxHeight.value = `${Math.max(availableHeight, 200)}px`;
};

// 监听窗口大小变化
onMounted(() => {
  // 使用 nextTick 确保 DOM 已经渲染，再加一个setTimeout确保布局完成
  nextTick(() => {
    setTimeout(() => {
      updateScrollbarHeight();
    }, 100);
  });
  window.addEventListener("resize", updateScrollbarHeight);
});

onUnmounted(() => {
  window.removeEventListener("resize", updateScrollbarHeight);
});
// 创建Elasticsearch服务实例
const createEsService = () => {
  const config = {
    url: props.server.url,
    username: props.server.username,
    password: props.server.password,
  };

  return new ElasticsearchService(config);
};

// 获取集群信息
const fetchClusterInfo = async () => {
  try {
    props.onUpdateLoading({ ...props.loading, cluster: true });
    const service = createEsService();

    // 获取集群健康状态
    const healthResponse = await service.getClusterHealth();

    // 获取集群信息
    const infoResponse = await service.getClusterInfo();

    // 获取节点信息
    const nodesResponse = await service.getNodesInfo();
    const nodeCount = Object.keys(nodesResponse.nodes || {}).length;

    props.onUpdateClusterInfo({
      name: healthResponse.cluster_name || "elasticsearch_cluster",
      nodes: nodeCount,
      status: healthResponse.status,
      version: infoResponse.version?.number || "N/A",
    });
  } catch (error: any) {
    console.error("获取集群信息失败:", error);
    // 显示错误信息
    props.onUpdateClusterInfo({
      name: "获取失败",
      nodes: 0,
      status: "red",
      version: "N/A",
    });
  } finally {
    props.onUpdateLoading({ ...props.loading, cluster: false });
  }
};

// 获取索引信息
const fetchIndicesInfo = async () => {
  try {
    props.onUpdateLoading({ ...props.loading, indices: true });
    const service = createEsService();

    // 获取索引统计信息
    const statsResponse = await service.getIndicesStats();

    // 计算索引数量和文档总数
    const indices = statsResponse.indices || {};
    const indexCount = Object.keys(indices).length;
    let totalDocs = 0;
    let totalSize = 0;

    for (const indexName in indices) {
      const index = indices[indexName];
      totalDocs += index.primaries?.docs?.count || 0;
      totalSize += index.primaries?.store?.size_in_bytes || 0;
    }

    // 格式化大小
    const formatBytes = (bytes: number) => {
      if (bytes === 0) return "0 Bytes";
      const k = 1024;
      const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
    };

    props.onUpdateIndicesInfo({
      count: indexCount,
      documents: totalDocs.toLocaleString(),
      size: formatBytes(totalSize),
    });
  } catch (error: any) {
    console.error("获取索引信息失败:", error);
    // 显示错误信息
    props.onUpdateIndicesInfo({
      count: 0,
      documents: "N/A",
      size: "N/A",
    });
  } finally {
    props.onUpdateLoading({ ...props.loading, indices: false });
  }
};

// 获取分片信息

const fetchShardData = async () => {
  try {
    props.onUpdateLoading({ ...props.loading, shards: true });

    const service = createEsService();

    // 获取分片信息

    const shardResponse: any = await service.getShards();

    // 处理分片数据并去重

    const shardMap = new Map<string, ShardData>();

    const shards = (shardResponse || [])

      .filter((shard: any) => shard && shard.index && shard.shard !== undefined)

      .map((shard: any) => ({
        index: shard.index,

        shard: shard.shard,

        prirep: shard.prirep,

        type: shard.prirep === "p" ? "primary" : "replica",

        status: shard.state,

        node: shard.node,
      }))

      // 使用 Map 来去重，以 index-shard-prirep 作为唯一标识

      .filter((shard: ShardData) => {
        const key = `${shard.index}-${shard.shard}-${shard.prirep}`;

        if (shardMap.has(key)) {
          return false;
        }

        shardMap.set(key, shard);

        return true;
      });

    props.onUpdateShardData(shards);
  } catch (error: any) {
    console.error("获取分片信息失败:", error);

    // 显示空数据

    props.onUpdateShardData([]);
  } finally {
    props.onUpdateLoading({ ...props.loading, shards: false });
  }
};

// 组件挂载时获取数据
onMounted(async () => {
  if (props.server && props.server.connected) {
    await Promise.all([
      fetchClusterInfo(),
      fetchIndicesInfo(),
      fetchShardData(),
    ]);
  }
});

// 导出刷新函数，供父组件调用
defineExpose({
  refreshData: async () => {
    if (props.server && props.server.connected) {
      await Promise.all([
        fetchClusterInfo(),
        fetchIndicesInfo(),
        fetchShardData(),
      ]);
      // 数据刷新后重新计算高度
      nextTick(() => {
        setTimeout(() => {
          updateScrollbarHeight();
        }, 100);
      });
    }
  },
});

const mergeCells = (row: ShardData, column: any, rowIndex: number) => {
  if (column.key !== "index") return;
  const tableData = props.shardData;
  const current = row.index;
  const prev = tableData[rowIndex - 1]?.index;
  //const next = tableData[rowIndex + 1]?.index;

  // 如果上一行相同 -> 隐藏本行
  if (prev === current) {
    return { rowspan: 0, colspan: 0 };
  }

  // 如果是新组 -> 向下统计 rowspan
  let rowspan = 1;
  for (let i = rowIndex + 1; i < tableData.length; i++) {
    if (tableData[i].index === current) {
      rowspan++;
    } else {
      break;
    }
  }

  return { rowspan, colspan: 1 };
};
</script>

<style scoped>
.status-content {
  padding: 16px 0;
  display: flex;
  flex-direction: column;
  height: 100%;
  max-height: 100%;
}

.info-section {
  flex: 0 0 auto;
}

.shard-section {
  flex: 1;
  overflow: hidden;
  margin-top: 16px;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.info-card {
  height: auto;
}

.shard-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.shard-list-container {
  flex: 1;
  height: 100%;
  overflow: hidden;
}

:deep(.n-scrollbar) {
  height: 100%;
}

.shard-list-header {
  font-weight: bold;
  border-bottom: 1px solid #eee;
}

.shard-list-item {
  border-bottom: 1px solid #f5f5f5;
}

.shard-list-row {
  display: flex;
  flex-direction: row;
}

.shard-list-cell {
  flex: 1;
  padding: 8px 12px;
  min-height: 40px;
  display: flex;
  align-items: center;
}

.index-cell {
  flex: 2;
}
</style>