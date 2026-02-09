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
                <span class="btn-icon">{{ store.isRecording ? '⏹' : '⏺' }}</span>
                {{ store.isRecording ? '停止录制' : '开始录制' }}
            </button>
        </div>

        <!-- Playback Controls -->
        <div class="control-group">
            <button class="btn" :class="{ 'btn-warning': store.isPlaying, 'btn-success': !store.isPlaying }"
                @click="store.togglePlayback" :disabled="store.isRecording || !store.hasEvents">
                <span class="btn-icon">{{ store.isPlaying ? '⏹' : '▶' }}</span>
                {{ store.isPlaying ? '停止回放' : '开始回放' }}
            </button>
        </div>

        <!-- File Operations -->
        <div class="control-group file-group">
            <button class="btn btn-secondary" @click="store.saveScript" :disabled="!store.hasEvents">
                <span class="btn-icon">💾</span>
                保存
            </button>
            <button class="btn btn-secondary" @click="store.loadScript">
                <span class="btn-icon">📂</span>
                加载
            </button>
            <button class="btn btn-ghost" @click="store.clearScript" :disabled="!store.hasEvents">
                <span class="btn-icon">🗑</span>
                清空
            </button>
        </div>

        <!-- Macro Toggle -->
        <div class="control-group macro-group">
            <button class="btn" :class="{ 'btn-purple': store.isMacroActive, 'btn-secondary': !store.isMacroActive }"
                @click="store.toggleMacroListener">
                <span class="btn-icon">🔗</span>
                {{ store.isMacroActive ? '宏: 开' : '宏: 关' }}
            </button>
        </div>
    </div>
</template>

<style scoped>
.control-bar {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 12px 20px;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
}

.control-group {
    display: flex;
    gap: 8px;
}

.file-group {
    margin-left: auto;
}

.macro-group {
    border-left: 1px solid var(--border-color);
    padding-left: 16px;
}

/* Button Styles */
.btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 16px;
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all var(--transition-fast);
    background: var(--bg-tertiary);
    color: var(--text-primary);
}

.btn:hover:not(:disabled) {
    background: var(--bg-hover);
}

.btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.btn-icon {
    font-size: 14px;
}

.btn-primary {
    background: var(--accent-primary);
    color: white;
}

.btn-primary:hover:not(:disabled) {
    background: #4c94e0;
}

.btn-success {
    background: var(--accent-success);
    color: white;
}

.btn-success:hover:not(:disabled) {
    background: #36a045;
}

.btn-danger {
    background: var(--accent-danger);
    color: white;
    animation: pulse-btn 1s infinite;
}

@keyframes pulse-btn {

    0%,
    100% {
        box-shadow: 0 0 0 0 rgba(248, 81, 73, 0.4);
    }

    50% {
        box-shadow: 0 0 0 8px rgba(248, 81, 73, 0);
    }
}

.btn-warning {
    background: var(--accent-warning);
    color: white;
}

.btn-secondary {
    background: var(--bg-tertiary);
    border-color: var(--border-color);
}

.btn-ghost {
    background: transparent;
    color: var(--text-secondary);
}

.btn-ghost:hover:not(:disabled) {
    background: var(--bg-tertiary);
    color: var(--text-primary);
}

.btn-purple {
    background: var(--accent-purple);
    color: white;
}

.btn-purple:hover:not(:disabled) {
    background: #9164e0;
}
</style>
