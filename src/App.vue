<script setup lang="ts">
import { ref } from 'vue'
import Sidebar from './components/Sidebar.vue'
import { NMessageProvider } from 'naive-ui'
import { RouterView } from 'vue-router'

const activeMenu = ref('server')

const handleMenuChange = (menu: string) => {
  activeMenu.value = menu
}
</script>

<template>
  <n-message-provider>
    <div class="app">
      <div class="main-container">
        <Sidebar @menu-change="handleMenuChange" />
        <RouterView style="width: 100%;height: 100%;" />
      </div>
    </div>
  </n-message-provider>
</template>

<style scoped>
body{
  margin: 0 auto;
}
.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: #f5f5f5;
}

.main-container {
  display: flex;
  flex: 1;
  overflow: hidden;
  height: 100vh;
}
</style>