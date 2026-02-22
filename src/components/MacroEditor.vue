<template>
    <div class="macro-editor">
        <div class="editor-header">
            <button class="btn-back" @click="store.currentView = 'home'">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
                    stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="19" y1="12" x2="5" y2="12"></line>
                    <polyline points="12 19 5 12 12 5"></polyline>
                </svg>
                Back to Editor
            </button>
            <h2>Macro Management</h2>
            <div class="macro-toggle">
                <span class="toggle-label">Macro Listener</span>
                <button class="btn-toggle" :class="{ 'active': store.isMacroActive }"
                    @click="store.toggleMacroListener">
                    {{ store.isMacroActive ? 'ON' : 'OFF' }}
                </button>
            </div>
        </div>

        <div class="editor-content">
            <!-- Create New Macro -->
            <div class="panel create-panel">
                <h3>Create New Macro</h3>
                <div class="form-group">
                    <label>Macro Name</label>
                    <input v-model="newMacroName" type="text" placeholder="e.g. Farm Loop" />
                </div>

                <div class="form-row">
                    <div class="form-group">
                        <label>Trigger Type</label>
                        <div class="trigger-type-switch">
                            <button :class="{ active: triggerType === 'key' }"
                                @click="triggerType = 'key'">Keyboard</button>
                            <button :class="{ active: triggerType === 'mouse' }"
                                @click="triggerType = 'mouse'">Mouse</button>
                        </div>
                    </div>

                    <div class="form-group">
                        <label>Trigger Value</label>
                        <div v-if="triggerType === 'key'" class="input-recorder">
                            <button class="btn-record" :class="{ recording: isRecordingTrigger }"
                                @click="startTriggerRecording">
                                {{ isRecordingTrigger ? 'Press any key...' : (recordedKey || 'Click to set key') }}
                            </button>
                        </div>
                        <select v-else v-model="selectedMouseButton">
                            <option value="left">Left Button</option>
                            <option value="right">Right Button</option>
                            <option value="middle">Middle Button</option>
                            <option value="back">Back Button</option>
                            <option value="forward">Forward Button</option>
                        </select>
                    </div>
                </div>

                <div class="form-group">
                    <label>Execute Script</label>
                    <select v-model="selectedScript" class="script-select">
                        <option v-if="store.savedScripts.length === 0" disabled value="">No saved scripts found</option>
                        <option v-for="script in store.savedScripts" :key="script.path" :value="script.path">
                            {{ script.name }}
                        </option>
                    </select>
                </div>

                <button class="btn-primary btn-create" @click="createMacro" :disabled="!canCreate">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="12" y1="5" x2="12" y2="19"></line>
                        <line x1="5" y1="12" x2="19" y2="12"></line>
                    </svg>
                    Add Binding
                </button>
            </div>

            <!-- Macro List -->
            <div class="macro-list">
                <div v-if="store.macros.length === 0" class="empty-state">
                    No macros defined. Create one to get started.
                </div>
                <div v-else class="macro-grid">
                    <div v-for="macro in store.macros" :key="macro.id" class="macro-card"
                        :class="{ disabled: !macro.enabled }">
                        <div class="macro-header">
                            <span class="macro-name">{{ macro.name }}</span>
                            <div class="macro-actions">
                                <button class="btn-icon small"
                                    @click="store.toggleMacroEnabled(macro.id, !macro.enabled)" title="Toggle Enable">
                                    <svg v-if="macro.enabled" xmlns="http://www.w3.org/2000/svg" width="16" height="16"
                                        viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                                        stroke-linecap="round" stroke-linejoin="round" class="icon-green">
                                        <polyline points="20 6 9 17 4 12"></polyline>
                                    </svg>
                                    <svg v-else xmlns="http://www.w3.org/2000/svg" width="16" height="16"
                                        viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                                        stroke-linecap="round" stroke-linejoin="round" class="icon-gray">
                                        <circle cx="12" cy="12" r="10"></circle>
                                        <line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line>
                                    </svg>
                                </button>
                                <button class="btn-icon small danger" @click="store.removeMacro(macro.id)"
                                    title="Delete">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                                        fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                        stroke-linejoin="round">
                                        <polyline points="3 6 5 6 21 6"></polyline>
                                        <path
                                            d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2">
                                        </path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                        <div class="macro-details">
                            <div class="detail-item">
                                <span class="label">Trigger:</span>
                                <span class="value badge">{{ formatTrigger(macro.trigger) }}</span>
                            </div>
                            <div class="detail-arrow">⬇</div>
                            <div class="detail-item">
                                <span class="label">Script:</span>
                                <span class="value code">{{ getScriptName(macro.script_path) }}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useScriptStore } from '../stores/scriptStore';
import type { MacroTrigger } from '../types/script';

const store = useScriptStore();

// Form State
const newMacroName = ref('');
const triggerType = ref<'key' | 'mouse'>('key');
const selectedMouseButton = ref('right'); // Default safe option
const selectedScript = ref('');
const isRecordingTrigger = ref(false);
const recordedKey = ref('');

// Validation
const canCreate = computed(() => {
    if (!newMacroName.value) return false;
    if (!selectedScript.value) return false;
    if (triggerType.value === 'key' && !recordedKey.value) return false;
    return true;
});

// Key Recording Logic
function startTriggerRecording() {
    isRecordingTrigger.value = true;
    recordedKey.value = 'Press key...';
}

function handleKeyDown(e: KeyboardEvent) {
    if (isRecordingTrigger.value) {
        e.preventDefault();
        e.stopPropagation();
        recordedKey.value = e.key;
        isRecordingTrigger.value = false;
    }
}

onMounted(() => {
    window.addEventListener('keydown', handleKeyDown, true); // Capture phase
});

onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyDown, true);
});

// Helpers
function formatTrigger(trigger: MacroTrigger): string {
    if (trigger.trigger_type === 'KeyPress') {
        return `Key: ${trigger.key.type === 'Char' ? trigger.key.value.toUpperCase() : trigger.key.value}`;
    } else {
        return `Mouse: ${trigger.button}`;
    }
}

function getScriptName(path: string): string {
    // Extract filename from path
    const parts = path.split(/[/\\]/);
    return parts[parts.length - 1];
}

async function createMacro() {
    await store.createMacroBinding(
        newMacroName.value,
        triggerType.value,
        triggerType.value === 'key' ? recordedKey.value : selectedMouseButton.value,
        selectedScript.value
    );

    // Reset form
    newMacroName.value = '';
    // Keep script selected for easier multi-binding creation
}
</script>

<style scoped>
.macro-editor {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: var(--color-bg-primary);
    z-index: 1000;
    display: flex;
    flex-direction: column;
    padding: 0;
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

.macro-toggle {
    display: flex;
    align-items: center;
    gap: 12px;
}

.toggle-label {
    font-size: 14px;
    color: var(--color-text-secondary);
}

.btn-toggle {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-disabled);
    border: 1px solid var(--color-border);
    padding: 6px 16px;
    border-radius: 20px;
    cursor: pointer;
    font-weight: 600;
    transition: all 0.3s ease;
    min-width: 60px;
}

.btn-toggle.active {
    background-color: var(--color-accent);
    color: white;
    border-color: var(--color-accent);
    box-shadow: 0 0 10px rgba(99, 102, 241, 0.3);
}

.editor-content {
    flex: 1;
    padding: 20px;
    display: flex;
    gap: 20px;
    overflow: hidden;
}

.create-panel {
    width: 350px;
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 20px;
}

.create-panel h3 {
    margin: 0 0 10px 0;
    font-size: 16px;
    color: var(--color-text-primary);
}

.form-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.form-group label {
    font-size: 12px;
    color: var(--color-text-secondary);
    font-weight: 500;
}

.form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
}

input,
select {
    background-color: var(--color-bg-primary);
    border: 1px solid var(--color-border);
    color: var(--color-text-primary);
    padding: 8px 12px;
    border-radius: 6px;
    outline: none;
    font-size: 14px;
}

input:focus,
select:focus {
    border-color: var(--color-accent);
}

.trigger-type-switch {
    display: flex;
    background-color: var(--color-bg-primary);
    padding: 2px;
    border-radius: 6px;
    border: 1px solid var(--color-border);
}

.trigger-type-switch button {
    flex: 1;
    background: none;
    border: none;
    font-size: 12px;
    padding: 6px;
    cursor: pointer;
    color: var(--color-text-secondary);
    border-radius: 4px;
}

.trigger-type-switch button.active {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-primary);
    font-weight: 600;
}

.btn-record {
    width: 100%;
    background-color: var(--color-bg-primary);
    border: 1px dashed var(--color-border);
    color: var(--color-text-secondary);
    padding: 8px;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
}

.btn-record:hover {
    border-color: var(--color-accent);
    color: var(--color-accent);
}

.btn-record.recording {
    background-color: rgba(239, 68, 68, 0.1);
    border-color: #ef4444;
    color: #ef4444;
    animation: pulse 1.5s infinite;
}

.btn-create {
    margin-top: auto;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
}

.macro-list {
    flex: 1;
    overflow-y: auto;
    padding-right: 4px;
}

.empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 200px;
    color: var(--color-text-disabled);
    border: 2px dashed var(--color-border);
    border-radius: 8px;
}

.macro-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 16px;
}

.macro-card {
    background-color: var(--color-bg-secondary);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    transition: transform 0.2s, box-shadow 0.2s;
}

.macro-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    border-color: var(--color-accent-dim);
}

.macro-card.disabled {
    opacity: 0.6;
    filter: grayscale(1);
}

.macro-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.macro-name {
    font-weight: 600;
    color: var(--color-text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.macro-actions {
    display: flex;
    gap: 4px;
}

.btn-icon.small {
    padding: 4px;
    width: 24px;
    height: 24px;
}

.macro-details {
    background-color: var(--color-bg-primary);
    padding: 10px;
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    align-items: center;
}

.detail-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    justify-content: space-between;
}

.detail-arrow {
    color: var(--color-text-disabled);
    font-size: 12px;
}

.label {
    font-size: 12px;
    color: var(--color-text-secondary);
}

.badge {
    background-color: var(--color-bg-tertiary);
    color: var(--color-text-primary);
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 12px;
    font-family: monospace;
}

.code {
    color: var(--color-accent);
    font-size: 12px;
    font-family: monospace;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 150px;
}

@keyframes pulse {
    0% {
        opacity: 1;
    }

    50% {
        opacity: 0.6;
    }

    100% {
        opacity: 1;
    }
}

/* Scrollbar styling */
::-webkit-scrollbar {
    width: 8px;
}

::-webkit-scrollbar-track {
    background: var(--color-bg-primary);
}

::-webkit-scrollbar-thumb {
    background: var(--color-border);
    border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
    background: var(--color-text-disabled);
}
</style>
