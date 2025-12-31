<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { StartupItem } from "../types/startup";
import StartupItemComponent from "./StartupItem.vue";

const items = ref<StartupItem[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);
const searchQuery = ref("");
const autoMinimizeSettings = ref<Set<string>>(new Set());
const processNameMappings = ref<Record<string, string>>({});
const minimizeExecTimes = ref<Record<string, number>>({});
const autoStartEnabled = ref(false);
const autoStartPriority = ref(false);
const autoStartLoading = ref(false);
const autoExitAfterMinimize = ref(false);

// 监控状态
const monitorStatus = ref({
  running: false,
  count: 0,
});

// 定时器引用
let monitorStatusTimer: number | null = null;

// 选项卡状态管理
const selectedTab = ref("all");
const tabs = [
  { id: "all", label: "全部" },
  { id: "registry-system", label: "注册表（系统）" },
  { id: "registry-user", label: "注册表（用户）" },
  { id: "folder", label: "启动文件夹" },
];

const filteredItems = computed(() => {
  let result = items.value;

  // 按来源类型过滤
  if (selectedTab.value === "registry-system") {
    result = result.filter(
      (item) =>
        item.source_type === "registry" &&
        item.source_location.includes("HKEY_LOCAL_MACHINE")
    );
  } else if (selectedTab.value === "registry-user") {
    result = result.filter(
      (item) =>
        item.source_type === "registry" &&
        item.source_location.includes("HKEY_CURRENT_USER")
    );
  } else if (selectedTab.value !== "all") {
    result = result.filter((item) => item.source_type === selectedTab.value);
  }

  // 按搜索词过滤
  if (searchQuery.value.trim()) {
    const query = searchQuery.value.toLowerCase();
    result = result.filter(
      (item) =>
        item.name.toLowerCase().includes(query) ||
        item.path.toLowerCase().includes(query) ||
        (item.description && item.description.toLowerCase().includes(query))
    );
  }

  return result;
});

const stats = computed(() => {
  const total = items.value.length;
  const enabled = items.value.filter((item) => item.enabled).length;
  const disabled = total - enabled;
  return { total, enabled, disabled };
});

const loadAutoMinimizeSettings = async () => {
  try {
    const settings = await invoke<string[]>("get_auto_minimize_settings");
    autoMinimizeSettings.value = new Set(settings);
    const mappings = await invoke<Record<string, string>>(
      "get_process_name_mappings"
    );
    processNameMappings.value = mappings;
  } catch (e) {
    console.error("Failed to load auto-minimize settings:", e);
  }
};

// Check if there are items that need monitoring and start/stop monitor accordingly
const updateMonitorState = async () => {
  const hasItemsToMonitor = autoMinimizeSettings.value.size > 0;
  if (hasItemsToMonitor) {
    // Check if running in autostart mode
    const isAutostart = await invoke<boolean>("is_autostart_mode");
    await invoke("start_process_monitor", { autostart: isAutostart });
  }
  // Monitor will auto-stop when no items need monitoring
};

const loadItems = async () => {
  loading.value = true;
  error.value = null;
  try {
    items.value = await invoke<StartupItem[]>("get_startup_items");
    await loadAutoMinimizeSettings();
    // Start monitor if there are items to monitor
    await updateMonitorState();
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  } finally {
    loading.value = false;
  }
};

const handleToggle = async (item: StartupItem, enabled: boolean) => {
  try {
    await invoke("toggle_startup_item", { item, enable: enabled });
    const index = items.value.findIndex((i) => i.id === item.id);
    if (index !== -1) {
      items.value[index] = { ...items.value[index], enabled };
    }
  } catch (e) {
    alert(`操作失败: ${e instanceof Error ? e.message : String(e)}`);
    await loadItems();
  }
};

const handleDelete = async (item: StartupItem) => {
  try {
    await invoke("delete_startup_item", { item });
    items.value = items.value.filter((i) => i.id !== item.id);
  } catch (e) {
    alert(`删除失败: ${e instanceof Error ? e.message : String(e)}`);
  }
};

const handleAutoMinimizeChange = async (
  item: StartupItem,
  enabled: boolean
) => {
  try {
    await invoke("set_auto_minimize", { itemId: item.id, enabled });
    if (enabled) {
      autoMinimizeSettings.value.add(item.id);
    } else {
      autoMinimizeSettings.value.delete(item.id);
    }
    // Trigger reactivity
    autoMinimizeSettings.value = new Set(autoMinimizeSettings.value);
    // Update monitor state based on new settings
    await updateMonitorState();
  } catch (e) {
    alert(`设置失败: ${e instanceof Error ? e.message : String(e)}`);
  }
};

const isAutoMinimize = (itemId: string) => {
  return autoMinimizeSettings.value.has(itemId);
};

const getProcessNameMapping = (itemId: string) => {
  return processNameMappings.value[itemId] || null;
};

const getMinimizeExecTime = (itemId: string) => {
  return minimizeExecTimes.value[itemId] || 0;
};

const handleProcessNameMappingChange = async (
  item: StartupItem,
  processName: string | null
) => {
  try {
    await invoke("set_process_name_mapping", { itemId: item.id, processName });
    if (processName) {
      processNameMappings.value[item.id] = processName;
    } else {
      delete processNameMappings.value[item.id];
    }
  } catch (e) {
    alert(`设置失败: ${e instanceof Error ? e.message : String(e)}`);
  }
};

const loadAutoStartSetting = async () => {
  try {
    autoStartEnabled.value = await invoke<boolean>("get_auto_start_enabled");
    autoStartPriority.value = await invoke<boolean>("get_auto_start_priority");
    autoExitAfterMinimize.value = await invoke<boolean>(
      "get_auto_exit_enabled"
    );
  } catch (e) {
    console.error("Failed to load auto-start setting:", e);
  }
};

const handleAutoStartChange = async () => {
  if (autoStartLoading.value) return;
  autoStartLoading.value = true;
  const newValue = !autoStartEnabled.value;
  try {
    await invoke("set_auto_start_enabled", {
      enabled: newValue,
      priority: autoStartPriority.value,
    });
    autoStartEnabled.value = newValue;
  } catch (e) {
    alert(`设置失败: ${e instanceof Error ? e.message : String(e)}`);
  } finally {
    autoStartLoading.value = false;
  }
};

const handleAutoStartPriorityChange = async () => {
  if (autoStartLoading.value || !autoStartEnabled.value) return;
  autoStartLoading.value = true;
  const newValue = !autoStartPriority.value;
  try {
    await invoke("set_auto_start_enabled", {
      enabled: true,
      priority: newValue,
    });
    autoStartPriority.value = newValue;
  } catch (e) {
    alert(`设置失败: ${e instanceof Error ? e.message : String(e)}`);
  } finally {
    autoStartLoading.value = false;
  }
};

const handleAutoExitChange = async () => {
  try {
    const newValue = !autoExitAfterMinimize.value;
    await invoke("set_auto_exit_enabled", { enabled: newValue });
    autoExitAfterMinimize.value = newValue;
  } catch (e) {
    alert(`设置失败: ${e instanceof Error ? e.message : String(e)}`);
  }
};

// 获取监控状态
const fetchMonitorStatus = async () => {
  try {
    const [running, count] = await invoke<[boolean, number]>(
      "get_monitor_status"
    );
    monitorStatus.value = { running, count };
    // Also fetch minimize execution times
    const times = await invoke<Record<string, number>>(
      "get_minimize_exec_times"
    );
    minimizeExecTimes.value = times;
  } catch (error) {
    console.error("Failed to fetch monitor status:", error);
  }
};

onMounted(() => {
  loadItems();
  loadAutoStartSetting();

  // 初始化监控状态
  fetchMonitorStatus();

  // 设置定时器，每2秒更新一次监控状态
  monitorStatusTimer = window.setInterval(fetchMonitorStatus, 2000);
});

onUnmounted(() => {
  // 清除定时器
  if (monitorStatusTimer) {
    clearInterval(monitorStatusTimer);
    monitorStatusTimer = null;
  }
});
</script>

<template>
  <div class="startup-list-container">
    <!-- Header -->
    <div class="list-header">
      <div class="header-left">
        <label class="auto-start-toggle">
          <span class="auto-start-label">开机启动到托盘</span>
          <button
            class="toggle-btn-mini"
            :class="{ enabled: autoStartEnabled, loading: autoStartLoading }"
            @click="handleAutoStartChange"
            :disabled="autoStartLoading"
          >
            <span class="toggle-track-mini">
              <span class="toggle-thumb-mini"></span>
            </span>
          </button>
        </label>
        <label
          v-if="autoStartEnabled"
          class="auto-start-toggle priority-toggle"
        >
          <span class="auto-start-label">优先启动</span>
          <button
            class="toggle-btn-mini"
            :class="{ enabled: autoStartPriority, loading: autoStartLoading }"
            @click="handleAutoStartPriorityChange"
            :disabled="autoStartLoading"
            title="优先于其他启动项启动（需要管理员权限）"
          >
            <span class="toggle-track-mini">
              <span class="toggle-thumb-mini"></span>
            </span>
          </button>
        </label>
        <label v-if="autoStartEnabled" class="auto-start-toggle">
          <span class="auto-start-label">最小化任务完成后退出</span>
          <button
            class="toggle-btn-mini"
            :class="{ enabled: autoExitAfterMinimize }"
            @click="handleAutoExitChange"
          >
            <span class="toggle-track-mini">
              <span class="toggle-thumb-mini"></span>
            </span>
          </button>
        </label>
      </div>
      <div class="header-right">
        <div class="header-divider"></div>
        <div class="search-box">
          <svg
            class="search-icon"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <circle cx="11" cy="11" r="8" />
            <line x1="21" y1="21" x2="16.65" y2="16.65" />
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索应用..."
            class="search-input"
          />
        </div>
        <button
          class="refresh-btn"
          @click="loadItems"
          :disabled="loading"
          title="刷新"
        >
          <svg
            width="18"
            height="18"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            :class="{ spinning: loading }"
          >
            <polyline points="23 4 23 10 17 10" />
            <polyline points="1 20 1 14 7 14" />
            <path
              d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"
            />
          </svg>
        </button>
      </div>
    </div>

    <!-- 选项卡 -->
    <div class="tabs-container">
      <div class="tabs-left">
        <div
          v-for="tab in tabs"
          :key="tab.id"
          class="tab-item"
          :class="{ active: selectedTab === tab.id }"
          @click="selectedTab = tab.id"
        >
          {{ tab.label }}
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="loading-state">
      <div class="skeleton-item" v-for="i in 5" :key="i">
        <div class="skeleton-icon"></div>
        <div class="skeleton-content">
          <div class="skeleton-line short"></div>
          <div class="skeleton-line long"></div>
          <div class="skeleton-line medium"></div>
        </div>
        <div class="skeleton-actions">
          <div class="skeleton-toggle"></div>
          <div class="skeleton-btn"></div>
        </div>
      </div>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="error-state">
      <svg
        width="48"
        height="48"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <circle cx="12" cy="12" r="10" />
        <line x1="12" y1="8" x2="12" y2="12" />
        <line x1="12" y1="16" x2="12.01" y2="16" />
      </svg>
      <p class="error-title">加载失败</p>
      <p class="error-message">{{ error }}</p>
      <button class="retry-btn" @click="loadItems">重试</button>
    </div>

    <!-- Empty State -->
    <div v-else-if="filteredItems.length === 0" class="empty-state">
      <svg
        width="48"
        height="48"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
      >
        <path d="M22 12h-4l-3 9L9 3l-3 9H2" />
      </svg>
      <p v-if="searchQuery">没有找到匹配的启动项</p>
      <p v-else>没有发现任何启动项</p>
    </div>

    <!-- Items List -->
    <div v-else class="items-list">
      <TransitionGroup name="list">
        <StartupItemComponent
          v-for="item in filteredItems"
          :key="item.id"
          :item="item"
          :auto-minimize="isAutoMinimize(item.id)"
          :process-name-mapping="getProcessNameMapping(item.id)"
          :minimize-exec-time="getMinimizeExecTime(item.id)"
          @toggle="handleToggle"
          @delete="handleDelete"
          @update:auto-minimize="handleAutoMinimizeChange"
          @update:process-name-mapping="handleProcessNameMappingChange"
        />
      </TransitionGroup>
    </div>

    <!-- 状态栏 -->
    <div class="status-bar">
      <div class="monitor-status">
        <span
          class="status-indicator"
          :class="{ running: monitorStatus.running && monitorStatus.count > 0 }"
        ></span>
        <span class="status-text">
          {{
            monitorStatus.running && monitorStatus.count > 0
              ? `正在监控 ${monitorStatus.count} 个进程`
              : "未在监控进程"
          }}
        </span>
      </div>
      <div class="status-right">
        <div class="stats" v-if="!loading && !error">
          <span class="stat-item total">{{ stats.total }} 个项目</span>
          <span class="stat-divider">|</span>
          <span class="stat-item enabled">{{ stats.enabled }} 个启用</span>
          <span class="stat-divider">|</span>
          <span class="stat-item disabled">{{ stats.disabled }} 个禁用</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.startup-list-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}

/* Header */
.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: transparent;
  flex-shrink: 0;
}

/* 选项卡样式 */
.tabs-container {
  display: flex;
  justify-content: space-between;
  padding: 0 20px;
  flex-shrink: 0;
  border-bottom: 1px solid #e0e0e0;
}

.tabs-left {
  display: flex;
  gap: 8px;
  align-items: center;
  flex-shrink: 0;
}

.tabs-right {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  height: 100%;
  padding: 6px 0;
  margin-left: auto;
}

.tab-item {
  padding: 6px 16px;
  font-size: 13px;
  font-weight: 500;
  color: #666;
  cursor: pointer;
  border-radius: 6px 6px 0 0;
  transition: all 0.2s ease;
  border-bottom: 2px solid transparent;
}

.tab-item:hover {
  color: #2196f3;
}

.tab-item.active {
  color: #2196f3;
  border-bottom-color: #2196f3;
  background-color: rgba(33, 150, 243, 0.05);
}

.header-left {
  display: flex;
  flex-direction: row;
  gap: 16px;
  align-items: center;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  justify-content: flex-end;
}

.auto-start-toggle {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.auto-start-label {
  font-size: 13px;
  color: #666;
  user-select: none;
}

.header-divider {
  width: 1px;
  height: 20px;
  background: #e0e0e0;
}

/* Mini Toggle Switch */
.toggle-btn-mini {
  background: none;
  border: none;
  padding: 2px;
  cursor: pointer;
  outline: none;
}

.toggle-btn-mini:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.toggle-track-mini {
  display: block;
  width: 36px;
  height: 20px;
  background: #ccc;
  border-radius: 10px;
  position: relative;
  transition: background 0.2s ease;
}

.toggle-btn-mini.enabled .toggle-track-mini {
  background: #4caf50;
}

.toggle-thumb-mini {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  background: white;
  border-radius: 50%;
  transition: transform 0.2s ease;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}

.toggle-btn-mini.enabled .toggle-thumb-mini {
  transform: translateX(16px);
}

.toggle-btn-mini.loading .toggle-thumb-mini {
  animation: pulse 0.5s infinite;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.search-box {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 12px;
  color: #999;
  pointer-events: none;
}

.search-input {
  padding: 8px 12px 8px 36px;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  font-size: 14px;
  width: 200px;
  transition: all 0.2s ease;
  background: #fff;
  color: #1a1a1a;
}

.search-input:focus {
  outline: none;
  border-color: #2196f3;
  box-shadow: 0 0 0 3px rgba(33, 150, 243, 0.1);
}

.search-input::placeholder {
  color: #999;
}

.refresh-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: transparent;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  color: #999;
  transition: all 0.2s ease;
}

.refresh-btn:hover {
  background: #f0f0f0;
  color: #666;
}

.refresh-btn:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.refresh-btn svg.spinning {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

/* Items List */
.items-list {
  flex: 1;
  overflow-y: auto;
  padding: 12px 20px 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;

  /* 自定义滚动条样式 */
  scrollbar-width: thin;
  scrollbar-color: #ccc #f5f5f5;
}

/* WebKit 滚动条样式 */
.items-list::-webkit-scrollbar {
  width: 6px;
}

.items-list::-webkit-scrollbar-track {
  background: #f5f5f5;
  border-radius: 3px;
}

.items-list::-webkit-scrollbar-thumb {
  background: #ccc;
  border-radius: 3px;
}

/* 深色模式滚动条 */
.dark .items-list {
  scrollbar-color: #555 #3a3a3a;
}

.dark .items-list::-webkit-scrollbar-track {
  background: #3a3a3a;
}

.dark .items-list::-webkit-scrollbar-thumb {
  background: #555;
}

/* List Transitions */
.list-move,
.list-enter-active,
.list-leave-active {
  transition: all 0.3s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}

.list-leave-active {
  position: absolute;
}

/* Loading State */
.loading-state {
  flex: 1;
  padding: 0 20px 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.skeleton-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: #fff;
  border-radius: 12px;
}

.skeleton-icon {
  width: 48px;
  height: 48px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: 8px;
}

.skeleton-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.skeleton-line {
  height: 12px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: 4px;
}

.skeleton-line.short {
  width: 40%;
}

.skeleton-line.medium {
  width: 30%;
}

.skeleton-line.long {
  width: 70%;
}

.skeleton-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.skeleton-toggle {
  width: 44px;
  height: 24px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: 12px;
}

.skeleton-btn {
  width: 36px;
  height: 36px;
  background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: 8px;
}

@keyframes shimmer {
  0% {
    background-position: 200% 0;
  }
  100% {
    background-position: -200% 0;
  }
}

/* Error State */
.error-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #999;
  gap: 12px;
}

.error-title {
  font-size: 18px;
  font-weight: 600;
  color: #e53935;
  margin: 0;
}

.error-message {
  font-size: 14px;
  color: #999;
  margin: 0;
  text-align: center;
  max-width: 300px;
}

.retry-btn {
  margin-top: 8px;
  padding: 10px 24px;
  background: #2196f3;
  color: white;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.retry-btn:hover {
  background: #1976d2;
}

/* Empty State */
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #999;
  gap: 12px;
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}

/* Dark Mode */
/* Dark Mode */
.dark .stats {
  color: #888;
}

.dark .stat-divider {
  color: #444;
}

.dark .tabs-container {
  border-bottom-color: #444;
}

.dark .tab-item {
  color: #aaa;
}

.dark .tab-item:hover {
  color: #42a5f5;
}

.dark .tab-item.active {
  color: #42a5f5;
  border-bottom-color: #42a5f5;
  background-color: rgba(66, 165, 245, 0.1);
}

.dark .search-input {
  background: #3a3a3a;
  border-color: #4a4a4a;
  color: #f0f0f0;
}

.dark .search-input:focus {
  border-color: #2196f3;
  box-shadow: 0 0 0 3px rgba(33, 150, 243, 0.2);
}

.dark .refresh-btn {
  background: transparent;
  color: #777;
}

.dark .refresh-btn:hover {
  background: #4a4a4a;
  color: #aaa;
}

.dark .skeleton-item {
  background: #3a3a3a;
}

.dark .skeleton-icon,
.dark .skeleton-line,
.dark .skeleton-toggle,
.dark .skeleton-btn {
  background: linear-gradient(90deg, #4a4a4a 25%, #555 50%, #4a4a4a 75%);
  background-size: 200% 100%;
}

.dark .error-state,
.dark .empty-state {
  color: #888;
}

.dark .auto-start-label {
  color: #aaa;
}

.dark .header-divider {
  background: #444;
}

.dark .toggle-track-mini {
  background: #555;
}

/* 状态栏样式 */
.status-bar {
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  background-color: rgba(0, 0, 0, 0.03);
  border-top: 1px solid #e0e0e0;
  flex-shrink: 0;
  font-size: 12px;
  color: #666;
}

.status-right {
  display: flex;
  align-items: center;
  justify-content: flex-end;
}

.status-right .stats {
  font-size: 12px;
  color: #666;
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-right .stat-item.enabled {
  color: #4caf50;
}

.status-right .stat-item.disabled {
  color: #999;
}

.status-right .stat-divider {
  color: #ddd;
}

.monitor-status {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: #999;
  transition: background-color 0.3s ease;
}

.status-indicator.running {
  background-color: #4caf50;
}

.status-text {
  font-size: 12px;
  color: #666;
}

/* 深色模式下的状态栏样式 */
.dark .status-bar {
  background-color: rgba(255, 255, 255, 0.05);
  border-top: 1px solid #444;
}

.dark .status-text {
  color: #aaa;
}

.dark .status-right .stats {
  color: #aaa;
}

.dark .status-right .stat-divider {
  color: #444;
}

.dark .status-right .stat-item.enabled {
  color: #66bb6a;
}

.dark .status-right .stat-item.disabled {
  color: #777;
}

.dark .status-indicator {
  background-color: #666;
}

.dark .status-indicator.running {
  background-color: #66bb6a;
}
</style>
