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
                Back
            </button>
            <h2>Script Editor</h2>
            <div class="header-actions">
                <!-- Placeholder for potentially other actions -->
            </div>
        </div>

        <div class="editor-body">
            <!-- Left Sidebar: Script List -->
            <div class="sidebar">
                <div class="sidebar-header">
                    <h3>Scripts</h3>
                    <button class="btn-icon small" @click="refreshScripts" title="Refresh List">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none"
                            stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M23 4v6h-6"></path>
                            <path d="M1 20v-6h6"></path>
                            <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
                        </svg>
                    </button>
                </div>
                <div class="script-list">
                    <div v-if="store.savedScripts.length === 0" class="empty-list">
                        No scripts found.
                    </div>
                    <div v-for="script in store.savedScripts" :key="script.path" class="script-item"
                        :class="{ active: currentScriptPath === script.path }" @click="loadScriptForEdit(script.path)">
                        <span class="script-name">{{ script.name }}</span>
                    </div>
                </div>
            </div>

            <!-- Right Content: Editor -->
            <div class="main-editor">
                <div v-if="!currentScript" class="empty-state">
                    Please select a script from the list to edit.
                </div>
                <template v-else>
                    <div class="toolbar">
                        <div class="script-info">
                            <h3>{{ currentScript.name }}</h3>
                            <span class="event-count">{{ currentScript.events.length }} events</span>
                        </div>
                        <div class="toolbar-actions">
                            <button class="btn btn-secondary" @click="saveChanges">
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                                    fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                    stroke-linejoin="round">
                                    <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
                                    <polyline points="17 21 17 13 7 13 7 21"></polyline>
                                    <polyline points="7 3 7 8 15 8"></polyline>
                                </svg>
                                Save
                            </button>
                            <button class="btn btn-primary" @click="loadIntoPlayback">
                                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                                    fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                    stroke-linejoin="round">
                                    <path d="M5 12h14"></path>
                                    <path d="M12 5l7 7-7 7"></path>
                                </svg>
                                Load
                            </button>
                        </div>
                    </div>

                    <div class="events-container">
                        <div v-if="groups.length === 0" class="no-events">
                            No events in this script.
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
                                        <button class="btn-icon small danger" @click="deleteGroup(index)"
                                            title="Delete Group">
                                            🗑️
                                        </button>
                                    </div>
                                    <div class="group-toggle">
                                        {{ group.expanded ? '▼' : '▶' }}
                                    </div>
                                </div>
                                <div v-show="group.expanded" class="group-body">
                                    <div v-for="(event, eIndex) in group.events" :key="eIndex" class="event-item">
                                        <span class="event-desc">{{ getEventDescription(event) }}</span>
                                        <div class="event-controls">
                                            <span class="label">Delay:</span>
                                            <input type="number" v-model.number="event.delay_ms" class="delay-input"
                                                min="0">
                                            <span class="unit">ms</span>
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
import type { Script } from '../types/script';
import { getEventDescription } from '../types/script';
import { groupEvents, formatGroupTitle, type EventGroup } from '../utils/eventGrouping';

const store = useScriptStore();
const currentScriptPath = ref<string | null>(null);
const currentScript = ref<Script | null>(null);
const groups = ref<EventGroup[]>([]);

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

function deleteEvent(groupIndex: number, eventIndex: number) {
    if (!currentScript.value) return;

    const group = groups.value[groupIndex];
    const realIndex = group.startIndex + eventIndex;

    currentScript.value.events.splice(realIndex, 1);
}

function deleteGroup(groupIndex: number) {
    if (!currentScript.value) return;
    if (!confirm('Are you sure you want to delete this entire group of events?')) return;

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
    if (!currentScript.value || !currentScriptPath.value) return;
    try {
        await invoke('save_script', { script: currentScript.value, path: currentScriptPath.value });
        store.showNotification('Saved successfully!', 'success');
    } catch (e) {
        store.showNotification('Save failed: ' + e, 'error');
    }
}

async function loadIntoPlayback() {
    if (!currentScript.value) return;
    store.currentScript = JSON.parse(JSON.stringify(currentScript.value));
    store.statusMessage = `Loaded ${currentScript.value.name}`;
    store.currentView = 'home';
    store.showNotification(`Loaded ${currentScript.value.name} for playback.`, 'info');
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

.script-info h3 {
    margin: 0;
    font-size: 16px;
    color: var(--color-text-primary);
}

.event-count {
    font-size: 12px;
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

.event-item:last-child {
    border-bottom: none;
}

.event-controls {
    display: flex;
    align-items: center;
    gap: 8px;
}

.label {
    font-size: 11px;
    color: var(--color-text-disabled);
}

.delay-input {
    width: 60px;
    padding: 2px 6px;
    border-radius: 4px;
    border: 1px solid var(--color-border);
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-primary);
    font-size: 12px;
    text-align: right;
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
