//! Task manager module - handles task definitions and trigger logic
//! Listener moved to input_manager

use crate::player;
use crate::script::{KeyboardKey, Script, Task};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

/// Global task state
static TASK_STATE: Lazy<Arc<TaskState>> = Lazy::new(|| Arc::new(TaskState::new()));

/// Task state manager
pub struct TaskState {
    /// Whether task listening is active
    is_active: AtomicBool,
    /// Registered tasks (key: ID, value: task definition)
    tasks: RwLock<HashMap<String, Task>>,
}

impl TaskState {
    pub fn new() -> Self {
        Self {
            is_active: AtomicBool::new(false),
            tasks: RwLock::new(HashMap::new()),
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active.load(Ordering::SeqCst)
    }

    pub fn set_active(&self, active: bool) {
        self.is_active.store(active, Ordering::SeqCst);
    }

    /// Add or update a task
    pub fn add_task(&self, task: Task) {
        self.tasks.write().insert(task.id.clone(), task);
    }

    /// Remove a task by ID
    pub fn remove_task(&self, id: &str) {
        self.tasks.write().remove(id);
    }

    /// Get all tasks
    pub fn get_all_tasks(&self) -> Vec<Task> {
        self.tasks.read().values().cloned().collect()
    }

    /// Find task by trigger key
    pub fn find_by_trigger(&self, key: &KeyboardKey) -> Option<Task> {
        self.tasks
            .read()
            .values()
            .find(|t| t.trigger_key.as_ref() == Some(key))
            .cloned()
    }

    /// Find task by stop key
    pub fn find_by_stop(&self, key: &KeyboardKey) -> Option<Task> {
        self.tasks
            .read()
            .values()
            .find(|t| t.stop_key.as_ref() == Some(key))
            .cloned()
    }

    /// Check if a key press should trigger or stop a task
    pub fn check_key_event(&self, key: &KeyboardKey) -> bool {
        if !self.is_active() {
            return false;
        }

        // 1. Check if it's a stop key for a running task
        if player::is_playing() {
            if let Some(_task) = self.find_by_stop(key) {
                player::stop_playback();
                return true;
            }
        }

        // 2. Check if it's a trigger key for a task
        if let Some(task) = self.find_by_trigger(key) {
            if task.enabled && !task.script_path.is_empty() {
                // If already playing, stop first?
                // Or only play if not playing?
                if player::is_playing() {
                    player::stop_playback();
                    // Optional: delay or wait for stop
                }

                let path = task.script_path.clone();
                let loop_config = task.loop_config.clone();
                let speed_multiplier = task.speed_multiplier;

                // Spawn thread to execute task script
                thread::spawn(move || {
                    if let Ok(content) = fs::read_to_string(&path) {
                        match serde_json::from_str::<Script>(&content) {
                            Ok(mut script) => {
                                // Override script settings with task settings
                                script.loop_config = loop_config;
                                script.speed_multiplier = speed_multiplier;
                                let _ = player::play_script(script);
                            }
                            Err(e) => eprintln!("Failed to parse script {}: {}", path, e),
                        }
                    } else {
                        eprintln!("Failed to read script: {}", path);
                    }
                });
                return true;
            }
        }
        false
    }
}

impl Default for TaskState {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the global task state
pub fn get_state() -> Arc<TaskState> {
    Arc::clone(&TASK_STATE)
}

/// Start task listening
pub fn start_task_listener() -> Result<(), String> {
    let state = get_state();
    state.set_active(true);
    Ok(())
}

/// Stop task listening
pub fn stop_task_listener() {
    get_state().set_active(false);
}

/// Add a new task
pub fn add_task(task: Task) {
    get_state().add_task(task);
}

/// Remove a task by ID
pub fn remove_task(id: &str) {
    get_state().remove_task(id);
}

/// Get all registered tasks
pub fn get_all_tasks() -> Vec<Task> {
    get_state().get_all_tasks()
}

/// Toggle task enabled state
pub fn toggle_task(id: &str, enabled: bool) {
    let state = get_state();
    let mut tasks = state.tasks.write();
    if let Some(task) = tasks.get_mut(id) {
        task.enabled = enabled;
    }
}

pub fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("task_{}", duration.as_nanos())
}
