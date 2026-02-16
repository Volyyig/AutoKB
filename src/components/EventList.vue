<script setup lang="ts">
import { computed } from 'vue';
import { useScriptStore } from '../stores/scriptStore';
import { getEventDescription, formatDuration } from '../types/script';
import { groupEvents, formatGroupTitle, type EventGroup } from '../utils/eventGrouping';

const store = useScriptStore();

// Watch script events and group them
const groups = computed(() => {
    return groupEvents(store.currentScript.events);
});

// Helper to check if a group contains the selected event
function isGroupSelected(group: EventGroup): boolean {
    if (store.selectedEventIndex === null) return false;
    return store.selectedEventIndex >= group.startIndex &&
        store.selectedEventIndex < (group.startIndex + group.events.length);
}

function selectGroup(group: EventGroup) {
    // Select the first event of the group? Or maybe we toggle expansion?
    // User wants "Same grouping logic" as editor.
    // In EventList (Home View), we usually select single events to modify params in ParamEditor.
    // So if we group, we need to allow expanding to see single events.
    if (isGroupSelected(group)) {
        // Deselect
        store.selectedEventIndex = null;
    } else {
        // Select first event
        store.selectedEventIndex = group.startIndex;
    }
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
            <div v-for="(group, index) in groups" :key="index" class="group-item"
                :class="{ active: isGroupSelected(group) }">

                <!-- Group Header -->
                <div class="group-header" @click="selectGroup(group)">
                    <div class="group-icon">
                        <span v-if="group.type === 'KeyPress' || group.type === 'KeyRelease'">⌨️</span>
                        <span v-else-if="group.type.startsWith('Mouse')">🖱️</span>
                        <span v-else>❓</span>
                    </div>
                    <div class="group-info">
                        <span class="group-title">{{ formatGroupTitle(group.type) }}</span>
                        <span class="group-count">{{ group.events.length }} events</span>
                    </div>
                </div>

                <!-- Expanded Events (Only if group contains selected event or is playing?) -->
                <!-- We show details if this group is 'active' (contains selection) -->
                <div v-if="isGroupSelected(group)" class="group-details">
                    <div v-for="(event, eIndex) in group.events" :key="eIndex" class="detail-row"
                        :class="{ selected: store.selectedEventIndex === (group.startIndex + eIndex) }"
                        @click.stop="store.selectedEventIndex = (group.startIndex + eIndex)">
                        <span class="detail-index">{{ group.startIndex + eIndex + 1 }}</span>
                        <span class="detail-desc">{{ getEventDescription(event) }}</span>
                        <span class="detail-delay">+{{ event.delay_ms }}ms</span>
                    </div>
                </div>
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
    color: var(--color-text-disabled);
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
    color: var(--color-text-secondary);
}

/* Events Container */
.events-container {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
}

.group-item {
    background-color: var(--color-bg-tertiary);
    border: 1px solid transparent;
    border-radius: 6px;
    margin-bottom: 4px;
    overflow: hidden;
}

.group-item.active {
    border-color: var(--color-accent);
}

.group-header {
    display: flex;
    align-items: center;
    padding: 8px 12px;
    cursor: pointer;
    transition: background 0.2s;
}

.group-header:hover {
    background-color: var(--color-hover);
}

.group-icon {
    font-size: 16px;
    margin-right: 10px;
    width: 20px;
    display: flex;
    justify-content: center;
}

.group-info {
    flex: 1;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.group-title {
    font-size: 13px;
    color: var(--color-text-primary);
    font-weight: 500;
}

.group-count {
    font-size: 11px;
    color: var(--color-text-secondary);
    background-color: var(--color-bg-primary);
    padding: 2px 6px;
    border-radius: 10px;
}

.group-details {
    background-color: var(--color-bg-secondary);
    border-top: 1px solid var(--color-border);
}

.detail-row {
    display: flex;
    align-items: center;
    padding: 6px 12px 6px 36px;
    font-size: 12px;
    color: var(--color-text-secondary);
    cursor: pointer;
    border-bottom: 1px solid var(--color-border);
}

.detail-row:last-child {
    border-bottom: none;
}

.detail-row:hover {
    background-color: var(--color-hover);
    color: var(--color-text-primary);
}

.detail-row.selected {
    background-color: rgba(99, 102, 241, 0.15);
    /* Accent color dim */
    color: var(--color-text-primary);
}

.detail-index {
    width: 30px;
    color: var(--color-text-disabled);
    font-family: monospace;
}

.detail-desc {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    font-family: monospace;
}

.detail-delay {
    color: var(--color-accent);
    font-family: monospace;
}

/* Stats Bar */
.stats-bar {
    display: flex;
    justify-content: space-between;
    padding: 10px 16px;
    background: var(--color-bg-tertiary);
    border-top: 1px solid var(--color-border);
    font-size: 12px;
    color: var(--color-text-secondary);
}
</style>
