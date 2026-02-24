<template>
    <div class="script-library">
        <div class="flex items-center justify-between mb-8">
            <div>
                <h2 class="text-2xl font-bold text-text-main">脚本库</h2>
                <p class="text-text-muted text-sm mt-1">管理和测试您的自动化脚本</p>
            </div>
            <button @click="store.saveScript()"
                class="flex items-center gap-2 bg-primary text-white px-4 py-2 rounded-lg text-sm font-bold hover:bg-primary/90 transition-colors shadow-sm shadow-primary/20">
                <span class="material-symbols-outlined text-sm">upload_file</span> 导入脚本
            </button>
        </div>

        <!-- Grid of Scripts -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div v-for="script in store.savedScripts" :key="script.path"
                class="group bg-surface-main rounded-xl border border-border-main p-5 hover:shadow-xl hover:border-primary/30 transition-all cursor-pointer flex flex-col h-48">
                <div class="flex items-start justify-between mb-3">
                    <div class="p-2 bg-surface-soft rounded-lg group-hover:bg-primary/10 transition-colors">
                        <span
                            class="material-symbols-outlined text-text-muted group-hover:text-primary transition-colors">description</span>
                    </div>
                    <div class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                        <button @click.stop="editScript(script.path)"
                            class="p-1.5 hover:bg-surface-soft rounded-md text-text-muted hover:text-primary transition-colors"
                            title="编辑">
                            <span class="material-symbols-outlined text-lg">edit</span>
                        </button>
                        <button @click.stop="deleteScript(script.path)"
                            class="p-1.5 hover:bg-error-bg text-slate-400 hover:text-error transition-colors"
                            title="删除">
                            <span class="material-symbols-outlined text-lg">delete</span>
                        </button>
                    </div>
                </div>

                <div class="flex-1 overflow-hidden" @click="playScript(script.path)">
                    <h3 class="font-bold text-text-main truncate group-hover:text-primary transition-colors">
                        {{ script.name }}</h3>
                    <p class="text-xs text-text-muted mt-2 line-clamp-2 leading-relaxed">{{ script.description || '无描述'
                        }}</p>
                </div>

                <div class="pt-4 mt-auto border-t border-border-main flex items-center justify-between">
                    <span class="text-[10px] text-text-muted font-mono">FILE: {{ getBasename(script.path) }}</span>
                    <button @click.stop="playScript(script.path)"
                        class="flex items-center gap-1 text-xs font-bold text-primary hover:bg-primary/10 px-2 py-1 rounded transition-colors uppercase tracking-wider">
                        <span class="material-symbols-outlined text-sm">play_arrow</span> 运行
                    </button>
                </div>
            </div>

            <!-- Add New Card -->
            <button @click="store.currentView = 'visual-editor'"
                class="flex flex-col items-center justify-center p-8 bg-surface-soft border-2 border-dashed border-border-main rounded-xl hover:border-primary/50 hover:bg-primary/5 transition-all text-text-muted hover:text-primary h-48 group">
                <span
                    class="material-symbols-outlined text-4xl mb-2 group-hover:scale-110 transition-transform">add_task</span>
                <span class="text-sm font-bold">创建新脚本</span>
            </button>
        </div>

        <div v-if="store.savedScripts.length === 0"
            class="mt-12 text-center py-20 bg-surface-main rounded-2xl border border-border-main">
            <span class="material-symbols-outlined text-6xl text-text-muted/20 mb-4">folder_open</span>
            <p class="text-text-muted">脚本库空空如也，快去创建一个吧！</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useScriptStore } from '../stores/scriptStore';
import { invoke } from '@tauri-apps/api/core';

const store = useScriptStore();

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
    if (!confirm('确定要删除此脚本吗？')) return;
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
