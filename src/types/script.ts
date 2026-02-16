/**
 * TypeScript type definitions for AutoKB
 * Mirrors the Rust data structures
 */

// Mouse button types
export type MouseButton = 'left' | 'right' | 'middle' | 'unknown';

// Keyboard key representation
export type KeyboardKey =
    | { type: 'Char'; value: string }
    | { type: 'Special'; value: string };

// Single input event types
export type ScriptEvent =
    | {
        event_type: 'KeyPress';
        key: KeyboardKey;
        delay_ms: number;
    }
    | {
        event_type: 'KeyRelease';
        key: KeyboardKey;
        delay_ms: number;
    }
    | {
        event_type: 'MousePress';
        button: MouseButton;
        x: number;
        y: number;
        delay_ms: number;
    }
    | {
        event_type: 'MouseRelease';
        button: MouseButton;
        x: number;
        y: number;
        delay_ms: number;
    }
    | {
        event_type: 'MouseMove';
        x: number;
        y: number;
        delay_ms: number;
    }
    | {
        event_type: 'MouseScroll';
        delta_x: number;
        delta_y: number;
        delay_ms: number;
    };

// Macro trigger
export type MacroTrigger =
    | { trigger_type: 'KeyPress'; key: KeyboardKey }
    | { trigger_type: 'MousePress'; button: MouseButton };

// Macro definition
export interface MacroDefinition {
    id: string;
    name: string;
    trigger: MacroTrigger;
    script_path: string;
    enabled: boolean;
}

// Saved script info
export interface SavedScript {
    name: string;
    path: string;
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
    macro_active: boolean;
}

// Hotkey event payload
export interface HotkeyEvent {
    action: 'recording-started' | 'recording-stopped' | 'playback-requested' | 'playback-stopped' | 'emergency-stop';
    recording: boolean;
    playing: boolean;
}

// Helper functions
export function getEventDelay(event: ScriptEvent): number {
    return event.delay_ms;
}

export function getKeyDisplay(key: KeyboardKey): string {
    if (key.type === 'Char') {
        return key.value.toUpperCase();
    }
    return key.value;
}

export function getEventDescription(event: ScriptEvent): string {
    switch (event.event_type) {
        case 'KeyPress':
            return `Key Down: ${getKeyDisplay(event.key)}`;
        case 'KeyRelease':
            return `Key Up: ${getKeyDisplay(event.key)}`;
        case 'MousePress':
            return `Mouse Down: ${event.button}`;
        case 'MouseRelease':
            return `Mouse Up: ${event.button}`;
        case 'MouseMove':
            return `Move: (${Math.round(event.x)}, ${Math.round(event.y)})`;
        case 'MouseScroll':
            return `Scroll: (${event.delta_x}, ${event.delta_y})`;
        default:
            return 'Unknown Event';
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
        name: 'Untitled Script',
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
