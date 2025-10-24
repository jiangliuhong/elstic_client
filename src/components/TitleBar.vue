<template>
  <div class="title-bar">
    <div class="title-bar-left">
      <div class="app-title">ElasticSearch</div>
    </div>
    <div class="title-bar-right">
      <button class="window-control minimize">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <path d="M0 6h12v1H0z" fill="currentColor"></path>
        </svg>
      </button>
      <button class="window-control maximize">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <path
            d="M1 1h10v10H1V1zm1 1v8h8V2H2z"
            fill="currentColor"
            stroke="currentColor"
            stroke-width="0.5"
          ></path>
        </svg>
      </button>
      <button class="window-control close">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <path
            d="M6 4.586L10.293.293l1.414 1.414L7.414 6l4.293 4.293-1.414 1.414L6 7.414l-4.293 4.293-1.414-1.414L4.586 6 .293 1.707 1.707.293 6 4.586z"
            fill="currentColor"
          ></path>
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
// 简化版本，暂时移除Tauri API调用以避免构建错误
</script>

<style scoped>
.title-bar {
  height: 100px;
  background-color: #333;
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 16px;
  -webkit-app-region: drag;
}

.title-bar-left {
  font-size: 24px;
  font-weight: bold;
}

.title-bar-right {
  display: flex;
  gap: 16px;
  -webkit-app-region: no-drag;
}

.window-control {
  width: 40px;
  height: 30px;
  background: none;
  border: none;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.window-control:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.window-control.close:hover {
  background-color: #e81123;
}
</style>