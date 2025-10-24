<template>
  <div class="main-content">
    <div class="sub-content" v-if="activeMenu === 'server'">
      <ServerView />
    </div>
    <div v-else-if="activeMenu === 'data'">
      <div class="placeholder">数据浏览页面</div>
    </div>
    <div v-else-if="activeMenu === 'settings'">
      <div class="placeholder">设置页面</div>
    </div>
    <div v-else>
      <div class="placeholder">内容区域</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import ServerView from '../views/ServerView.vue'

const props = defineProps<{
  activeMenu: string
}>()

// 我们可以直接使用props.activeMenu而不需要创建本地响应式变量
</script>

<style scoped>
.main-content {
  flex: 1;
  /* padding: 16px; */
  background-color: #ffffff;
  overflow-y: auto;
  border-left: 1px solid #e0e0e0;
  height: 100vh;
  box-sizing: border-box;
}

.placeholder {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  font-size: 20px;
  color: #999999;
  background-color: #fafafa;
  border-radius: 4px;
}
.sub-content{
  height: 100vh;
}
</style>