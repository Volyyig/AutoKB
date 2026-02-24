//! Hotkey module - global hotkey state management
//! State only (listener in input_manager)

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::sync::Arc;

/// Global hotkey state
static HOTKEY_STATE: Lazy<Arc<HotkeyState>> = Lazy::new(|| Arc::new(HotkeyState::new()));

/// Hotkey state manager
/// Hotkeys are handled by the low-level listener in `input_manager.rs`
pub struct HotkeyState {
    // Current emergency stop key
    stop_key: Mutex<rdev::Key>,
}

impl HotkeyState {
    pub fn new() -> Self {
        Self {
            stop_key: Mutex::new(rdev::Key::Escape),
        }
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
