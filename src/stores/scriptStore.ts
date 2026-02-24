import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { save, open } from '@tauri-apps/plugin-dialog';
import type { Script, ScriptEvent, Task, AppState, HotkeyEvent, SavedScript } from '../types/script';
import { createEmptyScript } from '../types/script';

export interface Notification {
    id: number;
    message: string;
    type: 'success' | 'error' | 'info';
    duration: number;
}

export const useScriptStore = defineStore('script', () => {
    // State
    const currentScript = ref<Script>(createEmptyScript());
    const isRecording = ref(false);
    const isPlaying = ref(false);
    const isTaskListenerActive = ref(false);
    const tasks = ref<Task[]>([]);
    const savedScripts = ref<SavedScript[]>([]);
    const selectedEventIndex = ref<number | null>(null);
    const statusMessage = ref('就绪');
    const currentView = ref<'home' | 'macro-editor' | 'visual-editor'>('home');
    const activeTab = ref<'tasks' | 'scripts' | 'logs' | 'settings'>('tasks');
    const notifications = ref<Notification[]>([]);
    let nextNotificationId = 0;

    // Appearance State
    const theme = ref({
        primary: localStorage.getItem('theme-primary') || '#135bec',
        background: localStorage.getItem('theme-background') || '#f6f6f8',
        surface: localStorage.getItem('theme-surface') || '#ffffff',
        surfaceSoft: localStorage.getItem('theme-surface-soft') || '#f8fafc',
        textMain: localStorage.getItem('theme-text-main') || '#0f172a',
        textMuted: localStorage.getItem('theme-text-muted') || '#64748b',
        borderMain: localStorage.getItem('theme-border-main') || '#e2e8f0',
        // Status colors - main
        success: localStorage.getItem('theme-success') || '#22c55e',
        error: localStorage.getItem('theme-error') || '#ef4444',
        warning: localStorage.getItem('theme-warning') || '#f59e0b',
        info: localStorage.getItem('theme-info') || '#3b82f6',
        // Status colors - background
        successBg: localStorage.getItem('theme-successBg') || '#f0fdf4',
        errorBg: localStorage.getItem('theme-errorBg') || '#fef2f2',
        warningBg: localStorage.getItem('theme-warningBg') || '#fffbeb',
        infoBg: localStorage.getItem('theme-infoBg') || '#eff6ff',
        // Status colors - border
        successBorder: localStorage.getItem('theme-successBorder') || '#bbf7d0',
        errorBorder: localStorage.getItem('theme-errorBorder') || '#fecaca',
        warningBorder: localStorage.getItem('theme-warningBorder') || '#fde68a',
        infoBorder: localStorage.getItem('theme-infoBorder') || '#bfdbfe',
        isDark: localStorage.getItem('theme-dark') === 'true'
    });

    const themePresets = [
        {
            name: '清新蓝',
            primary: '#135bec', background: '#f6f6f8', surface: '#ffffff',
            surfaceSoft: '#f8fafc', textMain: '#0f172a', textMuted: '#64748b',
            borderMain: '#e2e8f0',
            success: '#22c55e', error: '#ef4444', warning: '#f59e0b', info: '#3b82f6',
            successBg: '#f0fdf4', errorBg: '#fef2f2', warningBg: '#fffbeb', infoBg: '#eff6ff',
            successBorder: '#bbf7d0', errorBorder: '#fecaca', warningBorder: '#fde68a', infoBorder: '#bfdbfe',
            isDark: false
        },
        {
            name: '深邃蓝',
            primary: '#3b82f6', background: '#0f172a', surface: '#1e293b',
            surfaceSoft: '#1e293b80', textMain: '#f8fafc', textMuted: '#94a3b8',
            borderMain: '#1e293b',
            success: '#4ade80', error: '#f87171', warning: '#fbbf24', info: '#60a5fa',
            successBg: '#14532d', errorBg: '#7f1d1d', warningBg: '#78350f', infoBg: '#1e3a8a',
            successBorder: '#166534', errorBorder: '#991b1b', warningBorder: '#92400e', infoBorder: '#1e40af',
            isDark: true
        },
        {
            name: '紫色魅影',
            primary: '#8b5cf6', background: '#020617', surface: '#0f172a',
            surfaceSoft: '#1e293b', textMain: '#f8fafc', textMuted: '#94a3b8',
            borderMain: '#1e293b',
            success: '#4ade80', error: '#f87171', warning: '#fbbf24', info: '#60a5fa',
            successBg: '#14532d', errorBg: '#7f1d1d', warningBg: '#78350f', infoBg: '#1e3a8a',
            successBorder: '#166534', errorBorder: '#991b1b', warningBorder: '#92400e', infoBorder: '#1e40af',
            isDark: true
        },
        {
            name: '极简白',
            primary: '#0f172a', background: '#ffffff', surface: '#ffffff',
            surfaceSoft: '#f1f5f9', textMain: '#0f172a', textMuted: '#64748b',
            borderMain: '#f1f5f9',
            success: '#22c55e', error: '#ef4444', warning: '#f59e0b', info: '#3b82f6',
            successBg: '#f0fdf4', errorBg: '#fef2f2', warningBg: '#fffbeb', infoBg: '#eff6ff',
            successBorder: '#bbf7d0', errorBorder: '#fecaca', warningBorder: '#fde68a', infoBorder: '#bfdbfe',
            isDark: false
        },
    ];

    function applyTheme() {
        const root = document.documentElement;
        root.style.setProperty('--primary', theme.value.primary);
        root.style.setProperty('--background', theme.value.background);
        root.style.setProperty('--surface', theme.value.surface);
        root.style.setProperty('--surface-soft', theme.value.surfaceSoft);
        root.style.setProperty('--text-main', theme.value.textMain);
        root.style.setProperty('--text-muted', theme.value.textMuted);
        root.style.setProperty('--border-main', theme.value.borderMain);

        // Apply status colors - main
        root.style.setProperty('--success', theme.value.success);
        root.style.setProperty('--error', theme.value.error);
        root.style.setProperty('--warning', theme.value.warning);
        root.style.setProperty('--info', theme.value.info);

        // Apply status colors - background
        root.style.setProperty('--success-bg', theme.value.successBg);
        root.style.setProperty('--error-bg', theme.value.errorBg);
        root.style.setProperty('--warning-bg', theme.value.warningBg);
        root.style.setProperty('--info-bg', theme.value.infoBg);

        // Apply status colors - border
        root.style.setProperty('--success-border', theme.value.successBorder);
        root.style.setProperty('--error-border', theme.value.errorBorder);
        root.style.setProperty('--warning-border', theme.value.warningBorder);
        root.style.setProperty('--info-border', theme.value.infoBorder);

        if (theme.value.isDark) {
            root.classList.add('dark');
        } else {
            root.classList.remove('dark');
        }

        // Save to localStorage
        localStorage.setItem('theme-primary', theme.value.primary);
        localStorage.setItem('theme-background', theme.value.background);
        localStorage.setItem('theme-surface', theme.value.surface);
        localStorage.setItem('theme-surface-soft', theme.value.surfaceSoft);
        localStorage.setItem('theme-text-main', theme.value.textMain);
        localStorage.setItem('theme-text-muted', theme.value.textMuted);
        localStorage.setItem('theme-border-main', theme.value.borderMain);
        localStorage.setItem('theme-success', theme.value.success);
        localStorage.setItem('theme-error', theme.value.error);
        localStorage.setItem('theme-warning', theme.value.warning);
        localStorage.setItem('theme-info', theme.value.info);
        localStorage.setItem('theme-successBg', theme.value.successBg);
        localStorage.setItem('theme-errorBg', theme.value.errorBg);
        localStorage.setItem('theme-warningBg', theme.value.warningBg);
        localStorage.setItem('theme-infoBg', theme.value.infoBg);
        localStorage.setItem('theme-successBorder', theme.value.successBorder);
        localStorage.setItem('theme-errorBorder', theme.value.errorBorder);
        localStorage.setItem('theme-warningBorder', theme.value.warningBorder);
        localStorage.setItem('theme-infoBorder', theme.value.infoBorder);
        localStorage.setItem('theme-dark', String(theme.value.isDark));
    }

    function setTheme(newTheme: any) {
        theme.value = { ...newTheme };
        applyTheme();
    }

    // Computed
    const eventCount = computed(() => currentScript.value.events.length);
    const totalDuration = computed(() =>
        currentScript.value.events.reduce((sum, e) => {
            if (e.event_type === 'Delay') {
                return sum + e.duration_ms;
            }
            return sum;
        }, 0)
    );
    const hasEvents = computed(() => eventCount.value > 0);

    // Actions

    function showNotification(message: string, type: 'success' | 'error' | 'info' = 'info', duration = 3000) {
        const id = nextNotificationId++;
        const notification: Notification = { id, message, type, duration };
        notifications.value.push(notification);

        if (duration > 0) {
            setTimeout(() => {
                removeNotification(id);
            }, duration);
        }
    }

    function removeNotification(id: number) {
        const index = notifications.value.findIndex(n => n.id === id);
        if (index !== -1) {
            notifications.value.splice(index, 1);
        }
    }

    async function startRecording() {
        try {
            await invoke('start_recording');
            isRecording.value = true;
            currentScript.value.events = [];
            statusMessage.value = '正在录制...';
        } catch (error) {
            statusMessage.value = `录制失败: ${error}`;
        }
    }

    async function stopRecording() {
        try {
            const events = await invoke<ScriptEvent[]>('stop_recording');
            currentScript.value.events = events;
            currentScript.value.modified_at = new Date().toISOString();
            isRecording.value = false;
            statusMessage.value = `录制完成 (${events.length} 个事件)`;
        } catch (error) {
            statusMessage.value = `停止录制失败: ${error}`;
            isRecording.value = false;
        }
    }

    async function toggleRecording() {
        if (isRecording.value) {
            await stopRecording();
        } else {
            await startRecording();
        }
    }

    async function startPlayback() {
        if (!hasEvents.value) {
            statusMessage.value = '没有可播放的事件';
            return;
        }

        try {
            await invoke('play_script', { script: currentScript.value });
            isPlaying.value = true;
            statusMessage.value = '正在播放...';
        } catch (error) {
            statusMessage.value = `播放失败: ${error}`;
        }
    }

    async function stopPlayback() {
        try {
            await invoke('stop_playback');
            isPlaying.value = false;
            statusMessage.value = '播放已停止';
        } catch (error) {
            statusMessage.value = `停止播放失败: ${error}`;
        }
    }

    async function togglePlayback() {
        if (isPlaying.value) {
            await stopPlayback();
        } else {
            await startPlayback();
        }
    }

    async function saveScript() {
        try {
            const defaultDir = await invoke<string>('get_scripts_dir');
            const path = await save({
                defaultPath: `${defaultDir}/${currentScript.value.name}.autokb`,
                filters: [{ name: 'AutoKB Script', extensions: ['autokb'] }],
            });

            if (path) {
                await invoke('save_script', { script: currentScript.value, path });
                statusMessage.value = `已保存至 ${path}`;
                await listSavedScripts();
            }
        } catch (error) {
            statusMessage.value = `保存失败: ${error}`;
        }
    }

    async function loadScript() {
        try {
            const defaultDir = await invoke<string>('get_scripts_dir');
            const path = await open({
                defaultPath: defaultDir,
                filters: [{ name: 'AutoKB Script', extensions: ['autokb'] }],
                multiple: false,
            });

            if (path) {
                await loadScriptByPath(path as string);
            }
        } catch (error) {
            statusMessage.value = `加载失败: ${error}`;
        }
    }

    async function loadScriptByPath(path: string) {
        try {
            const script = await invoke<Script>('load_script', { path });
            currentScript.value = script;
            statusMessage.value = `已加载 ${script.name}`;
            return script;
        } catch (error) {
            statusMessage.value = `加载失败: ${error}`;
            throw error;
        }
    }

    async function updateEventDelay(index: number, delayMs: number) {
        const events = await invoke<ScriptEvent[]>('update_event_delay', {
            events: currentScript.value.events,
            index,
            delay_ms: delayMs,
        });
        currentScript.value.events = events;
        currentScript.value.modified_at = new Date().toISOString();
    }

    async function deleteEvent(index: number) {
        const events = await invoke<ScriptEvent[]>('delete_event', {
            events: currentScript.value.events,
            index,
        });
        currentScript.value.events = events;
        currentScript.value.modified_at = new Date().toISOString();
        if (selectedEventIndex.value === index) {
            selectedEventIndex.value = null;
        }
    }

    async function scaleDelays(factor: number) {
        const events = await invoke<ScriptEvent[]>('scale_delays', {
            events: currentScript.value.events,
            factor,
        });
        currentScript.value.events = events;
        currentScript.value.modified_at = new Date().toISOString();
    }

    function updateLoopConfig(count: number, delayBetweenMs: number) {
        currentScript.value.loop_config.count = count;
        currentScript.value.loop_config.delay_between_ms = delayBetweenMs;
        currentScript.value.modified_at = new Date().toISOString();
    }

    function updateSpeed(multiplier: number) {
        currentScript.value.speed_multiplier = multiplier;
        currentScript.value.modified_at = new Date().toISOString();
    }

    async function loadTasks() {
        try {
            tasks.value = await invoke<Task[]>('get_all_tasks');
        } catch (error) {
            console.error('Failed to load tasks:', error);
        }
    }

    async function listSavedScripts() {
        try {
            savedScripts.value = await invoke<SavedScript[]>('list_saved_scripts');
        } catch (error) {
            console.error('Failed to list saved scripts:', error);
        }
    }

    async function toggleTaskListener() {
        try {
            if (isTaskListenerActive.value) {
                await invoke('stop_task_listener');
                isTaskListenerActive.value = false;
                statusMessage.value = '任务监听已停止';
            } else {
                await invoke('start_task_listener');
                isTaskListenerActive.value = true;
                statusMessage.value = '任务监听已激活';
            }
        } catch (error) {
            statusMessage.value = `任务监听切换失败: ${error}`;
        }
    }

    async function createTaskBinding(
        name: string,
        triggerKey?: string,
        stopKey?: string,
        scriptPath?: string
    ) {
        try {
            const task = await invoke<Task>('create_task_binding', {
                name,
                triggerKey,
                stopKey,
                scriptPath,
            });
            tasks.value.push(task);
            statusMessage.value = `任务 "${name}" 已创建`;
        } catch (error) {
            statusMessage.value = `创建任务失败: ${error}`;
        }
    }

    async function removeTask(id: string) {
        try {
            await invoke('remove_task', { id });
            tasks.value = tasks.value.filter(t => t.id !== id);
            statusMessage.value = '任务已删除';
        } catch (error) {
            statusMessage.value = `删除任务失败: ${error}`;
        }
    }

    async function toggleTaskEnabled(id: string, enabled: boolean) {
        try {
            await invoke('toggle_task', { id, enabled });
            const task = tasks.value.find(t => t.id === id);
            if (task) {
                task.enabled = enabled;
            }
        } catch (error) {
            console.error('Failed to toggle task:', error);
        }
    }

    function clearScript() {
        currentScript.value = createEmptyScript();
        selectedEventIndex.value = null;
        statusMessage.value = '脚本已清空';
    }

    function createNewDraftScript(): Script {
        const script = createEmptyScript();
        script.name = `新建脚本_${new Date().toLocaleTimeString().replace(/:/g, '-')}`;
        return script;
    }

    async function syncState() {
        try {
            const state = await invoke<AppState>('get_app_state');
            isRecording.value = state.recording;
            isPlaying.value = state.playing;
            // Note: backend field name changed in get_app_state 
            isTaskListenerActive.value = (state as any).task_listener_active;
        } catch (error) {
            console.error('Failed to sync state:', error);
        }
    }

    async function init() {
        await listen<HotkeyEvent>('hotkey-event', (event) => {
            const payload = event.payload;
            isRecording.value = payload.recording;
            isPlaying.value = payload.playing;

            switch (payload.action) {
                case 'emergency-stop':
                    statusMessage.value = '紧急停止';
                    break;
                case 'playback-stopped':
                    statusMessage.value = '播放已停止';
                    break;
            }
        });

        await syncState();
        await loadTasks();
        await listSavedScripts();
        applyTheme();
    }

    async function handleFrontendEvent(e: KeyboardEvent) {
        if (!isRecording.value) return;

        let keyPayload;
        if (e.key.length === 1) {
            keyPayload = { type: 'Char', value: e.key };
        } else {
            let keyName = e.key;
            const keyMap: Record<string, string> = {
                ' ': 'Space',
                'Control': 'ControlLeft',
                'Shift': 'ShiftLeft',
                'Alt': 'Alt',
                'Meta': 'MetaLeft',
                'ArrowUp': 'UpArrow',
                'ArrowDown': 'DownArrow',
                'ArrowLeft': 'LeftArrow',
                'ArrowRight': 'RightArrow',
                'Enter': 'Return'
            };
            if (keyMap[keyName]) {
                keyName = keyMap[keyName];
            }
            keyPayload = { type: 'Special', value: keyName };
        }

        try {
            await invoke('record_frontend_event', {
                event: {
                    event_type: e.type === 'keydown' ? 'KeyPress' : 'KeyRelease',
                    key: keyPayload
                }
            });
        } catch (error) {
            console.error('Failed to record frontend event:', error);
        }
    }

    return {
        // State
        currentScript,
        isRecording,
        isPlaying,
        isTaskListenerActive,
        tasks,
        savedScripts,
        selectedEventIndex,
        statusMessage,
        currentView,
        activeTab,
        // Computed
        eventCount,
        totalDuration,
        hasEvents,
        // Actions
        startRecording,
        stopRecording,
        toggleRecording,
        startPlayback,
        stopPlayback,
        togglePlayback,
        saveScript,
        loadScript,
        loadScriptByPath,
        updateEventDelay,
        deleteEvent,
        scaleDelays,
        updateLoopConfig,
        updateSpeed,
        loadTasks,
        listSavedScripts,
        toggleTaskListener,
        createTaskBinding,
        removeTask,
        toggleTaskEnabled,
        clearScript,
        syncState,
        init,
        handleFrontendEvent,
        createNewDraftScript,
        notifications,
        showNotification,
        removeNotification,
        theme,
        setTheme,
        themePresets,
        applyTheme
    };
});
