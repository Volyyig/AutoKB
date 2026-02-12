/**
 * Pinia store for script state management
 */
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { save, open } from '@tauri-apps/plugin-dialog';
import type { Script, ScriptEvent, MacroDefinition, AppState, HotkeyEvent } from '../types/script';
import { createEmptyScript } from '../types/script';

export const useScriptStore = defineStore('script', () => {
    // State
    const currentScript = ref<Script>(createEmptyScript());
    const isRecording = ref(false);
    const isPlaying = ref(false);
    const isMacroActive = ref(false);
    const macros = ref<MacroDefinition[]>([]);
    const selectedEventIndex = ref<number | null>(null);
    const statusMessage = ref('就绪');

    // Computed
    const eventCount = computed(() => currentScript.value.events.length);
    const totalDuration = computed(() =>
        currentScript.value.events.reduce((sum, e) => sum + e.delay_ms, 0)
    );
    const hasEvents = computed(() => eventCount.value > 0);

    // Actions

    /**
     * Start recording events
     */
    async function startRecording() {
        try {
            await invoke('start_recording');
            isRecording.value = true;
            currentScript.value.events = [];
            statusMessage.value = '录制中... 按 F9 停止';
        } catch (error) {
            statusMessage.value = `录制失败: ${error}`;
        }
    }

    /**
     * Stop recording and get recorded events
     */
    async function stopRecording() {
        try {
            const events = await invoke<ScriptEvent[]>('stop_recording');
            currentScript.value.events = events;
            currentScript.value.modified_at = new Date().toISOString();
            isRecording.value = false;
            statusMessage.value = `录制完成，共 ${events.length} 个事件`;
        } catch (error) {
            statusMessage.value = `停止录制失败: ${error}`;
            isRecording.value = false;
        }
    }

    /**
     * Toggle recording state
     */
    async function toggleRecording() {
        if (isRecording.value) {
            await stopRecording();
        } else {
            await startRecording();
        }
    }

    /**
     * Start playback
     */
    async function startPlayback() {
        if (!hasEvents.value) {
            statusMessage.value = '没有可回放的事件';
            return;
        }

        try {
            await invoke('play_script', { script: currentScript.value });
            isPlaying.value = true;
            statusMessage.value = '回放中... 按 F10 停止';
        } catch (error) {
            statusMessage.value = `回放失败: ${error}`;
        }
    }

    /**
     * Stop playback
     */
    async function stopPlayback() {
        try {
            await invoke('stop_playback');
            isPlaying.value = false;
            statusMessage.value = '回放已停止';
        } catch (error) {
            statusMessage.value = `停止回放失败: ${error}`;
        }
    }

    /**
     * Toggle playback state
     */
    async function togglePlayback() {
        if (isPlaying.value) {
            await stopPlayback();
        } else {
            await startPlayback();
        }
    }

    /**
     * Save script to file
     */
    async function saveScript() {
        try {
            const path = await save({
                defaultPath: `${currentScript.value.name}.autokb`,
                filters: [{ name: 'AutoKB Script', extensions: ['autokb'] }],
            });

            if (path) {
                await invoke('save_script', { script: currentScript.value, path });
                statusMessage.value = `已保存到 ${path}`;
            }
        } catch (error) {
            statusMessage.value = `保存失败: ${error}`;
        }
    }

    /**
     * Load script from file
     */
    async function loadScript() {
        try {
            const path = await open({
                filters: [{ name: 'AutoKB Script', extensions: ['autokb'] }],
                multiple: false,
            });

            if (path) {
                const script = await invoke<Script>('load_script', { path });
                currentScript.value = script;
                statusMessage.value = `已加载 ${script.name}`;
            }
        } catch (error) {
            statusMessage.value = `加载失败: ${error}`;
        }
    }

    /**
     * Update event delay at index
     */
    async function updateEventDelay(index: number, delayMs: number) {
        const events = await invoke<ScriptEvent[]>('update_event_delay', {
            events: currentScript.value.events,
            index,
            delayMs,
        });
        currentScript.value.events = events;
        currentScript.value.modified_at = new Date().toISOString();
    }

    /**
     * Delete event at index
     */
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

    /**
     * Scale all delays
     */
    async function scaleDelays(factor: number) {
        const events = await invoke<ScriptEvent[]>('scale_delays', {
            events: currentScript.value.events,
            factor,
        });
        currentScript.value.events = events;
        currentScript.value.modified_at = new Date().toISOString();
    }

    /**
     * Update loop config
     */
    function updateLoopConfig(count: number, delayBetweenMs: number) {
        currentScript.value.loop_config.count = count;
        currentScript.value.loop_config.delay_between_ms = delayBetweenMs;
        currentScript.value.modified_at = new Date().toISOString();
    }

    /**
     * Update speed multiplier
     */
    function updateSpeed(multiplier: number) {
        currentScript.value.speed_multiplier = multiplier;
        currentScript.value.modified_at = new Date().toISOString();
    }

    /**
     * Load macros from backend
     */
    async function loadMacros() {
        try {
            macros.value = await invoke<MacroDefinition[]>('get_all_macros');
        } catch (error) {
            console.error('Failed to load macros:', error);
        }
    }

    /**
     * Toggle macro listener
     */
    async function toggleMacroListener() {
        try {
            if (isMacroActive.value) {
                await invoke('stop_macro_listener');
                isMacroActive.value = false;
                statusMessage.value = '宏监听已停止';
            } else {
                await invoke('start_macro_listener');
                isMacroActive.value = true;
                statusMessage.value = '宏监听已启动';
            }
        } catch (error) {
            statusMessage.value = `宏监听切换失败: ${error}`;
        }
    }

    /**
     * Create input mapping macro
     */
    async function createInputMapping(
        name: string,
        triggerType: string,
        triggerValue: string,
        actionType: string,
        actionValue: string
    ) {
        try {
            const macro = await invoke<MacroDefinition>('create_input_mapping', {
                name,
                triggerType,
                triggerValue,
                actionType,
                actionValue,
            });
            macros.value.push(macro);
            statusMessage.value = `宏 "${name}" 已创建`;
        } catch (error) {
            statusMessage.value = `创建宏失败: ${error}`;
        }
    }

    /**
     * Remove macro
     */
    async function removeMacro(id: string) {
        try {
            await invoke('remove_macro', { id });
            macros.value = macros.value.filter(m => m.id !== id);
            statusMessage.value = '宏已删除';
        } catch (error) {
            statusMessage.value = `删除宏失败: ${error}`;
        }
    }

    /**
     * Toggle macro enabled state
     */
    async function toggleMacroEnabled(id: string, enabled: boolean) {
        try {
            await invoke('toggle_macro', { id, enabled });
            const macro = macros.value.find(m => m.id === id);
            if (macro) {
                macro.enabled = enabled;
            }
        } catch (error) {
            console.error('Failed to toggle macro:', error);
        }
    }

    /**
     * Clear current script
     */
    function clearScript() {
        currentScript.value = createEmptyScript();
        selectedEventIndex.value = null;
        statusMessage.value = '脚本已清空';
    }

    /**
     * Sync state with backend
     */
    async function syncState() {
        try {
            const state = await invoke<AppState>('get_app_state');
            isRecording.value = state.recording;
            isPlaying.value = state.playing;
            isMacroActive.value = state.macro_active;
        } catch (error) {
            console.error('Failed to sync state:', error);
        }
    }

    /**
     * Initialize store - setup hotkey listener
     */
    async function init() {
        // Listen for hotkey events from backend
        await listen<HotkeyEvent>('hotkey-event', (event) => {
            const payload = event.payload;
            isRecording.value = payload.recording;
            isPlaying.value = payload.playing;

            switch (payload.action) {
                case 'recording-started':
                    currentScript.value.events = [];
                    statusMessage.value = '录制中... 按 F9 停止';
                    break;
                case 'recording-stopped':
                    // Fetch recorded events
                    invoke<ScriptEvent[]>('get_recorded_events').then(events => {
                        currentScript.value.events = events;
                        statusMessage.value = `录制完成，共 ${events.length} 个事件`;
                    });
                    break;
                case 'playback-requested':
                    // Start playback with current script
                    if (hasEvents.value && !isPlaying.value) {
                        startPlayback();
                    }
                    break;
                case 'playback-stopped':
                    statusMessage.value = '回放已停止';
                    break;
                case 'emergency-stop':
                    statusMessage.value = '已紧急停止';
                    break;
            }
        });

        // Sync initial state
        await syncState();
        await loadMacros();
    }

    /**
     * Handle frontend keyboard events (when window is focused)
     */
    async function handleFrontendEvent(e: KeyboardEvent) {
        // Handle shortcuts first
        if (e.key === 'F9' && e.type === 'keydown') {
            await toggleRecording();
            return;
        }
        if (e.key === 'F10' && e.type === 'keydown') {
            await togglePlayback();
            return;
        }

        // Only handle recording events if recording
        if (!isRecording.value) return;

        // Map key to Backend format
        let keyPayload;
        if (e.key.length === 1) {
            keyPayload = { Char: e.key };
        } else {
            // Map common keys to match Rust rdev names
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
            keyPayload = { Special: keyName };
        }

        try {
            await invoke('record_frontend_event', {
                event: {
                    event_type: e.type === 'keydown' ? 'KeyPress' : 'KeyRelease',
                    key: keyPayload,
                    delay_ms: 0 // Backend will override
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
        isMacroActive,
        macros,
        selectedEventIndex,
        statusMessage,
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
        updateEventDelay,
        deleteEvent,
        scaleDelays,
        updateLoopConfig,
        updateSpeed,
        loadMacros,
        toggleMacroListener,
        createInputMapping,
        removeMacro,
        toggleMacroEnabled,
        clearScript,
        syncState,
        init,
        handleFrontendEvent,
    };
});
