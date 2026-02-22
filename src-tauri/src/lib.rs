//! AutoKB - Desktop Automation Application
//! Main Tauri entry point with all commands

mod hotkey;
mod input_manager;
mod macro_trigger;
mod player;
mod recorder;
mod script;

use script::{KeyboardKey, MacroDefinition, MacroTrigger, MouseButton, Script, ScriptEvent};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use tauri::{WebviewUrl, WebviewWindowBuilder};

// Show main window after setup is complete (prevents white flash)
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
    // Hide main window
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    // Show overlay (Red)
    input_manager::show_overlay(&app, "#f85149");

    recorder::start_recording()
}

/// Stop recording and return recorded events
#[tauri::command]
fn stop_recording(app: tauri::AppHandle) -> Vec<ScriptEvent> {
    // Hide overlay
    input_manager::hide_overlay(&app);

    // Show main window
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
    recorder::record_event_direct(event);
}

// ============================================================================
// Playback Commands
// ============================================================================

/// Play a script
#[tauri::command]
fn play_script(app: tauri::AppHandle, script: Script) -> Result<(), String> {
    // Hide main window
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    // Show overlay (Blue)
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
    // Hide main window
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    // Show overlay (Blue)
    input_manager::show_overlay(&app, "#58a6ff");

    player::play_events(events, speed_multiplier)
}

/// Stop playback
#[tauri::command]
fn stop_playback(app: tauri::AppHandle) {
    // Hide overlay
    input_manager::hide_overlay(&app);

    // Show main window
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

/// Get default scripts directory
#[tauri::command]
fn get_scripts_dir(app: tauri::AppHandle) -> Result<String, String> {
    let dir = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app local data dir: {}", e))?
        .join("scripts");

    // Create directory if it doesn't exist
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    dir.to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid path".to_string())
}

// ============================================================================
// Macro Commands
// ============================================================================

/// Add a new macro
#[tauri::command]
fn add_macro(macro_def: MacroDefinition) {
    macro_trigger::add_macro(macro_def);
}

/// Remove a macro by ID
#[tauri::command]
fn remove_macro(id: String) {
    macro_trigger::remove_macro(&id);
}

/// Get all macros
#[tauri::command]
fn get_all_macros() -> Vec<MacroDefinition> {
    macro_trigger::get_all_macros()
}

/// Toggle macro enabled state
#[tauri::command]
fn toggle_macro(id: String, enabled: bool) {
    macro_trigger::toggle_macro(&id, enabled);
}

/// Start macro listener
#[tauri::command]
fn start_macro_listener() -> Result<(), String> {
    macro_trigger::start_macro_listener()
}

/// Stop macro listener
#[tauri::command]
fn stop_macro_listener() {
    macro_trigger::stop_macro_listener();
}

/// Create a macro binding
#[tauri::command]
fn create_macro_binding(
    name: String,
    trigger_type: String,
    trigger_value: String,
    script_path: String,
) -> Result<MacroDefinition, String> {
    // Parse trigger
    let trigger = match trigger_type.as_str() {
        "mouse" => {
            let button = match trigger_value.as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                "back" => MouseButton::Back,
                "forward" => MouseButton::Forward,
                _ => return Err("Invalid mouse button".to_string()),
            };
            MacroTrigger::MousePress { button }
        }
        "key" => {
            let key = if trigger_value.len() == 1 {
                KeyboardKey::Char(trigger_value.chars().next().unwrap())
            } else {
                KeyboardKey::Special(trigger_value)
            };
            MacroTrigger::KeyPress { key }
        }
        _ => return Err("Invalid trigger type".to_string()),
    };

    let macro_def = macro_trigger::create_macro_binding(name, trigger, script_path);
    macro_trigger::add_macro(macro_def.clone());
    Ok(macro_def)
}

#[derive(serde::Serialize)]
struct SavedScript {
    name: String,
    path: String,
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
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    scripts.push(SavedScript {
                        name: name.to_string(),
                        path: path.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }
    Ok(scripts)
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
        event.set_delay_ms(delay_ms);
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
        let new_delay = (event.delay_ms() as f64 * factor) as u64;
        event.set_delay_ms(new_delay);
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
    macro_active: bool,
}

#[tauri::command]
fn get_app_state() -> AppState {
    AppState {
        recording: recorder::is_recording(),
        playing: player::is_playing(),
        macro_active: macro_trigger::get_state().is_active(),
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
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

// ... (existing code)

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init()) // Add shell plugin
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        if shortcut.matches(Modifiers::empty(), Code::F9) {
                            println!("Global F9 pressed");
                            if recorder::is_recording() {
                                let _ = stop_recording(app.clone());
                                input_manager::emit_event(
                                    "hotkey-event",
                                    crate::hotkey::HotkeyEvent {
                                        action: "recording-stopped".to_string(),
                                        recording: false,
                                        playing: player::is_playing(),
                                    },
                                );
                            } else if !player::is_playing() {
                                let _ = start_recording(app.clone());
                                input_manager::emit_event(
                                    "hotkey-event",
                                    crate::hotkey::HotkeyEvent {
                                        action: "recording-started".to_string(),
                                        recording: true,
                                        playing: false,
                                    },
                                );
                            }
                        }
                        if shortcut.matches(Modifiers::empty(), Code::F10) {
                            println!("Global F10 pressed");
                            if player::is_playing() {
                                stop_playback(app.clone());
                                input_manager::emit_event(
                                    "hotkey-event",
                                    crate::hotkey::HotkeyEvent {
                                        action: "playback-stopped".to_string(),
                                        recording: recorder::is_recording(),
                                        playing: false,
                                    },
                                );
                            } else {
                                input_manager::emit_event(
                                    "hotkey-event",
                                    crate::hotkey::HotkeyEvent {
                                        action: "playback-requested".to_string(),
                                        recording: recorder::is_recording(),
                                        playing: false,
                                    },
                                );
                            }
                        }
                        // if shortcut.matches(Modifiers::empty(), Code::Escape) {
                        //     println!("Global Escape pressed");
                        //     let was_recording = recorder::is_recording();
                        //     let was_playing = player::is_playing();

                        //     if was_recording {
                        //         let _ = recorder::stop_recording();
                        //     }
                        //     if was_playing {
                        //         player::stop_playback();
                        //     }

                        //     if was_recording || was_playing {
                        //         if let Some(window) = app.get_webview_window("main") {
                        //             let _ = window.show();
                        //             let _ = window.set_focus();
                        //         }
                        //         input_manager::hide_overlay(app);

                        //         input_manager::emit_event(
                        //             "hotkey-event",
                        //             crate::hotkey::HotkeyEvent {
                        //                 action: "emergency-stop".to_string(),
                        //                 recording: false,
                        //                 playing: false,
                        //             },
                        //         );
                        //     }
                        // }
                    }
                })
                .build(),
        )
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::GlobalShortcutExt;
                app.global_shortcut()
                    .register(Shortcut::new(None, Code::F9))?;
                app.global_shortcut()
                    .register(Shortcut::new(None, Code::F10))?;
                // app.global_shortcut()
                //     .register(Shortcut::new(None, Code::Escape))?;
            }

            // Initialize unified input manager (handles hotkeys, recording, macros)
            input_manager::init(app.handle().clone());

            // create overlay window
            let _ = WebviewWindowBuilder::new(
                app,
                "overlay",
                WebviewUrl::App(PathBuf::from("overlay.html")),
            )
            // .inner_size(width, height)
            // .position(-100., -100.)
            .decorations(false)
            .transparent(true)
            .resizable(false)
            .maximizable(true)
            .always_on_top(true)
            .visible(false)
            .fullscreen(true)
            .skip_taskbar(true)
            .build();
            // if let Some(window) = app.get_webview_window("overlay") {
            //     let _ = window.set_decorations(false);
            //     let _ = window.set_always_on_top(true);
            //     let _ = window.maximize();
            //     let _ = window.set_ignore_cursor_events(true);
            // }

            // System Tray Setup
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
            let menu = Menu::with_items(app, &[&quit_i]).unwrap();

            let _tray = TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
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
                // If it's the main window, hide it instead of closing
                if window.label() == "main" {
                    let _ = window.hide();
                    api.prevent_close();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Window
            release_main_window,
            release_overlay_window,
            // Recording
            start_recording,
            stop_recording,
            is_recording,
            get_recorded_events,
            record_frontend_event,
            // Playback
            play_script,
            play_events,
            stop_playback,
            is_playing,
            // Script files
            save_script,
            load_script,
            get_scripts_dir,
            // Macros
            add_macro,
            remove_macro,
            get_all_macros,
            toggle_macro,
            start_macro_listener,
            stop_macro_listener,
            create_macro_binding,
            list_saved_scripts,
            // Script editing
            update_event_delay,
            delete_event,
            scale_delays,
            // App state
            get_app_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
