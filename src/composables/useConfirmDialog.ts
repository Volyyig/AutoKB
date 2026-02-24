import { reactive } from 'vue';

/**
 * 确认对话框选项接口
 */
export interface ConfirmOptions {
  title?: string;              // 对话框标题，默认 "确认操作"
  message: string;             // 对话框消息内容
  confirmText?: string;        // 确认按钮文本，默认 "确认"
  cancelText?: string;         // 取消按钮文本，默认 "取消"
  confirmButtonClass?: string; // 确认按钮自定义样式类
}

/**
 * 对话框状态接口
 */
export interface DialogState {
  isVisible: boolean;                          // 对话框是否可见
  title: string;                               // 当前标题
  message: string;                             // 当前消息
  confirmText: string;                         // 确认按钮文本
  cancelText: string;                          // 取消按钮文本
  confirmButtonClass: string;                  // 确认按钮样式类
  resolver: ((value: boolean) => void) | null; // Promise resolver
}

/**
 * 全局对话框状态
 */
export const dialogState = reactive<DialogState>({
  isVisible: false,
  title: '',
  message: '',
  confirmText: '确认',
  cancelText: '取消',
  confirmButtonClass: '',
  resolver: null
});

/**
 * 确认对话框组合式 API
 */
export function useConfirmDialog() {
  /**
   * 显示确认对话框并返回用户选择
   * @param options 对话框选项
   * @returns Promise<boolean> - true 表示确认，false 表示取消
   */
  const confirm = (options: ConfirmOptions): Promise<boolean> => {
    return new Promise<boolean>((resolve) => {
      // 更新对话框状态
      dialogState.title = options.title || '确认操作';
      dialogState.message = options.message;
      dialogState.confirmText = options.confirmText || '确认';
      dialogState.cancelText = options.cancelText || '取消';
      dialogState.confirmButtonClass = options.confirmButtonClass || '';
      dialogState.resolver = resolve;
      dialogState.isVisible = true;
    });
  };

  /**
   * 处理确认操作
   */
  const handleConfirm = () => {
    if (dialogState.resolver) {
      dialogState.resolver(true);
      dialogState.resolver = null;
    }
    dialogState.isVisible = false;
  };

  /**
   * 处理取消操作
   */
  const handleCancel = () => {
    if (dialogState.resolver) {
      dialogState.resolver(false);
      dialogState.resolver = null;
    }
    dialogState.isVisible = false;
  };

  return {
    confirm,
    handleConfirm,
    handleCancel,
    dialogState
  };
}
