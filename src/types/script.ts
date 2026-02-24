/**
 * TypeScript type definitions for AutoKB
 * Mirrors the Rust data structures
 */

// Mouse button types
export type MouseButton = 'left' | 'right' | 'middle' | 'back' | 'forward';

// Keyboard key representation
export type KeyboardKey =
    | { type: 'Char'; value: string }
    | { type: 'Special'; value: string };

// Single input event types
export type ScriptEvent =
    | {
        event_type: 'Delay';
        duration_ms: number;
    }
    | {
        event_type: 'KeyPress';
        key: KeyboardKey;
    }
    | {
        event_type: 'KeyRelease';
        key: KeyboardKey;
    }
    | {
        event_type: 'MousePress';
        button: MouseButton;
        x: number;
        y: number;
    }
    | {
        event_type: 'MouseRelease';
        button: MouseButton;
        x: number;
        y: number;
    }
    | {
        event_type: 'MouseMove';
        x: number;
        y: number;
    }
    | {
        event_type: 'MouseScroll';
        delta_x: number;
        delta_y: number;
    };

// A task definition - trigger + action
export interface Task {
    id: string;
    name: string;
    description: string;
    trigger_key?: KeyboardKey;
    stop_key?: KeyboardKey;
    script_path: string;
    enabled: boolean;
    loop_config: LoopConfig;
    speed_multiplier: number;
}

// Saved script info
export interface SavedScript {
    name: string;
    path: string;
    description: string;
    modified_at: string;
}

// Loop configuration
export interface LoopConfig {
    count: number;
    delay_between_ms: number;
}

// Complete script
export interface Script {
    name: string;
    description: string;
    created_at: string;
    modified_at: string;
    events: ScriptEvent[];
    loop_config: LoopConfig;
    speed_multiplier: number;
}

// App state
export interface AppState {
    recording: boolean;
    playing: boolean;
    task_listener_active: boolean;
}

// Hotkey event payload
export interface HotkeyEvent {
    action: 'recording-started' | 'recording-stopped' | 'playback-requested' | 'playback-stopped' | 'emergency-stop';
    recording: boolean;
    playing: boolean;
}

// Helper functions
export function getEventDelay(event: ScriptEvent): number {
    if (event.event_type === 'Delay') {
        return event.duration_ms;
    }
    return 0;
}

export function getKeyDisplay(key: KeyboardKey): string {
    if (key.type === 'Char') {
        return key.value.toUpperCase();
    }
    return key.value;
}

export function getEventDescription(event: ScriptEvent): string {
    const translateButton = (btn: MouseButton) => {
        const map: Record<MouseButton, string> = {
            left: '左键',
            right: '右键',
            middle: '中键',
            back: '后退键',
            forward: '前进键'
        };
        return map[btn] || btn;
    };

    switch (event.event_type) {
        case 'Delay':
            return `等待: ${formatDuration(event.duration_ms)}`;
        case 'KeyPress':
            return `键盘按下 (${getKeyDisplay(event.key)})`;
        case 'KeyRelease':
            return `键盘弹起 (${getKeyDisplay(event.key)})`;
        case 'MousePress':
            return `鼠标按下 (${translateButton(event.button)})`;
        case 'MouseRelease':
            return `鼠标弹起 (${translateButton(event.button)})`;
        case 'MouseMove':
            return `鼠标移动 (${Math.round(event.x)}, ${Math.round(event.y)})`;
        case 'MouseScroll':
            return `鼠标滚动 (${event.delta_x}, ${event.delta_y})`;
        default:
            return '未知事件';
    }
}

export function formatDuration(ms: number): string {
    if (ms < 1000) {
        return `${ms}ms`;
    }
    return `${(ms / 1000).toFixed(2)}s`;
}

export function createEmptyScript(): Script {
    return {
        name: '未命名脚本',
        description: '',
        created_at: new Date().toISOString(),
        modified_at: new Date().toISOString(),
        events: [],
        loop_config: {
            count: 1,
            delay_between_ms: 0,
        },
        speed_multiplier: 1.0,
    };
}
