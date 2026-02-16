<template>
    <div class="toast-container">
        <TransitionGroup name="toast">
            <div v-for="notification in store.notifications" :key="notification.id" class="toast"
                :class="notification.type" @click="store.removeNotification(notification.id)">
                <div class="toast-icon">
                    <span v-if="notification.type === 'success'">✓</span>
                    <span v-else-if="notification.type === 'error'">✕</span>
                    <span v-else>ℹ</span>
                </div>
                <div class="toast-message">{{ notification.message }}</div>
            </div>
        </TransitionGroup>
    </div>
</template>

<script setup lang="ts">
import { useScriptStore } from '../stores/scriptStore';

const store = useScriptStore();
</script>

<style scoped>
.toast-container {
    position: fixed;
    top: 20px;
    right: 20px;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 10px;
    pointer-events: none;
}

.toast {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    background-color: var(--color-bg-secondary);
    color: var(--color-text-primary);
    border-radius: 6px;
    box-shadow: var(--shadow-md);
    border-left: 4px solid var(--color-accent);
    min-width: 250px;
    max-width: 400px;
    cursor: pointer;
    pointer-events: auto;
    font-size: 14px;
}

.toast.success {
    border-left-color: var(--color-success);
}

.toast.error {
    border-left-color: var(--color-danger);
}

.toast.info {
    border-left-color: var(--color-accent);
}

.toast-icon {
    margin-right: 10px;
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background-color: rgba(255, 255, 255, 0.1);
}

.toast.success .toast-icon {
    color: var(--color-success);
    background-color: rgba(16, 185, 129, 0.1);
}

.toast.error .toast-icon {
    color: var(--color-danger);
    background-color: rgba(239, 68, 68, 0.1);
}

.toast-message {
    flex: 1;
}

/* Transitions */
.toast-enter-active,
.toast-leave-active {
    transition: all 0.3s ease;
}

.toast-enter-from {
    opacity: 0;
    transform: translateX(30px);
}

.toast-leave-to {
    opacity: 0;
    transform: translateX(30px);
}
</style>
