<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted } from "vue";

// 从全局变量获取应用版本号
const appVersion = __APP_VERSION__;

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

// 菜单逻辑
const menuVisible = ref(false);

function toggleMenu() {
  menuVisible.value = !menuVisible.value;
}

function closeMenu() {
  menuVisible.value = false;
}

// 模态窗逻辑
const showResetConfirm = ref(false);
const showSuccessModal = ref(false);
const successMessage = ref("");
const errorMessage = ref("");
const showErrorModal = ref(false);

function handleResetClick() {
  showResetConfirm.value = true;
  closeMenu();
}

// 最小化到托盘处理函数
async function minimizeToTray() {
  try {
    // 使用Tauri 2.0 API隐藏窗口到托盘
    await appWindow.hide();
  } catch (error) {
    showError(
      `最小化到托盘失败：${
        error instanceof Error ? error.message : String(error)
      }`,
    );
  }
  closeMenu();
}

function showSuccess(message: string) {
  successMessage.value = message;
  showSuccessModal.value = true;
}

function showError(message: string) {
  errorMessage.value = message;
  showErrorModal.value = true;
}

function handleSuccessClose() {
  showSuccessModal.value = false;
  // 调用后端的reload_app命令重新加载页面
  invoke("reload_app");
}

function handleErrorClose() {
  showErrorModal.value = false;
}

const confirmReset = async () => {
  showResetConfirm.value = false;
  try {
    await invoke("reset_settings");
    showSuccess("配置已成功清除，应用将重新加载。");
  } catch (error) {
    showError(
      `清除配置失败：${error instanceof Error ? error.message : String(error)}`,
    );
  }
};

const cancelReset = () => {
  showResetConfirm.value = false;
};

const openConfigFolder = async () => {
  try {
    await invoke("open_config_folder");
  } catch (error) {
    showError(
      `打开配置文件夹失败：${error instanceof Error ? error.message : String(error)}`,
    );
  }
};

onMounted(() => {
  // 初始化主题
  const savedTheme = localStorage.getItem("theme");
  const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  const shouldBeDark = savedTheme ? savedTheme === "dark" : prefersDark;

  isDarkMode.value = shouldBeDark;
  document.documentElement.classList.toggle("dark", shouldBeDark);

  // 点击外部关闭菜单
  document.addEventListener("click", (e) => {
    const menuBtn = document.querySelector(".menu-btn");
    const menuDropdown = document.querySelector(".menu-dropdown");
    if (
      menuBtn &&
      menuDropdown &&
      !menuBtn.contains(e.target as Node) &&
      !menuDropdown.contains(e.target as Node)
    ) {
      closeMenu();
    }
  });
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
        <span class="titlebar-version">v{{ appVersion }}</span>
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

      <!-- 菜单按钮 -->
      <div class="menu-container">
        <button class="titlebar-btn menu-btn" @click="toggleMenu">
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
            <line x1="3" y1="12" x2="21" y2="12" />
            <line x1="3" y1="6" x2="21" y2="6" />
            <line x1="3" y1="18" x2="21" y2="18" />
          </svg>
        </button>

        <!-- 下拉菜单 -->
        <div class="menu-dropdown" v-if="menuVisible">
          <button class="menu-item" @click="minimizeToTray">
            最小化到托盘
          </button>
          <button class="menu-item" @click="handleResetClick">清除配置</button>
        </div>
      </div>

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

  <!-- 清除配置确认模态窗 -->
  <Teleport to="body">
    <div v-if="showResetConfirm" class="modal-overlay" @click="cancelReset">
      <div class="modal-content" @click.stop>
        <div class="modal-header">确认清除配置</div>
        <div class="modal-body">
          确定要清除所有配置吗？此操作将删除所有已保存的设置，且无法恢复。
        </div>
        <div class="modal-actions">
          <a
            href="javascript:void(0)"
            class="text-link"
            @click="openConfigFolder"
          >
            打开配置文件夹
          </a>
          <div class="modal-actions-right">
            <button class="btn-cancel" @click="cancelReset">取消</button>
            <button class="btn-confirm" @click="confirmReset">清除</button>
          </div>
        </div>
      </div>
    </div>

    <!-- 成功提示模态窗 -->
    <div
      v-if="showSuccessModal"
      class="modal-overlay"
      @click="handleSuccessClose"
    >
      <div class="modal-content" @click.stop>
        <div class="modal-header">操作成功</div>
        <div class="modal-body">
          {{ successMessage }}
        </div>
        <div class="modal-actions">
          <div class="modal-actions-right"></div>
          <button class="btn-confirm" @click="handleSuccessClose">确定</button>
        </div>
      </div>
    </div>

    <!-- 错误提示模态窗 -->
    <div v-if="showErrorModal" class="modal-overlay" @click="handleErrorClose">
      <div class="modal-content" @click.stop>
        <div class="modal-header">操作失败</div>
        <div class="modal-body">
          {{ errorMessage }}
        </div>
        <div class="modal-actions">
          <div class="modal-actions-right"></div>
          <button class="btn-confirm" @click="handleErrorClose">确定</button>
        </div>
      </div>
    </div>
  </Teleport>
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

/* 菜单样式 */
.menu-container {
  position: relative;
  height: 100%;
}

.menu-btn:hover {
  background-color: rgba(0, 0, 0, 0.1);
  color: #1a1a1a;
}

.dark .menu-btn:hover {
  background-color: rgba(255, 255, 255, 0.1);
  color: #f0f0f0;
}

.menu-dropdown {
  position: absolute;
  top: calc(100% + 2px);
  left: 0;
  background-color: #ffffff;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  min-width: 120px;
  z-index: 1001;
  overflow: hidden;
  transition: all 0.2s ease;
}

.dark .menu-dropdown {
  background-color: #333333;
  border: 1px solid #555555;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
}

.menu-item {
  display: block;
  width: 100%;
  padding: 8px 16px;
  text-align: left;
  border: none;
  background: transparent;
  cursor: pointer;
  font-size: 13px;
  color: #333333;
  transition: all 0.2s ease;
}

.menu-item:hover {
  background-color: #f0f0f0;
  color: #1a1a1a;
}

.dark .menu-item {
  color: #cccccc;
}

.dark .menu-item:hover {
  background-color: #444444;
  color: #ffffff;
}

/* 模态窗样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.modal-content {
  background: white;
  border-radius: 16px;
  padding: 24px;
  max-width: 400px;
  width: 90%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.modal-header {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 16px;
  color: #1a1a1a;
}

.modal-body {
  font-size: 14px;
  color: #666;
  margin-bottom: 24px;
  line-height: 1.5;
}

.modal-body strong {
  color: #1a1a1a;
}

.modal-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.modal-actions-right {
  display: flex;
  gap: 12px;
}

.btn-cancel,
.btn-confirm,
.btn-secondary {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
}

.btn-cancel {
  background: #f5f5f5;
  color: #666;
}

.btn-cancel:hover {
  background: #e0e0e0;
}

.btn-secondary {
  background: #2196f3;
  color: white;
}

.btn-secondary:hover {
  background: #1976d2;
}

.btn-confirm {
  background: #e53935;
  color: white;
}

.btn-confirm:hover {
  background: #c62828;
}

/* 深色模式 */
.dark .modal-content {
  background: #3a3a3a;
}

.dark .modal-header {
  color: #f0f0f0;
}

.dark .modal-body {
  color: #aaa;
}

.dark .modal-body strong {
  color: #f0f0f0;
}

.dark .btn-cancel {
  background: #4a4a4a;
  color: #aaa;
}

.dark .btn-cancel:hover {
  background: #555;
}

.dark .btn-secondary {
  background: #42a5f5;
  color: white;
}

.dark .btn-secondary:hover {
  background: #1e88e5;
}

/* 文字链接样式 */
.text-link {
  color: #2196f3;
  text-decoration: none;
  font-size: 14px;
  padding: 10px 0;
  transition: color 0.2s ease;
}

.text-link:hover {
  color: #1976d2;
}

.dark .text-link {
  color: #42a5f5;
}

.dark .text-link:hover {
  color: #1e88e5;
}
</style>
