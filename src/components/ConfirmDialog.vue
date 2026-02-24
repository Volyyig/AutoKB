<template>
  <Transition name="dialog">
    <div v-if="dialogState.isVisible" class="fixed inset-0 z-[9998] flex items-center justify-center" @click="handleBackdropClick">
      <!-- Backdrop -->
      <div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>
      
      <!-- Dialog Container -->
      <div 
        class="relative z-[9999] bg-surface-main rounded-xl shadow-2xl w-full max-w-md mx-4 overflow-hidden"
        role="dialog"
        aria-modal="true"
        :aria-labelledby="dialogState.title ? 'dialog-title' : undefined"
        aria-describedby="dialog-message"
        @click.stop
      >
        <!-- Dialog Content -->
        <div class="p-6">
          <!-- Title -->
          <h3 
            v-if="dialogState.title" 
            id="dialog-title"
            class="text-lg font-bold text-text-main mb-3"
          >
            {{ dialogState.title }}
          </h3>
          
          <!-- Message -->
          <p 
            id="dialog-message"
            class="text-text-muted text-sm leading-relaxed"
          >
            {{ dialogState.message }}
          </p>
        </div>
        
        <!-- Actions -->
        <div class="flex items-center justify-end gap-3 px-6 py-4 bg-surface-soft border-t border-border-main">
          <button
            ref="cancelButtonRef"
            @click="handleCancel"
            class="px-4 py-2 rounded-lg font-medium text-sm text-text-muted hover:bg-surface-main transition-colors"
          >
            {{ dialogState.cancelText }}
          </button>
          <button
            ref="confirmButtonRef"
            @click="handleConfirm"
            :class="[
              'px-4 py-2 rounded-lg font-medium text-sm text-white transition-colors',
              dialogState.confirmButtonClass || 'bg-primary hover:bg-primary/90'
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
/* Dialog transition animations */
.dialog-enter-active,
.dialog-leave-active {
  transition: opacity 200ms ease-out;
}

.dialog-enter-active .relative,
.dialog-leave-active .relative {
  transition: transform 200ms ease-out, opacity 200ms ease-out;
}

.dialog-enter-from,
.dialog-leave-to {
  opacity: 0;
}

.dialog-enter-from .relative,
.dialog-leave-to .relative {
  transform: scale(0.95);
  opacity: 0;
}

.dialog-enter-to .relative,
.dialog-leave-from .relative {
  transform: scale(1);
  opacity: 1;
}
</style>
