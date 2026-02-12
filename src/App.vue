<script setup lang="ts">
import { useScriptStore } from './stores/scriptStore';
import ControlBar from './components/ControlBar.vue';
import EventList from './components/EventList.vue';
import ParamEditor from './components/ParamEditor.vue';
import MacroPanel from './components/MacroPanel.vue';

import { onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const store = useScriptStore();

onMounted(async () => {
  // Show main window after setup is complete (prevents white flash)
  invoke('release_main_window');
  await invoke('release_overlay_window');
  // ... existing init
  window.addEventListener('keydown', store.handleFrontendEvent);
  window.addEventListener('keyup', store.handleFrontendEvent);
});

onUnmounted(() => {
  window.removeEventListener('keydown', store.handleFrontendEvent);
  window.removeEventListener('keyup', store.handleFrontendEvent);
});

onMounted(() => {
  store.init();
});
</script>

<template>
  <div class="app">
    <!-- Header -->
    <header class="header">
      <div class="logo">
        <span class="logo-icon">⚡</span>
        <h1>AutoKB</h1>
      </div>
      <div class="status">
        <span class="status-indicator" :class="{
          recording: store.isRecording,
          playing: store.isPlaying,
          'macro-active': store.isMacroActive
        }"></span>
        <span class="status-text">{{ store.statusMessage }}</span>
      </div>
    </header>

    <!-- Control Bar -->
    <ControlBar />

    <!-- Main Content -->
    <main class="main-content">
      <!-- Left Panel: Event List -->
      <section class="panel event-panel">
        <div class="panel-header">
          <h2>📝 事件列表</h2>
          <span class="event-count">{{ store.eventCount }} 个事件</span>
        </div>
        <EventList />
      </section>

      <!-- Right Panel: Parameter Editor & Macros -->
      <aside class="panel settings-panel">
        <div class="panel-header">
          <h2>⚙️ 参数设置</h2>
        </div>
        <ParamEditor />

        <div class="panel-header macro-header">
          <h2>🔗 宏管理</h2>
        </div>
        <MacroPanel />
      </aside>
    </main>

    <!-- Footer -->
    <footer class="footer">
      <div class="hotkey-hints">
        <span class="hint"><kbd>F9</kbd> 录制</span>
        <span class="hint"><kbd>F10</kbd> 回放</span>
        <span class="hint"><kbd>Esc</kbd> 停止</span>
      </div>
    </footer>
  </div>
</template>

<style>
/* ============================================
   Global Styles & Dark Theme
   ============================================ */
:root {
  --bg-primary: #0d1117;
  --bg-secondary: #161b22;
  --bg-tertiary: #21262d;
  --bg-hover: #30363d;

  --text-primary: #e6edf3;
  --text-secondary: #8b949e;
  --text-muted: #6e7681;

  --accent-primary: #58a6ff;
  --accent-success: #3fb950;
  --accent-warning: #d29922;
  --accent-danger: #f85149;
  --accent-purple: #a371f7;

  --border-color: #30363d;
  --border-subtle: #21262d;

  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.3);
  --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.4);
  --shadow-lg: 0 8px 24px rgba(0, 0, 0, 0.5);

  --radius-sm: 6px;
  --radius-md: 8px;
  --radius-lg: 12px;

  --transition-fast: 0.15s ease;
  --transition-normal: 0.25s ease;

  font-family: 'Inter', 'Segoe UI', system-ui, sans-serif;
  font-size: 14px;
  line-height: 1.5;
  color: var(--text-primary);
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  background: var(--bg-primary);
  overflow: hidden;
}

/* Scrollbar */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-tertiary);
}

::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted);
}

/* App Layout */
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-primary);
}

/* Header */
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 20px;
  background: linear-gradient(180deg, var(--bg-secondary) 0%, var(--bg-primary) 100%);
  border-bottom: 1px solid var(--border-color);
}

.logo {
  display: flex;
  align-items: center;
  gap: 10px;
}

.logo-icon {
  font-size: 24px;
  filter: drop-shadow(0 0 8px var(--accent-primary));
}

.logo h1 {
  font-size: 20px;
  font-weight: 600;
  background: linear-gradient(135deg, var(--accent-primary), var(--accent-purple));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
}

.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--text-muted);
  transition: var(--transition-normal);
}

.status-indicator.recording {
  background: var(--accent-danger);
  box-shadow: 0 0 8px var(--accent-danger);
  animation: pulse 1s infinite;
}

.status-indicator.playing {
  background: var(--accent-success);
  box-shadow: 0 0 8px var(--accent-success);
  animation: pulse 1s infinite;
}

.status-indicator.macro-active {
  background: var(--accent-purple);
  box-shadow: 0 0 8px var(--accent-purple);
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

.status-text {
  color: var(--text-secondary);
  font-size: 13px;
}

/* Main Content */
.main-content {
  display: grid;
  grid-template-columns: 1fr 320px;
  gap: 16px;
  padding: 16px;
  flex: 1;
  overflow: hidden;
}

/* Panels */
.panel {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--bg-tertiary);
  border-bottom: 1px solid var(--border-color);
}

.panel-header h2 {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.event-count {
  font-size: 12px;
  color: var(--text-muted);
  padding: 2px 8px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
}

.event-panel {
  overflow: hidden;
}

.settings-panel {
  overflow-y: auto;
}

.macro-header {
  margin-top: 16px;
}

/* Footer */
.footer {
  padding: 10px 20px;
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
}

.hotkey-hints {
  display: flex;
  justify-content: center;
  gap: 24px;
}

.hint {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--text-muted);
  font-size: 12px;
}

kbd {
  padding: 2px 6px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-family: inherit;
  font-size: 11px;
  color: var(--text-secondary);
}
</style>