//! Script data structures for AutoKB
//! Defines keyboard/mouse events and script containers

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Mouse button types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Unknown,
}

impl From<rdev::Button> for MouseButton {
    fn from(btn: rdev::Button) -> Self {
        match btn {
            rdev::Button::Left => MouseButton::Left,
            rdev::Button::Right => MouseButton::Right,
            rdev::Button::Middle => MouseButton::Middle,
            rdev::Button::Unknown(1) => MouseButton::Back,
            rdev::Button::Unknown(2) => MouseButton::Forward,
            _ => MouseButton::Unknown,
        }
    }
}

impl From<MouseButton> for enigo::Button {
    fn from(btn: MouseButton) -> Self {
        match btn {
            MouseButton::Left => enigo::Button::Left,
            MouseButton::Right => enigo::Button::Right,
            MouseButton::Middle => enigo::Button::Middle,
            MouseButton::Back => enigo::Button::Back,
            MouseButton::Forward => enigo::Button::Forward,
            MouseButton::Unknown => enigo::Button::Left,
        }
    }
}

/// Keyboard key representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value")]
pub enum KeyboardKey {
    /// Character key (a-z, 0-9, symbols)
    Char(char),
    /// Special key (Enter, Escape, F1-F12, etc.)
    Special(String),
}

impl From<rdev::Key> for KeyboardKey {
    fn from(key: rdev::Key) -> Self {
        match key {
            rdev::Key::Alt => KeyboardKey::Special("Alt".to_string()),
            rdev::Key::AltGr => KeyboardKey::Special("AltGr".to_string()),
            rdev::Key::Backspace => KeyboardKey::Special("Backspace".to_string()),
            rdev::Key::CapsLock => KeyboardKey::Special("CapsLock".to_string()),
            rdev::Key::ControlLeft => KeyboardKey::Special("ControlLeft".to_string()),
            rdev::Key::ControlRight => KeyboardKey::Special("ControlRight".to_string()),
            rdev::Key::Delete => KeyboardKey::Special("Delete".to_string()),
            rdev::Key::DownArrow => KeyboardKey::Special("DownArrow".to_string()),
            rdev::Key::End => KeyboardKey::Special("End".to_string()),
            rdev::Key::Escape => KeyboardKey::Special("Escape".to_string()),
            rdev::Key::F1 => KeyboardKey::Special("F1".to_string()),
            rdev::Key::F2 => KeyboardKey::Special("F2".to_string()),
            rdev::Key::F3 => KeyboardKey::Special("F3".to_string()),
            rdev::Key::F4 => KeyboardKey::Special("F4".to_string()),
            rdev::Key::F5 => KeyboardKey::Special("F5".to_string()),
            rdev::Key::F6 => KeyboardKey::Special("F6".to_string()),
            rdev::Key::F7 => KeyboardKey::Special("F7".to_string()),
            rdev::Key::F8 => KeyboardKey::Special("F8".to_string()),
            rdev::Key::F9 => KeyboardKey::Special("F9".to_string()),
            rdev::Key::F10 => KeyboardKey::Special("F10".to_string()),
            rdev::Key::F11 => KeyboardKey::Special("F11".to_string()),
            rdev::Key::F12 => KeyboardKey::Special("F12".to_string()),
            rdev::Key::Home => KeyboardKey::Special("Home".to_string()),
            rdev::Key::LeftArrow => KeyboardKey::Special("LeftArrow".to_string()),
            rdev::Key::MetaLeft => KeyboardKey::Special("MetaLeft".to_string()),
            rdev::Key::MetaRight => KeyboardKey::Special("MetaRight".to_string()),
            rdev::Key::PageDown => KeyboardKey::Special("PageDown".to_string()),
            rdev::Key::PageUp => KeyboardKey::Special("PageUp".to_string()),
            rdev::Key::Return => KeyboardKey::Special("Return".to_string()),
            rdev::Key::RightArrow => KeyboardKey::Special("RightArrow".to_string()),
            rdev::Key::ShiftLeft => KeyboardKey::Special("ShiftLeft".to_string()),
            rdev::Key::ShiftRight => KeyboardKey::Special("ShiftRight".to_string()),
            rdev::Key::Space => KeyboardKey::Special("Space".to_string()),
            rdev::Key::Tab => KeyboardKey::Special("Tab".to_string()),
            rdev::Key::UpArrow => KeyboardKey::Special("UpArrow".to_string()),
            rdev::Key::PrintScreen => KeyboardKey::Special("PrintScreen".to_string()),
            rdev::Key::ScrollLock => KeyboardKey::Special("ScrollLock".to_string()),
            rdev::Key::Pause => KeyboardKey::Special("Pause".to_string()),
            rdev::Key::NumLock => KeyboardKey::Special("NumLock".to_string()),
            rdev::Key::Insert => KeyboardKey::Special("Insert".to_string()),
            rdev::Key::Num0 => KeyboardKey::Char('0'),
            rdev::Key::Num1 => KeyboardKey::Char('1'),
            rdev::Key::Num2 => KeyboardKey::Char('2'),
            rdev::Key::Num3 => KeyboardKey::Char('3'),
            rdev::Key::Num4 => KeyboardKey::Char('4'),
            rdev::Key::Num5 => KeyboardKey::Char('5'),
            rdev::Key::Num6 => KeyboardKey::Char('6'),
            rdev::Key::Num7 => KeyboardKey::Char('7'),
            rdev::Key::Num8 => KeyboardKey::Char('8'),
            rdev::Key::Num9 => KeyboardKey::Char('9'),
            rdev::Key::KeyA => KeyboardKey::Char('a'),
            rdev::Key::KeyB => KeyboardKey::Char('b'),
            rdev::Key::KeyC => KeyboardKey::Char('c'),
            rdev::Key::KeyD => KeyboardKey::Char('d'),
            rdev::Key::KeyE => KeyboardKey::Char('e'),
            rdev::Key::KeyF => KeyboardKey::Char('f'),
            rdev::Key::KeyG => KeyboardKey::Char('g'),
            rdev::Key::KeyH => KeyboardKey::Char('h'),
            rdev::Key::KeyI => KeyboardKey::Char('i'),
            rdev::Key::KeyJ => KeyboardKey::Char('j'),
            rdev::Key::KeyK => KeyboardKey::Char('k'),
            rdev::Key::KeyL => KeyboardKey::Char('l'),
            rdev::Key::KeyM => KeyboardKey::Char('m'),
            rdev::Key::KeyN => KeyboardKey::Char('n'),
            rdev::Key::KeyO => KeyboardKey::Char('o'),
            rdev::Key::KeyP => KeyboardKey::Char('p'),
            rdev::Key::KeyQ => KeyboardKey::Char('q'),
            rdev::Key::KeyR => KeyboardKey::Char('r'),
            rdev::Key::KeyS => KeyboardKey::Char('s'),
            rdev::Key::KeyT => KeyboardKey::Char('t'),
            rdev::Key::KeyU => KeyboardKey::Char('u'),
            rdev::Key::KeyV => KeyboardKey::Char('v'),
            rdev::Key::KeyW => KeyboardKey::Char('w'),
            rdev::Key::KeyX => KeyboardKey::Char('x'),
            rdev::Key::KeyY => KeyboardKey::Char('y'),
            rdev::Key::KeyZ => KeyboardKey::Char('z'),
            _ => KeyboardKey::Special("Unknown".to_string()),
        }
    }
}

/// A single input event (keyboard or mouse)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum ScriptEvent {
    /// Key press event
    KeyPress {
        key: KeyboardKey,
        /// Delay in milliseconds before this event
        delay_ms: u64,
    },
    /// Key release event
    KeyRelease { key: KeyboardKey, delay_ms: u64 },
    /// Mouse button press
    MousePress {
        button: MouseButton,
        x: f64,
        y: f64,
        delay_ms: u64,
    },
    /// Mouse button release
    MouseRelease {
        button: MouseButton,
        x: f64,
        y: f64,
        delay_ms: u64,
    },
    /// Mouse movement
    MouseMove { x: f64, y: f64, delay_ms: u64 },
    /// Mouse scroll
    MouseScroll {
        delta_x: i64,
        delta_y: i64,
        delay_ms: u64,
    },
}

impl ScriptEvent {
    /// Get the delay in milliseconds for this event
    pub fn delay_ms(&self) -> u64 {
        match self {
            ScriptEvent::KeyPress { delay_ms, .. } => *delay_ms,
            ScriptEvent::KeyRelease { delay_ms, .. } => *delay_ms,
            ScriptEvent::MousePress { delay_ms, .. } => *delay_ms,
            ScriptEvent::MouseRelease { delay_ms, .. } => *delay_ms,
            ScriptEvent::MouseMove { delay_ms, .. } => *delay_ms,
            ScriptEvent::MouseScroll { delay_ms, .. } => *delay_ms,
        }
    }

    /// Set the delay in milliseconds for this event
    pub fn set_delay_ms(&mut self, new_delay: u64) {
        match self {
            ScriptEvent::KeyPress { delay_ms, .. } => *delay_ms = new_delay,
            ScriptEvent::KeyRelease { delay_ms, .. } => *delay_ms = new_delay,
            ScriptEvent::MousePress { delay_ms, .. } => *delay_ms = new_delay,
            ScriptEvent::MouseRelease { delay_ms, .. } => *delay_ms = new_delay,
            ScriptEvent::MouseMove { delay_ms, .. } => *delay_ms = new_delay,
            ScriptEvent::MouseScroll { delay_ms, .. } => *delay_ms = new_delay,
        }
    }
}

/// Macro trigger condition - what triggers the macro
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "trigger_type")]
pub enum MacroTrigger {
    /// Triggered by pressing a keyboard key
    KeyPress { key: KeyboardKey },
    /// Triggered by pressing a mouse button
    MousePress { button: MouseButton },
}

/// A macro definition - trigger + action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroDefinition {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// What triggers this macro
    pub trigger: MacroTrigger,
    /// Events to execute when triggered
    pub events: Vec<ScriptEvent>,
    /// Whether the macro is enabled
    pub enabled: bool,
}

/// Loop configuration for script execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopConfig {
    /// Number of times to repeat (0 = infinite)
    pub count: u32,
    /// Delay between loops in milliseconds
    pub delay_between_ms: u64,
}

impl Default for LoopConfig {
    fn default() -> Self {
        Self {
            count: 1,
            delay_between_ms: 0,
        }
    }
}

/// A complete script with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    /// Script name
    pub name: String,
    /// Script description
    pub description: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// List of events
    pub events: Vec<ScriptEvent>,
    /// Loop configuration
    pub loop_config: LoopConfig,
    /// Speed multiplier (1.0 = normal, 2.0 = double speed)
    pub speed_multiplier: f64,
}

impl Default for Script {
    fn default() -> Self {
        Self {
            name: "Untitled Script".to_string(),
            description: String::new(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            events: Vec::new(),
            loop_config: LoopConfig::default(),
            speed_multiplier: 1.0,
        }
    }
}

impl Script {
    /// Create a new script with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Add an event to the script
    pub fn add_event(&mut self, event: ScriptEvent) {
        self.events.push(event);
        self.modified_at = Utc::now();
    }

    /// Clear all events
    pub fn clear_events(&mut self) {
        self.events.clear();
        self.modified_at = Utc::now();
    }

    /// Get total duration in milliseconds
    pub fn total_duration_ms(&self) -> u64 {
        self.events.iter().map(|e| e.delay_ms()).sum()
    }
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Hotkey for start/stop recording (default: F9)
    pub recording_hotkey: String,
    /// Hotkey for start/stop playback (default: F10)
    pub playback_hotkey: String,
    /// Emergency stop hotkey (default: Escape)
    pub stop_hotkey: String,
    /// Macro definitions
    pub macros: Vec<MacroDefinition>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            recording_hotkey: "F9".to_string(),
            playback_hotkey: "F10".to_string(),
            stop_hotkey: "Escape".to_string(),
            macros: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_creation() {
        let script = Script::new("Test Script");
        assert_eq!(script.name, "Test Script");
        assert!(script.events.is_empty());
    }

    #[test]
    fn test_serialization() {
        let script = Script::new("Test");
        let json = serde_json::to_string(&script).unwrap();
        let parsed: Script = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, "Test");
    }
}
