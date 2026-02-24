<script setup lang="ts">
import { useScriptStore } from '../stores/scriptStore';

const store = useScriptStore();

const updateThemeColor = (key: string, value: string) => {
    (store.theme as any)[key] = value;
    store.applyTheme();
};

const toggleDarkMode = () => {
    store.theme.isDark = !store.theme.isDark;
    store.applyTheme();
};
</script>

<template>
    <div class="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
        <section>
            <h3 class="text-lg font-bold mb-4 flex items-center gap-2">
                <span class="material-symbols-outlined text-primary">palette</span>
                外观设置
            </h3>

            <div class="bg-surface-main rounded-xl border border-border-main p-6 space-y-6">
                <!-- Presets -->
                <div>
                    <label class="text-sm font-medium text-text-muted mb-3 block">预设主题</label>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
                        <button v-for="preset in store.themePresets" :key="preset.name" @click="store.setTheme(preset)"
                            class="flex flex-col items-center gap-2 p-3 rounded-lg border border-border-main hover:border-primary transition-all group">
                            <div class="w-full aspect-video rounded border border-border-main flex"
                                :style="{ backgroundColor: preset.background }">
                                <div class="w-1/3 h-full border-r border-border-main"
                                    :style="{ backgroundColor: preset.surface }"></div>
                                <div class="flex-1 p-2 flex flex-col gap-1">
                                    <div class="h-1 w-full rounded-full" :style="{ backgroundColor: preset.primary }">
                                    </div>
                                    <div class="h-1 w-2/3 rounded-full opacity-30"
                                        :style="{ backgroundColor: preset.textMain }"></div>
                                </div>
                            </div>
                            <span class="text-xs font-medium">{{ preset.name }}</span>
                        </button>
                    </div>
                </div>

                <div class="h-px bg-slate-100 dark:bg-slate-800"></div>

                <!-- Custom Colors -->
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <label class="text-sm font-medium">前景色 (Primary)</label>
                            <input type="color" :value="store.theme.primary"
                                @input="(e: any) => updateThemeColor('primary', e.target.value)"
                                class="w-10 h-10 p-0 border-0 bg-transparent cursor-pointer rounded overflow-hidden" />
                        </div>
                        <div class="flex items-center justify-between">
                            <label class="text-sm font-medium">文字色 (Main)</label>
                            <input type="color" :value="store.theme.textMain"
                                @input="(e: any) => updateThemeColor('textMain', e.target.value)"
                                class="w-10 h-10 p-0 border-0 bg-transparent cursor-pointer rounded overflow-hidden" />
                        </div>
                    </div>
                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <label class="text-sm font-medium">背景色 (Background)</label>
                            <input type="color" :value="store.theme.background"
                                @input="(e: any) => updateThemeColor('background', e.target.value)"
                                class="w-10 h-10 p-0 border-0 bg-transparent cursor-pointer rounded overflow-hidden" />
                        </div>
                        <div class="flex items-center justify-between">
                            <label class="text-sm font-medium">次要文字 (Muted)</label>
                            <input type="color" :value="store.theme.textMuted"
                                @input="(e: any) => updateThemeColor('textMuted', e.target.value)"
                                class="w-10 h-10 p-0 border-0 bg-transparent cursor-pointer rounded overflow-hidden" />
                        </div>
                    </div>
                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <label class="text-sm font-medium">卡片色 (Surface)</label>
                            <input type="color" :value="store.theme.surface"
                                @input="(e: any) => updateThemeColor('surface', e.target.value)"
                                class="w-10 h-10 p-0 border-0 bg-transparent cursor-pointer rounded overflow-hidden" />
                        </div>
                        <div class="flex items-center justify-between">
                            <label class="text-sm font-medium">边框色 (Border)</label>
                            <input type="color" :value="store.theme.borderMain"
                                @input="(e: any) => updateThemeColor('borderMain', e.target.value)"
                                class="w-10 h-10 p-0 border-0 bg-transparent cursor-pointer rounded overflow-hidden" />
                        </div>
                    </div>
                </div>

                <!-- <div class="h-px bg-slate-100 dark:bg-slate-800"></div> -->

                <!-- Dark Mode Toggle -->
                <!-- <div class="flex items-center justify-between">
                    <div>
                        <p class="font-medium text-text-main">夜间模式</p>
                        <p class="text-sm text-text-muted">切换深色或浅色主题基础</p>
                    </div>
                    <button @click="toggleDarkMode" class="w-14 h-7 rounded-full transition-colors relative"
                        :class="store.theme.isDark ? 'bg-primary' : 'bg-slate-300'">
                        <div class="absolute top-1 left-1 w-5 h-5 bg-white rounded-full transition-transform shadow-sm"
                            :class="store.theme.isDark ? 'translate-x-7' : 'translate-x-0'"></div>
                    </button>
                </div> -->
            </div>
        </section>

        <section>
            <h3 class="text-lg font-bold mb-4 flex items-center gap-2">
                <span class="material-symbols-outlined text-primary">info</span>
                关于 AutoKB
            </h3>
            <div class="bg-surface-main rounded-xl border border-border-main p-6">
                <div class="flex items-center gap-4 mb-4">
                    <div class="bg-primary/10 p-3 rounded-2xl text-primary">
                        <span class="material-symbols-outlined text-3xl">auto_fix_high</span>
                    </div>
                    <div>
                        <h4 class="font-bold text-text-main">AutoKB Desktop Automation</h4>
                        <p class="text-sm text-text-muted">Version 1.0.0 (Build 20260224)</p>
                    </div>
                </div>
                <p class="text-sm text-text-muted leading-relaxed mb-4">
                    AutoKB 是一款强大的桌面端自动化工具，支持录制、回放及复杂的任务编排。
                    基于 Tauri 2.0 与 Rust 构建，提供高性能与极低的系统占用。
                </p>
                <div class="flex gap-4">
                    <a href="#" class="text-primary text-sm font-medium hover:underline">检查更新</a>
                    <a href="#" class="text-primary text-sm font-medium hover:underline">开源许可</a>
                    <a href="#" class="text-primary text-sm font-medium hover:underline">文档</a>
                </div>
            </div>
        </section>
    </div>
</template>
