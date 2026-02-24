<script setup lang="ts">
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import TaskDashboard from './components/TaskDashboard.vue';
import ScriptLibrary from './components/ScriptLibrary.vue';
import SettingsView from './components/SettingsView.vue';
import VisualScriptEditor from './components/VisualScriptEditor.vue';
import ToastNotification from './components/ToastNotification.vue';
import ConfirmDialog from './components/ConfirmDialog.vue';
import { useScriptStore } from './stores/scriptStore';

const store = useScriptStore();

onMounted(async () => {
  invoke('release_main_window');
  await invoke('release_overlay_window');
  await store.init();
});

const navItems = [
  { id: 'tasks', name: '任务', icon: 'account_tree' },
  { id: 'scripts', name: '脚本', icon: 'edit_note' },
  { id: 'logs', name: '日志', icon: 'description' },
  { id: 'settings', name: '设置', icon: 'settings' },
];
</script>

<template>
  <div class="app-container">
    <!-- Sidebar Navigation -->
    <aside class="sidebar">
      <div class="sidebar-header">
        <div class="logo-container">
          <span class="material-symbols-outlined logo-icon">auto_fix_high</span>
        </div>
        <div class="app-info">
          <h1 class="app-title">AutoKB</h1>
          <p class="app-version">版本 v2.4.0</p>
        </div>
      </div>

      <nav class="nav-menu">
        <button v-for="item in navItems" :key="item.id" @click="store.activeTab = item.id as any" :class="[
          'nav-button',
          store.activeTab === item.id ? 'active' : 'inactive'
        ]">
          <span class="material-symbols-outlined">{{ item.icon }}</span>
          <span>{{ item.name }}</span>
        </button>
      </nav>

      <div class="sidebar-footer">
        <button @click="store.currentView = 'visual-editor'" class="new-script-button">
          <span class="material-symbols-outlined">add_circle</span>
          <span>新建脚本</span>
        </button>
      </div>
    </aside>

    <!-- Main Content Area -->
    <main class="main-content">
      <!-- Header -->
      <header class="header">
        <div class="header-left">
          <h2 class="header-title">Desktop Automation</h2>
          <div class="search-wrapper">
            <span class="material-symbols-outlined search-icon">search</span>
            <input class="search-input" placeholder="搜索任务..." type="text" />
          </div>
        </div>
        <div class="header-right">
          <div class="status-indicator">
            <span :class="['status-dot', store.isRecording ? 'recording' : store.isPlaying ? 'playing' : 'idle']"></span>
            <span class="status-text">{{ store.statusMessage }}</span>
          </div>
          <button class="notification-button">
            <span class="material-symbols-outlined">notifications</span>
            <span class="notification-badge"></span>
          </button>
        </div>
      </header>

      <!-- Dashboard Content -->
      <div class="content-area">
        <Transition name="page" mode="out-in">
          <div :key="store.activeTab">
            <TaskDashboard v-if="store.activeTab === 'tasks'" />
            <div v-else-if="store.activeTab === 'scripts'">
              <ScriptLibrary />
            </div>
            <div v-else-if="store.activeTab === 'logs'" class="logs-placeholder">
              运行日志（正在开发中...）
            </div>
            <div v-else-if="store.activeTab === 'settings'">
              <SettingsView />
            </div>
          </div>
        </Transition>
      </div>
    </main>

    <!-- Visual Script Editor Overlay -->
    <Transition name="slide-up">
      <VisualScriptEditor v-if="store.currentView === 'visual-editor'" @close="store.currentView = 'home'" />
    </Transition>

    <ToastNotification />
    <ConfirmDialog />
  </div>
</template>

<style scoped>
/* App Container */
.app-container {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background-color: var(--background);
  font-family: var(--font-display);
}

/* Sidebar */
.sidebar {
  width: 16rem; /* 256px */
  flex-shrink: 0;
  border-right: 1px solid var(--border-main);
  background-color: var(--surface);
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 1.5rem;
  border-bottom: 1px solid var(--border-main);
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.logo-container {
  background-color: var(--primary);
  padding: 0.5rem;
  border-radius: 0.5rem;
  color: white;
}

.logo-icon {
  display: block;
  font-size: 1.5rem;
}

.app-info {
  display: flex;
  flex-direction: column;
}

.app-title {
  font-size: 1.125rem;
  font-weight: 700;
  line-height: 1;
}

.app-version {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin-top: 0.25rem;
}

/* Navigation Menu */
.nav-menu {
  flex: 1;
  padding: 1.5rem 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.nav-button {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.625rem 0.75rem;
  border-radius: 0.5rem;
  transition: color 150ms, background-color 150ms;
  font-weight: 500;
  text-align: left;
  border: none;
  cursor: pointer;
  width: 100%;
  background-color: transparent;
  color: var(--text-muted);
}

.nav-button.active {
  background-color: rgba(19, 91, 236, 0.1);
  color: var(--primary);
}

.nav-button.inactive {
  color: var(--text-muted);
  background-color: transparent;
}

.nav-button.inactive:hover {
  background-color: var(--surface-soft);
}

/* Sidebar Footer */
.sidebar-footer {
  padding: 1rem;
  border-top: 1px solid var(--border-main);
}

.new-script-button {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  background-color: var(--primary);
  color: white;
  font-weight: 600;
  padding: 0.625rem 1rem;
  border-radius: 0.5rem;
  transition: background-color 150ms;
  border: none;
  cursor: pointer;
}

.new-script-button:hover {
  background-color: rgba(19, 91, 236, 0.9);
}

.new-script-button .material-symbols-outlined {
  font-size: 1.25rem;
}

/* Main Content */
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Header */
.header {
  height: 4rem;
  border-bottom: 1px solid var(--border-main);
  background-color: var(--surface);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 2rem;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  flex: 1;
}

.header-title {
  font-size: 1.25rem;
  font-weight: 700;
  letter-spacing: -0.025em;
}

.search-wrapper {
  position: relative;
  width: 100%;
  max-width: 28rem;
}

.search-icon {
  position: absolute;
  left: 0.75rem;
  top: 50%;
  transform: translateY(-50%);
  color: #94a3b8;
  font-size: 1.25rem;
}

.search-input {
  width: 100%;
  padding: 0.5rem 1rem 0.5rem 2.5rem;
  border-radius: 0.5rem;
  border: 1px solid var(--border-main);
  background-color: var(--surface-soft);
  font-size: 0.875rem;
}

.search-input:focus {
  outline: none;
  ring: 2px;
  ring-color: var(--primary);
  border-color: transparent;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 1rem;
}

/* Status Indicator */
.status-indicator {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  background-color: var(--surface-soft);
  padding: 0.375rem 0.75rem;
  border-radius: 9999px;
  border: 1px solid var(--border-main);
}

.status-dot {
  width: 0.5rem;
  height: 0.5rem;
  border-radius: 9999px;
}

.status-dot.recording {
  background-color: var(--error);
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

.status-dot.playing {
  background-color: var(--success);
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

.status-dot.idle {
  background-color: rgba(100, 116, 139, 0.4);
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.status-text {
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--text-muted);
}

/* Notification Button */
.notification-button {
  padding: 0.5rem;
  color: var(--text-muted);
  border-radius: 0.5rem;
  position: relative;
  border: none;
  background: transparent;
  cursor: pointer;
}

.notification-button:hover {
  background-color: var(--surface-soft);
}

.notification-badge {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  width: 0.5rem;
  height: 0.5rem;
  background-color: var(--error);
  border-radius: 9999px;
  border: 2px solid white;
}

:root.dark .notification-badge {
  border-color: #0f172a;
}

/* Content Area */
.content-area {
  flex: 1;
  overflow-y: auto;
  padding: 2rem;
}

/* Logs Placeholder */
.logs-placeholder {
  padding: 1rem;
  text-align: center;
  color: var(--text-muted);
}

/* Transitions */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: transform 0.4s cubic-bezier(0.16, 1, 0.3, 1), opacity 0.4s ease;
}

.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(100%);
  opacity: 0;
}
</style>
