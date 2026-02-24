<script setup lang="ts">
import { useScriptStore } from '../stores/scriptStore';

const store = useScriptStore();

function getStatusClass(task: any) {
    if (!task.enabled) return 'bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-300';
    return 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400';
}

function getStatusDotClass(task: any) {
    if (!task.enabled) return 'bg-slate-400';
    return 'bg-green-500 animate-pulse';
}
</script>

<template>
    <div class="task-dashboard">
        <div class="flex flex-wrap items-end justify-between gap-4 mb-8">
            <div>
                <h1 class="text-3xl font-black text-slate-900 dark:text-white">我的工作流</h1>
                <p class="text-slate-500 dark:text-slate-400 mt-2">管理并高效运行您的本地自动化任务，让工作更简单。</p>
            </div>
            <div class="flex gap-3">
                <button
                    class="flex items-center gap-2 px-4 py-2 border border-slate-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-900 hover:bg-slate-50 dark:hover:bg-slate-800 font-medium transition-colors">
                    <span class="material-symbols-outlined text-xl">filter_list</span>
                    <span>筛选</span>
                </button>
                <button
                    class="flex items-center gap-2 px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary/90 font-medium transition-colors shadow-sm shadow-primary/20">
                    <span class="material-symbols-outlined text-xl">add</span>
                    <span>新建任务</span>
                </button>
            </div>
        </div>

        <!-- Workflow Table Card -->
        <div
            class="bg-white dark:bg-slate-900 rounded-xl border border-slate-200 dark:border-slate-800 shadow-sm overflow-hidden">
            <div class="overflow-x-auto">
                <table class="w-full text-left">
                    <thead
                        class="bg-slate-50 dark:bg-slate-800/50 text-slate-500 dark:text-slate-400 text-xs uppercase tracking-wider font-semibold">
                        <tr>
                            <th class="px-6 py-4">名称</th>
                            <th class="px-6 py-4">状态</th>
                            <th class="px-6 py-4">触发键</th>
                            <th class="px-6 py-4 text-right">操作</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-200 dark:divide-slate-800">
                        <tr v-for="task in store.tasks" :key="task.id"
                            class="hover:bg-slate-50 dark:hover:bg-slate-800/30 transition-colors">
                            <td class="px-6 py-5">
                                <div class="flex items-center gap-3">
                                    <div
                                        class="w-10 h-10 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
                                        <span class="material-symbols-outlined">account_tree</span>
                                    </div>
                                    <div>
                                        <p class="font-semibold text-slate-900 dark:text-slate-100">{{ task.name }}</p>
                                        <p class="text-xs text-slate-500">{{ task.description || '无描述' }}</p>
                                    </div>
                                </div>
                            </td>
                            <td class="px-6 py-5">
                                <span
                                    :class="['inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium', getStatusClass(task)]">
                                    <span :class="['w-1.5 h-1.5 rounded-full', getStatusDotClass(task)]"></span>
                                    {{ task.enabled ? '已启用' : '已禁用' }}
                                </span>
                            </td>
                            <td class="px-6 py-5 text-sm text-slate-500 dark:text-slate-400">
                                <span v-if="task.trigger_key"
                                    class="px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded border border-slate-200 dark:border-slate-700 font-mono">
                                    {{ task.trigger_key.type === 'Char' ? task.trigger_key.value.toUpperCase() :
                                        task.trigger_key.value }}
                                </span>
                                <span v-else>未设置</span>
                            </td>
                            <td class="px-6 py-5">
                                <div class="flex justify-end gap-2">
                                    <button @click="store.toggleTaskEnabled(task.id, !task.enabled)"
                                        class="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-600 dark:text-slate-400 rounded-lg transition-colors"
                                        :title="task.enabled ? '禁用' : '启用'">
                                        <span class="material-symbols-outlined">{{ task.enabled ? 'pause' : 'play_arrow'
                                        }}</span>
                                    </button>
                                    <button
                                        class="p-2 hover:bg-slate-100 dark:hover:bg-slate-800 text-slate-600 dark:text-slate-400 rounded-lg transition-colors"
                                        title="编辑">
                                        <span class="material-symbols-outlined">edit</span>
                                    </button>
                                    <button @click="store.removeTask(task.id)"
                                        class="p-2 hover:bg-red-100 dark:hover:bg-red-900/30 text-red-600 rounded-lg transition-colors"
                                        title="删除">
                                        <span class="material-symbols-outlined">delete</span>
                                    </button>
                                </div>
                            </td>
                        </tr>
                        <tr v-if="store.tasks.length === 0">
                            <td colspan="4" class="px-6 py-10 text-center text-slate-500 dark:text-slate-400">
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
.task-dashboard {
    animation: fade-in 0.3s ease-out;
}

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
