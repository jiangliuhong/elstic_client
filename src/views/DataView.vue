<template>
  <div class="data-view">
    <ServerTabs />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { getIsAuthenticated } from '../stores/authStore'
import ServerTabs from '../components/ServerTabs.vue'

const router = useRouter()

onMounted(() => {
  // 检查认证状态
  if (!getIsAuthenticated()) {
    router.replace('/login')
  }
})
</script>

<style scoped>
.data-view {
  background-color: #ffffff;
  border-radius: 4px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  height: 100%;
}
</style>