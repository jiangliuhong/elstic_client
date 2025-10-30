<template>
  <div class="data-view">
    <ServerTabs />
  </div>
</template>

<script setup lang="ts">
import ServerTabs from '../components/ServerTabs.vue'
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