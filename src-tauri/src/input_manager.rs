//! Unified input manager - single event loop for recorder, hotkeys, and macros
//! Replaces individual listeners to avoid conflicts and improve performance

use crate::macro_trigger;
use crate::player;
use crate::recorder;
use crate::script::{KeyboardKey, MouseButton, ScriptEvent};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rdev::{Event, EventType, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use tauri::{AppHandle, Emitter};

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
            println!("{:?}", event); // Debug print removed
            handle_event(event, manager);
        }) {
            eprintln!("Input listener error: {:?}", error);
        }
    });
}

/// Emit an event to the frontend
pub fn emit_event(event_name: &str, payload: impl serde::Serialize + Clone) {
    INPUT_MANAGER.emit_event(event_name, payload);
}

fn handle_event(event: Event, manager: &InputManager) {
    // 1. Check Hotkeys first
    if let EventType::KeyPress(key) = event.event_type {
        match key {
            Key::F9 => {
                // Toggle Recording
                if recorder::is_recording() {
                    let _ = recorder::stop_recording();
                    manager.emit_event(
                        "hotkey-event",
                        crate::hotkey::HotkeyEvent {
                            action: "recording-stopped".to_string(),
                            recording: false,
                            playing: player::is_playing(),
                        },
                    );
                } else if !player::is_playing() {
                    let _ = recorder::start_recording();
                    manager.emit_event(
                        "hotkey-event",
                        crate::hotkey::HotkeyEvent {
                            action: "recording-started".to_string(),
                            recording: true,
                            playing: false,
                        },
                    );
                }
                return; // Don't process hotkey further
            }
            Key::F10 => {
                // Toggle Playback
                if player::is_playing() {
                    player::stop_playback();
                    manager.emit_event(
                        "hotkey-event",
                        crate::hotkey::HotkeyEvent {
                            action: "playback-stopped".to_string(),
                            recording: recorder::is_recording(),
                            playing: false,
                        },
                    );
                } else {
                    manager.emit_event(
                        "hotkey-event",
                        crate::hotkey::HotkeyEvent {
                            action: "playback-requested".to_string(),
                            recording: recorder::is_recording(),
                            playing: false,
                        },
                    );
                }
                return;
            }
            Key::Escape => {
                // Emergency Stop
                let was_recording = recorder::is_recording();
                let was_playing = player::is_playing();

                if was_recording {
                    let _ = recorder::stop_recording();
                }
                if was_playing {
                    player::stop_playback();
                }

                if was_recording || was_playing {
                    manager.emit_event(
                        "hotkey-event",
                        crate::hotkey::HotkeyEvent {
                            action: "emergency-stop".to_string(),
                            recording: false,
                            playing: false,
                        },
                    );
                }
                return;
            }
            _ => {}
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
