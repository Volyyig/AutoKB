<script setup lang="ts">
import { useScriptStore } from '../stores/scriptStore';

const store = useScriptStore();
</script>

<template>
    <div class="control-bar">
        <!-- Recording Controls -->
        <div class="control-group">
            <button class="btn" :class="{ 'btn-danger': store.isRecording, 'btn-primary': !store.isRecording }"
                @click="store.toggleRecording" :disabled="store.isPlaying">
                <span class="btn-icon">
                    <svg v-if="store.isRecording" xmlns="http://www.w3.org/2000/svg" width="16" height="16"
                        viewBox="0 0 24 24" fill="currentColor" stroke="none">
                        <rect x="6" y="6" width="12" height="12"></rect>
                    </svg>
                    <svg v-else xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                        fill="currentColor" stroke="none">
                        <circle cx="12" cy="12" r="10"></circle>
                    </svg>
                </span>
                {{ store.isRecording ? 'Stop Recording' : 'Start Recording' }}
            </button>
        </div>

        <!-- Playback Controls -->
        <div class="control-group">
            <button class="btn" :class="{ 'btn-warning': store.isPlaying, 'btn-success': !store.isPlaying }"
                @click="store.togglePlayback" :disabled="store.isRecording || !store.hasEvents">
                <span class="btn-icon">
                    <svg v-if="store.isPlaying" xmlns="http://www.w3.org/2000/svg" width="16" height="16"
                        viewBox="0 0 24 24" fill="currentColor" stroke="none">
                        <rect x="6" y="6" width="12" height="12"></rect>
                    </svg>
                    <svg v-else xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                        fill="currentColor" stroke="none">
                        <polygon points="5 3 19 12 5 21 5 3"></polygon>
                    </svg>
                </span>
                {{ store.isPlaying ? 'Stop Playback' : 'Start Playback' }}
            </button>
        </div>

        <!-- File Operations -->
        <div class="control-group file-group">
            <button class="btn btn-secondary" @click="store.saveScript" :disabled="!store.hasEvents"
                title="Save Script">
                <span class="btn-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path>
                        <polyline points="17 21 17 13 7 13 7 21"></polyline>
                        <polyline points="7 3 7 8 15 8"></polyline>
                    </svg>
                </span>
                Save
            </button>
            <button class="btn btn-secondary" @click="store.loadScript" title="Load Script">
                <span class="btn-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
                    </svg>
                </span>
                Load
            </button>
        </div>

        <!-- Macro Controls -->
        <div class="control-group macro-group">
            <button class="btn" :class="store.isMacroActive ? 'btn-purple' : 'btn-secondary'"
                @click="store.currentView = 'macro-editor'">
                <span class="btn-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none"
                        stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                        <polyline points="15 3 21 3 21 9"></polyline>
                        <line x1="10" y1="14" x2="21" y2="3"></line>
                    </svg>
                </span>
                Manage Macros
            </button>
            <div class="macro-status" v-if="store.isMacroActive" title="Macros Active">
                <span class="status-dot-pulse"></span>
            </div>
        </div>
    </div>
</template>

<style scoped>
.control-bar {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 12px 20px;
    background: var(--color-bg-secondary);
    border-bottom: 1px solid var(--color-border);
}

.control-group {
    display: flex;
    gap: 8px;
    align-items: center;
}

.file-group {
    margin-left: auto;
    padding-right: 16px;
    border-right: 1px solid var(--color-border);
}

.macro-group {
    padding-left: 0;
}

/* Button Styles */
.btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border: 1px solid transparent;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    background: var(--color-bg-tertiary);
    color: var(--color-text-primary);
    height: 36px;
}

.btn:hover:not(:disabled) {
    background: var(--color-hover);
    border-color: var(--color-border);
}

.btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
}

.btn-primary {
    background: var(--color-accent);
    color: white;
}

.btn-primary:hover:not(:disabled) {
    background: var(--color-accent-hover);
}

.btn-success {
    background: var(--color-success);
    color: white;
}

.btn-success:hover:not(:disabled) {
    filter: brightness(1.1);
}

.btn-danger {
    background: var(--color-danger);
    color: white;
    animation: pulse-border 1.5s infinite;
}

.btn-warning {
    background: var(--color-warning);
    color: white;
}

.btn-secondary {
    background: var(--color-bg-primary);
    border-color: var(--color-border);
}

.btn-purple {
    background: rgba(99, 102, 241, 0.1);
    color: var(--color-accent);
    border: 1px solid var(--color-accent);
}

.btn-purple:hover:not(:disabled) {
    background: rgba(99, 102, 241, 0.2);
}

.macro-status {
    display: flex;
    align-items: center;
    padding: 0 4px;
}

.status-dot-pulse {
    width: 8px;
    height: 8px;
    background-color: var(--color-success);
    border-radius: 50%;
    box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.7);
    animation: pulse-green 2s infinite;
}

@keyframes pulse-border {
    0% {
        box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.4);
    }

    70% {
        box-shadow: 0 0 0 6px rgba(239, 68, 68, 0);
    }

    100% {
        box-shadow: 0 0 0 0 rgba(239, 68, 68, 0);
    }
}

@keyframes pulse-green {
    0% {
        transform: scale(0.95);
        box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.7);
    }

    70% {
        transform: scale(1);
        box-shadow: 0 0 0 6px rgba(16, 185, 129, 0);
    }

    100% {
        transform: scale(0.95);
        box-shadow: 0 0 0 0 rgba(16, 185, 129, 0);
    }
}
</style>
