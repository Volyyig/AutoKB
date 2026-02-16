/**
 * Pinia store for script state management
 */
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { save, open } from '@tauri-apps/plugin-dialog';
import type { Script, ScriptEvent, MacroDefinition, AppState, HotkeyEvent, SavedScript } from '../types/script';
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
    const isMacroActive = ref(false);
    const macros = ref<MacroDefinition[]>([]);
    const savedScripts = ref<SavedScript[]>([]);
    const selectedEventIndex = ref<number | null>(null);
    const statusMessage = ref('Ready');
    const currentView = ref<'home' | 'macro-editor' | 'visual-editor'>('home');
    const notifications = ref<Notification[]>([]);
    let nextNotificationId = 0;

    // Computed
    const eventCount = computed(() => currentScript.value.events.length);
    const totalDuration = computed(() =>
        currentScript.value.events.reduce((sum, e) => sum + e.delay_ms, 0)
    );
    const hasEvents = computed(() => eventCount.value > 0);

    // Actions

    /**
     * Show a notification toast
     */
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

    /**
     * Remove a notification by ID
     */
    function removeNotification(id: number) {
        const index = notifications.value.findIndex(n => n.id === id);
        if (index !== -1) {
            notifications.value.splice(index, 1);
        }
    }

    /**
     * Start recording events
     */
    async function startRecording() {
        try {
            await invoke('start_recording');
            isRecording.value = true;
            currentScript.value.events = [];
            statusMessage.value = 'Recording... Press F9 to Stop';
        } catch (error) {
            statusMessage.value = `Recording Failed: ${error}`;
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
            statusMessage.value = `Recording Complete (${events.length} events)`;
        } catch (error) {
            statusMessage.value = `Stop Recording Failed: ${error}`;
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
            statusMessage.value = 'No events to play';
            return;
        }

        try {
            await invoke('play_script', { script: currentScript.value });
            isPlaying.value = true;
            statusMessage.value = 'Playing... Press F10 to Stop';
        } catch (error) {
            statusMessage.value = `Playback Failed: ${error}`;
        }
    }

    /**
     * Stop playback
     */
    async function stopPlayback() {
        try {
            await invoke('stop_playback');
            isPlaying.value = false;
            statusMessage.value = 'Playback Stopped';
        } catch (error) {
            statusMessage.value = `Stop Playback Failed: ${error}`;
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
            const defaultDir = await invoke<string>('get_scripts_dir');
            const path = await save({
                defaultPath: `${defaultDir}/${currentScript.value.name}.autokb`,
                filters: [{ name: 'AutoKB Script', extensions: ['autokb'] }],
            });

            if (path) {
                await invoke('save_script', { script: currentScript.value, path });
                statusMessage.value = `Saved to ${path}`;
                await listSavedScripts(); // Refresh saved scripts list
            }
        } catch (error) {
            statusMessage.value = `Save Failed: ${error}`;
        }
    }

    /**
     * Load script from file
     */
    async function loadScript() {
        try {
            const defaultDir = await invoke<string>('get_scripts_dir');
            const path = await open({
                defaultPath: defaultDir,
                filters: [{ name: 'AutoKB Script', extensions: ['autokb'] }],
                multiple: false,
            });

            if (path) {
                const script = await invoke<Script>('load_script', { path });
                currentScript.value = script;
                statusMessage.value = `Loaded ${script.name}`;
            }
        } catch (error) {
            statusMessage.value = `Load Failed: ${error}`;
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
     * List saved scripts
     */
    async function listSavedScripts() {
        try {
            savedScripts.value = await invoke<SavedScript[]>('list_saved_scripts');
        } catch (error) {
            console.error('Failed to list saved scripts:', error);
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
                statusMessage.value = 'Macro Listener Stopped';
            } else {
                await invoke('start_macro_listener');
                isMacroActive.value = true;
                statusMessage.value = 'Macro Listener Active';
            }
        } catch (error) {
            statusMessage.value = `Macro Toggle Failed: ${error}`;
        }
    }

    /**
     * Create macro binding
     */
    async function createMacroBinding(
        name: String,
        triggerType: String,
        triggerValue: String,
        scriptPath: String
    ) {
        try {
            const macro = await invoke<MacroDefinition>('create_macro_binding', {
                name,
                triggerType,
                triggerValue,
                scriptPath,
            });
            macros.value.push(macro);
            statusMessage.value = `Macro "${name}" Created`;
        } catch (error) {
            statusMessage.value = `Create Macro Failed: ${error}`;
        }
    }

    /**
     * Remove macro
     */
    async function removeMacro(id: string) {
        try {
            await invoke('remove_macro', { id });
            macros.value = macros.value.filter(m => m.id !== id);
            statusMessage.value = 'Macro Deleted';
        } catch (error) {
            statusMessage.value = `Delete Macro Failed: ${error}`;
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
        statusMessage.value = 'Script Cleared';
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
                    statusMessage.value = 'Recording... Press F9 to Stop';
                    break;
                case 'recording-stopped':
                    // Fetch recorded events
                    invoke<ScriptEvent[]>('get_recorded_events').then(events => {
                        currentScript.value.events = events;
                        statusMessage.value = `Recording Complete (${events.length} events)`;
                    });
                    break;
                case 'playback-requested':
                    // Start playback with current script
                    if (hasEvents.value && !isPlaying.value) {
                        startPlayback();
                    }
                    break;
                case 'playback-stopped':
                    statusMessage.value = 'Playback Stopped';
                    break;
                case 'emergency-stop':
                    statusMessage.value = 'Emergency Stop';
                    break;
            }
        });

        // Sync initial state
        await syncState();
        await loadMacros();
        await listSavedScripts();
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
        savedScripts,
        selectedEventIndex,
        statusMessage,
        currentView,
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
        listSavedScripts,
        toggleMacroListener,
        createMacroBinding,
        removeMacro,
        toggleMacroEnabled,
        clearScript,
        syncState,
        init,
        handleFrontendEvent,
        // Notifications
        notifications,
        showNotification,
        removeNotification
    };
});
