//! Unified input manager - single event loop for recorder, hotkeys, and macros
//! Replaces individual listeners to avoid conflicts and improve performance

use crate::macro_trigger;
use crate::player;
use crate::recorder;
use crate::script::{KeyboardKey, MouseButton, ScriptEvent};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rdev::{Event, EventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use tauri::{AppHandle, Emitter, Manager};

/// Global input manager state
static INPUT_MANAGER: Lazy<Arc<InputManager>> = Lazy::new(|| Arc::new(InputManager::new()));

pub struct InputManager {
    is_running: AtomicBool,
    app_handle: Mutex<Option<AppHandle>>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            is_running: AtomicBool::new(false),
            app_handle: Mutex::new(None),
        }
    }

    pub fn set_app_handle(&self, handle: AppHandle) {
        *self.app_handle.lock() = Some(handle);
    }

    pub fn emit_event(&self, event_name: &str, payload: impl serde::Serialize + Clone) {
        if let Some(handle) = self.app_handle.lock().as_ref() {
            let _ = handle.emit(event_name, payload);
        }
    }
}

pub fn init(app_handle: AppHandle) {
    let manager = &INPUT_MANAGER;
    manager.set_app_handle(app_handle);

    if manager.is_running.swap(true, Ordering::SeqCst) {
        return;
    }

    thread::spawn(move || {
        let manager = &INPUT_MANAGER;

        if let Err(error) = rdev::listen(move |event| {
            // println!("{:?}", event); // Debug print removed
            handle_event(event, manager);
        }) {
            eprintln!("Input listener error: {:?}", error);
        }
    });
}

/// Helper to show overlay with specific color
pub fn show_overlay(app: &AppHandle, color: &str) {
    if let Some(window) = app.get_webview_window("overlay") {
        let _ = window.show();

        let script = format!("document.body.style.borderColor = '{}';", color);
        let _ = window.eval(&script);
    }
}

/// Helper to hide overlay
pub fn hide_overlay(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("overlay") {
        let _ = window.hide();
    }
}

/// Emit an event to the frontend
pub fn emit_event(event_name: &str, payload: impl serde::Serialize + Clone) {
    INPUT_MANAGER.emit_event(event_name, payload);
}

/// Called by player when playback finishes naturally
pub fn on_playback_finish() {
    let manager = &INPUT_MANAGER;
    if let Some(handle) = manager.app_handle.lock().as_ref() {
        if let Some(window) = handle.get_webview_window("main") {
            let _ = window.show();
            // window.set_focus(); // REMOVED: Do not steal focus during recording/playback
        }
        hide_overlay(handle);
    }
}

fn handle_event(event: Event, _manager: &InputManager) {
    // 1. Hotkeys are now handled by tauri-plugin-global-shortcut in lib.rs
    // This removes hotkey-related keys from this low-level listener to avoid conflicts
    let hotkey_state = crate::hotkey::get_state();
    if let EventType::KeyPress(key) | EventType::KeyRelease(key) = event.event_type {
        if hotkey_state.get_all_keys().contains(&key) {
            return;
        }
    }

    // 2. Playback Protection
    if player::is_playing() {
        return;
    }

    // 3. Handle Recording
    if recorder::is_recording() {
        let elapsed = recorder::get_state().get_elapsed_ms();
        match event.event_type {
            EventType::KeyPress(key) => {
                recorder::get_state().commit_event(ScriptEvent::KeyPress {
                    key: KeyboardKey::from(key),
                    delay_ms: elapsed,
                });
            }
            EventType::KeyRelease(key) => {
                recorder::get_state().commit_event(ScriptEvent::KeyRelease {
                    key: KeyboardKey::from(key),
                    delay_ms: elapsed,
                });
            }
            EventType::ButtonPress(button) => {
                let (x, y) = recorder::get_state().get_mouse_position();
                recorder::get_state().commit_event(ScriptEvent::MousePress {
                    button: MouseButton::from(button),
                    x,
                    y,
                    delay_ms: elapsed,
                });
            }
            EventType::ButtonRelease(button) => {
                let (x, y) = recorder::get_state().get_mouse_position();
                recorder::get_state().commit_event(ScriptEvent::MouseRelease {
                    button: MouseButton::from(button),
                    x,
                    y,
                    delay_ms: elapsed,
                });
            }
            EventType::MouseMove { x, y } => {
                recorder::get_state().update_mouse_position(x, y);
                // Throttle mouse move recording: ONLY record if time >= 20ms
                if elapsed >= 20 {
                    recorder::get_state().commit_event(ScriptEvent::MouseMove {
                        x,
                        y,
                        delay_ms: elapsed,
                    });
                }
            }
            EventType::Wheel { delta_x, delta_y } => {
                recorder::get_state().commit_event(ScriptEvent::MouseScroll {
                    delta_x,
                    delta_y,
                    delay_ms: elapsed,
                });
            }
        }
    }

    // 4. Handle Macros
    if macro_trigger::get_state().is_active() && !recorder::is_recording() {
        match event.event_type {
            EventType::KeyPress(key) => {
                let trigger = crate::script::MacroTrigger::KeyPress {
                    key: KeyboardKey::from(key),
                };
                macro_trigger::get_state().check_and_execute(&trigger);
            }
            EventType::ButtonPress(button) => {
                let trigger = crate::script::MacroTrigger::MousePress {
                    button: MouseButton::from(button),
                };
                macro_trigger::get_state().check_and_execute(&trigger);
            }
            _ => {}
        }
    }
}
