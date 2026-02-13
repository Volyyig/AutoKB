# ⚡ AutoKeyButton

AutoKB is a powerful, lightweight desktop automation tool built with **Tauri v2** and **Vue 3**. It allows you to record, edit, and play back keyboard and mouse sequences with ease, and manage complex macros via global hotkeys.

## ✨ Features

- **🔴 High-Precision Recording**: Capture keyboard and mouse events in real-time.
- **▶️ Flexible Playback**: Replay recorded sequences with adjustable speed and delay.
- **⌨️ Global Hotkeys**: Control your automation from anywhere in the OS.
- **🔗 Macro Management**: Map specific triggers (keys/mouse) to complex action sequences.
- **📊 Event Editor**: Fine-tune your steps, adjust delays, or delete unwanted events.
- **🖥️ Minimal Overlay**: Visual feedback during recording and playback without obstructing your view.
- **🌓 Modern UI**: A sleek, dark-themed interface built for efficiency.

## 🛠️ Tech Stack

- **Backend**: Rust, [Tauri v2](https://tauri.app/)
- **Frontend**: [Vue 3](https://vuejs.org/), TypeScript, [Vite](https://vitejs.dev/)
- **State Management**: [Pinia](https://pinia.vuejs.org/)
- **Libraries**: `rdev` (event capture), `enigo` (event simulation)

## ⌨️ Global Hotkeys

| Key | Action |
| :--- | :--- |
| **F9** | Start / Stop Recording |
| **F10** | Play / Stop Script |
| **Esc** | Emergency Stop (Stop all recording/playback) |

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (Project uses `pnpm` or `npm`)

### Development

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/autokb.git
   cd autokb
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

### Building

To build the production application:
```bash
npm run tauri build
```

## 📖 Usage Guide

1. **Recording**: Press `F9` or click the record button. The main window will hide, and a red overlay will appear. Perform your actions, then press `F9` again.
2. **Editing**: Review the captured events in the "Event List". You can adjust individual delays or delete steps.
3. **Playback**: Press `F10` or click play. A blue overlay signals active playback.
4. **Macros**: Use the Macro Panel to create custom mappings, like triggering a complex combo with a single key press.

## ⚖️ License

Distributed under the MIT License. See `LICENSE` for more information.
