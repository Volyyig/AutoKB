# ⚡ AutoKeyButton

AutoKB is a powerful desktop automation tool designed to streamline repetitive tasks through intelligent input recording, playback, and macro management. Built with Rust and Tauri, it offers a high-performance, low-latency experience with a modern, glassmorphic UI.

---

## ✨ Key Functionalities

### 🔴 Intelligent Recording
- **Global Capture**: Record keyboard and mouse events across your entire system, not just within the application window.
- **Smart Filtering**: Automatically handles key repeats and complex input sequences to ensure playback fidelity.
- **Privacy First**: Recording status is clearly indicated via a global screen overlay.

### ⏯️ Precision Playback
- **High Fidelity**: Reproduces recorded actions with millisecond precision.
- **Speed Customization**: Adjust playback speed in real-time (from slow-motion for debugging to high-speed execution).
- **Control Overlay**: A visual "Blue Mode" overlay indicates when playback is active, preventing accidental manual interference.

### 🔗 Advanced Macro Management
- **Input Mapping**: Map complex event sequences to single key triggers or mouse buttons.
- **Toggle Listeners**: Enable or disable global macro listeners with a single click.
- **State Persistence**: Your macros and scripts are saved locally for instant access across sessions.

### 📝 Script Fine-Tuning
- **Event List**: View every captured event in a detailed chronologically ordered list.
- **In-Place Editing**: Modify delays between events or delete unwanted steps directly from the UI.
- **Batch Scaling**: Scale all delays in a script simultaneously to speed up or slow down the entire workflow.

---

## ⌨️ Fast Controls

Quickly manage your automation without switching windows:

| Action | Hotkey |
| :--- | :--- |
| **Start/Stop Recording** | `F9` |
| **Start/Stop Playback** | `F10` |

---

## 🎨 Professional Interface
- **Dark Mode by Default**: A sleek, focused interface using a GitHub-inspired dark theme.
- **Real-time Status**: Dynamic indicators show exactly what the engine is doing (Recording, Playing, or Macro Active).
- **System Tray Integration**: Run AutoKB in the background and access it instantly from your tray.

---

## 🚀 Getting Started

1. **Launch AutoKB**.
2. Press **`F9`** to start recording your workflow.
3. Perform the tasks you wish to automate.
4. Press **`F9`** again to finalize the script.
5. Use the **Event List** to refine your actions or press **`F10`** to test the playback.
