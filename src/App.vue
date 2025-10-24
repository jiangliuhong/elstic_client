<script setup lang="ts">
import { ref } from 'vue'
import Sidebar from './components/Sidebar.vue'
import MainContent from './components/MainContent.vue'

const activeMenu = ref('server')

const handleMenuChange = (menu: string) => {
  activeMenu.value = menu
}
</script>

<template>
  <div class="app">
    <div class="main-container">
      <Sidebar @menu-change="handleMenuChange" />
      <MainContent :active-menu="activeMenu" />
    </div>
  </div>
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