<script setup lang="ts">
import { useScriptStore } from '../stores/scriptStore';
import { useConfirmDialog } from '../composables/useConfirmDialog';

const store = useScriptStore();
const { confirm } = useConfirmDialog();

function getStatusClass(task: any) {
    if (!task.enabled) return 'status-disabled';
    return 'status-enabled';
}

function getStatusDotClass(task: any) {
    if (!task.enabled) return 'dot-disabled';
    return 'dot-enabled';
}

async function removeTask(taskId: string) {
    const confirmed = await confirm({
        title: '删除任务',
        message: '确定要删除此任务吗？此操作无法撤销。',
        confirmText: '删除',
        cancelText: '取消'
    });

    if (confirmed) {
        store.removeTask(taskId);
    }
}
</script>

<template>
    <div class="task-dashboard">
        <div class="dashboard-header">
            <div class="header-content">
                <h1 class="dashboard-title">我的工作流</h1>
                <p class="dashboard-subtitle">管理并高效运行您的本地自动化任务，让工作更简单。</p>
            </div>
            <div class="header-actions">
                <button class="action-button secondary">
                    <span class="material-symbols-outlined icon">filter_list</span>
                    <span>筛选</span>
                </button>
                <button class="action-button primary">
                    <span class="material-symbols-outlined icon">add</span>
                    <span>新建任务</span>
                </button>
            </div>
        </div>

        <!-- Workflow Table Card -->
        <div class="table-container">
            <div class="table-wrapper">
                <table class="task-table">
                    <thead class="table-header">
                        <tr>
                            <th class="table-header-cell">名称</th>
                            <th class="table-header-cell">状态</th>
                            <th class="table-header-cell">触发键</th>
                            <th class="table-header-cell align-right">操作</th>
                        </tr>
                    </thead>
                    <tbody class="table-body">
                        <tr v-for="task in store.tasks" :key="task.id" class="table-row">
                            <td class="table-cell">
                                <div class="task-info">
                                    <div class="task-icon">
                                        <span class="material-symbols-outlined">account_tree</span>
                                    </div>
                                    <div class="task-details">
                                        <p class="task-name">{{ task.name }}</p>
                                        <p class="task-description">{{ task.description || '无描述' }}</p>
                                    </div>
                                </div>
                            </td>
                            <td class="table-cell">
                                <span :class="['status-badge', getStatusClass(task)]">
                                    <span :class="['status-dot', getStatusDotClass(task)]"></span>
                                    {{ task.enabled ? '已启用' : '已禁用' }}
                                </span>
                            </td>
                            <td class="table-cell trigger-key">
                                <span v-if="task.trigger_key" class="key-badge">
                                    {{ task.trigger_key.type === 'Char' ? task.trigger_key.value.toUpperCase() :
                                        task.trigger_key.value }}
                                </span>
                                <span v-else>未设置</span>
                            </td>
                            <td class="table-cell">
                                <div class="action-buttons">
                                    <button @click="store.toggleTaskEnabled(task.id, !task.enabled)"
                                        class="icon-button"
                                        :title="task.enabled ? '禁用' : '启用'">
                                        <span class="material-symbols-outlined">{{ task.enabled ? 'pause' : 'play_arrow'
                                            }}</span>
                                    </button>
                                    <button class="icon-button" title="编辑">
                                        <span class="material-symbols-outlined">edit</span>
                                    </button>
                                    <button @click="removeTask(task.id)" class="icon-button delete" title="删除">
                                        <span class="material-symbols-outlined">delete</span>
                                    </button>
                                </div>
                            </td>
                        </tr>
                        <tr v-if="store.tasks.length === 0">
                            <td colspan="4" class="empty-state">
                                暂无任务，点击右上角新建。
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</template>

<style scoped>
/* Dashboard Container */
.task-dashboard {
    animation: fade-in 0.3s ease-out;
}

/* Dashboard Header */
.dashboard-header {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-end;
    justify-content: space-between;
    gap: 1rem;
    margin-bottom: 2rem;
}

.header-content {
    flex: 1;
}

.dashboard-title {
    font-size: 1.875rem;
    font-weight: 900;
    color: var(--text-main);
}

.dashboard-subtitle {
    color: var(--text-muted);
    margin-top: 0.5rem;
}

.header-actions {
    display: flex;
    gap: 0.75rem;
}

/* Action Buttons */
.action-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    font-weight: 500;
    transition: background-color 150ms, color 150ms, box-shadow 150ms;
    cursor: pointer;
}

.action-button.secondary {
    border: 1px solid var(--border-main);
    background-color: var(--surface);
    color: var(--text-main);
}

.action-button.secondary:hover {
    background-color: var(--surface-soft);
}

.action-button.primary {
    background-color: var(--primary);
    color: white;
    border: none;
    box-shadow: 0 1px 2px 0 rgba(19, 91, 236, 0.2);
}

.action-button.primary:hover {
    background-color: rgba(19, 91, 236, 0.9);
}

.action-button .icon {
    font-size: 1.25rem;
}

/* Table Container */
.table-container {
    background-color: var(--surface);
    border-radius: 0.75rem;
    border: 1px solid var(--border-main);
    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
    overflow: hidden;
}

.table-wrapper {
    overflow-x: auto;
}

/* Table */
.task-table {
    width: 100%;
    text-align: left;
}

/* Table Header */
.table-header {
    background-color: var(--surface-soft);
    color: var(--text-muted);
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 600;
}

.table-header-cell {
    padding: 1rem 1.5rem;
}

.table-header-cell.align-right {
    text-align: right;
}

/* Table Body */
.table-body {
    border-top: 1px solid var(--border-main);
}

.table-body tr {
    border-bottom: 1px solid var(--border-main);
}

.table-body tr:last-child {
    border-bottom: none;
}

.table-row {
    transition: background-color 150ms;
}

.table-row:hover {
    background-color: var(--surface-soft);
}

.table-cell {
    padding: 1.25rem 1.5rem;
}

/* Task Info */
.task-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
}

.task-icon {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: 0.5rem;
    background-color: rgba(19, 91, 236, 0.1);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--primary);
}

.task-details {
    flex: 1;
}

.task-name {
    font-weight: 600;
    color: var(--text-main);
}

.task-description {
    font-size: 0.75rem;
    color: var(--text-muted);
}

/* Status Badge */
.status-badge {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.25rem 0.625rem;
    border-radius: 9999px;
    font-size: 0.75rem;
    font-weight: 500;
}

.status-badge.status-disabled {
    background-color: var(--surface-soft);
    color: var(--text-muted);
}

.status-badge.status-enabled {
    background-color: var(--success-bg);
    color: var(--success);
}

.status-dot {
    width: 0.375rem;
    height: 0.375rem;
    border-radius: 9999px;
}

.status-dot.dot-disabled {
    background-color: rgba(100, 116, 139, 0.4);
}

.status-dot.dot-enabled {
    background-color: var(--success);
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes pulse {
    0%, 100% {
        opacity: 1;
    }
    50% {
        opacity: 0.5;
    }
}

/* Trigger Key */
.trigger-key {
    font-size: 0.875rem;
    color: var(--text-muted);
}

.key-badge {
    padding: 0.25rem 0.5rem;
    background-color: var(--surface-soft);
    border-radius: 0.25rem;
    border: 1px solid var(--border-main);
    font-family: monospace;
}

/* Action Buttons */
.action-buttons {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
}

.icon-button {
    padding: 0.5rem;
    color: var(--text-muted);
    border-radius: 0.5rem;
    transition: background-color 150ms, color 150ms;
    cursor: pointer;
    border: none;
    background: transparent;
}

.icon-button:hover {
    background-color: var(--surface-soft);
}

.icon-button.delete:hover {
    background-color: var(--error-bg);
    color: var(--error);
}

/* Empty State */
.empty-state {
    padding: 2.5rem 1.5rem;
    text-align: center;
    color: var(--text-muted);
}

/* Animation */
@keyframes fade-in {
    from {
        opacity: 0;
        transform: translateY(10px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
}
</style>
