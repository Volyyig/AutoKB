//! Recording module - captures keyboard and mouse events
//! State management only (event loop moved to input_manager)

use crate::script::ScriptEvent;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Global recording state
static RECORDING_STATE: Lazy<Arc<RecordingState>> = Lazy::new(|| Arc::new(RecordingState::new()));

/// Recording state manager
pub struct RecordingState {
    /// Whether recording is active
    is_recording: AtomicBool,
    /// Recorded events
    events: Mutex<Vec<ScriptEvent>>,
    /// Recording start time
    start_time: Mutex<Option<Instant>>,
    /// Last event time
    last_event_time: Mutex<Option<Instant>>,
    /// Current mouse position
    mouse_position: Mutex<(f64, f64)>,
}

impl RecordingState {
    pub fn new() -> Self {
        Self {
            is_recording: AtomicBool::new(false),
            events: Mutex::new(Vec::new()),
            start_time: Mutex::new(None),
            last_event_time: Mutex::new(None),
            mouse_position: Mutex::new((0.0, 0.0)),
        }
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }

    pub fn start(&self) {
        self.events.lock().clear();
        *self.start_time.lock() = Some(Instant::now());
        *self.last_event_time.lock() = Some(Instant::now());
        self.is_recording.store(true, Ordering::SeqCst);
    }

    pub fn stop(&self) {
        self.is_recording.store(false, Ordering::SeqCst);
    }

    pub fn add_event(&self, event: ScriptEvent) {
        if self.is_recording() {
            self.events.lock().push(event);
        }
    }

    pub fn get_events(&self) -> Vec<ScriptEvent> {
        self.events.lock().clone()
    }

    pub fn get_elapsed_ms(&self) -> u64 {
        let last_time = self.last_event_time.lock();
        let now = Instant::now();
        last_time
            .map(|t| now.duration_since(t).as_millis() as u64)
            .unwrap_or(0)
    }

    pub fn commit_event(&self, event: ScriptEvent) {
        if !self.is_recording() {
            return;
        }

        // Update time
        let mut last_time = self.last_event_time.lock();
        *last_time = Some(Instant::now());

        // Add event
        self.events.lock().push(event);
    }

    // Helper to update position without adding event (not used with new logic but kept for safety)
    pub fn update_mouse_position(&self, x: f64, y: f64) {
        *self.mouse_position.lock() = (x, y);
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        *self.mouse_position.lock()
    }
}

impl Default for RecordingState {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the global recording state
pub fn get_state() -> Arc<RecordingState> {
    Arc::clone(&RECORDING_STATE)
}

/// Start recording (flag only)
pub fn start_recording() -> Result<(), String> {
    let state = get_state();

    if state.is_recording() {
        return Err("Already recording".to_string());
    }

    state.start();
    Ok(())
}

/// Stop recording and return recorded events
pub fn stop_recording() -> Vec<ScriptEvent> {
    let state = get_state();
    state.stop();
    state.get_events()
}

/// Check if currently recording
pub fn is_recording() -> bool {
    get_state().is_recording()
}

/// Record an event directly (e.g. from frontend)
pub fn record_event_direct(event: ScriptEvent) {
    let state = get_state();
    if state.is_recording() {
        // Recalculate delay based on last event time to ensure consistency
        // Note: The event passed in might have a delay relative to frontend time,
        // but we should probably just use the backend timer for consistency
        // OR rely on the limit that this is ONLY called when focused.
        // Better: Use the same commit_event logic which manages time.
        // We override the delay with the backend's elapsed time.
        let elapsed = state.get_elapsed_ms();
        let mut event_clone = event;
        event_clone.set_delay_ms(elapsed);
        state.commit_event(event_clone);
    }
}

/// Get currently recorded events (for real-time display)
pub fn get_recorded_events() -> Vec<ScriptEvent> {
    get_state().get_events()
}
