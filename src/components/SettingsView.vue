<script setup lang="ts">
import { useScriptStore } from '../stores/scriptStore';

const store = useScriptStore();

const updateThemeColor = (key: string, value: string) => {
    (store.theme as any)[key] = value;
    store.applyTheme();
};

// Commented out as the dark mode toggle UI is currently disabled
// const toggleDarkMode = () => {
//     store.theme.isDark = !store.theme.isDark;
//     store.applyTheme();
// };
</script>

<template>
    <div class="settings-container">
        <section>
            <h3 class="section-title">
                <span class="material-symbols-outlined section-icon">palette</span>
                外观设置
            </h3>

            <div class="settings-card">
                <!-- Presets -->
                <div>
                    <label class="form-label">预设主题</label>
                    <div class="preset-grid">
                        <button v-for="preset in store.themePresets" :key="preset.name" @click="store.setTheme(preset)"
                            class="preset-button">
                            <div class="preset-preview"
                                :style="{ backgroundColor: preset.background }">
                                <div class="preset-sidebar"
                                    :style="{ backgroundColor: preset.surface }"></div>
                                <div class="preset-content">
                                    <div class="preset-bar-primary" :style="{ backgroundColor: preset.primary }">
                                    </div>
                                    <div class="preset-bar-text"
                                        :style="{ backgroundColor: preset.textMain }"></div>
                                </div>
                            </div>
                            <span class="preset-name">{{ preset.name }}</span>
                        </button>
                    </div>
                </div>

                <div class="divider"></div>

                <!-- Custom Colors -->
                <div class="color-grid">
                    <div class="color-column">
                        <div class="color-control">
                            <label class="color-label">前景色 (Primary)</label>
                            <input type="color" :value="store.theme.primary"
                                @input="(e: any) => updateThemeColor('primary', e.target.value)"
                                class="color-picker" />
                        </div>
                        <div class="color-control">
                            <label class="color-label">文字色 (Main)</label>
                            <input type="color" :value="store.theme.textMain"
                                @input="(e: any) => updateThemeColor('textMain', e.target.value)"
                                class="color-picker" />
                        </div>
                    </div>
                    <div class="color-column">
                        <div class="color-control">
                            <label class="color-label">背景色 (Background)</label>
                            <input type="color" :value="store.theme.background"
                                @input="(e: any) => updateThemeColor('background', e.target.value)"
                                class="color-picker" />
                        </div>
                        <div class="color-control">
                            <label class="color-label">次要文字 (Muted)</label>
                            <input type="color" :value="store.theme.textMuted"
                                @input="(e: any) => updateThemeColor('textMuted', e.target.value)"
                                class="color-picker" />
                        </div>
                    </div>
                    <div class="color-column">
                        <div class="color-control">
                            <label class="color-label">卡片色 (Surface)</label>
                            <input type="color" :value="store.theme.surface"
                                @input="(e: any) => updateThemeColor('surface', e.target.value)"
                                class="color-picker" />
                        </div>
                        <div class="color-control">
                            <label class="color-label">边框色 (Border)</label>
                            <input type="color" :value="store.theme.borderMain"
                                @input="(e: any) => updateThemeColor('borderMain', e.target.value)"
                                class="color-picker" />
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section>
            <h3 class="section-title">
                <span class="material-symbols-outlined section-icon">info</span>
                关于 AutoKB
            </h3>
            <div class="settings-card">
                <div class="about-header">
                    <div class="about-icon">
                        <span class="material-symbols-outlined">auto_fix_high</span>
                    </div>
                    <div>
                        <h4 class="about-title">AutoKB Desktop Automation</h4>
                        <p class="about-version">Version 1.0.0 (Build 20260224)</p>
                    </div>
                </div>
                <p class="about-description">
                    AutoKB 是一款强大的桌面端自动化工具，支持录制、回放及复杂的任务编排。
                    基于 Tauri 2.0 与 Rust 构建，提供高性能与极低的系统占用。
                </p>
                <div class="about-links">
                    <a href="#" class="about-link">检查更新</a>
                    <a href="#" class="about-link">开源许可</a>
                    <a href="#" class="about-link">文档</a>
                </div>
            </div>
        </section>
    </div>
</template>

<style scoped>
/* Container */
.settings-container {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    animation: fadeIn 500ms ease-out, slideInFromBottom 500ms ease-out;
}

@keyframes fadeIn {
    from {
        opacity: 0;
    }
    to {
        opacity: 1;
    }
}

@keyframes slideInFromBottom {
    from {
        transform: translateY(1rem);
    }
    to {
        transform: translateY(0);
    }
}

/* Section Title */
.section-title {
    font-size: 1.125rem;
    font-weight: 700;
    margin-bottom: 1rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.section-icon {
    color: var(--primary);
}

/* Settings Card */
.settings-card {
    background-color: var(--surface);
    border-radius: 0.75rem;
    border: 1px solid var(--border-main);
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

/* Form Label */
.form-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-muted);
    margin-bottom: 0.75rem;
    display: block;
}

/* Preset Grid */
.preset-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
}

@media (min-width: 768px) {
    .preset-grid {
        grid-template-columns: repeat(4, 1fr);
    }
}

.preset-button {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    border-radius: 0.5rem;
    border: 1px solid var(--border-main);
    background: transparent;
    cursor: pointer;
    transition: border-color 150ms ease;
}

.preset-button:hover {
    border-color: var(--primary);
}

.preset-preview {
    width: 100%;
    aspect-ratio: 16 / 9;
    border-radius: 0.25rem;
    border: 1px solid var(--border-main);
    display: flex;
    overflow: hidden;
}

.preset-sidebar {
    width: 33.333%;
    height: 100%;
    border-right: 1px solid var(--border-main);
}

.preset-content {
    flex: 1;
    padding: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.preset-bar-primary {
    height: 0.25rem;
    width: 100%;
    border-radius: 9999px;
}

.preset-bar-text {
    height: 0.25rem;
    width: 66.666%;
    border-radius: 9999px;
    opacity: 0.3;
}

.preset-name {
    font-size: 0.75rem;
    font-weight: 500;
}

/* Divider */
.divider {
    height: 1px;
    background-color: #e2e8f0;
}

:root.dark .divider {
    background-color: #1e293b;
}

/* Color Grid */
.color-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1.5rem;
}

@media (min-width: 768px) {
    .color-grid {
        grid-template-columns: repeat(3, 1fr);
    }
}

.color-column {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.color-control {
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.color-label {
    font-size: 0.875rem;
    font-weight: 500;
}

.color-picker {
    width: 2.5rem;
    height: 2.5rem;
    padding: 0;
    border: 0;
    background-color: transparent;
    cursor: pointer;
    border-radius: 0.25rem;
    overflow: hidden;
}

/* About Section */
.about-header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
}

.about-icon {
    background-color: rgba(19, 91, 236, 0.1);
    padding: 0.75rem;
    border-radius: 1rem;
    color: var(--primary);
    display: flex;
    align-items: center;
    justify-content: center;
}

.about-icon .material-symbols-outlined {
    font-size: 1.875rem;
}

.about-title {
    font-weight: 700;
    color: var(--text-main);
}

.about-version {
    font-size: 0.875rem;
    color: var(--text-muted);
}

.about-description {
    font-size: 0.875rem;
    color: var(--text-muted);
    line-height: 1.625;
    margin-bottom: 1rem;
}

.about-links {
    display: flex;
    gap: 1rem;
}

.about-link {
    color: var(--primary);
    font-size: 0.875rem;
    font-weight: 500;
    text-decoration: none;
}

.about-link:hover {
    text-decoration: underline;
}
</style>
