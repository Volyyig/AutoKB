<template>
    <div class="script-library">
        <div class="library-header">
            <div>
                <h2 class="library-title">脚本库</h2>
                <p class="library-subtitle">管理和测试您的自动化脚本</p>
            </div>
            <button @click="store.saveScript()" class="import-button">
                <span class="material-symbols-outlined">upload_file</span> 导入脚本
            </button>
        </div>

        <!-- Grid of Scripts -->
        <div class="script-grid">
            <div v-for="script in store.savedScripts" :key="script.path" class="script-card">
                <div class="card-header">
                    <div class="card-icon">
                        <span class="material-symbols-outlined">description</span>
                    </div>
                    <div class="card-actions">
                        <button @click.stop="editScript(script.path)" class="action-button edit-button" title="编辑">
                            <span class="material-symbols-outlined">edit</span>
                        </button>
                        <button @click.stop="deleteScript(script.path)" class="action-button delete-button" title="删除">
                            <span class="material-symbols-outlined">delete</span>
                        </button>
                    </div>
                </div>

                <div class="card-content" @click="playScript(script.path)">
                    <h3 class="card-title">{{ script.name }}</h3>
                    <p class="card-description">{{ script.description || '无描述' }}</p>
                </div>

                <div class="card-footer">
                    <span class="card-filename">FILE: {{ getBasename(script.path) }}</span>
                    <button @click.stop="playScript(script.path)" class="play-button">
                        <span class="material-symbols-outlined">play_arrow</span> 运行
                    </button>
                </div>
            </div>

            <!-- Add New Card -->
            <button @click="store.currentView = 'visual-editor'" class="add-card">
                <span class="material-symbols-outlined add-icon">add_task</span>
                <span class="add-text">创建新脚本</span>
            </button>
        </div>

        <div v-if="store.savedScripts.length === 0" class="empty-state">
            <span class="material-symbols-outlined empty-icon">folder_open</span>
            <p class="empty-text">脚本库空空如也，快去创建一个吧！</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useScriptStore } from '../stores/scriptStore';
import { invoke } from '@tauri-apps/api/core';
import { useConfirmDialog } from '../composables/useConfirmDialog';

const store = useScriptStore();
const { confirm } = useConfirmDialog();

async function playScript(path: string) {
    try {
        await store.loadScriptByPath(path);
        await store.startPlayback();
    } catch (error) {
        store.showNotification('运行失败: ' + error, 'error');
    }
}

async function editScript(path: string) {
    try {
        await store.loadScriptByPath(path);
        store.currentView = 'visual-editor';
    } catch (error) {
        store.showNotification('加载失败: ' + error, 'error');
    }
}

async function deleteScript(path: string) {
    const confirmed = await confirm({
        title: '删除脚本',
        message: '确定要彻底删除此脚本吗？此操作无法撤销。',
        confirmText: '删除',
        cancelText: '取消'
    });
    
    if (!confirmed) return;
    
    try {
        await invoke('delete_script', { path });
        await store.listSavedScripts();
        store.showNotification('删除成功', 'success');
    } catch (error) {
        store.showNotification('删除失败: ' + error, 'error');
    }
}

function getBasename(path: string) {
    return path.split(/[\\/]/).pop() || '';
}
</script>

<style scoped>
/* Container */
.script-library {
  padding: 0;
}

/* Header */
.library-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 2rem;
}

.library-title {
  font-size: 1.5rem;
  font-weight: bold;
  color: var(--text-main);
}

.library-subtitle {
  color: var(--text-muted);
  font-size: 0.875rem;
  margin-top: 0.25rem;
}

.import-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background-color: var(--primary);
  color: white;
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  font-weight: bold;
  border: none;
  cursor: pointer;
  transition: background-color 150ms;
  box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
}

.import-button:hover {
  background-color: color-mix(in srgb, var(--primary) 90%, black);
}

.import-button .material-symbols-outlined {
  font-size: 0.875rem;
}

/* Grid Layout */
.script-grid {
  display: grid;
  grid-template-columns: repeat(1, minmax(0, 1fr));
  gap: 1.5rem;
}

@media (min-width: 768px) {
  .script-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (min-width: 1024px) {
  .script-grid {
    grid-template-columns: repeat(3, minmax(0, 1fr));
  }
}

/* Script Card */
.script-card {
  background-color: var(--surface);
  border-radius: 0.75rem;
  border: 1px solid var(--border-main);
  padding: 1.25rem;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  height: 12rem;
  transition: box-shadow 150ms, border-color 150ms;
}

.script-card:hover {
  box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
  border-color: color-mix(in srgb, var(--primary) 30%, transparent);
}

/* Card Header */
.card-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: 0.75rem;
}

.card-icon {
  padding: 0.5rem;
  background-color: var(--surface-soft);
  border-radius: 0.5rem;
  transition: background-color 150ms;
}

.script-card:hover .card-icon {
  background-color: color-mix(in srgb, var(--primary) 10%, transparent);
}

.card-icon .material-symbols-outlined {
  color: var(--text-muted);
  transition: color 150ms;
}

.script-card:hover .card-icon .material-symbols-outlined {
  color: var(--primary);
}

/* Card Actions */
.card-actions {
  display: flex;
  gap: 0.25rem;
  opacity: 0;
  transition: opacity 150ms;
}

.script-card:hover .card-actions {
  opacity: 1;
}

.action-button {
  padding: 0.375rem;
  border-radius: 0.375rem;
  border: none;
  background: transparent;
  cursor: pointer;
  transition: background-color 150ms, color 150ms;
}

.action-button .material-symbols-outlined {
  font-size: 1.125rem;
}

.edit-button {
  color: var(--text-muted);
}

.edit-button:hover {
  background-color: var(--surface-soft);
  color: var(--primary);
}

.delete-button {
  color: rgb(148 163 184);
}

.delete-button:hover {
  background-color: var(--error-bg);
  color: var(--error);
}

/* Card Content */
.card-content {
  flex: 1;
  overflow: hidden;
}

.card-title {
  font-weight: bold;
  color: var(--text-main);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: color 150ms;
}

.script-card:hover .card-title {
  color: var(--primary);
}

.card-description {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin-top: 0.5rem;
  line-height: 1.625;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* Card Footer */
.card-footer {
  padding-top: 1rem;
  margin-top: auto;
  border-top: 1px solid var(--border-main);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.card-filename {
  font-size: 0.625rem;
  color: var(--text-muted);
  font-family: monospace;
}

.play-button {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  font-size: 0.75rem;
  font-weight: bold;
  color: var(--primary);
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  border: none;
  background: transparent;
  cursor: pointer;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  transition: background-color 150ms;
}

.play-button:hover {
  background-color: color-mix(in srgb, var(--primary) 10%, transparent);
}

.play-button .material-symbols-outlined {
  font-size: 0.875rem;
}

/* Add New Card */
.add-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  background-color: var(--surface-soft);
  border: 2px dashed var(--border-main);
  border-radius: 0.75rem;
  color: var(--text-muted);
  height: 12rem;
  cursor: pointer;
  transition: border-color 150ms, background-color 150ms, color 150ms;
}

.add-card:hover {
  border-color: color-mix(in srgb, var(--primary) 50%, transparent);
  background-color: color-mix(in srgb, var(--primary) 5%, transparent);
  color: var(--primary);
}

.add-icon {
  font-size: 2.25rem;
  margin-bottom: 0.5rem;
  transition: transform 150ms;
}

.add-card:hover .add-icon {
  transform: scale(1.1);
}

.add-text {
  font-size: 0.875rem;
  font-weight: bold;
}

/* Empty State */
.empty-state {
  margin-top: 3rem;
  text-align: center;
  padding: 5rem 0;
  background-color: var(--surface);
  border-radius: 1rem;
  border: 1px solid var(--border-main);
}

.empty-icon {
  font-size: 3.75rem;
  color: color-mix(in srgb, var(--text-muted) 20%, transparent);
  margin-bottom: 1rem;
  display: block;
}

.empty-text {
  color: var(--text-muted);
}
</style>
