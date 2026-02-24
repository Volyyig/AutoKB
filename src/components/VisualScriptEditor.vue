<template>
    <div class="visual-editor">
        <!-- Header -->
        <div class="editor-header">
            <button class="btn-back" @click="store.currentView = 'home'">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
                    stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="19" y1="12" x2="5" y2="12"></line>
                    <polyline points="12 19 5 12 12 5"></polyline>
                </svg>
                返回
            </button>
            <h2>脚本编辑器</h2>
            <div class="header-actions">
                <!-- Placeholder for potentially other actions -->
            </div>
        </div>

        <div class="editor-body">
            <!-- Left Sidebar: Script List -->
            <div class="sidebar">
                <div class="sidebar-header">
                    <h3>脚本列表</h3>
                    <div class="sidebar-actions">
                        <button class="btn-icon small" @click="loadCurrentRecording" title="编辑当前录制"
                            :disabled="!store.hasEvents">
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                                fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                stroke-linejoin="round">
                                <circle cx="12" cy="12" r="10"></circle>
                                <path d="M12 8v8"></path>
                                <path d="M8 12h8"></path>
                            </svg>
                        </button>
                        <button class="btn-icon small" @click="addNewScript" title="新建脚本">
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                                fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                stroke-linejoin="round">
                                <line x1="12" y1="5" x2="12" y2="19"></line>
                                <line x1="5" y1="12" x2="19" y2="12"></line>
                            </svg>
                        </button>
                        <button class="btn-icon small" @click="refreshScripts" title="刷新列表">
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                                fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                stroke-linejoin="round">
                                <path d="M23 4v6h-6"></path>
                                <path d="M1 20v-6h6"></path>
                                <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
                            </svg>
                        </button>
                    </div>
                </div>
                <div class="script-list">
                    <div v-if="store.savedScripts.length === 0" class="empty-list">
                        未找到脚本。
                    </div>
                    <div v-for="script in store.savedScripts" :key="script.path" class="script-item"
                        :class="{ active: currentScriptPath === script.path }" @click="loadScriptForEdit(script.path)">
                        <span class="script-name">{{ script.name }}</span>
                        <button class="btn-delete-script" @click.stop="deleteScript(script.path)"
                            title="删除脚本">🗑️</button>
                    </div>
                </div>
            </div>

            <!-- Right Content: Editor -->
            <div class="main-editor">
                <div v-if="!currentScript" class="empty-state">
                    请从列表中选择要编辑的脚本。
                </div>
                <template v-else>
                    <div class="toolbar">
                        <div class="script-info">
                            <input v-model="currentScript.name" class="script-name-input" placeholder="脚本名称" />
                            <span class="event-count">{{ currentScript.events.length }} 个事件</span>
                        </div>
                        <div class="toolbar-actions">
                            <div class="add-event-dropdown">
                                <button class="btn btn-secondary" @click="showAddMenu = !showAddMenu">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                                        fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round">
                                        <line x1="12" y1="5" x2="12" y2="19"></line>
                                        <line x1="5" y1="12" x2="19" y2="12"></line>
                                    </svg>
                                    添加事件
                                </button>
                                <div v-if="showAddMenu" class="dropdown-menu">
                                    <button @click="addEventTemplate('Delay')">⌛ 等待时间</button>
                                    <button @click="addEventTemplate('KeyPress')">⌨️ 键盘按下</button>
                                    <button @click="addEventTemplate('KeyRelease')">⌨️ 键盘弹起</button>
                                    <button @click="addEventTemplate('MousePress')">🖱️ 鼠标按下</button>
                                    <button @click="addEventTemplate('MouseRelease')">🖱️ 鼠标弹起</button>
                                    <button @click="addEventTemplate('MouseMove')">🎯 鼠标移动</button>
                                    <button @click="addEventTemplate('MouseScroll')">🎡 鼠标滚动</button>
                                </div>
                            </div>
                            <button class="btn btn-secondary" @click="saveChanges">
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                                    fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                    stroke-linejoin="round">
                                    <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
                                    <polyline points="17 21 17 13 7 13 7 21"></polyline>
                                    <polyline points="7 3 7 8 15 8"></polyline>
                                </svg>
                                保存
                            </button>
                            <button class="btn btn-primary" @click="loadIntoPlayback">
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                                    fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                    stroke-linejoin="round">
                                    <path d="M5 12h14"></path>
                                    <path d="M12 5l7 7-7 7"></path>
                                </svg>
                                载入
                            </button>
                        </div>
                    </div>

                    <div class="events-container">
                        <div v-if="groups.length === 0" class="no-events">
                            脚本中没有事件。
                        </div>
                        <div v-else class="event-groups">
                            <div v-for="(group, index) in groups" :key="index" class="event-group"
                                :class="{ expanded: group.expanded }">
                                <div class="group-header" @click="toggleGroup(index)">
                                    <div class="group-icon">
                                        <span v-if="group.type === 'KeyPress' || group.type === 'KeyRelease'">⌨️</span>
                                        <span v-else-if="group.type.startsWith('Mouse')">🖱️</span>
                                        <span v-else>❓</span>
                                    </div>
                                    <div class="group-title">
                                        {{ formatGroupTitle(group.type) }}
                                        <span class="badge">{{ group.events.length }}</span>
                                    </div>
                                    <div class="group-actions" @click.stop>
                                        <button class="btn-icon small danger" @click="deleteGroup(index)" title="删除组">
                                            🗑️
                                        </button>
                                    </div>
                                    <div class="group-toggle">
                                        {{ group.expanded ? '▼' : '▶' }}
                                    </div>
                                </div>
                                <div v-show="group.expanded" class="group-body">
                                    <div v-for="(event, eIndex) in group.events" :key="eIndex" class="event-item">
                                        <div class="event-info">
                                            <span class="event-desc">{{ getEventDescription(event) }}</span>
                                        </div>
                                        <div class="event-controls">
                                            <!-- Delay -->
                                            <template v-if="event.event_type === 'Delay'">
                                                <input type="number" v-model.number="event.duration_ms"
                                                    class="small-input" min="0">
                                                <span class="unit">ms</span>
                                            </template>

                                            <!-- Key Events -->
                                            <template
                                                v-if="event.event_type === 'KeyPress' || event.event_type === 'KeyRelease'">
                                                <button class="btn-capture"
                                                    :class="{ capturing: capturingIndex?.groupIndex === index && capturingIndex?.eventIndex === eIndex }"
                                                    @click="startCapture(index, eIndex)">
                                                    {{ capturingIndex?.groupIndex === index &&
                                                        capturingIndex?.eventIndex === eIndex ? '等待按键...' :
                                                        getKeyDisplay(event.key) }}
                                                </button>
                                            </template>

                                            <!-- Mouse Press/Release -->
                                            <template
                                                v-if="event.event_type === 'MousePress' || event.event_type === 'MouseRelease'">
                                                <select v-model="event.button" class="small-select">
                                                    <option value="left">左键</option>
                                                    <option value="right">右键</option>
                                                    <option value="middle">中键</option>
                                                </select>
                                                <span class="label">X:</span>
                                                <input type="number" v-model.number="event.x" class="small-input coord">
                                                <span class="label">Y:</span>
                                                <input type="number" v-model.number="event.y" class="small-input coord">
                                            </template>

                                            <!-- Mouse Move -->
                                            <template v-if="event.event_type === 'MouseMove'">
                                                <span class="label">X:</span>
                                                <input type="number" v-model.number="event.x" class="small-input coord">
                                                <span class="label">Y:</span>
                                                <input type="number" v-model.number="event.y" class="small-input coord">
                                            </template>

                                            <!-- Mouse Scroll -->
                                            <template v-if="event.event_type === 'MouseScroll'">
                                                <span class="label">X:</span>
                                                <input type="number" v-model.number="event.delta_x"
                                                    class="small-input coord">
                                                <span class="label">Y:</span>
                                                <input type="number" v-model.number="event.delta_y"
                                                    class="small-input coord">
                                            </template>

                                            <div class="reorder-btns">
                                                <button class="btn-reorder" @click="moveEvent(index, eIndex, 'up')"
                                                    :disabled="index === 0 && eIndex === 0">▲</button>
                                                <button class="btn-reorder" @click="moveEvent(index, eIndex, 'down')"
                                                    :disabled="index === groups.length - 1 && eIndex === group.events.length - 1">▼</button>
                                            </div>
                                            <div class="insert-btn-container">
                                                <button class="btn-icon small" @click="toggleInsertMenu(index, eIndex)"
                                                    title="在此之后插入">
                                                    ➕
                                                </button>
                                                <div v-if="insertMenuIndex?.groupIndex === index && insertMenuIndex?.eventIndex === eIndex"
                                                    class="dropdown-menu insert-menu">
                                                    <button
                                                        @click="addEventTemplate('Delay', group.startIndex + eIndex)">⌛
                                                        等待</button>
                                                    <button
                                                        @click="addEventTemplate('KeyPress', group.startIndex + eIndex)">⌨️
                                                        按下</button>
                                                    <button
                                                        @click="addEventTemplate('KeyRelease', group.startIndex + eIndex)">⌨️
                                                        弹起</button>
                                                    <button
                                                        @click="addEventTemplate('MousePress', group.startIndex + eIndex)">🖱️
                                                        按下</button>
                                                    <button
                                                        @click="addEventTemplate('MouseRelease', group.startIndex + eIndex)">🖱️
                                                        弹起</button>
                                                    <button
                                                        @click="addEventTemplate('MouseMove', group.startIndex + eIndex)">🎯
                                                        移动</button>
                                                </div>
                                            </div>
                                            <button class="btn-icon small danger"
                                                @click="deleteEvent(index, eIndex)">✕</button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </template>
            </div>
        </div>
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

const store = useScriptStore();
const currentScriptPath = ref<string | null>(null);
const currentScript = ref<Script | null>(null);
const groups = ref<EventGroup[]>([]);
const showAddMenu = ref(false);
const insertMenuIndex = ref<{ groupIndex: number, eventIndex: number } | null>(null);
const capturingIndex = ref<{ groupIndex: number, eventIndex: number } | null>(null);

// Watch currentScript and build `groups` ref
// Watch currentScript and build `groups` ref
watch(() => currentScript.value, (newScript) => {
    if (!newScript) {
        groups.value = [];
        return;
    }

    // Capture existing expanded states by index
    const expandedStates = groups.value.map(g => g.expanded);

    // Recreate groups
    const newGroups = groupEvents(newScript.events);

    // Restore expanded states where possible
    newGroups.forEach((group, i) => {
        if (expandedStates[i]) {
            group.expanded = true;
        }
    });

    groups.value = newGroups;
}, { deep: true, immediate: true });

function toggleGroup(index: number) {
    if (groups.value[index]) {
        groups.value[index].expanded = !groups.value[index].expanded;
    }
}

// --- Editing Functions ---

async function deleteScript(path: string) {
    if (!confirm(`确定要彻底删除脚本吗？`)) return;
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
    store.showNotification('已加载当前录制以供编辑。', 'info');
}

function addNewScript() {
    const newScript = store.createNewDraftScript();
    currentScript.value = newScript;
    currentScriptPath.value = null; // New script hasn't been saved yet
    store.showNotification('已创建新脚本，请在保存时指定路径。', 'info');
}

function toggleInsertMenu(groupIndex: number, eventIndex: number) {
    if (insertMenuIndex.value?.groupIndex === groupIndex && insertMenuIndex.value?.eventIndex === eventIndex) {
        insertMenuIndex.value = null;
    } else {
        insertMenuIndex.value = { groupIndex, eventIndex };
    }
}

function addEventTemplate(type: string, insertAfterIndex?: number) {
    if (!currentScript.value) return;

    let event: any = { event_type: type };

    switch (type) {
        case 'Delay':
            event.duration_ms = 100;
            break;
        case 'KeyPress':
        case 'KeyRelease':
            event.key = { type: 'Char', value: 'a' };
            break;
        case 'MousePress':
        case 'MouseRelease':
            event.button = 'left';
            event.x = 0;
            event.y = 0;
            break;
        case 'MouseMove':
            event.x = 0;
            event.y = 0;
            break;
        case 'MouseScroll':
            event.delta_x = 0;
            event.delta_y = 0;
            break;
    }

    if (typeof insertAfterIndex === 'number') {
        currentScript.value.events.splice(insertAfterIndex + 1, 0, event);
        insertMenuIndex.value = null;
    } else {
        currentScript.value.events.push(event);
        showAddMenu.value = false;
    }

    // Expand the group if we just added to it
    setTimeout(() => {
        // Find the group that contains this event
        // (Just expanding all for simplicity in this case)
        groups.value.forEach(g => { g.expanded = true; });
    }, 0);
}

function moveEvent(groupIndex: number, eventIndex: number, direction: 'up' | 'down') {
    if (!currentScript.value) return;
    const group = groups.value[groupIndex];
    const realIndex = group.startIndex + eventIndex;
    const targetIndex = direction === 'up' ? realIndex - 1 : realIndex + 1;

    if (targetIndex < 0 || targetIndex >= currentScript.value.events.length) return;

    const [event] = currentScript.value.events.splice(realIndex, 1);
    currentScript.value.events.splice(targetIndex, 0, event);
}

function startCapture(groupIndex: number, eventIndex: number) {
    capturingIndex.value = { groupIndex, eventIndex };
    window.addEventListener('keydown', onCaptureKeyDown, { once: true });
}

function onCaptureKeyDown(e: KeyboardEvent) {
    if (!capturingIndex.value || !currentScript.value) return;

    // Check if still in editor view. If not, don't do anything.
    if (store.currentView !== 'visual-editor') {
        capturingIndex.value = null;
        return;
    }

    e.preventDefault();
    e.stopPropagation();

    const { groupIndex, eventIndex } = capturingIndex.value;

    // Find event again because groups might have changed if we are reactive, 
    // but here we use a simple stable index approach.
    const group = groups.value[groupIndex];
    if (!group) {
        capturingIndex.value = null;
        return;
    }
    const realIndex = group.startIndex + eventIndex;
    const event = currentScript.value.events[realIndex];

    if (event && (event.event_type === 'KeyPress' || event.event_type === 'KeyRelease')) {
        if (e.key.length === 1) {
            event.key = { type: 'Char', value: e.key };
        } else {
            // Map special keys
            let keyName = e.key;
            const keyMap: Record<string, string> = {
                ' ': 'Space',
                'Control': 'ControlLeft',
                'Shift': 'ShiftLeft',
                'Alt': 'Alt',
                'Meta': 'MetaLeft',
                'ArrowUp': 'UpArrow',
                'ArrowDown': 'DownArrow',
                'ArrowLeft': 'LeftArrow',
                'ArrowRight': 'RightArrow',
                'Enter': 'Return',
                'Backspace': 'Backspace',
                'Escape': 'Escape',
                'Tab': 'Tab',
                'CapsLock': 'CapsLock',
                'Delete': 'Delete',
                'End': 'End',
                'Home': 'Home',
                'PageUp': 'PageUp',
                'PageDown': 'PageDown'
            };
            if (keyMap[keyName]) keyName = keyMap[keyName];
            event.key = { type: 'Special', value: keyName };
        }
    }

    capturingIndex.value = null;
}

function deleteEvent(groupIndex: number, eventIndex: number) {
    if (!currentScript.value) return;

    const group = groups.value[groupIndex];
    const realIndex = group.startIndex + eventIndex;

    currentScript.value.events.splice(realIndex, 1);
}

function deleteGroup(groupIndex: number) {
    if (!currentScript.value) return;
    if (!confirm('确定要删除整个事件组吗？')) return;

    const group = groups.value[groupIndex];
    currentScript.value.events.splice(group.startIndex, group.events.length);
}


// ---

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

        // If it's a new script, ask for path
        if (!path) {
            const defaultDir = await invoke<string>('get_scripts_dir');
            const savePath = await save({
                defaultPath: `${defaultDir}/${currentScript.value.name}.autokb`,
                filters: [{ name: 'AutoKB Script', extensions: ['autokb'] }],
            });
            if (!savePath) return; // User cancelled
            path = savePath;
            currentScriptPath.value = path;
        }

        await invoke('save_script', { script: currentScript.value, path });
        store.showNotification('保存成功！', 'success');
        await store.listSavedScripts(); // Refresh list
    } catch (e) {
        store.showNotification('保存失败: ' + e, 'error');
    }
}

async function loadIntoPlayback() {
    if (!currentScript.value) return;
    store.currentScript = JSON.parse(JSON.stringify(currentScript.value));
    store.statusMessage = `已加载 ${currentScript.value.name}`;
    store.currentView = 'home';
    store.showNotification(`已加载 ${currentScript.value.name} 以供回放。`, 'info');
}

onMounted(() => {
    refreshScripts();
});
</script>

<style scoped>
.visual-editor {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: var(--color-bg-primary);
    z-index: 1000;
    display: flex;
    flex-direction: column;
}

.editor-header {
    height: 60px;
    border-bottom: 1px solid var(--color-border);
    display: flex;
    align-items: center;
    padding: 0 20px;
    background-color: var(--color-bg-secondary);
    justify-content: space-between;
}

.editor-header h2 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    color: var(--color-text-primary);
}

.btn-back {
    display: flex;
    align-items: center;
    gap: 8px;
    background: none;
    border: none;
    color: var(--color-text-secondary);
    cursor: pointer;
    font-size: 14px;
    padding: 8px 12px;
    border-radius: 6px;
    transition: background 0.2s;
}

.btn-back:hover {
    background-color: var(--color-hover);
    color: var(--color-text-primary);
}

.editor-body {
    flex: 1;
    display: flex;
    overflow: hidden;
}

.sidebar {
    width: 250px;
    background-color: var(--color-bg-secondary);
    border-right: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
}

.sidebar-header {
    padding: 15px;
    border-bottom: 1px solid var(--color-border);
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.sidebar-actions {
    display: flex;
    gap: 8px;
}

.sidebar-header h3 {
    margin: 0;
    font-size: 14px;
    color: var(--color-text-secondary);
    text-transform: uppercase;
}

.script-list {
    flex: 1;
    overflow-y: auto;
}

.script-item {
    padding: 12px 15px;
    cursor: pointer;
    border-bottom: 1px solid var(--color-border);
    transition: background 0.2s;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.btn-delete-script {
    background: none;
    border: none;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.2s;
}

.script-item:hover .btn-delete-script {
    opacity: 1;
}

.script-item:hover {
    background-color: var(--color-hover);
}

.script-item.active {
    background-color: var(--color-bg-tertiary);
    border-left: 3px solid var(--color-accent);
}

.script-name {
    font-size: 14px;
    color: var(--color-text-primary);
}

.main-editor {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: var(--color-bg-primary);
}

.empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-text-disabled);
}

.toolbar {
    height: 60px;
    border-bottom: 1px solid var(--color-border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 20px;
    background-color: var(--color-bg-tertiary);
}

.script-name-input {
    background: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    color: var(--color-text-primary);
    font-size: 16px;
    font-weight: 600;
    padding: 4px 8px;
    border-radius: 4px;
    width: 200px;
}

.script-name-input:focus {
    outline: none;
    border-color: var(--color-accent);
}

.script-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
}

.event-count {
    font-size: 11px;
    color: var(--color-text-secondary);
}

.toolbar-actions {
    display: flex;
    gap: 10px;
}

.btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border-radius: 6px;
    font-size: 13px;
    cursor: pointer;
    border: 1px solid transparent;
}

.btn-primary {
    background-color: var(--color-accent);
    color: white;
}

.btn-secondary {
    background-color: var(--color-bg-primary);
    border-color: var(--color-border);
    color: var(--color-text-primary);
}

/* Dropdown */
.add-event-dropdown {
    position: relative;
}

.dropdown-menu {
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: 5px;
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    box-shadow: var(--shadow-md);
    z-index: 1100;
    width: 150px;
    display: flex;
    flex-direction: column;
    padding: 4px;
}

.dropdown-menu button {
    background: none;
    border: none;
    color: var(--color-text-primary);
    text-align: left;
    padding: 8px 12px;
    font-size: 13px;
    cursor: pointer;
    border-radius: 4px;
    transition: background 0.2s;
}

.dropdown-menu button:hover {
    background-color: var(--color-accent-dim);
}

.events-container {
    flex: 1;
    overflow-y: auto;
    padding: 20px;
}

.event-group {
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    margin-bottom: 8px;
    overflow: hidden;
}

.group-header {
    display: flex;
    align-items: center;
    padding: 10px 15px;
    cursor: pointer;
    background-color: var(--color-bg-secondary);
    transition: background 0.2s;
}

.group-header:hover {
    background-color: var(--color-hover);
}

.group-icon {
    font-size: 18px;
    margin-right: 12px;
}

.group-title {
    flex: 1;
    font-size: 14px;
    font-weight: 500;
    color: var(--color-text-primary);
    display: flex;
    align-items: center;
    gap: 8px;
}

.group-actions {
    display: flex;
    align-items: center;
    gap: 4px;
    margin-right: 12px;
    opacity: 0;
    transition: opacity 0.2s;
}

.group-header:hover .group-actions {
    opacity: 1;
}

.btn-icon.small {
    padding: 4px;
    width: 24px;
    height: 24px;
    border-radius: 4px;
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    color: var(--color-text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
}

.btn-icon.small:hover:not(:disabled) {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-primary);
}

.btn-icon.small.danger:hover {
    background-color: var(--color-danger);
    color: white;
    border-color: var(--color-danger);
}

.btn-icon.small:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.badge {
    background-color: var(--color-bg-primary);
    padding: 2px 6px;
    border-radius: 10px;
    font-size: 11px;
    color: var(--color-text-secondary);
}

.group-toggle {
    font-size: 12px;
    color: var(--color-text-disabled);
}

.group-body {
    border-top: 1px solid var(--color-border);
    background-color: var(--color-bg-primary);
}

.event-item {
    padding: 8px 15px;
    border-bottom: 1px solid var(--color-border);
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-family: monospace;
    font-size: 13px;
    color: var(--color-text-secondary);
}

.event-info {
    flex: 1;
    min-width: 0;
}

.event-desc {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: block;
}

.event-item:last-child {
    border-bottom: none;
}

.event-controls {
    display: flex;
    align-items: center;
    gap: 6px;
}

.small-input {
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    color: var(--color-text-primary);
    padding: 2px 4px;
    border-radius: 4px;
    font-family: monospace;
    font-size: 12px;
}

.small-input.coord {
    width: 45px;
}

.small-input.char {
    width: 25px;
    text-align: center;
}

.small-input.special {
    width: 100px;
}

.btn-capture {
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    color: var(--color-accent);
    padding: 2px 8px;
    border-radius: 4px;
    font-family: monospace;
    font-size: 12px;
    cursor: pointer;
    min-width: 60px;
    text-align: center;
}

.btn-capture:hover {
    background: var(--color-hover);
    border-color: var(--color-accent);
}

.btn-capture.capturing {
    background: var(--color-accent-dim);
    border-color: var(--color-accent);
    color: var(--color-text-primary);
    animation: pulse-recording 1.5s infinite;
}

.reorder-btns {
    display: flex;
    flex-direction: column;
    gap: 1px;
}

.btn-reorder {
    background: none;
    border: none;
    color: var(--color-text-disabled);
    cursor: pointer;
    font-size: 8px;
    padding: 0 4px;
    line-height: 1;
}

.btn-reorder:hover:not(:disabled) {
    color: var(--color-text-primary);
}

.btn-reorder:disabled {
    opacity: 0.2;
    cursor: not-allowed;
}

@keyframes pulse-recording {
    0% {
        opacity: 1;
    }

    50% {
        opacity: 0.5;
    }

    100% {
        opacity: 1;
    }
}

.insert-btn-container {
    position: relative;
    display: flex;
    align-items: center;
}

.insert-menu {
    right: 0;
    left: auto;
    width: 100px;
    top: 0;
    transform: translateY(-50%);
}

.insert-menu button {
    padding: 4px 8px;
    font-size: 11px;
}

.small-select {
    background: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    color: var(--color-text-primary);
    padding: 1px 2px;
    border-radius: 4px;
    font-size: 12px;
}

.label {
    font-size: 11px;
    color: var(--color-text-disabled);
}

.unit {
    font-size: 11px;
    color: var(--color-text-disabled);
    width: 16px;
}

/* Scrollbar */
::-webkit-scrollbar {
    width: 6px;
}

::-webkit-scrollbar-track {
    background: transparent;
}

::-webkit-scrollbar-thumb {
    background: var(--color-border);
    border-radius: 3px;
}
</style>
