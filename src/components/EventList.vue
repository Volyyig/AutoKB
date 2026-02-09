<script setup lang="ts">
import { useScriptStore } from '../stores/scriptStore';
import { getEventIcon, getEventDescription, formatDuration } from '../types/script';

const store = useScriptStore();

function selectEvent(index: number) {
    store.selectedEventIndex = store.selectedEventIndex === index ? null : index;
}
</script>

<template>
    <div class="event-list">
        <!-- Empty State -->
        <div v-if="!store.hasEvents" class="empty-state">
            <div class="empty-icon">📭</div>
            <p>暂无录制事件</p>
            <p class="empty-hint">按 F9 或点击"开始录制"按钮开始录制</p>
        </div>

        <!-- Event Items -->
        <div v-else class="events-container">
            <div v-for="(event, index) in store.currentScript.events" :key="index" class="event-item"
                :class="{ selected: store.selectedEventIndex === index }" @click="selectEvent(index)">
                <span class="event-index">{{ index + 1 }}</span>
                <span class="event-icon">{{ getEventIcon(event) }}</span>
                <div class="event-info">
                    <span class="event-type">{{ event.event_type }}</span>
                    <span class="event-desc">{{ getEventDescription(event) }}</span>
                </div>
                <span class="event-delay">+{{ formatDuration(event.delay_ms) }}</span>
                <button class="delete-btn" @click.stop="store.deleteEvent(index)" title="删除事件">
                    ✕
                </button>
            </div>
        </div>

        <!-- Stats Bar -->
        <div v-if="store.hasEvents" class="stats-bar">
            <span>总事件: {{ store.eventCount }}</span>
            <span>总时长: {{ formatDuration(store.totalDuration) }}</span>
        </div>
    </div>
</template>

<style scoped>
.event-list {
    display: flex;
    flex-direction: column;
    flex: 1;
    overflow: hidden;
}

/* Empty State */
.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
    padding: 40px;
    color: var(--text-muted);
}

.empty-icon {
    font-size: 48px;
    margin-bottom: 16px;
    opacity: 0.5;
}

.empty-state p {
    margin: 4px 0;
}

.empty-hint {
    font-size: 12px;
    color: var(--text-muted);
}

/* Events Container */
.events-container {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
}

/* Event Item */
.event-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    margin-bottom: 4px;
    background: var(--bg-tertiary);
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: all var(--transition-fast);
}

.event-item:hover {
    background: var(--bg-hover);
    border-color: var(--border-color);
}

.event-item.selected {
    background: rgba(88, 166, 255, 0.1);
    border-color: var(--accent-primary);
}

.event-index {
    min-width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-primary);
    border-radius: var(--radius-sm);
    font-size: 11px;
    color: var(--text-muted);
}

.event-icon {
    font-size: 16px;
}

.event-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow: hidden;
}

.event-type {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
}

.event-desc {
    font-size: 11px;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.event-delay {
    font-size: 11px;
    color: var(--accent-primary);
    font-family: 'Consolas', monospace;
    padding: 2px 6px;
    background: rgba(88, 166, 255, 0.1);
    border-radius: var(--radius-sm);
}

.delete-btn {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: var(--radius-sm);
    opacity: 0;
    transition: all var(--transition-fast);
}

.event-item:hover .delete-btn {
    opacity: 1;
}

.delete-btn:hover {
    background: var(--accent-danger);
    color: white;
}

/* Stats Bar */
.stats-bar {
    display: flex;
    justify-content: space-between;
    padding: 10px 16px;
    background: var(--bg-tertiary);
    border-top: 1px solid var(--border-color);
    font-size: 12px;
    color: var(--text-muted);
}
</style>
