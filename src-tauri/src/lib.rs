//! AutoKB - Desktop Automation Application
//! Main Tauri entry point with all commands

mod hotkey;
mod input_manager;
mod macro_trigger;
mod player;
mod recorder;
mod script;

use script::{KeyboardKey, LoopConfig, Script, ScriptEvent, Task};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use tauri::{WebviewUrl, WebviewWindowBuilder};

// ============================================================================
// Window Commands
// ============================================================================

/// Show main window after setup is complete (prevents white flash)
#[tauri::command]
fn release_main_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
    }
}

#[tauri::command]
fn release_overlay_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("overlay") {
        let _ = window.set_ignore_cursor_events(true);
    }
}

// ============================================================================
// Recording Commands
// ============================================================================

/// Start recording keyboard/mouse events
#[tauri::command]
fn start_recording(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    input_manager::show_overlay(&app, "#f85149");
    recorder::start_recording()
}

/// Stop recording and return recorded events
#[tauri::command]
fn stop_recording(app: tauri::AppHandle) -> Vec<ScriptEvent> {
    input_manager::hide_overlay(&app);
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
    recorder::stop_recording()
}

/// Check if currently recording
#[tauri::command]
fn is_recording() -> bool {
    recorder::is_recording()
}

/// Get currently recorded events (for real-time display)
#[tauri::command]
fn get_recorded_events() -> Vec<ScriptEvent> {
    recorder::get_recorded_events()
}

/// Record an event from the frontend (for when window is focused)
#[tauri::command]
fn record_frontend_event(event: ScriptEvent) {
    recorder::get_state().commit_event(event);
}

// ============================================================================
// Playback Commands
// ============================================================================

/// Play a script
#[tauri::command]
fn play_script(app: tauri::AppHandle, script: Script) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    input_manager::show_overlay(&app, "#58a6ff");
    player::play_script(script)
}

/// Play a list of events with speed multiplier
#[tauri::command]
fn play_events(
    app: tauri::AppHandle,
    events: Vec<ScriptEvent>,
    speed_multiplier: f64,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    input_manager::show_overlay(&app, "#58a6ff");
    player::play_events(events, speed_multiplier)
}

/// Stop playback
#[tauri::command]
fn stop_playback(app: tauri::AppHandle) {
    input_manager::hide_overlay(&app);
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
    player::stop_playback()
}

/// Check if currently playing
#[tauri::command]
fn is_playing() -> bool {
    player::is_playing()
}

// ============================================================================
// Script File Commands
// ============================================================================

/// Save script to file
#[tauri::command]
fn save_script(script: Script, path: String) -> Result<(), String> {
    let json =
        serde_json::to_string_pretty(&script).map_err(|e| format!("Serialization error: {}", e))?;
    fs::write(&path, json).map_err(|e| format!("File write error: {}", e))?;
    Ok(())
}

/// Load script from file
#[tauri::command]
fn load_script(path: String) -> Result<Script, String> {
    let content = fs::read_to_string(&path).map_err(|e| format!("File read error: {}", e))?;
    let script: Script =
        serde_json::from_str(&content).map_err(|e| format!("Parse error: {}", e))?;
    Ok(script)
}

/// Delete a script file
#[tauri::command]
fn delete_script(path: String) -> Result<(), String> {
    fs::remove_file(path).map_err(|e| format!("File delete error: {}", e))?;
    Ok(())
}

/// Get default scripts directory
#[tauri::command]
fn get_scripts_dir(app: tauri::AppHandle) -> Result<String, String> {
    let dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data dir: {}", e))?
        .join("scripts");

    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    dir.to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid path".to_string())
}

#[derive(serde::Serialize)]
struct SavedScript {
    name: String,
    path: String,
    description: String,
    modified_at: String,
}

/// List saved scripts
#[tauri::command]
fn list_saved_scripts(app: tauri::AppHandle) -> Result<Vec<SavedScript>, String> {
    let script_dir_str = get_scripts_dir(app)?;
    let entries = fs::read_dir(script_dir_str).map_err(|e| e.to_string())?;

    let mut scripts = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("autokb") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(script) = serde_json::from_str::<Script>(&content) {
                        scripts.push(SavedScript {
                            name: script.name,
                            path: path.to_string_lossy().to_string(),
                            description: script.description,
                            modified_at: script.modified_at.to_rfc3339(),
                        });
                    }
                }
            }
        }
    }
    // Sort by modified_at desc
    scripts.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    Ok(scripts)
}

// ============================================================================
// Task Commands
// ============================================================================

/// Add a new task
#[tauri::command]
fn add_task(task: Task) {
    macro_trigger::add_task(task);
}

/// Remove a task by ID
#[tauri::command]
fn remove_task(id: String) {
    macro_trigger::remove_task(&id);
}

/// Get all tasks
#[tauri::command]
fn get_all_tasks() -> Vec<Task> {
    macro_trigger::get_all_tasks()
}

/// Toggle task enabled state
#[tauri::command]
fn toggle_task(id: String, enabled: bool) {
    macro_trigger::toggle_task(&id, enabled);
}

/// Start task listener
#[tauri::command]
fn start_task_listener() -> Result<(), String> {
    macro_trigger::start_task_listener()
}

/// Stop task listener
#[tauri::command]
fn stop_task_listener() {
    macro_trigger::stop_task_listener();
}

/// Create a task binding
#[tauri::command]
fn create_task_binding(
    name: String,
    trigger_key: Option<String>,
    stop_key: Option<String>,
    script_path: String,
) -> Result<Task, String> {
    let parse_key = |k: String| {
        if k.len() == 1 {
            KeyboardKey::Char(k.chars().next().unwrap())
        } else {
            KeyboardKey::Special(k)
        }
    };

    let task = Task {
        id: macro_trigger::uuid_simple(),
        name,
        description: String::new(),
        trigger_key: trigger_key.map(parse_key),
        stop_key: stop_key.map(parse_key),
        script_path,
        enabled: true,
        loop_config: LoopConfig::default(),
        speed_multiplier: 1.0,
    };

    macro_trigger::add_task(task.clone());
    Ok(task)
}

// ============================================================================
// Script Edit Commands
// ============================================================================

/// Update event delay at index
#[tauri::command]
fn update_event_delay(
    mut events: Vec<ScriptEvent>,
    index: usize,
    delay_ms: u64,
) -> Vec<ScriptEvent> {
    if let Some(event) = events.get_mut(index) {
        if let ScriptEvent::Delay { duration_ms } = event {
            *duration_ms = delay_ms;
        }
    }
    events
}

/// Delete event at index
#[tauri::command]
fn delete_event(mut events: Vec<ScriptEvent>, index: usize) -> Vec<ScriptEvent> {
    if index < events.len() {
        events.remove(index);
    }
    events
}

/// Scale all delays by a factor
#[tauri::command]
fn scale_delays(mut events: Vec<ScriptEvent>, factor: f64) -> Vec<ScriptEvent> {
    for event in &mut events {
        if let ScriptEvent::Delay { duration_ms } = event {
            *duration_ms = (*duration_ms as f64 * factor) as u64;
        }
    }
    events
}

// ============================================================================
// App State Commands
// ============================================================================

#[derive(Clone, serde::Serialize)]
struct AppState {
    recording: bool,
    playing: bool,
    task_listener_active: bool,
}

#[tauri::command]
fn get_app_state() -> AppState {
    AppState {
        recording: recorder::is_recording(),
        playing: player::is_playing(),
        task_listener_active: macro_trigger::get_state().is_active(),
    }
}

// ============================================================================
// Main Entry Point
// ============================================================================

use tauri::{
    menu::{Menu, MenuItem},
    tray::{
        MouseButton as TrayMouseButton, MouseButtonState as TrayMouseButtonState, TrayIconBuilder,
        TrayIconEvent,
    },
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            input_manager::init(app.handle().clone());

            let _ = WebviewWindowBuilder::new(
                app,
                "overlay",
                WebviewUrl::App(PathBuf::from("overlay.html")),
            )
            .focusable(false)
            .decorations(false)
            .transparent(true)
            .always_on_top(true)
            .visible(false)
            .fullscreen(true)
            .skip_taskbar(true)
            .build();

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
            let menu = Menu::with_items(app, &[&quit_i]).unwrap();

            let _tray = TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: TrayMouseButton::Left,
                        button_state: TrayMouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    let _ = window.hide();
                    api.prevent_close();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            release_main_window,
            release_overlay_window,
            start_recording,
            stop_recording,
            is_recording,
            get_recorded_events,
            record_frontend_event,
            play_script,
            play_events,
            stop_playback,
            is_playing,
            save_script,
            load_script,
            get_scripts_dir,
            delete_script,
            add_task,
            remove_task,
            get_all_tasks,
            toggle_task,
            start_task_listener,
            stop_task_listener,
            create_task_binding,
            list_saved_scripts,
            update_event_delay,
            delete_event,
            scale_delays,
            get_app_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
