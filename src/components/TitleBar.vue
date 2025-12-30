<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { ref, onMounted } from "vue";

const appWindow = getCurrentWindow();

function minimizeWindow() {
  appWindow.minimize();
}

function closeWindow() {
  appWindow.close();
}

// 主题切换逻辑
const isDarkMode = ref(false);

function toggleTheme() {
  isDarkMode.value = !isDarkMode.value;
  document.documentElement.classList.toggle("dark", isDarkMode.value);
  localStorage.setItem("theme", isDarkMode.value ? "dark" : "light");
}

onMounted(() => {
  // 初始化主题
  const savedTheme = localStorage.getItem("theme");
  const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  const shouldBeDark = savedTheme ? savedTheme === "dark" : prefersDark;

  isDarkMode.value = shouldBeDark;
  document.documentElement.classList.toggle("dark", shouldBeDark);
});
</script>

<template>
  <div class="titlebar" data-tauri-drag-region @contextmenu.prevent>
    <div class="titlebar-content" data-tauri-drag-region>
      <!-- 标题栏图标 -->
      <div class="titlebar-icon">
        <img
          src="../assets/app-icon.png"
          alt="应用图标"
          width="16"
          height="16"
        />
      </div>
      <div class="titlebar-text">
        <span class="titlebar-title">UM启动项管理</span>
        <span class="titlebar-version">v0.1.0</span>
      </div>
    </div>
    <div class="titlebar-buttons">
      <button class="titlebar-btn theme-toggle-btn" @click="toggleTheme">
        <svg
          v-if="!isDarkMode"
          width="12"
          height="12"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <circle cx="12" cy="12" r="5" />
          <line x1="12" y1="1" x2="12" y2="3" />
          <line x1="12" y1="21" x2="12" y2="23" />
          <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
          <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
          <line x1="1" y1="12" x2="3" y2="12" />
          <line x1="21" y1="12" x2="23" y2="12" />
          <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
          <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
        </svg>
        <svg
          v-else
          width="12"
          height="12"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
        </svg>
      </button>
      <button class="titlebar-btn minimize-btn" @click="minimizeWindow">
        <svg
          width="12"
          height="12"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <line x1="4" y1="12" x2="20" y2="12" />
        </svg>
      </button>
      <button class="titlebar-btn close-btn" @click="closeWindow">
        <svg
          width="12"
          height="12"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <line x1="18" y1="6" x2="6" y2="18" />
          <line x1="6" y1="6" x2="18" y2="18" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  height: 32px;
  background-color: #f6f6f6;
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  user-select: none;
  -webkit-user-select: none;
}

.titlebar-content {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-left: 12px;
}

.titlebar-icon {
  color: #2196f3;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.titlebar-icon img {
  display: block;
  margin: 0;
  padding: 0;
}

.titlebar-text {
  display: flex;
  align-items: center;
  gap: 8px;
}

.titlebar-title {
  font-size: 13px;
  font-weight: 500;
  color: #0f0f0f;
}

.titlebar-version {
  font-size: 11px;
  color: #666;
  opacity: 0.8;
}

.titlebar-buttons {
  display: flex;
  height: 100%;
}

.titlebar-btn {
  width: 46px;
  height: 100%;
  border: none;
  background: transparent;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  color: #666;
  padding: 0;
  box-shadow: none;
  border-radius: 0;
  transition: all 0.2s ease;
}

.theme-toggle-btn {
  background: transparent;
  color: #666;
}

.theme-toggle-btn:hover {
  background: rgba(0, 0, 0, 0.1);
  color: #1a1a1a;
}

.titlebar-btn:hover {
  background-color: rgba(0, 0, 0, 0.1);
}

.titlebar-btn:active {
  background-color: rgba(0, 0, 0, 0.15);
}

.close-btn:hover {
  background-color: #e81123;
  color: #ffffff;
}

.close-btn:active {
  background-color: #c50f1f;
  color: #ffffff;
}

/* Dark Mode */
.dark .titlebar {
  background-color: #2f2f2f;
}

.dark .titlebar-title {
  color: #f6f6f6;
}

.dark .titlebar-version {
  color: #aaa;
}

.dark .titlebar-icon {
  color: #42a5f5;
}

.dark .titlebar-btn {
  color: #aaa;
}

.dark .titlebar-btn:hover {
  background-color: rgba(255, 255, 255, 0.1);
  color: #f0f0f0;
}

.dark .titlebar-btn:active {
  background-color: rgba(255, 255, 255, 0.15);
}

.dark .theme-toggle-btn {
  color: #aaa;
}

.dark .theme-toggle-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #f0f0f0;
}
</style>
