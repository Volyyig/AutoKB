<template>
  <div class="visual-editor">
    <!-- Top Navigation Bar -->
    <header class="editor-header">
      <div class="header-left">
        <button @click="$emit('close')" class="back-button">
          <span class="material-symbols-outlined">arrow_back</span>
        </button>
        <div class="header-title">
          <span class="material-symbols-outlined title-icon">auto_fix_high</span>
          <h2 class="title-text">AutoFlow <span class="title-subtitle">脚本编辑器</span>
          </h2>
        </div>
      </div>
      <div class="header-actions">
        <div class="action-group">
          <button @click="loadCurrentRecording" :disabled="!store.hasEvents" class="action-button">
            <span class="material-symbols-outlined icon-sm">history</span> 导入录制
          </button>
          <button @click="loadIntoPlayback" :disabled="!currentScript" class="action-button">
            <span class="material-symbols-outlined icon-sm">play_arrow</span> 测试运行
          </button>
        </div>
        <button @click="saveChanges" :disabled="!currentScript" class="save-button">
          <span class="material-symbols-outlined icon-sm">save</span> {{ currentScriptPath ? '保存修改' : '保存为新脚本' }}
        </button>
      </div>
    </header>

    <!-- Main Workspace -->
    <main class="main-workspace">
      <!-- Sidebar: Library & Saved Scripts -->
      <aside class="sidebar">
        <div class="sidebar-header">
          <div class="sidebar-title-row">
            <h3 class="sidebar-title">已存脚本</h3>
            <button @click="addNewScript" class="add-script-button">
              <span class="material-symbols-outlined icon-sm">add</span>
            </button>
          </div>
          <div class="search-container">
            <span class="material-symbols-outlined search-icon">search</span>
            <input class="search-input" placeholder="搜索脚本..." type="text" />
          </div>
        </div>
        <div class="script-list">
          <div v-for="script in store.savedScripts" :key="script.path" @click="loadScriptForEdit(script.path)"
            :class="['script-item', currentScriptPath === script.path ? 'active' : 'inactive']">
            <div class="script-info">
              <span class="material-symbols-outlined script-icon">description</span>
              <span class="script-name">{{ script.name }}</span>
            </div>
            <button @click.stop="deleteScript(script.path)" class="delete-script-button">
              <span class="material-symbols-outlined icon-sm">delete</span>
            </button>
          </div>
          <div v-if="store.savedScripts.length === 0" class="empty-state">
            暂无已保存脚本
          </div>
        </div>
      </aside>

      <!-- Editor Content -->
      <section class="editor-content">
        <div v-if="!currentScript" class="empty-editor">
          <span class="material-symbols-outlined empty-icon">edit_square</span>
          <p>请选择脚本进行编辑或新建</p>
        </div>
        <template v-else>
          <!-- Canvas Toolbar (Simplified) -->
          <div class="canvas-toolbar">
            <div class="toolbar-info">
              <input v-model="currentScript.name" class="script-name-input" placeholder="脚本名称..." />
              <div class="toolbar-divider"></div>
              <span class="node-count">{{ currentScript.events.length }} 个节点</span>
            </div>
            <div class="add-menu-container">
              <button @click="showAddMenu = !showAddMenu" class="add-button">
                <span class="material-symbols-outlined">add_circle</span>
              </button>
              <div v-if="showAddMenu" class="add-menu">
                <button v-for="t in ['Delay', 'KeyPress', 'KeyRelease', 'MousePress', 'MouseRelease', 'MouseMove']"
                  :key="t" @click="addEventTemplate(t)" class="add-menu-item">
                  <span class="material-symbols-outlined menu-item-icon">{{ getIconForType(t) }}</span>
                  {{ formatGroupTitle(t) }}
                </button>
              </div>
            </div>
          </div>

          <!-- Canvas Content: List of Event Groups -->
          <div class="canvas-content canvas-grid">
            <div class="canvas-inner">
              <!-- Start Node -->
              <div class="start-node-container">
                <div class="start-node">
                  <span class="material-symbols-outlined start-node-icon">play_circle</span>
                  <span class="start-node-text">流程开始</span>
                </div>
                <div class="connector"></div>
              </div>

              <!-- Event Groups -->
              <div v-for="(group, index) in groups" :key="index" class="event-group-container">
                <div @click="toggleGroup(index)" :class="['event-group', group.expanded ? 'expanded' : '']">
                  <!-- Group Header -->
                  <div class="group-header">
                    <div class="group-header-left">
                      <span class="material-symbols-outlined group-icon">{{ getIconForType(group.type) }}</span>
                      <div class="group-info">
                        <div class="group-title-row">
                          <span class="group-title">{{ formatGroupTitle(group.type) }}</span>
                          <span class="group-badge">{{ group.events.length }} 步骤</span>
                        </div>
                        <p class="group-description">连续执行 {{ group.events.length }} 个同类动作</p>
                      </div>
                    </div>
                    <div class="group-header-right">
                      <button @click.stop="deleteGroup(index)" class="delete-group-button">
                        <span class="material-symbols-outlined icon-lg">delete</span>
                      </button>
                      <span class="material-symbols-outlined expand-icon" :class="{ 'rotated': group.expanded }">expand_more</span>
                    </div>
                  </div>

                  <!-- Group Details -->
                  <div v-if="group.expanded" @click.stop class="group-details">
                    <div v-for="(event, eIndex) in (group.events as any[])" :key="eIndex" class="event-item">
                      <div class="event-item-left">
                        <span class="event-index">{{ eIndex + 1 }}</span>
                        <!-- Custom settings per event type -->
                        <div class="event-controls">
                          <template v-if="event.event_type === 'Delay'">
                            <input type="number" v-model.number="event.duration_ms" class="event-input event-input-small" min="0">
                            <span class="event-label">ms</span>
                          </template>
                          <template v-else-if="event.event_type === 'KeyPress' || event.event_type === 'KeyRelease'">
                            <button @click="startCapture(index, eIndex)"
                              :class="['capture-button', capturingIndex?.groupIndex === index && capturingIndex?.eventIndex === eIndex ? 'capturing' : '']">
                              {{ capturingIndex?.groupIndex === index && capturingIndex?.eventIndex === eIndex ?
                                '等待输入...' : getKeyDisplay(event.key) }}
                            </button>
                          </template>
                          <template v-else-if="event.event_type.startsWith('Mouse')">
                            <span class="event-label">X:</span>
                            <input type="number" v-model.number="event.x" class="event-input event-input-tiny" />
                            <span class="event-label">Y:</span>
                            <input type="number" v-model.number="event.y" class="event-input event-input-tiny" />
                          </template>
                        </div>
                        <span class="event-description">{{ getEventDescription(event) }}</span>
                      </div>
                      <button @click.stop="deleteEvent(index, eIndex)" class="delete-event-button">
                        <span class="material-symbols-outlined icon-sm">close</span>
                      </button>
                    </div>
                  </div>
                </div>
                <!-- Connector arrow (if not last) -->
                <div v-if="index < groups.length - 1" class="connector-small"></div>
              </div>

              <!-- End Node -->
              <div class="end-node-container">
                <div class="connector-small"></div>
                <div class="end-node">
                  <span class="material-symbols-outlined end-node-icon">logout</span>
                  <span class="end-node-text">结束</span>
                </div>
              </div>
            </div>
          </div>
        </template>

        <!-- Breadcrumbs -->
        <div class="breadcrumbs">
          <span>工作流库</span>
          <span class="material-symbols-outlined breadcrumb-separator">chevron_right</span>
          <span class="breadcrumb-active">{{ currentScript?.name || '新脚本' }}</span>
        </div>
      </section>

      <!-- Right Sidebar: Properties Configuration (ParamEditor integrated) -->
      <aside v-if="currentScript" class="properties-sidebar">
        <div class="properties-header">
          <h3 class="properties-title">全局属性</h3>
          <span class="properties-badge">Script Config</span>
        </div>
        <div class="properties-content">
          <div class="property-section">
            <label class="property-label">脚本描述</label>
            <textarea v-model="currentScript.description" class="property-textarea" placeholder="简述该流程的作用..."></textarea>
          </div>

          <div class="property-section">
            <label class="property-label">执行模式</label>
            <div class="execution-mode-section">
              <div class="slider-container">
                <div class="slider-header">
                  <span class="slider-label">执行速度</span>
                  <span class="slider-value">{{ currentScript.speed_multiplier }}x</span>
                </div>
                <input type="range" v-model.number="currentScript.speed_multiplier" min="0.1" max="5.0" step="0.1"
                  class="slider-input" />
              </div>
              <div class="number-input-row">
                <span class="number-input-label">循环次数 (0=无限)</span>
                <input type="number" v-model.number="currentScript.loop_config.count" class="number-input" />
              </div>
              <div class="number-input-row">
                <span class="number-input-label">循环间隔 (ms)</span>
                <input type="number" v-model.number="currentScript.loop_config.delay_between_ms" class="number-input" />
              </div>
            </div>
          </div>

          <div class="info-box">
            <div class="info-box-content">
              <span class="material-symbols-outlined info-icon">lightbulb</span>
              <div class="info-text">
                编辑器会自动将连续的同类动作（如多个按键）折叠为组，以保持流程整洁度。
              </div>
            </div>
          </div>
        </div>
        <div class="properties-footer">
          <button @click="clearEvents" class="clear-button">
            清空所有节点
          </button>
        </div>
      </aside>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { useScriptStore } from '../stores/scriptStore';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import type { Script } from '../types/script';
import { getEventDescription, getKeyDisplay } from '../types/script';
import { groupEvents, formatGroupTitle, type EventGroup } from '../utils/eventGrouping';
import { useConfirmDialog } from '../composables/useConfirmDialog';

const store = useScriptStore();
const { confirm } = useConfirmDialog();
const currentScriptPath = ref<string | null>(null);
const currentScript = ref<Script | null>(null);
const groups = ref<EventGroup[]>([]);
const showAddMenu = ref(false);
const capturingIndex = ref<{ groupIndex: number, eventIndex: number } | null>(null);

defineEmits(['close']);

// Watch currentScript and build `groups` ref
watch(() => currentScript.value, (newScript) => {
  if (!newScript) {
    groups.value = [];
    return;
  }
  const expandedStates = groups.value.map(g => g.expanded);
  const newGroups = groupEvents(newScript.events);
  newGroups.forEach((group, i) => {
    if (expandedStates[i] !== undefined) {
      group.expanded = expandedStates[i];
    } else {
      group.expanded = true; // Default to expanded for visibility
    }
  });
  groups.value = newGroups;
}, { deep: true, immediate: true });

function toggleGroup(index: number) {
  if (groups.value[index]) {
    groups.value[index].expanded = !groups.value[index].expanded;
  }
}

function getIconForType(type: string) {
  const map: Record<string, string> = {
    'Delay': 'timer',
    'KeyPress': 'keyboard',
    'KeyRelease': 'keyboard',
    'MousePress': 'mouse',
    'MouseRelease': 'mouse',
    'MouseMove': 'near_me',
    'MouseScroll': 'expand',
  };
  return map[type] || 'settings';
}

// --- Editing Functions ---

async function deleteScript(path: string) {
  const confirmed = await confirm({
    title: '删除脚本',
    message: '确定要彻底删除此脚本吗？此操作无法撤销。',
    confirmText: '删除',
    cancelText: '取消'
  });
  
  if (!confirmed) return;
  
  try {
    await invoke('delete_script', { path });
    if (currentScriptPath.value === path) {
      currentScript.value = null;
      currentScriptPath.value = null;
    }
    await store.listSavedScripts();
    store.showNotification('删除成功。', 'success');
  } catch (e) {
    store.showNotification('删除失败: ' + e, 'error');
  }
}

function loadCurrentRecording() {
  currentScript.value = JSON.parse(JSON.stringify(store.currentScript));
  currentScriptPath.value = null;
  store.showNotification('已加载当前录制。', 'info');
}

function addNewScript() {
  currentScript.value = store.createNewDraftScript();
  currentScriptPath.value = null;
  store.showNotification('已创建新脚本载入。', 'info');
}

function addEventTemplate(type: string) {
  if (!currentScript.value) return;
  let event: any = { event_type: type };
  switch (type) {
    case 'Delay': event.duration_ms = 100; break;
    case 'KeyPress':
    case 'KeyRelease': event.key = { type: 'Char', value: 'a' }; break;
    case 'MousePress':
    case 'MouseRelease':
      event.button = 'left'; event.x = 0; event.y = 0; break;
    case 'MouseMove': event.x = 0; event.y = 0; break;
    case 'MouseScroll': event.delta_x = 0; event.delta_y = 100; break;
  }
  currentScript.value.events.push(event);
  showAddMenu.value = false;
}

function startCapture(groupIndex: number, eventIndex: number) {
  capturingIndex.value = { groupIndex, eventIndex };
  window.addEventListener('keydown', onCaptureKeyDown, { once: true });
}

function onCaptureKeyDown(e: KeyboardEvent) {
  if (!capturingIndex.value || !currentScript.value) return;
  e.preventDefault();
  e.stopPropagation();
  const { groupIndex, eventIndex } = capturingIndex.value;
  const group = groups.value[groupIndex];
  if (group) {
    const realIndex = group.startIndex + eventIndex;
    const event = currentScript.value.events[realIndex];
    if (event && (event.event_type === 'KeyPress' || event.event_type === 'KeyRelease')) {
      if (e.key.length === 1) {
        event.key = { type: 'Char', value: e.key };
      } else {
        let keyName = e.key;
        const keyMap: Record<string, string> = {
          ' ': 'Space', 'Control': 'ControlLeft', 'Shift': 'ShiftLeft', 'Alt': 'Alt',
          'Meta': 'MetaLeft', 'ArrowUp': 'UpArrow', 'ArrowDown': 'DownArrow',
          'ArrowLeft': 'LeftArrow', 'ArrowRight': 'RightArrow', 'Enter': 'Return',
          'Backspace': 'Backspace', 'Escape': 'Escape', 'Tab': 'Tab'
        };
        if (keyMap[keyName]) keyName = keyMap[keyName];
        event.key = { type: 'Special', value: keyName };
      }
    }
  }
  capturingIndex.value = null;
}

function deleteEvent(groupIndex: number, eventIndex: number) {
  if (!currentScript.value) return;
  const group = groups.value[groupIndex];
  currentScript.value.events.splice(group.startIndex + eventIndex, 1);
}

async function deleteGroup(groupIndex: number) {
  if (!currentScript.value) return;
  
  const confirmed = await confirm({
    title: '删除事件组',
    message: '确定要删除整个事件组吗？此操作无法撤销。',
    confirmText: '删除',
    cancelText: '取消'
  });
  
  if (!confirmed) return;
  
  const group = groups.value[groupIndex];
  currentScript.value.events.splice(group.startIndex, group.events.length);
}

async function clearEvents() {
  if (currentScript.value) {
    const confirmed = await confirm({
      title: '清空所有节点',
      message: '确定要清空所有节点吗？此操作无法撤销。',
      confirmText: '清空',
      cancelText: '取消'
    });
    
    if (confirmed) {
      currentScript.value.events = [];
    }
  }
}

async function refreshScripts() {
  await store.listSavedScripts();
}

async function loadScriptForEdit(path: string) {
  try {
    const script = await invoke<Script>('load_script', { path });
    currentScript.value = script;
    currentScriptPath.value = path;
  } catch (e) {
    console.error("Failed to load script", e);
  }
}

async function saveChanges() {
  if (!currentScript.value) return;
  try {
    let path = currentScriptPath.value;
    if (!path) {
      const defaultDir = await invoke<string>('get_scripts_dir');
      const savePath = await save({
        defaultPath: `${defaultDir}/${currentScript.value.name}.autokb`,
        filters: [{ name: 'AutoKB Script', extensions: ['autokb'] }],
      });
      if (!savePath) return;
      path = savePath;
      currentScriptPath.value = path;
    }
    await invoke('save_script', { script: currentScript.value, path });
    store.showNotification('保存成功！', 'success');
    await store.listSavedScripts();
  } catch (e) {
    store.showNotification('保存失败: ' + e, 'error');
  }
}

async function loadIntoPlayback() {
  if (!currentScript.value) return;
  store.currentScript = JSON.parse(JSON.stringify(currentScript.value));
  store.statusMessage = `已载入 ${currentScript.value.name}`;
  store.showNotification(`已载入脚本。`, 'info');
}

onMounted(() => refreshScripts());
</script>

<style scoped>
/* Editor Container */
.visual-editor {
  position: fixed;
  inset: 0;
  z-index: 1000;
  background-color: var(--background);
  display: flex;
  flex-direction: column;
  font-family: var(--font-display);
  -webkit-font-smoothing: antialiased;
  overflow: hidden;
  height: 100vh;
}

/* Header */
.editor-header {
  display: flex;
  height: 3.5rem;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border-main);
  background-color: var(--surface);
  padding: 0 1rem;
  flex-shrink: 0;
  z-index: 50;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.back-button {
  padding: 0.5rem;
  border-radius: 0.5rem;
  color: var(--text-muted);
  transition: background-color 150ms;
}

.back-button:hover {
  background-color: var(--surface-soft);
}

.header-title {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.title-icon {
  color: var(--primary);
  font-size: 1.875rem;
}

.title-text {
  font-size: 1.125rem;
  font-weight: 700;
  letter-spacing: -0.025em;
}

.title-subtitle {
  color: var(--text-muted);
  font-weight: 400;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.action-group {
  display: flex;
  background-color: var(--surface-soft);
  padding: 0.25rem;
  border-radius: 0.5rem;
  margin-right: 0.5rem;
  border: 1px solid var(--border-main);
  gap: 1rem
}

.action-button {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.375rem 0.75rem;
  border-radius: 0.375rem;
  font-size: 0.875rem;
  font-weight: 500;
  transition: all 150ms;
}

.action-button:hover {
  background-color: var(--surface);
}

.action-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.save-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background-color: var(--primary);
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  font-weight: 700;
  transition: background-color 150ms;
  box-shadow: 0 1px 2px 0 rgba(19, 91, 236, 0.2);
}

.save-button:hover {
  background-color: rgba(19, 91, 236, 0.9);
}

.save-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Main Workspace */
.main-workspace {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* Sidebar */
.sidebar {
  width: 16rem;
  border-right: 1px solid var(--border-main);
  background-color: var(--surface);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.sidebar-header {
  padding: 1rem;
  border-bottom: 1px solid var(--border-main);
}

.sidebar-title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
}

.sidebar-title {
  font-size: 0.75rem;
  font-weight: 700;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.add-script-button {
  padding: 0.25rem;
  border-radius: 0.25rem;
  color: var(--primary);
}

.add-script-button:hover {
  background-color: var(--surface-soft);
}

.search-container {
  position: relative;
}

.search-icon {
  position: absolute;
  left: 0.75rem;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  font-size: 1.125rem;
}

.search-input {
  width: 100%;
  padding-left: 2.5rem;
  padding-right: 1rem;
  padding-top: 0.5rem;
  padding-bottom: 0.5rem;
  background-color: var(--surface-soft);
  border: 1px solid var(--border-main);
  border-radius: 0.5rem;
  font-size: 0.875rem;
}

.search-input:focus {
  outline: 2px solid rgba(19, 91, 236, 0.2);
  outline-offset: 0;
}

.script-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.script-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.625rem;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: all 150ms;
}

.script-item:hover {
  background-color: var(--surface-soft);
}

.script-item.active {
  background-color: rgba(19, 91, 236, 0.05);
  color: var(--primary);
}

.script-item.inactive {
  color: var(--text-muted);
}

.script-info {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  overflow: hidden;
}

.script-icon {
  font-size: 1.125rem;
  opacity: 0.6;
}

.script-name {
  font-size: 0.875rem;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.delete-script-button {
  padding: 0.25rem;
  opacity: 0;
  border-radius: 0.25rem;
  color: var(--error);
  transition: all 150ms;
}

.script-item:hover .delete-script-button {
  opacity: 1;
}

.delete-script-button:hover {
  background-color: var(--error-bg);
}

.empty-state {
  padding: 1rem;
  text-align: center;
  font-size: 0.75rem;
  color: var(--text-muted);
}

/* Editor Content */
.editor-content {
  flex: 1;
  background-color: var(--background);
  position: relative;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.empty-editor {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  opacity: 0.6;
}

.empty-icon {
  font-size: 3.75rem;
  margin-bottom: 1rem;
}

/* Canvas Toolbar */
.canvas-toolbar {
  position: absolute;
  top: 1rem;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 0.75rem;
  background-color: var(--surface);
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  border: 1px solid var(--border-main);
  border-radius: 9999px;
  padding: 0.5rem 1rem;
  z-index: 20;
}

.toolbar-info {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.script-name-input {
  background-color: transparent;
  border: none;
  padding: 0;
  font-size: 0.875rem;
  font-weight: 700;
  width: 8rem;
  color: var(--text-main);
}

.script-name-input:focus {
  outline: none;
}

.toolbar-divider {
  width: 1px;
  height: 1rem;
  background-color: var(--border-main);
  margin: 0 0.25rem;
}

.node-count {
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--text-muted);
}

.add-menu-container {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  position: relative;
}

.add-button {
  padding: 0.375rem;
  border-radius: 9999px;
  color: var(--primary);
  transition: background-color 150ms;
}

.add-button:hover {
  background-color: var(--surface-soft);
}

.add-menu {
  position: absolute;
  top: 100%;
  margin-top: 0.5rem;
  left: 0;
  width: 12rem;
  background-color: var(--surface);
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  border: 1px solid var(--border-main);
  border-radius: 0.75rem;
  padding: 0.5rem;
  z-index: 100;
}

.add-menu-item {
  width: 100%;
  text-align: left;
  padding: 0.5rem;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.add-menu-item:hover {
  background-color: var(--surface-soft);
}

.menu-item-icon {
  font-size: 0.875rem;
  opacity: 0.6;
}

/* Canvas Content */
.canvas-content {
  flex: 1;
  overflow-y: auto;
  padding: 5rem 1.25rem 4rem;
  scroll-behavior: smooth;
}

.canvas-grid {
  background-size: 20px 20px;
  background-image: radial-gradient(circle, #e2e8f0 1px, transparent 1px);
}

.dark .canvas-grid {
  background-image: radial-gradient(circle, var(--border-main) 1px, transparent 1px);
}

.canvas-inner {
  max-width: 36rem;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
}

/* Flow Nodes */
.start-node-container {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.start-node {
  width: 10rem;
  height: 3.5rem;
  background-color: var(--surface);
  border: 2px solid var(--primary);
  border-radius: 0.75rem;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
}

.start-node-icon {
  color: var(--primary);
  margin-right: 0.5rem;
}

.start-node-text {
  font-weight: 700;
  font-size: 0.875rem;
  color: var(--text-main);
}

.connector {
  width: 2px;
  height: 2rem;
  background-color: var(--border-main);
  margin-top: 0.5rem;
}

.connector-small {
  width: 2px;
  height: 2rem;
  background-color: var(--border-main);
  margin: 0.25rem 0;
}

/* Event Groups */
.event-group-container {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.event-group {
  width: 100%;
  background-color: var(--surface);
  border: 1px solid var(--border-main);
  border-radius: 0.75rem;
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  transition: all 150ms;
  cursor: pointer;
  overflow: hidden;
}

.event-group:hover {
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.event-group.expanded {
  box-shadow: 0 0 0 2px rgba(19, 91, 236, 0.2);
  border-color: rgba(19, 91, 236, 0.5);
}

.group-header {
  padding: 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: var(--surface-soft);
}

.group-header-left {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.group-icon {
  color: var(--primary);
  padding: 0.375rem;
  background-color: rgba(19, 91, 236, 0.1);
  border-radius: 0.5rem;
  transition: transform 150ms;
}

.event-group:hover .group-icon {
  transform: scale(1.1);
}

.group-info {
  display: flex;
  flex-direction: column;
}

.group-title-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.group-title {
  font-size: 0.875rem;
  font-weight: 700;
  color: var(--text-main);
}

.group-badge {
  font-size: 0.625rem;
  padding: 0.125rem 0.375rem;
  background-color: var(--surface-soft);
  border: 1px solid var(--border-main);
  border-radius: 9999px;
  font-weight: 700;
  text-transform: uppercase;
  color: var(--text-muted);
}

.group-description {
  font-size: 0.625rem;
  color: var(--text-muted);
  margin-top: 0.125rem;
}

.group-header-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.delete-group-button {
  padding: 0.375rem;
  border-radius: 0.5rem;
  color: var(--error);
  opacity: 0;
  transition: all 150ms;
}

.event-group:hover .delete-group-button {
  opacity: 1;
}

.delete-group-button:hover {
  background-color: var(--error-bg);
}

.expand-icon {
  color: var(--text-muted);
  transition: transform 150ms;
}

.expand-icon.rotated {
  transform: rotate(180deg);
}

/* Group Details */
.group-details {
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  border-top: 1px solid var(--border-main);
}

.event-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.5rem;
  background-color: var(--background);
  border-radius: 0.5rem;
  border: 1px solid transparent;
  transition: border-color 150ms;
}

.event-item:hover {
  border-color: var(--border-main);
}

.event-item-left {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  overflow: hidden;
  flex: 1;
}

.event-index {
  font-size: 0.75rem;
  color: var(--text-muted);
  font-family: monospace;
  width: 1rem;
  flex-shrink: 0;
}

.event-controls {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.event-input {
  padding: 0.25rem 0.5rem;
  font-size: 0.75rem;
  background-color: var(--surface);
  color: var(--text-main);
  border: 1px solid var(--border-main);
  border-radius: 0.25rem;
  outline: none;
}

.event-input:focus {
  outline: 1px solid var(--primary);
  outline-offset: 0;
}

.event-input-small {
  width: 5rem;
}

.event-input-tiny {
  width: 3.5rem;
  padding: 0.125rem 0.25rem;
}

.event-label {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.capture-button {
  padding: 0.25rem 0.75rem;
  font-size: 0.75rem;
  font-family: monospace;
  border-radius: 0.25rem;
  border: 1px dashed var(--border-main);
  background-color: var(--surface);
  color: var(--text-main);
  transition: all 150ms;
}

.capture-button.capturing {
  background-color: rgba(19, 91, 236, 0.1);
  border-color: var(--primary);
  color: var(--primary);
}

.event-description {
  font-size: 0.625rem;
  color: var(--text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  opacity: 0.4;
}

@media (max-width: 768px) {
  .event-description {
    display: none;
  }
}

.delete-event-button {
  padding: 0.25rem;
  transition: color 150ms;
  flex-shrink: 0;
}

.delete-event-button:hover {
  color: var(--error);
}

/* End Node */
.end-node-container {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.end-node {
  width: 8rem;
  height: 2.5rem;
  background-color: var(--surface-soft);
  border: 2px solid var(--border-main);
  border-radius: 0.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
}

.end-node-icon {
  font-size: 0.875rem;
  margin-right: 0.5rem;
}

.end-node-text {
  font-weight: 700;
  font-size: 0.75rem;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  color: var(--text-muted);
}

/* Breadcrumbs */
.breadcrumbs {
  height: 2.5rem;
  border-top: 1px solid var(--border-main);
  background-color: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(12px);
  padding: 0 1.5rem;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  font-size: 0.625rem;
  font-weight: 700;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.1em;
  z-index: 20;
}

.dark .breadcrumbs {
  background-color: rgba(30, 41, 59, 0.8);
}

.breadcrumb-separator {
  font-size: 0.625rem;
}

.breadcrumb-active {
  color: var(--primary);
}

/* Right Sidebar */
.properties-sidebar {
  width: 18rem;
  border-left: 1px solid var(--border-main);
  background-color: var(--surface);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

.properties-header {
  padding: 1rem;
  border-bottom: 1px solid var(--border-main);
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
}

.properties-title {
  font-weight: 700;
  font-size: 0.875rem;
  color: var(--text-main);
}

.properties-badge {
  font-size: 0.625rem;
  background-color: rgba(19, 91, 236, 0.1);
  color: var(--primary);
  padding: 0.125rem 0.5rem;
  border-radius: 9999px;
  font-weight: 700;
  text-transform: uppercase;
}

.properties-content {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.property-section {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.property-label {
  font-size: 0.625rem;
  font-weight: 900;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.property-textarea {
  /* width: 100%; */
  resize: none;
  font-size: 0.75rem;
  background-color: var(--surface-soft);
  color: var(--text-main);
  border: 1px solid var(--border-main);
  border-radius: 0.5rem;
  padding: 0.75rem;
  height: 6rem;
  outline: none;
  transition: all 150ms;
}

.property-textarea:focus {
  outline: 1px solid var(--primary);
  outline-offset: 0;
}

.execution-mode-section {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.slider-container {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.slider-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 0.75rem;
}

.slider-label {
  color: var(--text-muted);
}

.slider-value {
  font-family: monospace;
  font-weight: 700;
  color: var(--primary);
}

.slider-input {
  width: 100%;
  height: 0.375rem;
  background-color: var(--border-main);
  border-radius: 0.5rem;
  appearance: none;
  cursor: pointer;
  accent-color: var(--primary);
}

.slider-input::-webkit-slider-thumb {
  appearance: none;
  width: 14px;
  height: 14px;
  background: white;
  border: 2px solid var(--primary);
  border-radius: 50%;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.number-input-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.number-input-label {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.number-input {
  width: 5rem;
  padding: 0.375rem 0.5rem;
  font-size: 0.75rem;
  background-color: var(--surface-soft);
  color: var(--text-main);
  border: 1px solid var(--border-main);
  border-radius: 0.5rem;
  outline: none;
}

.number-input:focus {
  outline: 1px solid var(--primary);
  outline-offset: 0;
}

.info-box {
  padding: 1rem;
  background-color: var(--warning-bg);
  border-radius: 0.75rem;
  border: 1px solid var(--warning-border);
}

.info-box-content {
  display: flex;
  gap: 0.5rem;
  align-items: flex-start;
}

.info-icon {
  color: var(--warning);
  font-size: 1.125rem;
}

.info-text {
  font-size: 0.625rem;
  color: var(--warning);
  line-height: 1.6;
  font-weight: 500;
}

.properties-footer {
  padding: 1rem;
  border-top: 1px solid var(--border-main);
  background-color: var(--surface-soft);
}

.clear-button {
  width: 100%;
  padding: 0.625rem;
  background-color: var(--surface);
  color: var(--text-muted);
  border-radius: 0.5rem;
  font-size: 0.625rem;
  font-weight: 900;
  transition: all 150ms;
  text-transform: uppercase;
  letter-spacing: 0.1em;
  border: 1px solid var(--border-main);
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
}

.clear-button:hover {
  background-color: var(--error);
  color: white;
}

/* Icon size utilities */
.icon-sm {
  font-size: 0.875rem; /* 14px */
}

.icon-lg {
  font-size: 1.125rem; /* 18px */
}
</style>
