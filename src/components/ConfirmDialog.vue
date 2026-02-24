<template>
  <Transition name="dialog">
    <div v-if="dialogState.isVisible" class="dialog-overlay" @click="handleBackdropClick">
      <!-- Backdrop -->
      <div class="dialog-backdrop"></div>
      
      <!-- Dialog Container -->
      <div 
        class="dialog-container"
        role="dialog"
        aria-modal="true"
        :aria-labelledby="dialogState.title ? 'dialog-title' : undefined"
        aria-describedby="dialog-message"
        @click.stop
      >
        <!-- Dialog Content -->
        <div class="dialog-content">
          <!-- Title -->
          <h3 
            v-if="dialogState.title" 
            id="dialog-title"
            class="dialog-title"
          >
            {{ dialogState.title }}
          </h3>
          
          <!-- Message -->
          <p 
            id="dialog-message"
            class="dialog-message"
          >
            {{ dialogState.message }}
          </p>
        </div>
        
        <!-- Actions -->
        <div class="dialog-actions">
          <button
            ref="cancelButtonRef"
            @click="handleCancel"
            class="dialog-button dialog-button-cancel"
          >
            {{ dialogState.cancelText }}
          </button>
          <button
            ref="confirmButtonRef"
            @click="handleConfirm"
            :class="[
              'dialog-button dialog-button-confirm',
              dialogState.confirmButtonClass
            ]"
          >
            {{ dialogState.confirmText }}
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';
import { useConfirmDialog } from '../composables/useConfirmDialog';

const { dialogState, handleConfirm, handleCancel } = useConfirmDialog();

const confirmButtonRef = ref<HTMLButtonElement | null>(null);
const cancelButtonRef = ref<HTMLButtonElement | null>(null);

/**
 * Handle backdrop click (treat as cancel)
 */
const handleBackdropClick = () => {
  handleCancel();
};

/**
 * Handle keyboard events
 */
const handleKeydown = (event: KeyboardEvent) => {
  if (!dialogState.isVisible) return;
  
  if (event.key === 'Enter') {
    event.preventDefault();
    event.stopPropagation();
    handleConfirm();
  } else if (event.key === 'Escape') {
    event.preventDefault();
    event.stopPropagation();
    handleCancel();
  }
};

// Watch for dialog visibility changes to focus the confirm button
watch(() => dialogState.isVisible, (isVisible) => {
  if (isVisible) {
    // Focus the confirm button when dialog opens
    setTimeout(() => {
      confirmButtonRef.value?.focus();
    }, 100);
  }
});

// Add keyboard event listener
onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});

// Clean up event listener
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<style scoped>
/* Dialog overlay - fixed full screen with flex centering */
.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 9998;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Dialog backdrop - blurred dark background */
.dialog-backdrop {
  position: absolute;
  inset: 0;
  background-color: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
}

/* Dialog container - the modal box */
.dialog-container {
  position: relative;
  z-index: 9999;
  background-color: var(--surface);
  border-radius: 0.75rem;
  box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
  width: 100%;
  max-width: 28rem;
  margin: 0 1rem;
  overflow: hidden;
}

/* Dialog content area */
.dialog-content {
  padding: 1.5rem;
}

/* Dialog title */
.dialog-title {
  font-size: 1.125rem;
  font-weight: 700;
  color: var(--text-main);
  margin-bottom: 0.75rem;
}

/* Dialog message */
.dialog-message {
  color: var(--text-muted);
  font-size: 0.875rem;
  line-height: 1.625;
}

/* Dialog actions footer */
.dialog-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 1rem 1.5rem;
  background-color: var(--surface-soft);
  border-top: 1px solid var(--border-main);
}

/* Base button styles */
.dialog-button {
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  font-weight: 500;
  font-size: 0.875rem;
  transition: background-color 150ms, color 150ms;
  border: none;
  cursor: pointer;
}

/* Cancel button */
.dialog-button-cancel {
  color: var(--text-muted);
  background-color: transparent;
}

.dialog-button-cancel:hover {
  background-color: var(--surface);
}

/* Confirm button - default primary style */
.dialog-button-confirm {
  color: white;
  background-color: var(--primary);
}

.dialog-button-confirm:hover {
  background-color: var(--primary);
  opacity: 0.9;
}

/* Dialog transition animations */
.dialog-enter-active,
.dialog-leave-active {
  transition: opacity 200ms ease-out;
}

.dialog-enter-active .dialog-container,
.dialog-leave-active .dialog-container {
  transition: transform 200ms ease-out, opacity 200ms ease-out;
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
}

.dialog-enter-from .dialog-container,
.dialog-leave-to .dialog-container {
  transform: scale(0.95);
  opacity: 0;
}

.dialog-enter-to .dialog-container,
.dialog-leave-from .dialog-container {
  transform: scale(1);
  opacity: 1;
}
</style>
