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

## 🎨 Styling System

AutoKB uses a native CSS styling system with CSS variables for theming, providing a consistent and maintainable design across the application.

### CSS Variable Theme System

The application uses CSS custom properties (CSS variables) defined in `src/index.css` for all theme colors and design tokens:

**Core Theme Variables:**
- `--primary`: Primary brand color
- `--background`: Main background color
- `--surface`: Surface/card background color
- `--surface-soft`: Soft surface color for hover states
- `--text-main`: Primary text color
- `--text-muted`: Muted/secondary text color
- `--border-main`: Border color
- `--font-display`: Display font family

**Status Colors:**
- `--success`, `--success-bg`, `--success-border`: Success state colors
- `--error`, `--error-bg`, `--error-border`: Error state colors
- `--warning`, `--warning-bg`, `--warning-border`: Warning state colors
- `--info`, `--info-bg`, `--info-border`: Info state colors

### Dark Mode Implementation

Dark mode is implemented using CSS variable overrides with the `.dark` class on the root element:

```css
/* Light mode (default) */
:root {
  --primary: #135bec;
  --background: #f6f6f8;
  --text-main: #0f172a;
  /* ... other variables */
}

/* Dark mode */
:root.dark {
  --primary: #3b82f6;
  --background: #0f172a;
  --text-main: #f8fafc;
  /* ... other variables */
}
```

**Toggle Dark Mode:**

The theme is managed in the application store (`src/stores/scriptStore.ts`). To toggle dark mode:

```typescript
// In your component
import { useScriptStore } from '@/stores/scriptStore';

const store = useScriptStore();

// Toggle dark mode
store.theme.isDark = !store.theme.isDark;
store.applyTheme();
```

The `applyTheme()` method automatically adds or removes the `dark` class from the document root element, triggering the CSS variable overrides.

### Using Theme Colors in Components

All components use CSS variables for colors to ensure consistent theming:

```css
.my-component {
  background-color: var(--surface);
  color: var(--text-main);
  border: 1px solid var(--border-main);
}

.my-button:hover {
  background-color: var(--surface-soft);
}
```

This approach ensures that all components automatically adapt to theme changes without requiring component-level logic.

---

## 🚀 Getting Started

1. **Launch AutoKB**.
2. Press **`F9`** to start recording your workflow.
3. Perform the tasks you wish to automate.
4. Press **`F9`** again to finalize the script.
5. Use the **Event List** to refine your actions or press **`F10`** to test the playback.
