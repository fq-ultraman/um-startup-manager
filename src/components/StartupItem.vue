<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { StartupItem } from "../types/startup";

const props = defineProps<{
  item: StartupItem;
  autoMinimize: boolean;
  processNameMapping: string | null;
  minimizeBehavior: string | null;
  minimizeDelay: number;
  minimizeExecTime: number;
}>();

const hasCustomSettings = computed(() => {
  return (
    props.processNameMapping ||
    props.minimizeBehavior ||
    props.minimizeDelay > 0
  );
});

const emit = defineEmits<{
  toggle: [item: StartupItem, enabled: boolean];
  delete: [item: StartupItem];
  "update:autoMinimize": [item: StartupItem, enabled: boolean];
  "update:processNameMapping": [item: StartupItem, processName: string | null];
}>();

const isToggling = ref(false);
const isDeleting = ref(false);
const showDeleteConfirm = ref(false);
const showProcessNameModal = ref(false);
const customProcessName = ref(props.processNameMapping || "");
const minimizeBehavior = ref("minimize");
const minimizeDelay = ref(0);

watch(
  () => props.processNameMapping,
  (val) => {
    customProcessName.value = val || "";
  }
);

const openProcessNameModal = async () => {
  customProcessName.value = props.processNameMapping || "";
  try {
    minimizeBehavior.value = await invoke<string>("get_minimize_behavior", {
      itemId: props.item.id,
    });
    minimizeDelay.value = await invoke<number>("get_minimize_delay", {
      itemId: props.item.id,
    });
  } catch (e) {
    minimizeBehavior.value = "minimize";
    minimizeDelay.value = 0;
  }
  showProcessNameModal.value = true;
};

const saveProcessName = async () => {
  const value = customProcessName.value.trim() || null;
  emit("update:processNameMapping", props.item, value);
  try {
    await invoke("set_minimize_behavior", {
      itemId: props.item.id,
      behavior: minimizeBehavior.value,
    });
    await invoke("set_minimize_delay", {
      itemId: props.item.id,
      delay: minimizeDelay.value > 0 ? minimizeDelay.value : null,
    });
  } catch (e) {
    console.error("Failed to save settings:", e);
  }
  showProcessNameModal.value = false;
};

const sourceTypeLabel = computed(() => {
  if (props.item.source_type === "registry") {
    // Check if it's HKLM (system) or HKCU (user)
    return {
      main: "注册表",
      tag: props.item.source_location.includes("HKEY_LOCAL_MACHINE")
        ? "系统"
        : "用户",
    };
  }
  return { main: "启动文件夹", tag: "" };
});

const registrySubType = computed(() => {
  if (props.item.source_type === "registry") {
    return props.item.source_location.includes("HKEY_LOCAL_MACHINE")
      ? "system"
      : "user";
  }
  return "";
});

const formattedExecTime = computed(() => {
  if (!props.minimizeExecTime) return null;
  const date = new Date(props.minimizeExecTime);
  const year = date.getFullYear();
  const month = (date.getMonth() + 1).toString().padStart(2, "0");
  const day = date.getDate().toString().padStart(2, "0");
  const hours = date.getHours().toString().padStart(2, "0");
  const minutes = date.getMinutes().toString().padStart(2, "0");
  const seconds = date.getSeconds().toString().padStart(2, "0");
  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
});

const handleToggle = () => {
  if (isToggling.value) return;
  isToggling.value = true;
  emit("toggle", props.item, !props.item.enabled);
  setTimeout(() => {
    isToggling.value = false;
  }, 500);
};

const handleDelete = () => {
  showDeleteConfirm.value = true;
};

const confirmDelete = () => {
  if (isDeleting.value) return;
  isDeleting.value = true;
  showDeleteConfirm.value = false;
  emit("delete", props.item);
};

const cancelDelete = () => {
  showDeleteConfirm.value = false;
};

const handleAutoMinimizeChange = (event: Event) => {
  const checked = (event.target as HTMLInputElement).checked;
  emit("update:autoMinimize", props.item, checked);
};

const handleBadgeClick = async () => {
  try {
    if (props.item.source_type === "registry") {
      await invoke("open_registry_location", {
        path: props.item.source_location,
      });
    } else {
      await invoke("open_startup_folder", { path: props.item.source_location });
    }
  } catch (e) {
    console.error("Failed to open location:", e);
  }
};

const handlePathClick = async () => {
  try {
    await invoke("open_file_location", { path: props.item.path });
  } catch (e) {
    console.error("Failed to open file location:", e);
  }
};
</script>

<template>
  <div class="startup-item" :class="{ disabled: !item.enabled }">
    <div class="item-main-row">
      <div class="item-icon">
        <img v-if="item.icon" :src="item.icon" alt="App Icon" />
        <div v-else class="icon-placeholder">
          <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
            <circle cx="8.5" cy="8.5" r="1.5" />
            <polyline points="21 15 16 10 5 21" />
          </svg>
        </div>
      </div>
      <div class="item-info">
        <div class="item-name">
          {{ item.description || item.name }}
          <span
            v-if="!item.valid"
            class="item-invalid-badge"
            title="目标文件不存在"
          >
            无效
          </span>
          <span
            class="item-source-badge"
            :class="[item.source_type, registrySubType]"
            @click="handleBadgeClick"
            :title="
              item.source_type === 'registry'
                ? '点击打开注册表'
                : '点击打开启动文件夹'
            "
          >
            <template v-if="sourceTypeLabel.main && sourceTypeLabel.tag">
              <span class="badge-text">{{ sourceTypeLabel.main }}</span>
              <span class="badge-tag">{{ sourceTypeLabel.tag }}</span>
            </template>
            <template v-else>
              {{ sourceTypeLabel.main }}
            </template>
          </span>
        </div>
        <div
          class="item-path"
          :title="`点击打开该文件所在文件夹`"
          @click="handlePathClick"
        >
          {{ item.path }}
        </div>
        <div class="item-auto-minimize">
          <div class="auto-minimize-label">
            <label class="checkbox-wrapper">
              <input
                type="checkbox"
                :checked="autoMinimize"
                @change="handleAutoMinimizeChange"
                class="auto-minimize-checkbox"
              />
              <span class="checkbox-custom"></span>
            </label>
            <span class="checkbox-text">启动后自动最小化</span>
            <button
              v-if="autoMinimize"
              class="process-name-btn"
              @click="openProcessNameModal"
              :class="{ configured: hasCustomSettings }"
              title="设置实际进程名"
            >
              <svg
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <circle cx="12" cy="12" r="3" />
                <path
                  d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
                />
              </svg>
            </button>
            <span
              v-if="autoMinimize && formattedExecTime"
              class="exec-time-text"
              title="已执行最小化"
            >
              {{ formattedExecTime }}
            </span>
          </div>
        </div>
      </div>
      <div class="item-actions">
        <button
          class="toggle-btn"
          :class="{ enabled: item.enabled, loading: isToggling }"
          @click="handleToggle"
          :disabled="isToggling"
          :title="item.enabled ? '点击禁用' : '点击启用'"
        >
          <span class="toggle-track">
            <span class="toggle-thumb"></span>
          </span>
        </button>
        <button
          class="delete-btn"
          @click="handleDelete"
          :disabled="isDeleting"
          title="删除启动项"
        >
          <svg
            width="18"
            height="18"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <polyline points="3 6 5 6 21 6" />
            <path
              d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
            />
            <line x1="10" y1="11" x2="10" y2="17" />
            <line x1="14" y1="11" x2="14" y2="17" />
          </svg>
        </button>
      </div>
    </div>
    <Teleport to="body">
      <div v-if="showDeleteConfirm" class="modal-overlay" @click="cancelDelete">
        <div class="modal-content" @click.stop>
          <div class="modal-header">确认删除</div>
          <div class="modal-body">
            确定要删除启动项
            <strong>{{ item.description || item.name }}</strong>
            吗？此操作无法撤销。
          </div>
          <div class="modal-actions">
            <button class="btn-cancel" @click="cancelDelete">取消</button>
            <button class="btn-confirm" @click="confirmDelete">删除</button>
          </div>
        </div>
      </div>
      <div v-if="showProcessNameModal" class="modal-overlay">
        <div class="modal-content" @click.stop>
          <div class="modal-header">自动最小化设置</div>
          <div class="modal-body">
            <div class="setting-group">
              <div class="setting-label">最小化行为</div>
              <div class="radio-group">
                <label class="radio-item">
                  <input
                    type="radio"
                    v-model="minimizeBehavior"
                    value="minimize"
                  />
                  <span class="radio-custom"></span>
                  <span class="radio-text">最小化到任务栏</span>
                </label>
                <label class="radio-item">
                  <input
                    type="radio"
                    v-model="minimizeBehavior"
                    value="close"
                  />
                  <span class="radio-custom"></span>
                  <span class="radio-text">直接关闭窗口</span>
                </label>
              </div>
            </div>
            <div class="setting-group">
              <div class="setting-label">实际进程名</div>
              <p class="modal-hint">
                如果启动后运行的是另一个进程，请设置实际进程名。
              </p>
              <input
                type="text"
                v-model="customProcessName"
                @keyup.enter="saveProcessName"
                placeholder="进程名（不含.exe）"
                class="process-name-input"
              />
            </div>
            <div class="setting-group">
              <div class="setting-label">延迟执行 (秒)</div>
              <p class="modal-hint">
                设置延迟时间，监控到进程后将等待指定秒数再执行操作。
              </p>
              <input
                type="number"
                v-model.number="minimizeDelay"
                @keyup.enter="saveProcessName"
                placeholder="0（立即执行）"
                min="0"
                class="process-name-input"
              />
            </div>
          </div>
          <div class="modal-actions">
            <button class="btn-cancel" @click="showProcessNameModal = false">
              取消
            </button>
            <button class="btn-save" @click="saveProcessName">保存</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.startup-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 16px;
  background: #ffffff;
  border-radius: 12px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.startup-item.disabled {
  opacity: 0.6;
}

.item-main-row {
  display: flex;
  align-items: center;
  gap: 16px;
  min-height: 48px;
}

.item-icon {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.item-icon img {
  width: 40px;
  height: 40px;
  object-fit: contain;
}

.icon-placeholder {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f0f0f0;
  border-radius: 8px;
  color: #999;
}

.item-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 2px;
}

.item-name {
  font-size: 15px;
  font-weight: 600;
  color: #1a1a1a;
  display: flex;
  align-items: center;
  gap: 8px;
}

.item-source-badge {
  font-size: 10px;
  padding: 0 6px;
  border-radius: 4px;
  font-weight: 500;
  margin-left: 4px;
  cursor: pointer;
  transition: all 0.2s ease;
  display: inline-flex;
  align-items: center;
}

.badge-text {
  margin-right: 3px;
}

.badge-tag {
  font-size: 10px;
  font-weight: 400;
  padding: 1px 4px;
  border-radius: 8px;
  background-color: rgba(255, 255, 255, 0.5);
  color: inherit;
  vertical-align: middle;
  line-height: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.item-source-badge:hover {
  transform: scale(1.05);
}

.item-invalid-badge {
  font-size: 10px;
  padding: 0 6px;
  border-radius: 4px;
  font-weight: 500;
  background: #ffebee;
  color: #c62828;
}

.item-source-badge.registry {
  background: #e3f2fd;
  color: #1565c0;
}

.item-source-badge.registry:hover {
  background: #bbdefb;
}

.item-source-badge.registry.system {
  background: #fff3e0;
  color: #e65100;
}

.item-source-badge.registry.system:hover {
  background: #ffe0b2;
}

.item-source-badge.registry.user {
  background: #e3f2fd;
  color: #1565c0;
}

.item-source-badge.registry.user:hover {
  background: #bbdefb;
}

.item-source-badge.folder {
  background: #e8f5e9;
  color: #2e7d32;
}

.item-source-badge.folder:hover {
  background: #c8e6c9;
}

.item-path {
  font-size: 12px;
  color: #666;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  cursor: pointer;
  transition: color 0.2s ease;
}

.item-path:hover {
  color: #2196f3;
}

.auto-minimize-label {
  display: flex;
  align-items: center;
  gap: 6px;
  user-select: none;
}

.checkbox-wrapper {
  cursor: pointer;
  display: inline-block;
}

.auto-minimize-checkbox {
  display: none;
}

.checkbox-custom {
  width: 16px;
  height: 16px;
  border: 2px solid #ccc;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.checkbox-wrapper:hover .checkbox-custom {
  border-color: #2196f3;
}

.auto-minimize-checkbox:checked + .checkbox-custom {
  background: #2196f3;
  border-color: #2196f3;
}

.auto-minimize-checkbox:checked + .checkbox-custom::after {
  content: "";
  width: 4px;
  height: 8px;
  border: solid white;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
  margin-bottom: 2px;
}

.checkbox-text {
  font-size: 11px;
  color: #666;
}

.process-name-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  margin-left: 4px;
  background: transparent;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  color: #999;
  transition: all 0.2s ease;
}

.process-name-btn:hover {
  background: #f0f0f0;
  color: #666;
}

.process-name-btn.configured {
  color: #2196f3;
}

.process-name-btn.configured:hover {
  background: #e3f2fd;
}

.exec-time-text {
  font-size: 11px;
  color: #999;
  margin-left: 8px;
}

.process-name-input {
  width: 100%;
  padding: 10px 12px;
  font-size: 14px;
  border: 1px solid #ddd;
  border-radius: 8px;
  outline: none;
  transition: border-color 0.2s ease;
}

.process-name-input:focus {
  border-color: #2196f3;
}

.modal-hint {
  font-size: 13px;
  color: #666;
  margin: 0 0 12px 0;
}

.btn-save {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  background: #2196f3;
  border: none;
  color: white;
}

.btn-save:hover {
  background: #1976d2;
}

.setting-group {
  margin-bottom: 16px;
}

.setting-group:last-child {
  margin-bottom: 0;
}

.setting-label {
  font-size: 14px;
  font-weight: 500;
  color: #1a1a1a;
  margin-bottom: 6px;
}

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.radio-item {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  user-select: none;
}

.radio-item input[type="radio"] {
  display: none;
}

.radio-custom {
  width: 18px;
  height: 18px;
  border: 2px solid #ccc;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.radio-item:hover .radio-custom {
  border-color: #2196f3;
}

.radio-item input[type="radio"]:checked + .radio-custom {
  border-color: #2196f3;
}

.radio-item input[type="radio"]:checked + .radio-custom::after {
  content: "";
  width: 10px;
  height: 10px;
  background: #2196f3;
  border-radius: 50%;
}

.radio-text {
  font-size: 13px;
  color: #333;
}



.item-actions {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.toggle-btn {
  background: none;
  border: none;
  padding: 4px;
  cursor: pointer;
  outline: none;
}

.toggle-btn:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.toggle-track {
  display: block;
  width: 44px;
  height: 24px;
  background: #ccc;
  border-radius: 12px;
  position: relative;
  transition: background 0.2s ease;
}

.toggle-btn.enabled .toggle-track {
  background: #4caf50;
}

.toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background: white;
  border-radius: 50%;
  transition: transform 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle-btn.enabled .toggle-thumb {
  transform: translateX(20px);
}

.toggle-btn.loading .toggle-thumb {
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

.delete-btn {
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

.delete-btn:hover {
  background: #f0f0f0;
  color: #666;
}

.delete-btn:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

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
  justify-content: flex-end;
  gap: 12px;
}

.btn-cancel,
.btn-confirm {
  padding: 10px 20px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-cancel {
  background: #f5f5f5;
  border: none;
  color: #666;
}

.btn-cancel:hover {
  background: #e0e0e0;
}

.btn-confirm {
  background: #e53935;
  border: none;
  color: white;
}

.btn-confirm:hover {
  background: #c62828;
}

/* Dark Mode */
.dark .startup-item {
  background: #3a3a3a;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}
.dark .item-name {
  color: #f0f0f0;
}
.dark .item-path {
  color: #aaa;
}
.dark .item-path:hover {
  color: #64b5f6;
}
.dark .item-invalid-badge {
  background: #4a1515;
  color: #ef9a9a;
}
.dark .item-source-badge.registry {
  background: #1e3a5f;
  color: #90caf9;
}
.dark .item-source-badge.registry:hover {
  background: #2a4a6f;
}
.dark .item-source-badge.registry.system {
  background: #4a3000;
  color: #ffb74d;
}
.dark .item-source-badge.registry.system:hover {
  background: #5a4010;
}
.dark .item-source-badge.registry.user {
  background: #1e3a5f;
  color: #90caf9;
}
.dark .item-source-badge.registry.user:hover {
  background: #2a4a6f;
}
.dark .item-source-badge.folder {
  background: #1b3d20;
  color: #a5d6a7;
}

.dark .badge-tag {
  background-color: rgba(0, 0, 0, 0.3);
}
.dark .item-source-badge.folder:hover {
  background: #2a4d30;
}
.dark .icon-placeholder {
  background: #4a4a4a;
  color: #888;
}
.dark .toggle-track {
  background: #555;
}
.dark .delete-btn {
  background: transparent;
  color: #777;
}
.dark .delete-btn:hover {
  background: #4a4a4a;
  color: #aaa;
}
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
.dark .checkbox-custom {
  border-color: #555;
}
.dark .checkbox-text {
  color: #aaa;
}
.dark .checkbox-wrapper:hover .checkbox-custom {
  border-color: #2196f3;
}
.dark .process-name-btn {
  color: #777;
}
.dark .process-name-btn:hover {
  background: #4a4a4a;
  color: #aaa;
}
.dark .process-name-btn.configured {
  color: #42a5f5;
}
.dark .process-name-btn.configured:hover {
  background: rgba(66, 165, 245, 0.2);
}
.dark .process-name-input {
  background: #4a4a4a;
  border-color: #555;
  color: #f0f0f0;
}
.dark .process-name-input:focus {
  border-color: #2196f3;
}
.dark .modal-hint {
  color: #aaa;
}
.dark .setting-label {
  color: #f0f0f0;
}
.dark .radio-custom {
  border-color: #555;
}
.dark .radio-item:hover .radio-custom {
  border-color: #2196f3;
}
.dark .radio-text {
  color: #ccc;
}

.dark .exec-time-text {
  color: #888;
}
</style>
