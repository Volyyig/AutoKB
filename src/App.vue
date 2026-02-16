<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ControlBar from './components/ControlBar.vue';
import EventList from './components/EventList.vue';
import ParamEditor from './components/ParamEditor.vue';
import MacroEditor from './components/MacroEditor.vue';
import VisualScriptEditor from './components/VisualScriptEditor.vue';
import ToastNotification from './components/ToastNotification.vue';
import { useScriptStore } from './stores/scriptStore';
import { formatDuration } from './types/script';

const store = useScriptStore();

// Global key handler for shortcuts that should work even when window is focused
async function handleKeydown(e: KeyboardEvent) {
  await store.handleFrontendEvent(e);
}

onMounted(async () => {
  // Show main window after setup is complete (prevents white flash)
  invoke('release_main_window');
  await invoke('release_overlay_window');

  await store.init();
  window.addEventListener('keydown', handleKeydown);
  window.addEventListener('keyup', (e) => store.handleFrontendEvent(e));
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
  window.removeEventListener('keyup', (e) => store.handleFrontendEvent(e)); // Ensure consistent removal
});
</script>

<template>
  <div class="app">
    <!-- Main View -->
    <div class="main-view" v-show="store.currentView === 'home'">
      <header class="header">
        <div class="logo-area">
          <span class="logo-icon">🤖</span>
          <h1 class="title">AutoKB</h1>
        </div>
        <div class="status-bar">
          <span class="status-dot" :class="{
            'recording': store.isRecording,
            'playing': store.isPlaying
          }"></span>
          <span class="status-text">{{ store.statusMessage }}</span>
        </div>
      </header>

      <ControlBar />

      <main class="main-content">
        <section class="panel event-panel">
          <div class="panel-header">
            <h2>Current Script</h2>
            <div class="panel-actions">
              <span class="event-count">{{ store.eventCount }} events</span>
              <span class="duration" v-if="store.totalDuration > 0">
                {{ formatDuration(store.totalDuration) }}
              </span>
              <button class="btn-icon" @click="store.clearScript" title="Clear Script">
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none"
                  stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="3 6 5 6 21 6"></polyline>
                  <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                </svg>
              </button>
            </div>
          </div>
          <div class="panel-body">
            <EventList />
          </div>
        </section>

        <aside class="panel settings-panel">
          <div class="panel-header">
            <h2>Settings</h2>
          </div>
          <ParamEditor />
        </aside>
      </main>

      <footer class="footer">
        <div class="shortcuts">
          <span class="shortcut"><strong>F9</strong> Start/Stop Recording</span>
          <span class="shortcut"><strong>F10</strong> Start/Stop Playback</span>
        </div>
      </footer>
    </div>

    <!-- Macro Editor Overlay -->
    <Transition name="slide-up">
      <MacroEditor v-if="store.currentView === 'macro-editor'" />
    </Transition>

    <!-- Visual Script Editor Overlay -->
    <Transition name="slide-up">
      <VisualScriptEditor v-if="store.currentView === 'visual-editor'" />
    </Transition>

    <!-- Global Toast Notifications -->
    <ToastNotification />
  </div>
</template>

<style>
/* Global Styles */
:root {
  --color-bg-primary: #1e1e2e;
  --color-bg-secondary: #252538;
  --color-bg-tertiary: #2a2a3c;
  --color-text-primary: #e0e0e0;
  --color-text-secondary: #a0a0a0;
  --color-text-disabled: #5a5a5a;
  --color-accent: #6366f1;
  --color-accent-hover: #4f46e5;
  --color-accent-dim: rgba(99, 102, 241, 0.2);
  --color-border: #333344;
  --color-danger: #ef4444;
  --color-success: #10b981;
  --color-warning: #f59e0b;
  --color-hover: rgba(255, 255, 255, 0.05);

  --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);

  font-family: 'Inter', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
}

body {
  margin: 0;
  padding: 0;
  background-color: var(--color-bg-primary);
  color: var(--color-text-primary);
  overflow: hidden;
  user-select: none;
}

/* App Layout */
.app {
  height: 100vh;
  display: flex;
  flex-direction: column;
  position: relative;
}

.main-view {
  display: flex;
  flex-direction: column;
  height: 100%;
}

/* Header */
.header {
  height: 50px;
  padding: 0 20px;
  background-color: var(--color-bg-secondary);
  border-bottom: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.logo-area {
  display: flex;
  align-items: center;
  gap: 10px;
}

.logo-icon {
  font-size: 24px;
}

.title {
  font-size: 18px;
  font-weight: 700;
  margin: 0;
  background: linear-gradient(to right, #6366f1, #a855f7);
  background-clip: text;
  -webkit-text-fill-color: transparent;
}

.status-bar {
  display: flex;
  align-items: center;
  gap: 10px;
  background-color: var(--color-bg-primary);
  padding: 4px 12px;
  border-radius: 12px;
  border: 1px solid var(--color-border);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: var(--color-text-disabled);
  transition: background-color 0.3s;
}

.status-dot.recording {
  background-color: var(--color-danger);
  box-shadow: 0 0 8px var(--color-danger);
}

.status-dot.playing {
  background-color: var(--color-success);
  box-shadow: 0 0 8px var(--color-success);
}

.status-text {
  font-size: 12px;
  font-weight: 500;
  color: var(--color-text-secondary);
}

/* Main Content */
.main-content {
  flex: 1;
  display: grid;
  grid-template-columns: 1fr 300px;
  /* Event List + Settings Panel */
  gap: 0;
  overflow: hidden;
}

.panel {
  display: flex;
  flex-direction: column;
  background-color: var(--color-bg-primary);
}

.event-panel {
  border-right: 1px solid var(--color-border);
}

.settings-panel {
  background-color: var(--color-bg-secondary);
}

.panel-header {
  height: 48px;
  padding: 0 16px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: var(--color-bg-tertiary);
}

.panel-header h2 {
  font-size: 14px;
  font-weight: 600;
  margin: 0;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.panel-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.event-count,
.duration {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-family: monospace;
  background: var(--color-bg-primary);
  padding: 2px 6px;
  border-radius: 4px;
}

.panel-body {
  flex: 1;
  overflow: hidden;
  position: relative;
}

.btn-icon {
  background: none;
  border: none;
  color: var(--color-text-secondary);
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-icon:hover {
  background-color: var(--color-hover);
  color: var(--color-text-primary);
}

/* Footer */
.footer {
  height: 32px;
  background-color: var(--color-bg-tertiary);
  border-top: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 20px;
}

.shortcuts {
  display: flex;
  gap: 24px;
  font-size: 12px;
  color: var(--color-text-secondary);
}

.shortcut strong {
  color: var(--color-text-primary);
  background-color: var(--color-bg-primary);
  padding: 1px 5px;
  border-radius: 3px;
  border: 1px solid var(--color-border);
  font-family: monospace;
  margin-right: 4px;
}

/* Transitions */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(100%);
  opacity: 0;
}
</style>