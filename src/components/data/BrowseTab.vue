<template>
  <div class="browse-content">
    <n-card>
      <n-space vertical>
        <n-space>
          <n-select 
            v-model:value="localSelectedIndex" 
            :options="indexOptions" 
            placeholder="选择索引"
            style="width: 300px"
          />
          <n-select 
            v-if="localSelectedIndex"
            v-model:value="localSelectedFields" 
            :options="fieldOptions" 
            placeholder="选择字段"
            multiple
            :max-tag-count="3"
            style="width: 300px"
            @update:value="handleFieldChange"
          />
          <n-button @click="handleRefresh">刷新</n-button>
        </n-space>
        
        <div class="table-container" v-if="selectedData.length > 0">
          <n-data-table
            :columns="dataColumns"
            :data="selectedData"
            :bordered="true"
            :single-line="false"
            :pagination="false"
            :scroll-x="1200"
            :max-height="tableHeight"
          />
          
          <div class="pagination-container">
            <n-pagination
              v-model:page="pagination.page"
              v-model:page-size="pagination.pageSize"
              :item-count="pagination.itemCount"
              :page-sizes="pagination.pageSizes"
              show-size-picker
              @update:page="handlePageChange"
              @update:page-size="handlePageSizeChange"
            />
          </div>
        </div>
        <n-empty v-else description="请选择索引并加载数据" />
      </n-space>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { computed, watch, onMounted, onUnmounted, ref } from 'vue'
import { 
  NCard, 
  NSelect, 
  NButton, 
  NDataTable, 
  NEmpty, 
  NSpace,
  NPagination
} from 'naive-ui'

// 定义props
const props = defineProps<{
  indexOptions: Array<any>
  selectedIndex: string | null
  selectedData: Array<any>
  dataColumns: Array<any>
  pagination: any
  fieldOptions: Array<any>
  selectedFields: Array<string>
}>()

// 定义emits
const emit = defineEmits<{
  (e: 'update:selectedIndex', value: string | null): void
  (e: 'refresh'): void
  (e: 'loadIndexData', index: string): void
  (e: 'update:pagination', pagination: any): void
  (e: 'update:selectedFields', fields: Array<string>): void
}>()

// 窗口高度相关
const windowHeight = ref(window.innerHeight)

// 计算表格的动态高度
const tableHeight = computed(() => {
  // 减去其他元素的高度：选择区域(约60px) + 卡片内边距(约32px) + 分页控件(约70px) + 其他间距(约48px) + 额外40px缓冲
  // 确保至少为分页组件预留110px高度
  const availableHeight = windowHeight.value - 290
  const minHeight = Math.max(160, availableHeight) // 最小高度降低到160px，为分页留出更多空间
  return minHeight
})

// 创建本地响应式变量
const localSelectedIndex = computed({
  get: () => props.selectedIndex,
  set: (value) => emit('update:selectedIndex', value)
})

const localSelectedFields = computed({
  get: () => props.selectedFields,
  set: (value) => emit('update:selectedFields', value)
})

// 监听索引选择变化，自动加载数据
watch(localSelectedIndex, (newIndex) => {
  if (newIndex) {
    emit('loadIndexData', newIndex)
  }
})

// 处理刷新事件
const handleRefresh = () => {
  emit('refresh')
}

// 处理分页变化
const handlePageChange = (page: number) => {
  emit('update:pagination', { ...props.pagination, page })
}

// 处理页面大小变化
const handlePageSizeChange = (pageSize: number) => {
  emit('update:pagination', { ...props.pagination, pageSize, page: 1 })
}

// 处理字段选择变化
const handleFieldChange = (fields: Array<string>) => {
  emit('update:selectedFields', fields)
}

// 监听窗口大小变化
const handleResize = () => {
  windowHeight.value = window.innerHeight
}

onMounted(() => {
  window.addEventListener('resize', handleResize)
})

onUnmounted(() => {
  window.removeEventListener('resize', handleResize)
})
</script>

<style scoped>
.browse-content {
  padding: 16px 0;
}

.table-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 300px; /* 确保容器有最小高度 */
}

.pagination-container {
  display: flex;
  justify-content: center;
  padding: 8px 0;
  min-height: 60px; /* 确保分页组件有足够的空间 */
  flex-shrink: 0; /* 防止分页组件被压缩 */
}

/* 禁止多选换行 */
:deep(.n-select .n-base-selection .n-base-selection-tags .n-tag) {
  white-space: nowrap;
}

:deep(.n-select .n-base-selection .n-base-selection-tags) {
  flex-wrap: nowrap;
  overflow: hidden;
}
</style>