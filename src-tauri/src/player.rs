//! Playback module - simulates keyboard and mouse events
//! Uses enigo for input simulation

use crate::script::{KeyboardKey, Script, ScriptEvent};
use enigo::{Enigo, Keyboard, Mouse, Settings};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Global playback state
static PLAYBACK_STATE: Lazy<Arc<PlaybackState>> = Lazy::new(|| Arc::new(PlaybackState::new()));

/// Playback state manager
pub struct PlaybackState {
    /// Whether playback is active
    is_playing: AtomicBool,
    /// Current loop iteration
    current_loop: Mutex<u32>,
    /// Current event index
    current_event: Mutex<usize>,
    /// Stop requested flag
    stop_requested: AtomicBool,
}

impl PlaybackState {
    pub fn new() -> Self {
        Self {
            is_playing: AtomicBool::new(false),
            current_loop: Mutex::new(0),
            current_event: Mutex::new(0),
            stop_requested: AtomicBool::new(false),
        }
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::SeqCst)
    }

    pub fn start(&self) {
        *self.current_loop.lock() = 0;
        *self.current_event.lock() = 0;
        self.stop_requested.store(false, Ordering::SeqCst);
        self.is_playing.store(true, Ordering::SeqCst);
    }

    pub fn stop(&self) {
        self.stop_requested.store(true, Ordering::SeqCst);
        self.is_playing.store(false, Ordering::SeqCst);
    }

    pub fn should_stop(&self) -> bool {
        self.stop_requested.load(Ordering::SeqCst)
    }

    pub fn increment_loop(&self) -> u32 {
        let mut loop_count = self.current_loop.lock();
        *loop_count += 1;
        *loop_count
    }

    pub fn set_event_index(&self, index: usize) {
        *self.current_event.lock() = index;
    }

    pub fn finish(&self) {
        self.is_playing.store(false, Ordering::SeqCst);

        // Cleanup UI via input_manager
        crate::input_manager::on_playback_finish();

        // Emit event to frontend
        crate::input_manager::emit_event(
            "hotkey-event",
            crate::hotkey::HotkeyEvent {
                action: "playback-stopped".to_string(),
                recording: false,
                playing: false,
            },
        );
    }
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the global playback state
pub fn get_state() -> Arc<PlaybackState> {
    Arc::clone(&PLAYBACK_STATE)
}

/// Convert KeyboardKey to enigo Key
fn keyboard_key_to_enigo(key: &KeyboardKey) -> Option<enigo::Key> {
    match key {
        KeyboardKey::Char(c) => Some(enigo::Key::Unicode(*c)),
        KeyboardKey::Special(s) => match s.as_str() {
            "Alt" => Some(enigo::Key::Alt),
            "Backspace" => Some(enigo::Key::Backspace),
            "CapsLock" => Some(enigo::Key::CapsLock),
            "ControlLeft" | "ControlRight" => Some(enigo::Key::Control),
            "Delete" => Some(enigo::Key::Delete),
            "DownArrow" => Some(enigo::Key::DownArrow),
            "End" => Some(enigo::Key::End),
            "Escape" => Some(enigo::Key::Escape),
            "F1" => Some(enigo::Key::F1),
            "F2" => Some(enigo::Key::F2),
            "F3" => Some(enigo::Key::F3),
            "F4" => Some(enigo::Key::F4),
            "F5" => Some(enigo::Key::F5),
            "F6" => Some(enigo::Key::F6),
            "F7" => Some(enigo::Key::F7),
            "F8" => Some(enigo::Key::F8),
            "F9" => Some(enigo::Key::F9),
            "F10" => Some(enigo::Key::F10),
            "F11" => Some(enigo::Key::F11),
            "F12" => Some(enigo::Key::F12),
            "Home" => Some(enigo::Key::Home),
            "LeftArrow" => Some(enigo::Key::LeftArrow),
            "MetaLeft" | "MetaRight" => Some(enigo::Key::Meta),
            "PageDown" => Some(enigo::Key::PageDown),
            "PageUp" => Some(enigo::Key::PageUp),
            "Return" => Some(enigo::Key::Return),
            "RightArrow" => Some(enigo::Key::RightArrow),
            "ShiftLeft" | "ShiftRight" => Some(enigo::Key::Shift),
            "Space" => Some(enigo::Key::Space),
            "Tab" => Some(enigo::Key::Tab),
            "UpArrow" => Some(enigo::Key::UpArrow),
            _ => None,
        },
    }
}

/// Execute a single event
fn execute_event(
    enigo: &mut Enigo,
    event: &ScriptEvent,
    speed_multiplier: f64,
    use_recorded_position: bool,
) -> Result<(), String> {
    // Calculate adjusted delay
    let delay_ms = (event.delay_ms() as f64 / speed_multiplier) as u64;

    // Wait for the delay (interruptible)
    if delay_ms > 0 {
        let chunk_ms = 100; // Check stop every 100ms
        let mut remaining = delay_ms;

        while remaining > 0 {
            if get_state().should_stop() {
                return Err("Playback stopped".to_string());
            }

            let sleep_time = if remaining > chunk_ms {
                chunk_ms
            } else {
                remaining
            };
            thread::sleep(Duration::from_millis(sleep_time));
            remaining -= sleep_time;
        }
    }

    // Check if we should stop
    if get_state().should_stop() {
        return Err("Playback stopped".to_string());
    }

    match event {
        ScriptEvent::KeyPress { key, .. } => {
            if let Some(enigo_key) = keyboard_key_to_enigo(key) {
                enigo
                    .key(enigo_key, enigo::Direction::Press)
                    .map_err(|e| format!("Key press error: {:?}", e))?;
            }
        }
        ScriptEvent::KeyRelease { key, .. } => {
            if let Some(enigo_key) = keyboard_key_to_enigo(key) {
                enigo
                    .key(enigo_key, enigo::Direction::Release)
                    .map_err(|e| format!("Key release error: {:?}", e))?;
            }
        }
        ScriptEvent::MousePress { button, x, y, .. } => {
            if use_recorded_position {
                // Move to position first
                enigo
                    .move_mouse(*x as i32, *y as i32, enigo::Coordinate::Abs)
                    .map_err(|e| format!("Mouse move error: {:?}", e))?;
            }
            // Then press
            enigo
                .button((*button).into(), enigo::Direction::Press)
                .map_err(|e| format!("Mouse press error: {:?}", e))?;
        }
        ScriptEvent::MouseRelease { button, x, y, .. } => {
            if use_recorded_position {
                enigo
                    .move_mouse(*x as i32, *y as i32, enigo::Coordinate::Abs)
                    .map_err(|e| format!("Mouse move error: {:?}", e))?;
            }
            enigo
                .button((*button).into(), enigo::Direction::Release)
                .map_err(|e| format!("Mouse release error: {:?}", e))?;
        }
        ScriptEvent::MouseMove { x, y, .. } => {
            enigo
                .move_mouse(*x as i32, *y as i32, enigo::Coordinate::Abs)
                .map_err(|e| format!("Mouse move error: {:?}", e))?;
        }
        ScriptEvent::MouseScroll {
            delta_x, delta_y, ..
        } => {
            if *delta_y != 0 {
                enigo
                    .scroll(-*delta_y as i32, enigo::Axis::Vertical)
                    .map_err(|e| format!("Scroll error: {:?}", e))?;
            }
            if *delta_x != 0 {
                enigo
                    .scroll(-*delta_x as i32, enigo::Axis::Horizontal)
                    .map_err(|e| format!("Scroll error: {:?}", e))?;
            }
        }
    }

    Ok(())
}

/// Play a script
pub fn play_script(script: Script) -> Result<(), String> {
    let state = get_state();

    if state.is_playing() {
        return Err("Already playing".to_string());
    }

    if script.events.is_empty() {
        return Err("Script has no events".to_string());
    }

    state.start();

    thread::spawn(move || {
        let state = get_state();
        let settings = Settings::default();
        let mut enigo = match Enigo::new(&settings) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Failed to create Enigo: {:?}", e);
                state.finish();
                return;
            }
        };

        let loop_count = script.loop_config.count;
        let is_infinite = loop_count == 0;

        // Check if script has any mouse move events
        // If no mouse moves are present, we use the current mouse position for clicks
        // instead of the recorded coordinates (which might be 0,0)
        let has_mouse_moves = script
            .events
            .iter()
            .any(|e| matches!(e, ScriptEvent::MouseMove { .. }));

        loop {
            let current_iteration = state.increment_loop();

            // Check if we should stop (loop count reached or stop requested)
            if !is_infinite && current_iteration > loop_count {
                break;
            }

            if state.should_stop() {
                break;
            }

            // Execute all events
            for (index, event) in script.events.iter().enumerate() {
                state.set_event_index(index);

                if let Err(e) =
                    execute_event(&mut enigo, event, script.speed_multiplier, has_mouse_moves)
                {
                    eprintln!("Playback error: {}", e);
                    state.finish();
                    return;
                }

                if state.should_stop() {
                    break;
                }
            }

            // Delay between loops
            if script.loop_config.delay_between_ms > 0 && !state.should_stop() {
                thread::sleep(Duration::from_millis(script.loop_config.delay_between_ms));
            }
        }

        state.finish();
    });

    Ok(())
}

/// Play a list of events (without Script wrapper)
pub fn play_events(events: Vec<ScriptEvent>, speed_multiplier: f64) -> Result<(), String> {
    let script = Script {
        events,
        speed_multiplier,
        ..Default::default()
    };
    play_script(script)
}

/// Stop playback
pub fn stop_playback() {
    get_state().stop();
}

/// Check if currently playing
pub fn is_playing() -> bool {
    get_state().is_playing()
}
