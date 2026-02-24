<script setup lang="ts">
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import TaskDashboard from './components/TaskDashboard.vue';
import ScriptLibrary from './components/ScriptLibrary.vue';
import VisualScriptEditor from './components/VisualScriptEditor.vue';
import ToastNotification from './components/ToastNotification.vue';
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
  <div class="flex h-screen overflow-hidden bg-background-light dark:bg-background-dark font-display">
    <!-- Sidebar Navigation -->
    <aside
      class="w-64 flex-shrink-0 border-r border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 flex flex-col">
      <div class="p-6 border-b border-slate-200 dark:border-slate-800 flex items-center gap-3">
        <div class="bg-primary p-2 rounded-lg text-white">
          <span class="material-symbols-outlined block text-2xl">auto_fix_high</span>
        </div>
        <div>
          <h1 class="text-lg font-bold leading-none">AutoKB</h1>
          <p class="text-xs text-slate-500 dark:text-slate-400 mt-1">版本 v2.4.0</p>
        </div>
      </div>

      <nav class="flex-1 px-4 py-6 flex flex-col gap-2">
        <button v-for="item in navItems" :key="item.id" @click="store.activeTab = item.id as any" :class="[
          'flex items-center gap-3 px-3 py-2.5 rounded-lg transition-colors font-medium text-left',
          store.activeTab === item.id
            ? 'bg-primary/10 text-primary'
            : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800'
        ]">
          <span class="material-symbols-outlined">{{ item.icon }}</span>
          <span>{{ item.name }}</span>
        </button>
      </nav>

      <div class="p-4 border-t border-slate-200 dark:border-slate-800">
        <button @click="store.currentView = 'visual-editor'"
          class="w-full flex items-center justify-center gap-2 bg-primary hover:bg-primary/90 text-white font-semibold py-2.5 px-4 rounded-lg transition-colors">
          <span class="material-symbols-outlined text-xl">add_circle</span>
          <span>新建脚本</span>
        </button>
      </div>
    </aside>

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col overflow-hidden">
      <!-- Header -->
      <header
        class="h-16 border-b border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 flex items-center justify-between px-8">
        <div class="flex items-center gap-6 flex-1">
          <h2 class="text-xl font-bold tracking-tight">Desktop Automation</h2>
          <div class="relative w-full max-w-md">
            <span
              class="material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 text-xl">search</span>
            <input
              class="w-full pl-10 pr-4 py-2 rounded-lg border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800 focus:ring-2 focus:ring-primary focus:border-transparent text-sm"
              placeholder="搜索任务..." type="text" />
          </div>
        </div>
        <div class="flex items-center gap-4">
          <div
            class="flex items-center gap-3 bg-slate-100 dark:bg-slate-800 px-3 py-1.5 rounded-full border border-slate-200 dark:border-slate-700">
            <span
              :class="['w-2 h-2 rounded-full', store.isRecording ? 'bg-red-500 animate-pulse' : store.isPlaying ? 'bg-green-500 animate-pulse' : 'bg-slate-400']"></span>
            <span class="text-xs font-medium text-slate-600 dark:text-slate-400">{{ store.statusMessage }}</span>
          </div>
          <button class="p-2 text-slate-500 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg relative">
            <span class="material-symbols-outlined">notifications</span>
            <span
              class="absolute top-2 right-2 w-2 h-2 bg-red-500 rounded-full border-2 border-white dark:border-slate-900"></span>
          </button>
        </div>
      </header>

      <!-- Dashboard Content -->
      <div class="flex-1 overflow-y-auto p-8">
        <Transition name="page" mode="out-in">
          <div :key="store.activeTab">
            <TaskDashboard v-if="store.activeTab === 'tasks'" />
            <div v-else-if="store.activeTab === 'scripts'">
              <ScriptLibrary />
            </div>
            <div v-else-if="store.activeTab === 'logs'" class="p-4 text-center text-slate-500">
              运行日志（正在开发中...）
            </div>
            <div v-else-if="store.activeTab === 'settings'" class="p-4 text-center text-slate-500">
              设置页面（正在开发中...）
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
  </div>
</template>

<style>
/* Transitions are defined in index.css */
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
