<template>
  <div class="query-content">
    <n-tabs 
      v-model:value="localQueryTab" 
      type="card" 
      closable
      @close="handleQueryTabClose"
      @add="handleAddQueryTab"
    >
      <n-tab-pane 
        v-for="tab in queryTabs" 
        :key="tab.id" 
        :name="tab.id" 
        :tab="tab.name"
      >
        <n-space vertical>
          <n-radio-group v-model:value="tab.type" @update:value="(val) => handleQueryTypeChange(tab.id, val)">
            <n-radio-button value="basic">基本查询</n-radio-button>
            <n-radio-button value="advanced">高级查询</n-radio-button>
          </n-radio-group>
          
          <div v-if="tab.type === 'basic'">
            <n-form :model="tab.basicForm" label-placement="left" label-width="120">
              <n-form-item label="字段">
                <n-select 
                  v-model:value="tab.basicForm.field" 
                  :options="fieldOptions" 
                  placeholder="选择字段"
                />
              </n-form-item>
              <n-form-item label="操作符">
                <n-select 
                  v-model:value="tab.basicForm.operator" 
                  :options="operatorOptions" 
                  placeholder="选择操作符"
                />
              </n-form-item>
              <n-form-item label="值">
                <n-input v-model:value="tab.basicForm.value" placeholder="输入查询值" />
              </n-form-item>
            </n-form>
          </div>
          
          <div v-else>
            <n-form :model="tab.advancedForm" label-placement="left">
              <n-form-item label="查询DSL">
                <n-input
                  v-model:value="tab.advancedForm.dsl"
                  type="textarea"
                  :autosize="{ minRows: 10 }"
                  placeholder="请输入ElasticSearch查询DSL"
                />
              </n-form-item>
            </n-form>
          </div>
          
          <n-space>
            <n-button @click="handleRunQuery(tab.id)" type="primary">执行查询</n-button>
            <n-button @click="handleSaveQuery(tab.id)">保存查询</n-button>
          </n-space>
          
          <n-card title="查询结果" v-if="tab.result.length > 0">
            <n-data-table
              :columns="resultColumns"
              :data="tab.result"
              :bordered="true"
              :single-line="false"
              :pagination="resultPagination"
            />
          </n-card>
        </n-space>
      </n-tab-pane>
      
      <template #suffix>
        <n-button @click="handleAddQueryTab">+</n-button>
      </template>
    </n-tabs>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { 
  NTabs, 
  NTabPane, 
  NCard, 
  NDataTable, 
  NSelect, 
  NButton, 
  NSpace, 
  NRadioGroup, 
  NRadioButton, 
  NForm, 
  NFormItem, 
  NInput 
} from 'naive-ui'

// 定义props
const props = defineProps<{
  queryTab: string
  queryTabs: Array<any>
  fieldOptions: Array<any>
  operatorOptions: Array<any>
  resultColumns: Array<any>
  resultPagination: any
}>()

// 定义emits
const emit = defineEmits<{
  (e: 'update:queryTab', value: string): void
  (e: 'addQueryTab'): void
  (e: 'closeQueryTab', name: string): void
  (e: 'queryTypeChange', tabId: string, type: string): void
  (e: 'runQuery', tabId: string): void
  (e: 'saveQuery', tabId: string): void
}>()

// 创建本地响应式变量
const localQueryTab = computed({
  get: () => props.queryTab,
  set: (value) => emit('update:queryTab', value)
})

// 处理添加查询tab
const handleAddQueryTab = () => {
  emit('addQueryTab')
}

// 处理关闭查询tab
const handleQueryTabClose = (name: string) => {
  emit('closeQueryTab', name)
}

// 处理查询类型切换
const handleQueryTypeChange = (tabId: string, type: string) => {
  emit('queryTypeChange', tabId, type)
}

// 处理执行查询
const handleRunQuery = (tabId: string) => {
  emit('runQuery', tabId)
}

// 处理保存查询
const handleSaveQuery = (tabId: string) => {
  emit('saveQuery', tabId)
}
</script>

<style scoped>
.query-content {
  padding: 16px 0;
}
</style>