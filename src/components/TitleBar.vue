<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

function minimizeWindow() {
  appWindow.minimize();
}

function closeWindow() {
  appWindow.close();
}
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="titlebar-title" data-tauri-drag-region>tauri2-noframe</div>
    <div class="titlebar-buttons">
      <button class="titlebar-btn minimize-btn" @click="minimizeWindow">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <rect fill="currentColor" width="10" height="1" x="1" y="6"></rect>
        </svg>
      </button>
      <button class="titlebar-btn close-btn" @click="closeWindow">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <path
            fill="currentColor"
            d="M6.707 6l3.647-3.646a.5.5 0 0 0-.708-.708L6 5.293 2.354 1.646a.5.5 0 1 0-.708.708L5.293 6l-3.647 3.646a.5.5 0 0 0 .708.708L6 6.707l3.646 3.647a.5.5 0 0 0 .708-.708L6.707 6z"
          ></path>
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

.titlebar-title {
  padding-left: 12px;
  font-size: 13px;
  font-weight: 500;
  color: #0f0f0f;
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
  color: #0f0f0f;
  padding: 0;
  box-shadow: none;
  border-radius: 0;
  transition: background-color 0.15s;
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

@media (prefers-color-scheme: dark) {
  .titlebar {
    background-color: #2f2f2f;
  }

  .titlebar-title {
    color: #f6f6f6;
  }

  .titlebar-btn {
    color: #f6f6f6;
  }

  .titlebar-btn:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }

  .titlebar-btn:active {
    background-color: rgba(255, 255, 255, 0.15);
  }
}
</style>
