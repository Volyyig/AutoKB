//! Macro trigger module - handles macro definitions and trigger logic
//! Listener moved to input_manager

use crate::player;
use crate::script::{KeyboardKey, MacroDefinition, MacroTrigger, MouseButton, ScriptEvent};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

/// Global macro state
static MACRO_STATE: Lazy<Arc<MacroState>> = Lazy::new(|| Arc::new(MacroState::new()));

/// Macro state manager
pub struct MacroState {
    /// Whether macro listening is active
    is_active: AtomicBool,
    /// Registered macros (key: trigger identifier, value: macro definition)
    macros: RwLock<HashMap<String, MacroDefinition>>,
}

impl MacroState {
    pub fn new() -> Self {
        Self {
            is_active: AtomicBool::new(false),
            macros: RwLock::new(HashMap::new()),
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active.load(Ordering::SeqCst)
    }

    pub fn set_active(&self, active: bool) {
        self.is_active.store(active, Ordering::SeqCst);
    }

    /// Add or update a macro
    pub fn add_macro(&self, macro_def: MacroDefinition) {
        let trigger_id = get_trigger_id(&macro_def.trigger);
        self.macros.write().insert(trigger_id, macro_def);
    }

    /// Remove a macro by ID
    pub fn remove_macro(&self, id: &str) {
        self.macros.write().retain(|_, m| m.id != id);
    }

    /// Get all macros
    pub fn get_all_macros(&self) -> Vec<MacroDefinition> {
        self.macros.read().values().cloned().collect()
    }

    /// Find macro by trigger
    pub fn find_by_trigger(&self, trigger: &MacroTrigger) -> Option<MacroDefinition> {
        let trigger_id = get_trigger_id(trigger);
        self.macros.read().get(&trigger_id).cloned()
    }

    /// Check if a trigger matches a macro and execute if enabled
    pub fn check_and_execute(&self, trigger: &MacroTrigger) -> bool {
        if !self.is_active() {
            return false;
        }

        if let Some(macro_def) = self.find_by_trigger(trigger) {
            if macro_def.enabled && !macro_def.events.is_empty() {
                // Execute macro events
                let events = macro_def.events.clone();
                thread::spawn(move || {
                    let _ = player::play_events(events, 1.0);
                });
                return true;
            }
        }
        false
    }
}

impl Default for MacroState {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the global macro state
pub fn get_state() -> Arc<MacroState> {
    Arc::clone(&MACRO_STATE)
}

/// Generate a unique trigger identifier
fn get_trigger_id(trigger: &MacroTrigger) -> String {
    match trigger {
        MacroTrigger::KeyPress { key } => format!("key:{:?}", key),
        MacroTrigger::MousePress { button } => format!("mouse:{:?}", button),
    }
}

/// Start macro listening (flag only)
pub fn start_macro_listener() -> Result<(), String> {
    let state = get_state();
    state.set_active(true);
    Ok(())
}

/// Stop macro listening
pub fn stop_macro_listener() {
    get_state().set_active(false);
}

/// Add a new macro
pub fn add_macro(macro_def: MacroDefinition) {
    get_state().add_macro(macro_def);
}

/// Remove a macro by ID
pub fn remove_macro(id: &str) {
    get_state().remove_macro(id);
}

/// Get all registered macros
pub fn get_all_macros() -> Vec<MacroDefinition> {
    get_state().get_all_macros()
}

/// Toggle macro enabled state
pub fn toggle_macro(id: &str, enabled: bool) {
    let state = get_state();
    let macros = state.macros.read();
    if let Some(macro_def) = macros.values().find(|m| m.id == id) {
        let mut updated = macro_def.clone();
        drop(macros);
        updated.enabled = enabled;
        state.add_macro(updated);
    }
}

/// Create a simple macro: trigger one input with another
pub fn create_simple_macro(
    name: String,
    trigger: MacroTrigger,
    action_events: Vec<ScriptEvent>,
) -> MacroDefinition {
    MacroDefinition {
        id: uuid_simple(),
        name,
        trigger,
        events: action_events,
        enabled: true,
    }
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("macro_{}", duration.as_nanos())
}
