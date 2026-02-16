<script setup lang="ts">
import { useScriptStore } from '../stores/scriptStore';
import { getEventDescription, formatDuration } from '../types/script';

const store = useScriptStore();

function selectEvent(index: number) {
    store.selectedEventIndex = store.selectedEventIndex === index ? null : index;
}
</script>

<template>
    <div class="event-list">
        <!-- Empty State -->
        <div v-if="!store.hasEvents" class="empty-state">
            <div class="empty-icon">
                <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none"
                    stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                    <polyline points="14 2 14 8 20 8"></polyline>
                    <line x1="12" y1="18" x2="12" y2="12"></line>
                    <line x1="9" y1="15" x2="15" y2="15"></line>
                </svg>
            </div>
            <p>No events recorded</p>
            <p class="empty-hint">Press F9 or click "Start Recording"</p>
        </div>

        <!-- Event Items -->
        <div v-else class="events-container">
            <div v-for="(event, index) in store.currentScript.events" :key="index" class="event-item"
                :class="{ selected: store.selectedEventIndex === index }" @click="selectEvent(index)">
                <span class="event-index">{{ index + 1 }}</span>
                <span class="event-icon">
                    <!-- Key Events -->
                    <svg v-if="event.event_type.startsWith('Key')" xmlns="http://www.w3.org/2000/svg" width="16"
                        height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                        stroke-linecap="round" stroke-linejoin="round">
                        <rect x="3" y="5" width="18" height="14" rx="2" ry="2"></rect>
                        <line x1="7" y1="15" x2="7" y2="15"></line>
                        <line x1="11" y1="15" x2="13" y2="15"></line>
                    </svg>
                    <!-- Mouse Events -->
                    <svg v-else-if="event.event_type.startsWith('Mouse')" xmlns="http://www.w3.org/2000/svg" width="16"
                        height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                        stroke-linecap="round" stroke-linejoin="round">
                        <rect x="7" y="2" width="10" height="20" rx="5" ry="5"></rect>
                        <line x1="12" y1="6" x2="12" y2="10"></line>
                    </svg>
                    <!-- Fallback -->
                    <svg v-else xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24"
                        fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round">
                        <circle cx="12" cy="12" r="10"></circle>
                        <line x1="12" y1="8" x2="12" y2="16"></line>
                        <line x1="8" y1="12" x2="16" y2="12"></line>
                    </svg>
                </span>
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
