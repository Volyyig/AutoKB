//! Hotkey module - global hotkey state management
//! State only (listener moved to input_manager)

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::sync::Arc;

/// Global hotkey state
static HOTKEY_STATE: Lazy<Arc<HotkeyState>> = Lazy::new(|| Arc::new(HotkeyState::new()));

/// Hotkey state manager
pub struct HotkeyState {
    /// Recording hotkey (default: F9)
    recording_key: Mutex<rdev::Key>,
    /// Playback hotkey (default: F10)
    playback_key: Mutex<rdev::Key>,
    /// Stop hotkey (default: Escape)
    stop_key: Mutex<rdev::Key>,
}

impl HotkeyState {
    pub fn new() -> Self {
        Self {
            recording_key: Mutex::new(rdev::Key::F9),
            playback_key: Mutex::new(rdev::Key::F10),
            stop_key: Mutex::new(rdev::Key::Escape),
        }
    }

    pub fn get_recording_key(&self) -> rdev::Key {
        *self.recording_key.lock()
    }

    pub fn get_playback_key(&self) -> rdev::Key {
        *self.playback_key.lock()
    }

    pub fn get_stop_key(&self) -> rdev::Key {
        *self.stop_key.lock()
    }
}

impl Default for HotkeyState {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the global hotkey state
pub fn get_state() -> Arc<HotkeyState> {
    Arc::clone(&HOTKEY_STATE)
}

/// Hotkey event payload for frontend
#[derive(Clone, serde::Serialize)]
pub struct HotkeyEvent {
    pub action: String,
    pub recording: bool,
    pub playing: bool,
}

/// Update hotkey bindings
pub fn set_hotkeys(
    recording: Option<rdev::Key>,
    playback: Option<rdev::Key>,
    stop: Option<rdev::Key>,
) {
    let state = get_state();

    if let Some(key) = recording {
        *state.recording_key.lock() = key;
    }
    if let Some(key) = playback {
        *state.playback_key.lock() = key;
    }
    if let Some(key) = stop {
        *state.stop_key.lock() = key;
    }
}
