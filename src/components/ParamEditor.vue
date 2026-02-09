<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useScriptStore } from '../stores/scriptStore';

const store = useScriptStore();

// Local state for editing
const loopCount = ref(1);
const loopDelay = ref(0);
const speedMultiplier = ref(1.0);

// Selected event delay editing
const editingDelay = ref<number | null>(null);

// Sync with store
watch(() => store.currentScript.loop_config, (config) => {
    loopCount.value = config.count;
    loopDelay.value = config.delay_between_ms;
}, { immediate: true });

watch(() => store.currentScript.speed_multiplier, (speed) => {
    speedMultiplier.value = speed;
}, { immediate: true });

// Computed
const selectedEvent = computed(() => {
    if (store.selectedEventIndex !== null) {
        return store.currentScript.events[store.selectedEventIndex];
    }
    return null;
});

// Actions
function updateLoopConfig() {
    store.updateLoopConfig(loopCount.value, loopDelay.value);
}

function updateSpeed() {
    store.updateSpeed(speedMultiplier.value);
}

function updateSelectedDelay() {
    if (store.selectedEventIndex !== null && editingDelay.value !== null) {
        store.updateEventDelay(store.selectedEventIndex, editingDelay.value);
        editingDelay.value = null;
    }
}

function scaleAllDelays(factor: number) {
    store.scaleDelays(factor);
}

function adjustDelay(delta: number) {
    if (editingDelay.value !== null) {
        let newVal = editingDelay.value + delta;
        if (newVal < 0) newVal = 0;
        editingDelay.value = newVal;
    }
}

// Initialize editing delay when selection changes
watch(() => store.selectedEventIndex, () => {
    if (selectedEvent.value) {
        editingDelay.value = selectedEvent.value.delay_ms;
    }
});
</script>

<template>
    <div class="param-editor">
        <!-- Loop Settings -->
        <div class="param-section">
            <h3>🔄 循环设置</h3>
            <div class="param-row">
                <label>循环次数</label>
                <div class="input-group">
                    <input type="number" v-model.number="loopCount" min="0" @change="updateLoopConfig" />
                    <span class="input-hint">0 = 无限</span>
                </div>
            </div>
            <div class="param-row">
                <label>循环间隔</label>
                <div class="input-group">
                    <input type="number" v-model.number="loopDelay" min="0" step="100" @change="updateLoopConfig" />
                    <span class="input-unit">ms</span>
                </div>
            </div>
        </div>

        <!-- Speed Settings -->
        <div class="param-section">
            <h3>⚡ 速度设置</h3>
            <div class="param-row">
                <label>执行速度</label>
                <div class="speed-control">
                    <input type="range" v-model.number="speedMultiplier" min="0.25" max="4" step="0.25"
                        @change="updateSpeed" />
                    <span class="speed-value">{{ speedMultiplier }}x</span>
                </div>
            </div>
            <div class="speed-presets">
                <button v-for="preset in [0.5, 1, 2, 4]" :key="preset" class="preset-btn"
                    :class="{ active: speedMultiplier === preset }" @click="speedMultiplier = preset; updateSpeed()">
                    {{ preset }}x
                </button>
            </div>
        </div>

        <!-- Delay Scaling -->
        <div class="param-section">
            <h3>⏱ 延迟调整</h3>
            <div class="scale-buttons">
                <button class="scale-btn" @click="scaleAllDelays(0.5)">÷2</button>
                <button class="scale-btn" @click="scaleAllDelays(0.75)">-25%</button>
                <button class="scale-btn" @click="scaleAllDelays(1.25)">+25%</button>
                <button class="scale-btn" @click="scaleAllDelays(2)">×2</button>
            </div>
        </div>

        <!-- Selected Event Editor -->
        <div v-if="selectedEvent" class="param-section selected-event">
            <h3>📝 选中事件</h3>
            <div class="event-details">
                <div class="detail-row">
                    <span class="detail-label">类型:</span>
                    <span class="detail-value">{{ selectedEvent.event_type }}</span>
                </div>
                <div class="detail-row">
                    <span class="detail-label">延迟:</span>
                    <div class="input-group small">
                         <button class="adjust-btn" @click="adjustDelay(-10)" title="-10ms">«</button>
                         <button class="adjust-btn" @click="adjustDelay(-1)" title="-1ms">‹</button>
                        <input type="number" v-model.number="editingDelay" min="0" step="1" />
                         <button class="adjust-btn" @click="adjustDelay(1)" title="+1ms">›</button>
                         <button class="adjust-btn" @click="adjustDelay(10)" title="+10ms">»</button>
                        <span class="input-unit">ms</span>
                        <button class="apply-btn" @click="updateSelectedDelay">应用</button>
                    </div>
                </div>
            </div>
            <button class="delete-selected-btn" @click="store.deleteEvent(store.selectedEventIndex!)">
                🗑 删除此事件
            </button>
        </div>
    </div>
</template>

<style scoped>
.param-editor {
    padding: 12px;
}

.param-section {
    margin-bottom: 16px;
    padding: 12px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
}

.param-section h3 {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: 12px;
}

.param-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
}

.param-row:last-child {
    margin-bottom: 0;
}

.param-row label {
    font-size: 13px;
    color: var(--text-primary);
}

.input-group {
    display: flex;
    align-items: center;
    gap: 6px;
}

.input-group input[type="number"] {
    width: 80px;
    padding: 6px 8px;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-size: 13px;
    text-align: right;
}

.input-group input:focus {
    outline: none;
    border-color: var(--accent-primary);
}

.input-unit {
    font-size: 11px;
    color: var(--text-muted);
}

.input-hint {
    font-size: 10px;
    color: var(--text-muted);
}

/* Speed Control */
.speed-control {
    display: flex;
    align-items: center;
    gap: 10px;
}

.speed-control input[type="range"] {
    width: 100px;
    accent-color: var(--accent-primary);
}

.speed-value {
    min-width: 40px;
    font-size: 13px;
    font-weight: 500;
    color: var(--accent-primary);
}

.speed-presets {
    display: flex;
    gap: 6px;
    margin-top: 8px;
}

.preset-btn {
    flex: 1;
    padding: 4px 8px;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    font-size: 11px;
    cursor: pointer;
    transition: all var(--transition-fast);
}

.preset-btn:hover {
    background: var(--bg-hover);
}

.preset-btn.active {
    background: var(--accent-primary);
    border-color: var(--accent-primary);
    color: white;
}

/* Scale Buttons */
.scale-buttons {
    display: flex;
    gap: 6px;
}

.scale-btn {
    flex: 1;
    padding: 6px 10px;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    font-size: 12px;
    cursor: pointer;
    transition: all var(--transition-fast);
}

.scale-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
}

/* Selected Event */
.selected-event {
    border: 1px solid var(--accent-primary);
}

.event-details {
    margin-bottom: 12px;
}

.detail-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
}

.detail-label {
    font-size: 12px;
    color: var(--text-muted);
    min-width: 40px;
}

.detail-value {
    font-size: 12px;
    color: var(--text-primary);
    font-weight: 500;
}

.input-group.small input {
    width: 60px;
}

.apply-btn {
    padding: 4px 8px;
    background: var(--accent-primary);
    border: none;
    border-radius: var(--radius-sm);
    color: white;
    font-size: 11px;
    cursor: pointer;
}

.apply-btn:hover {
    background: #4c94e0;
}

.adjust-btn {
    width: 24px;
    height: 24px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 14px;
    line-height: 1;
}

.adjust-btn:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
}

.delete-selected-btn {
    width: 100%;
    padding: 8px;
    background: rgba(248, 81, 73, 0.1);
    border: 1px solid var(--accent-danger);
    border-radius: var(--radius-sm);
    color: var(--accent-danger);
    font-size: 12px;
    cursor: pointer;
    transition: all var(--transition-fast);
}

.delete-selected-btn:hover {
    background: var(--accent-danger);
    color: white;
}
</style>
