<script setup lang="ts">
import { ref } from 'vue';
import { useScriptStore } from '../stores/scriptStore';

const store = useScriptStore();

// Form state for creating new macro
const showForm = ref(false);
const macroName = ref('');
const triggerType = ref('mouse');
const triggerValue = ref('right');
const actionType = ref('mouse_click');
const actionValue = ref('left');

const mouseButtons = [
    { value: 'left', label: '左键' },
    { value: 'right', label: '右键' },
    { value: 'middle', label: '中键' },
];

async function createMacro() {
    if (!macroName.value.trim()) {
        return;
    }

    await store.createInputMapping(
        macroName.value,
        triggerType.value,
        triggerValue.value,
        actionType.value,
        actionValue.value
    );

    // Reset form
    macroName.value = '';
    showForm.value = false;
}

function getTriggerDisplay(macro: any): string {
    if (macro.trigger.trigger_type === 'KeyPress') {
        const key = macro.trigger.key;
        return `按键: ${key.type === 'Char' ? key.value.toUpperCase() : key.value}`;
    } else {
        return `鼠标: ${macro.trigger.button}`;
    }
}
</script>

<template>
    <div class="macro-panel">
        <!-- Macro List -->
        <div class="macro-list">
            <div v-if="store.macros.length === 0" class="empty-macros">
                <span>暂无宏定义</span>
            </div>

            <div v-for="macro in store.macros" :key="macro.id" class="macro-item" :class="{ disabled: !macro.enabled }">
                <div class="macro-info">
                    <span class="macro-name">{{ macro.name }}</span>
                    <span class="macro-trigger">{{ getTriggerDisplay(macro) }}</span>
                </div>
                <div class="macro-actions">
                    <button class="toggle-btn" :class="{ active: macro.enabled }"
                        @click="store.toggleMacroEnabled(macro.id, !macro.enabled)"
                        :title="macro.enabled ? '禁用' : '启用'">
                        {{ macro.enabled ? '✓' : '○' }}
                    </button>
                    <button class="remove-btn" @click="store.removeMacro(macro.id)" title="删除">
                        ✕
                    </button>
                </div>
            </div>
        </div>

        <!-- Add Macro Button -->
        <button v-if="!showForm" class="add-macro-btn" @click="showForm = true">
            ➕ 创建宏
        </button>

        <!-- Create Macro Form -->
        <div v-else class="macro-form">
            <div class="form-row">
                <label>宏名称</label>
                <input v-model="macroName" type="text" placeholder="例如: 右键转左键" />
            </div>

            <div class="form-row">
                <label>触发方式</label>
                <select v-model="triggerType">
                    <option value="mouse">鼠标按键</option>
                    <option value="key">键盘按键</option>
                </select>
            </div>

            <div class="form-row" v-if="triggerType === 'mouse'">
                <label>触发按键</label>
                <select v-model="triggerValue">
                    <option v-for="btn in mouseButtons" :key="btn.value" :value="btn.value">
                        {{ btn.label }}
                    </option>
                </select>
            </div>

            <div class="form-row" v-if="triggerType === 'key'">
                <label>触发按键</label>
                <input v-model="triggerValue" type="text" placeholder="输入按键..." maxlength="1" />
            </div>

            <div class="form-row">
                <label>执行动作</label>
                <select v-model="actionType">
                    <option value="mouse_click">鼠标点击</option>
                    <option value="key_press">键盘按键</option>
                </select>
            </div>

            <div class="form-row" v-if="actionType === 'mouse_click'">
                <label>点击按键</label>
                <select v-model="actionValue">
                    <option v-for="btn in mouseButtons" :key="btn.value" :value="btn.value">
                        {{ btn.label }}
                    </option>
                </select>
            </div>

            <div class="form-row" v-if="actionType === 'key_press'">
                <label>按下按键</label>
                <input v-model="actionValue" type="text" placeholder="输入按键..." maxlength="1" />
            </div>

            <div class="form-actions">
                <button class="cancel-btn" @click="showForm = false">取消</button>
                <button class="create-btn" @click="createMacro" :disabled="!macroName.trim()">
                    创建
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
.macro-panel {
    padding: 12px;
}

.macro-list {
    margin-bottom: 12px;
}

.empty-macros {
    padding: 16px;
    text-align: center;
    color: var(--text-muted);
    font-size: 12px;
}

.macro-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    margin-bottom: 6px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
    transition: all var(--transition-fast);
}

.macro-item.disabled {
    opacity: 0.5;
}

.macro-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
}

.macro-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
}

.macro-trigger {
    font-size: 11px;
    color: var(--text-muted);
}

.macro-actions {
    display: flex;
    gap: 6px;
}

.toggle-btn,
.remove-btn {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    font-size: 12px;
    cursor: pointer;
    transition: all var(--transition-fast);
}

.toggle-btn:hover {
    background: var(--accent-success);
    border-color: var(--accent-success);
    color: white;
}

.toggle-btn.active {
    background: var(--accent-success);
    border-color: var(--accent-success);
    color: white;
}

.remove-btn:hover {
    background: var(--accent-danger);
    border-color: var(--accent-danger);
    color: white;
}

/* Add Button */
.add-macro-btn {
    width: 100%;
    padding: 10px;
    background: var(--bg-tertiary);
    border: 1px dashed var(--border-color);
    border-radius: var(--radius-md);
    color: var(--text-secondary);
    font-size: 13px;
    cursor: pointer;
    transition: all var(--transition-fast);
}

.add-macro-btn:hover {
    background: var(--bg-hover);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
}

/* Form */
.macro-form {
    padding: 12px;
    background: var(--bg-tertiary);
    border-radius: var(--radius-md);
    border: 1px solid var(--accent-primary);
}

.form-row {
    margin-bottom: 10px;
}

.form-row label {
    display: block;
    font-size: 11px;
    color: var(--text-muted);
    margin-bottom: 4px;
}

.form-row input,
.form-row select {
    width: 100%;
    padding: 8px 10px;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-size: 13px;
}

.form-row input:focus,
.form-row select:focus {
    outline: none;
    border-color: var(--accent-primary);
}

.form-actions {
    display: flex;
    gap: 8px;
    margin-top: 12px;
}

.cancel-btn,
.create-btn {
    flex: 1;
    padding: 8px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    cursor: pointer;
    transition: all var(--transition-fast);
}

.cancel-btn {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
}

.cancel-btn:hover {
    background: var(--bg-hover);
}

.create-btn {
    background: var(--accent-primary);
    border: none;
    color: white;
}

.create-btn:hover:not(:disabled) {
    background: #4c94e0;
}

.create-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}
</style>
