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
          <n-button @click="handleRefresh">刷新</n-button>
        </n-space>
        
        <n-data-table
          v-if="selectedData.length > 0"
          :columns="dataColumns"
          :data="selectedData"
          :bordered="true"
          :single-line="false"
          :pagination="pagination"
        />
        <n-empty v-else description="请选择索引并加载数据" />
      </n-space>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { 
  NCard, 
  NSelect, 
  NButton, 
  NDataTable, 
  NEmpty, 
  NSpace 
} from 'naive-ui'

// 定义props
const props = defineProps<{
  indexOptions: Array<any>
  selectedIndex: string | null
  selectedData: Array<any>
  dataColumns: Array<any>
  pagination: any
}>()

// 定义emits
const emit = defineEmits<{
  (e: 'update:selectedIndex', value: string | null): void
  (e: 'refresh'): void
}>()

// 创建本地响应式变量
const localSelectedIndex = computed({
  get: () => props.selectedIndex,
  set: (value) => emit('update:selectedIndex', value)
})

// 处理刷新事件
const handleRefresh = () => {
  emit('refresh')
}
</script>

<style scoped>
.browse-content {
  padding: 16px 0;
}
</style>