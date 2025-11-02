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
                >●</span
              >
              <span
                v-else
                class="status-indicator disconnected"
                title="连接断开"
                >●</span
              >
            </div>
          </template>

          <!-- 服务器数据内容 -->
          <div class="server-content" :style="{ height: tabContentHeight }">
            <n-tabs
              v-model:value="serverTab.activeMainTab"
              type="card"
              closable
              animated
              @add="handleAddQueryTab"
            >
              <n-tab-pane name="status" tab="状态" :closable="false">
                <StatusTab
                  :ref="(el) => (statusTabRefs[serverTab.id] = el)"
                  :server="serverTab.server"
                  :loading="serverTab.loading"
                  :cluster-info="serverTab.clusterInfo"
                  :indices-info="serverTab.indicesInfo"
                  :shard-columns="serverTab.shardColumns"
                  :shard-data="serverTab.shardData"
                  :on-update-cluster-info="
                    (info) =>
                      updateServerData(serverTab.id, 'clusterInfo', info)
                  "
                  :on-update-indices-info="
                    (info) =>
                      updateServerData(serverTab.id, 'indicesInfo', info)
                  "
                  :on-update-shard-data="
                    (data) => updateServerData(serverTab.id, 'shardData', data)
                  "
                  :on-update-loading="
                    (loading) =>
                      updateServerData(serverTab.id, 'loading', loading)
                  "
                />
              </n-tab-pane>

              <n-tab-pane name="browse" tab="数据浏览" :closable="false">
                <BrowseTab
                  :index-options="serverTab.indexOptions"
                  :selected-index="serverTab.selectedIndex"
                  :selected-data="serverTab.selectedData"
                  :data-columns="serverTab.dataColumns"
                  :pagination="serverTab.pagination"
                  :field-options="serverTab.fieldOptions"
                  :selected-fields="serverTab.selectedFields"
                  @update:selectedIndex="
                    (index) =>
                      updateServerData(serverTab.id, 'selectedIndex', index)
                  "
                  @refresh="() => refreshServerData(serverTab.id)"
                  @loadIndexData="(index) => loadIndexData(serverTab.id, index)"
                  @update:pagination="(pagination) => updateServerData(serverTab.id, 'pagination', pagination)"
                  @update:selectedFields="(fields) => updateSelectedFields(serverTab.id, fields)"
                />
              </n-tab-pane>

              <!-- 动态查询tabs -->
              <n-tab-pane
                v-for="queryTab in serverTab.queryTabs"
                :key="queryTab.id"
                :name="queryTab.id"
                :tab="queryTab.name"
                closable
                @close="handleQueryTabClose(serverTab.id, queryTab.id)"
              >
                <n-space vertical style="padding-bottom: 16px">
                  <!-- 基本查询 -->
                  <div v-show="queryTab.type === 'basic'">
                    <n-space vertical>
                      <!-- 第一行：选择索引 -->
                      <n-space align="center">
                        <n-text>选择索引：</n-text>
                        <n-select
                          v-model:value="queryTab.basicForm.selectedIndex"
                          :options="serverTab.indexOptions"
                          placeholder="选择索引"
                          style="width: 200px"
                        />
                      </n-space>

                      <!-- 第二行：查询条件 -->
                      <div
                        v-for="(condition, index) in queryTab.basicForm
                          .conditions"
                        :key="index"
                      >
                        <n-space align="center">
                          <!-- 1. 下拉框（must、must_not、should） -->
                          <n-select
                            v-model:value="condition.type"
                            :options="[
                              { label: 'must', value: 'must' },
                              { label: 'must_not', value: 'must_not' },
                              { label: 'should', value: 'should' },
                            ]"
                            placeholder="选择类型"
                            style="width: 120px"
                          />

                          <!-- 2. 下拉框（字段选择） -->
                          <n-select
                            v-model:value="condition.field"
                            :options="serverTab.queryFieldOptions"
                            placeholder="选择字段"
                            filterable
                            style="width: 150px"
                            @update:value="
                              (val) =>
                                handleFieldChange(
                                  serverTab.id,
                                  queryTab.id,
                                  index,
                                  val
                                )
                            "
                          />

                          <!-- 3. 下拉框（查询方式） -->
                          <n-select
                            v-if="condition.field"
                            v-model:value="condition.operator"
                            :options="[
                              { label: 'match', value: 'match' },
                              { label: 'term', value: 'term' },
                              { label: 'wildcard', value: 'wildcard' },
                              { label: 'prefix', value: 'prefix' },
                              { label: 'range', value: 'range' },
                              { label: 'query_string', value: 'query_string' },
                              { label: 'text', value: 'text' },
                              { label: 'missing', value: 'missing' },
                            ]"
                            placeholder="选择查询方式"
                            style="width: 120px"
                          />

                          <!-- 4. 根据查询方式显示不同控件 -->
                          <!-- match, term, wildcard, query_string, text -->
                          <n-input
                            v-if="
                              condition.operator &&
                              [
                                'match',
                                'term',
                                'wildcard',
                                'query_string',
                                'text',
                              ].includes(condition.operator)
                            "
                            v-model:value="condition.value"
                            placeholder="输入值"
                            style="width: 200px"
                          />

                          <!-- range -->
                          <template v-if="condition.operator === 'range'">
                            <n-select
                              v-model:value="condition.rangeType.gt"
                              :options="[
                                { label: '大于', value: 'gt' },
                                { label: '大于等于', value: 'gte' },
                              ]"
                              style="width: 100px"
                            />
                            <n-input
                              v-model:value="condition.rangeType.gtValue"
                              placeholder="值"
                              style="width: 120px"
                            />
                            <n-select
                              v-model:value="condition.rangeType.lt"
                              :options="[
                                { label: '小于', value: 'lt' },
                                { label: '小于等于', value: 'lte' },
                              ]"
                              style="width: 100px"
                            />
                            <n-input
                              v-model:value="condition.rangeType.ltValue"
                              placeholder="值"
                              style="width: 120px"
                            />
                          </template>

                          <!-- 5. 添加和删除按钮 -->
                          <n-button
                            @click="addCondition(serverTab.id, queryTab.id)"
                            size="small"
                            type="primary"
                            >添加</n-button
                          >
                          <n-button
                            @click="
                              removeCondition(serverTab.id, queryTab.id, index)
                            "
                            size="small"
                            type="error"
                            :disabled="
                              queryTab.basicForm.conditions.length <= 1
                            "
                            >删除</n-button
                          >
                        </n-space>
                      </div>
                    </n-space>
                  </div>

                  <!-- 高级查询 -->
                  <div v-show="queryTab.type === 'advanced'">
                    <n-space vertical>
                      <!-- 第一行：选择索引 -->
                      <n-space align="center">
                        <n-text>选择索引：</n-text>
                        <n-select
                          v-model:value="queryTab.advancedForm.selectedIndex"
                          :options="serverTab.indexOptions"
                          placeholder="选择索引"
                          style="width: 200px"
                        />
                      </n-space>

                      <!-- 第二行：HTTP方法和路径 -->
                      <n-space align="center">
                        <n-select
                          v-model:value="queryTab.advancedForm.method"
                          :options="[
                            { label: 'POST', value: 'POST' },
                            { label: 'GET', value: 'GET' },
                            { label: 'PUT', value: 'PUT' },
                            { label: 'DELETE', value: 'DELETE' },
                          ]"
                          placeholder="选择方法"
                          style="width: 100px"
                        />
                        <n-input
                          v-model:value="queryTab.advancedForm.path"
                          placeholder="_search"
                          style="width: 200px"
                        />
                      </n-space>

                      <!-- 第三行：JSON输入框 -->
                      <n-form-item label="JSON内容">
                        <n-input
                          v-model:value="queryTab.advancedForm.json"
                          type="textarea"
                          :autosize="{ minRows: 10 }"
                          placeholder="请输入JSON内容"
                        />
                      </n-form-item>
                    </n-space>
                  </div>

                  <n-space>
                    <n-button
                      @click="runQuery(serverTab.id, queryTab.id)"
                      type="primary"
                      >执行查询</n-button
                    >
                    <n-button @click="saveQuery(serverTab.id, queryTab.id)"
                      >保存查询</n-button
                    >
                  </n-space>

                  <n-card
                    title="查询结果"
                    v-show="queryTab.result && queryTab.result.length > 0"
                  >
                    <div class="query-result-container">
                      <n-data-table
                        :columns="serverTab.resultColumns"
                        :data="queryTab.result"
                        :bordered="true"
                        :single-line="false"
                        :pagination="false"
                        :scroll-x="800"
                        :max-height="queryResultTableHeight"
                      />
                      
                      <div class="pagination-container">
                        <n-pagination
                          v-model:page="serverTab.resultPagination.page"
                          v-model:page-size="serverTab.resultPagination.pageSize"
                          :item-count="queryTab.result.length"
                          :page-sizes="[10, 20, 50]"
                          show-size-picker
                        />
                      </div>
                    </div>
                  </n-card>
                </n-space>
              </n-tab-pane>

              <template #suffix>
                <n-button @click="showAddQueryModal = true">新增查询</n-button>
              </template>
            </n-tabs>

            <!-- 新增查询弹窗 -->
            <n-modal
              v-model:show="showAddQueryModal"
              preset="dialog"
              title="新增查询"
            >
              <template #header>
                <div>新增查询</div>
              </template>

              <n-form
                :model="newQueryForm"
                label-placement="left"
                label-width="100"
              >
                <n-form-item label="查询名称" :required="true">
                  <n-input
                    v-model:value="newQueryForm.name"
                    placeholder="请输入查询名称"
                  />
                </n-form-item>

                <n-form-item label="类型" :required="true">
                  <n-select
                    v-model:value="newQueryForm.type"
                    :options="queryTypeOptions"
                    placeholder="选择查询类型"
                  />
                </n-form-item>
              </n-form>

              <template #action>
                <n-space>
                  <n-button @click="showAddQueryModal = false">取消</n-button>
                  <n-button type="primary" @click="handleAddQuery"
                    >确认</n-button
                  >
                </n-space>
              </template>
            </n-modal>
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
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from "vue";
import {
  NTabs,
  NTabPane,
  NResult,
  NButton,
  NModal,
  NForm,
  NFormItem,
  NInput,
  NSelect,
  NSpace,
  NRadioGroup,
  NRadioButton,
  NCard,
  NDataTable,
  NPagination,
  NText,
} from "naive-ui";
import { useRouter } from "vue-router";
import {
  getConnectedServers,
  removeConnectedServer,
} from "../stores/serverStore";
import StatusTab from "./data/StatusTab.vue";
import BrowseTab from "./data/BrowseTab.vue";

interface Server {
  id: string;
  name: string;
  url: string;
  username?: string;
  password?: string;
  connected?: boolean;
  connectionError?: string;
}

interface ServerTabData {
  id: string;
  name: string;
  server: Server;
  connected: boolean;

  // 活动的主tab
  activeMainTab: string;

  // 状态tab数据
  loading: {
    cluster: boolean;
    indices: boolean;
    shards: boolean;
  };
  clusterInfo: {
    name: string;
    nodes: number;
    status: string;
    version: string;
  };
  indicesInfo: {
    count: number;
    documents: string;
    size: string;
  };
  shardColumns: Array<{ title: string; key: string }>;
  shardData: Array<any>;

  // 数据浏览tab数据
  indexOptions: Array<{ label: string; value: string }>;
  selectedIndex: string | null;
  selectedData: Array<any>;
  dataColumns: Array<{ title: string; key: string }>;
  allDataColumns: Array<{ title: string; key: string; minWidth?: number }>;
  pagination: {
    page: number;
    pageSize: number;
    showSizePicker: boolean;
    pageSizes: number[];
    itemCount?: number;
    pageCount?: number;
  };
  selectedFields: Array<string>;
  fieldOptions: Array<{ label: string; value: string }>;

  // 查询tab数据
  queryTabs: Array<{
    id: string;
    name: string;
    type: string;
    basicForm: {
      selectedIndex: string;
      conditions: Array<{
        type: string;
        field: string;
        operator: string;
        value: string;
        rangeType: {
          gt: string;
          gtValue: string;
          lt: string;
          ltValue: string;
        };
      }>;
    };
    advancedForm: {
      selectedIndex: string;
      method: string;
      path: string;
      json: string;
    };
    result: Array<any>;
  }>;
  queryFieldOptions: Array<{ label: string; value: string }>;
  operatorOptions: Array<{ label: string; value: string }>;
  resultColumns: Array<{ title: string; key: string }>;
  resultPagination: {
    page: number;
    pageSize: number;
  };
}

const router = useRouter();
const statusTabRefs = ref<Record<string, any>>({});

// 弹窗显示状态
const showAddQueryModal = ref(false);

// 新增查询表单数据
const newQueryForm = ref({
  name: "",
  type: "basic",
});

// 查询类型选项
const queryTypeOptions = [
  { label: "基本查询", value: "basic" },
  { label: "高级查询", value: "advanced" },
];

// 活动页签ID
const activeTabId = ref<string>("");

// 服务器页签数据
const serverTabs = ref<ServerTabData[]>([]);

// 窗口高度相关
const windowHeight = ref(window.innerHeight);
const tabContentHeight = computed(() => {
  // 减去其他元素的高度（标题栏、导航栏等），这里假设需要减去120px
  return `${windowHeight.value - 120}px`;
});

// 计算查询结果表格的动态高度
const queryResultTableHeight = computed(() => {
  // 减去其他元素的高度：查询表单(约400px) + 标题(约50px) + 分页控件(约70px) + 间距 + 额外40px缓冲
  // 确保至少为分页组件预留110px高度
  const availableHeight = windowHeight.value - 560
  const minHeight = Math.max(110, availableHeight) // 最小高度降低到110px
  return minHeight
});

// 监听窗口大小变化
const handleResize = () => {
  windowHeight.value = window.innerHeight;
};

onMounted(() => {
  window.addEventListener("resize", handleResize);
});

onUnmounted(() => {
  window.removeEventListener("resize", handleResize);
});

// 获取已连接的服务器
const connectedServers = computed(() => getConnectedServers());

// 监听已连接服务器变化，同步页签
watch(
  connectedServers,
  (servers) => {
    nextTick(async () => {
      await syncTabsWithServers();
    });
  },
  { immediate: true, deep: true }
);

// 同步页签与服务器连接状态
const syncTabsWithServers = async () => {
  const currentServerIds = new Set(serverTabs.value.map((tab) => tab.id));
  const connectedServerIds = new Set(
    connectedServers.value.map((server) => server.id)
  );

  // 移除已断开连接的服务器页签
  serverTabs.value = serverTabs.value.filter((tab) =>
    connectedServerIds.has(tab.id)
  );

  // 添加新连接的服务器页签
  for (const server of connectedServers.value) {
    if (!currentServerIds.has(server.id)) {
      await addServerTab(server);
    }
  }

  // 如果没有活动页签，设置第一个为活动页签
  if (serverTabs.value.length > 0 && !activeTabId.value) {
    activeTabId.value = serverTabs.value[0].id;
  }
};

// 添加服务器页签
const addServerTab = async (server: Server) => {
  // 初始分片数据
  const initialShardData = [
    {
      index: "user_data",
      shard: "0",
      prirep: "p",
      type: "primary",
      status: "STARTED",
      node: "node-1",
    },
    {
      index: "user_data",
      shard: "1",
      prirep: "r",
      type: "replica",
      status: "STARTED",
      node: "node-2",
    },
    {
      index: "product_info",
      shard: "0",
      prirep: "p",
      type: "primary",
      status: "STARTED",
      node: "node-3",
    },
    {
      index: "order_history",
      shard: "0",
      prirep: "p",
      type: "primary",
      status: "STARTED",
      node: "node-1",
    },
    {
      index: "order_history",
      shard: "1",
      prirep: "r",
      type: "replica",
      status: "STARTED",
      node: "node-2",
    },
    {
      index: "user_logs",
      shard: "0",
      prirep: "p",
      type: "primary",
      status: "STARTED",
      node: "node-3",
    },
    {
      index: "user_logs",
      shard: "1",
      prirep: "r",
      type: "replica",
      status: "STARTED",
      node: "node-1",
    },
  ];

  // 从分片数据中提取唯一的索引
  const uniqueIndexes = [
    ...new Set(initialShardData.map((item) => item.index)),
  ];
  let indexOptions = uniqueIndexes.map((index) => ({
    label: index,
    value: index,
  }));

  // 尝试从实际服务器获取索引列表
  try {
    const { default: ElasticsearchService } = await import('../services/elasticsearchService');
    const service = new ElasticsearchService({
      url: server.url,
      username: server.username,
      password: server.password
    });
    
    const indices = await service.getIndices();
    indexOptions = indices.map(index => ({
      label: index.name,
      value: index.name
    }));
  } catch (error) {
    console.warn("无法获取实际索引列表，使用默认数据:", error);
  }

  const newTab: ServerTabData = {
    id: server.id,
    name: server.name,
    server: server,
    connected: server.connected || false,

    // 活动的主tab，默认为状态
    activeMainTab: "status",

    // 初始化状态tab数据
    loading: {
      cluster: false,
      indices: false,
      shards: false,
    },
    clusterInfo: {
      name: "elasticsearch_cluster",
      nodes: 3,
      status: "green",
      version: "N/A",
    },
    indicesInfo: {
      count: 4,
      documents: "1,245,678",
      size: "2.4 GB",
    },
    shardColumns: [
      { title: "索引", key: "index" },
      { title: "分片", key: "shard" },
      { title: "主/副本", key: "prirep" },
      { title: "类型", key: "type" },
      { title: "状态", key: "status" },
      { title: "节点", key: "node" },
    ],
    shardData: initialShardData,

    // 初始化数据浏览tab数据
    indexOptions: indexOptions,
    selectedIndex: null,
    selectedData: [],
    dataColumns: [
      { title: "ID", key: "id", minWidth: 120 },
      { title: "Name", key: "name", minWidth: 150 },
      { title: "Type", key: "type", minWidth: 100 },
      { title: "Created", key: "created", minWidth: 120 },
    ],
    allDataColumns: [
      { title: "ID", key: "id", minWidth: 120 },
      { title: "Name", key: "name", minWidth: 150 },
      { title: "Type", key: "type", minWidth: 100 },
      { title: "Created", key: "created", minWidth: 120 },
    ],
    pagination: {
      page: 1,
      pageSize: 10,
      showSizePicker: true,
      pageSizes: [10, 20, 50],
    },
    selectedFields: [],
    fieldOptions: [],

    // 初始化查询相关数据
    queryFieldOptions: [
      { label: "id", value: "id" },
      { label: "name", value: "name" },
      { label: "type", value: "type" },
      { label: "created", value: "created" },
    ],

    // 初始化查询tab数据
    queryTabs: [],
    operatorOptions: [
      { label: "等于", value: "equals" },
      { label: "包含", value: "contains" },
      { label: "大于", value: "greater" },
      { label: "小于", value: "less" },
    ],
    resultColumns: [
      { title: "ID", key: "id", minWidth: 120 },
      { title: "Name", key: "name", minWidth: 150 },
      { title: "Score", key: "score", minWidth: 80 },
    ],
    resultPagination: {
      page: 1,
      pageSize: 10,
    },
  };

  serverTabs.value.push(newTab);

  // 如果这是第一个页签，设置为活动页签
  if (serverTabs.value.length === 1) {
    activeTabId.value = newTab.id;
  }
};

// 处理页签关闭
const handleTabClose = (tabId: string) => {
  // 从已连接服务器列表中移除
  removeConnectedServer(tabId);

  // 使用 nextTick 确保 DOM 更新完成后再处理活动页签
  nextTick(() => {
    // 如果关闭的是当前活动页签，切换到下一个页签
    if (activeTabId.value === tabId) {
      const remainingTabs = serverTabs.value.filter((tab) => tab.id !== tabId);
      if (remainingTabs.length > 0) {
        const currentIndex = serverTabs.value.findIndex(
          (tab) => tab.id === tabId
        );
        // 切换到下一个页签，如果没有下一个，则切换到上一个
        const nextIndex =
          currentIndex < remainingTabs.length
            ? currentIndex
            : Math.max(0, currentIndex - 1);
        activeTabId.value = remainingTabs[nextIndex].id;
      } else {
        activeTabId.value = "";
      }
    }
  });
};

// 处理页签切换
const handleTabChange = (tabId: string) => {
  activeTabId.value = tabId;

  // 使用 nextTick 确保 DOM 更新完成后再刷新数据
  nextTick(() => {
    const statusTabRef = statusTabRefs.value[tabId];
    if (statusTabRef) {
      statusTabRef.refreshData();
    }
  });
};

// 更新服务器数据
const updateServerData = (serverId: string, dataType: string, data: any) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (tab) {
    (tab as any)[dataType] = data;

    // 如果更新的是分片数据，同时更新索引选项
    if (dataType === "shardData" && Array.isArray(data)) {
      const uniqueIndexes = [...new Set(data.map((item: any) => item.index))];
      tab.indexOptions = uniqueIndexes.map((index) => ({
        label: index,
        value: index,
      }));
    }

    // 如果选择了索引，获取该索引的字段信息
    if (dataType === "selectedIndex" && data) {
      fetchIndexFields(serverId, data);
    }

    // 如果更新的是分页信息，重新加载数据
    if (dataType === "pagination" && tab.selectedIndex) {
      loadIndexData(serverId, tab.selectedIndex);
    }
  }
};

// 更新选择的字段
const updateSelectedFields = (serverId: string, fields: Array<string>) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (!tab) return;

  // 更新选择的字段
  tab.selectedFields = fields;

  // 根据选择的字段过滤列
  if (fields.length === 0) {
    // 如果没有选择任何字段，显示所有列
    tab.dataColumns = [...tab.allDataColumns];
  } else {
    // 只显示选择的字段列
    const filteredColumns = tab.allDataColumns.filter(column => 
      fields.includes(column.key) || 
      ['_id', '_score'].includes(column.key) // 始终显示ID和Score列
    );
    tab.dataColumns = filteredColumns;
  }
};

// 获取索引字段信息
const fetchIndexFields = async (serverId: string, indexName: string) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (!tab) return;

  try {
    // 导入 ElasticsearchService
    const { default: ElasticsearchService } = await import('../services/elasticsearchService');
    
    // 创建服务实例
    const service = new ElasticsearchService({
      url: tab.server.url,
      username: tab.server.username,
      password: tab.server.password
    });

    // 获取索引映射
    const mappingResponse = await service.getIndexMapping(indexName);
    
    // 提取字段信息
    const fields = Object.keys(mappingResponse[indexName]?.mappings?.properties || {});
    
    // 构建字段选项
    const fieldOptions = fields.map(field => ({
      label: field,
      value: field
    }));

    // 更新数据浏览和查询的字段选项
    tab.fieldOptions = fieldOptions;
    tab.queryFieldOptions = fieldOptions;
  } catch (error) {
    console.error("获取索引字段失败:", error);
    // 使用默认字段
    const defaultFields = [
      { label: "id", value: "id" },
      { label: "name", value: "name" },
      { label: "type", value: "type" },
    ];
    tab.fieldOptions = defaultFields;
    tab.queryFieldOptions = defaultFields;
  }
};

// 刷新服务器数据
const refreshServerData = (serverId: string) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (tab && tab.selectedIndex) {
    loadIndexData(serverId, tab.selectedIndex);
  }
};

// 加载索引数据
const loadIndexData = async (serverId: string, indexName: string) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (!tab) return;

  try {
    // 导入 ElasticsearchService
    const { default: ElasticsearchService } = await import('../services/elasticsearchService');
    
    // 创建服务实例
    const service = new ElasticsearchService({
      url: tab.server.url,
      username: tab.server.username,
      password: tab.server.password
    });

    // 获取索引数据
    const response = await service.getIndexData(
      indexName, 
      tab.pagination.page, 
      tab.pagination.pageSize
    );

    // 获取索引映射以确定列
    const mappingResponse = await service.getIndexMapping(indexName);
    
    // 提取字段信息
    const fields = Object.keys(mappingResponse[indexName]?.mappings?.properties || {});
    
    // 构建所有列配置
    const allColumns = fields.map(field => ({
      title: field,
      key: field,
      ellipsis: true,
      resizable: true,
      minWidth: 120
    }));

    // 添加 ID 列
    allColumns.unshift({
      title: 'ID',
      key: '_id',
      width: 150,
      minWidth: 120
    });

    // 添加得分列
    allColumns.push({
      title: 'Score',
      key: '_score',
      width: 100,
      minWidth: 80
    });

    // 保存所有列配置
    tab.allDataColumns = allColumns;

    // 初始化字段选项
    tab.fieldOptions = fields.map(field => ({
      label: field,
      value: field
    }));

    // 初始化选择的字段（默认选择所有字段）
    tab.selectedFields = fields;

    // 初始显示所有列
    tab.dataColumns = [...allColumns];

    // 转换数据格式
    const data = response.hits.hits.map(hit => ({
      _id: hit._id,
      _score: hit._score,
      ...hit._source
    }));

    // 更新标签页数据
    updateServerData(serverId, 'selectedData', data);
    
    // 更新分页总数
    updateServerData(serverId, 'pagination', {
      ...tab.pagination,
      itemCount: response.hits.total.value,
      pageCount: Math.ceil(response.hits.total.value / tab.pagination.pageSize)
    });
    
  } catch (error) {
    console.error('加载索引数据失败:', error);
    // 使用模拟数据作为后备
    tab.selectedData = [];
    tab.allDataColumns = [
      { title: 'ID', key: '_id', minWidth: 120 },
      { title: 'Error', key: 'error', minWidth: 120 }
    ];
    tab.dataColumns = [...tab.allDataColumns];
    tab.fieldOptions = [];
    tab.selectedFields = [];
  }
};

// 处理新增查询按钮点击
const handleAddQueryTab = () => {
  showAddQueryModal.value = true;
};

// 处理新增查询
const handleAddQuery = () => {
  if (!newQueryForm.value.name.trim()) {
    return;
  }

  const activeServerTab = serverTabs.value.find(
    (tab) => tab.id === activeTabId.value
  );
  if (activeServerTab) {
    const newId = `query-${Date.now()}`;
    const newQueryTab = {
      id: newId,
      name: newQueryForm.value.name,
      type: newQueryForm.value.type,
      basicForm: {
        selectedIndex: "",
        conditions: [
          {
            type: "must",
            field: "",
            operator: "",
            value: "",
            rangeType: {
              gt: "gt",
              gtValue: "",
              lt: "lt",
              ltValue: "",
            },
          },
        ],
      },
      advancedForm: {
        selectedIndex: "",
        method: "POST",
        path: "_search",
        json: "",
      },
      result: [],
    };

    activeServerTab.queryTabs.push(newQueryTab);
    activeServerTab.activeMainTab = newId;
  }

  // 重置表单
  newQueryForm.value = {
    name: "",
    type: "basic",
  };
  // 关闭弹窗
  showAddQueryModal.value = false;
};

// 处理字段变化
const handleFieldChange = (
  serverId: string,
  tabId: string,
  conditionIndex: number,
  field: string
) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (tab) {
    const queryTab = tab.queryTabs.find((qt) => qt.id === tabId);
    if (queryTab) {
      // 清空操作符和值
      queryTab.basicForm.conditions[conditionIndex].operator = "";
      queryTab.basicForm.conditions[conditionIndex].value = "";
    }
  }
};

// 添加查询条件
const addCondition = (serverId: string, tabId: string) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (tab) {
    const queryTab = tab.queryTabs.find((qt) => qt.id === tabId);
    if (queryTab) {
      queryTab.basicForm.conditions.push({
        type: "must",
        field: "",
        operator: "",
        value: "",
        rangeType: {
          gt: "gt",
          gtValue: "",
          lt: "lt",
          ltValue: "",
        },
      });
    }
  }
};

// 删除查询条件
const removeCondition = (
  serverId: string,
  tabId: string,
  conditionIndex: number
) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (tab) {
    const queryTab = tab.queryTabs.find((qt) => qt.id === tabId);
    if (queryTab && queryTab.basicForm.conditions.length > 1) {
      queryTab.basicForm.conditions.splice(conditionIndex, 1);
    }
  }
};

// 执行查询
const runQuery = (serverId: string, tabId: string) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (tab) {
    const queryTab = tab.queryTabs.find((qt) => qt.id === tabId);
    if (queryTab) {
      // 模拟查询结果
      queryTab.result = [
        { id: "1", name: "Result 1", score: 0.95 },
        { id: "2", name: "Result 2", score: 0.87 },
      ];
    }
  }
};

// 保存查询
const saveQuery = (serverId: string, tabId: string) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (tab) {
    const queryTab = tab.queryTabs.find((qt) => qt.id === tabId);
    if (queryTab) {
      // 这里可以实现保存查询的逻辑
      console.log("保存查询:", queryTab);
    }
  }
};

// 跳转到服务器管理页面
const goToServerPage = () => {
  router.push("/server");
};

// 关闭查询tab
const handleQueryTabClose = (serverId: string, tabId: string) => {
  const tab = serverTabs.value.find((t) => t.id === serverId);
  if (tab) {
    const index = tab.queryTabs.findIndex((queryTab) => queryTab.id === tabId);
    if (index !== -1) {
      tab.queryTabs.splice(index, 1);
      // 如果关闭的是当前激活的tab，切换到状态tab
      if (tab.activeMainTab === tabId) {
        tab.activeMainTab = "status";
      }
    }
  }
};
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
  overflow-y: auto;
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

/* 查询结果容器样式 */
.query-result-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 250px; /* 确保容器有最小高度 */
}

.pagination-container {
  display: flex;
  justify-content: center;
  padding: 8px 0;
  min-height: 60px; /* 确保分页组件有足够的空间 */
  flex-shrink: 0; /* 防止分页组件被压缩 */
}

/* 确保页签内容高度正确 */
:deep(.n-tabs-nav) {
  margin-bottom: 0;
  position: sticky;
  top: 0;
  z-index: 10;
  background: white;
}

:deep(.n-tabs-content) {
  height: calc(100% - 46px);
  overflow: hidden;
  position: relative;
}

:deep(.n-tabs-nav) {
  position: sticky;
  top: 0;
  z-index: 10;
  background: white;
}

:deep(.n-tab-pane) {
  height: 100%;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

:deep(.n-tab-pane > div) {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
}
</style>