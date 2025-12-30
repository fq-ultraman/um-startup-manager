<script setup lang="ts">
import TitleBar from "./components/TitleBar.vue";
import StartupList from "./components/StartupList.vue";
</script>

<template>
  <div class="app-container" @contextmenu.prevent>
    <TitleBar />
    <main class="main-content" @selectstart.prevent>
      <StartupList />
    </main>
  </div>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;

  user-select: none;
  -webkit-user-select: none;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100%;
  overflow: hidden;
}

/* 基于类的主题切换 */
:root.dark {
  color: #f6f6f6;
  background-color: #2f2f2f;
}

/* 确保.app-container和.main-content也继承背景色 */
.dark .app-container,
.dark .main-content {
  background-color: #2f2f2f;
}
</style>

<style scoped>
.app-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.main-content {
  flex: 1;
  margin-top: 32px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
</style>
