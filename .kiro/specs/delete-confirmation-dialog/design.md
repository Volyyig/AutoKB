# 删除确认对话框 - 设计文档

## 概述

本设计文档描述了如何创建一个自定义的确认对话框组件，用于替换系统原生的 `confirm()` 对话框，并在所有删除操作中统一使用。该组件将与应用的整体设计风格保持一致，提供更好的用户体验。

## 架构

### 组件结构

```
src/components/
  ├── ConfirmDialog.vue          # 确认对话框组件
  └── (existing components)

src/composables/
  └── useConfirmDialog.ts        # 对话框组合式 API
```

### 设计模式

采用 **组合式 API + Promise** 模式：
- 使用 Vue 3 Composition API 创建可复用的对话框逻辑
- 通过 Promise 处理异步确认流程
- 使用全局状态管理对话框的显示和隐藏

## 组件和接口

### ConfirmDialog 组件

**Props:**
```typescript
interface ConfirmDialogProps {
  // 通过 composable 管理，不需要 props
}
```

**组件职责：**
- 渲染对话框 UI（标题、消息、按钮）
- 处理用户交互（点击按钮、键盘事件）
- 提供动画效果（淡入淡出）
- 响应遮罩层点击

### useConfirmDialog Composable

**接口定义：**
```typescript
interface ConfirmOptions {
  title?: string;           // 对话框标题，默认 "确认操作"
  message: string;          // 对话框消息内容
  confirmText?: string;     // 确认按钮文本，默认 "确认"
  cancelText?: string;      // 取消按钮文本，默认 "取消"
  confirmButtonClass?: string; // 确认按钮自定义样式类
}

interface UseConfirmDialog {
  confirm: (options: ConfirmOptions) => Promise<boolean>;
}
```

**使用示例：**
```typescript
const { confirm } = useConfirmDialog();

async function deleteScript(path: string) {
  const confirmed = await confirm({
    title: '删除脚本',
    message: '确定要彻底删除此脚本吗？此操作无法撤销。',
    confirmText: '删除',
    cancelText: '取消'
  });
  
  if (confirmed) {
    // 执行删除操作
  }
}
```

## 数据模型

### 对话框状态

```typescript
interface DialogState {
  isVisible: boolean;        // 对话框是否可见
  title: string;             // 当前标题
  message: string;           // 当前消息
  confirmText: string;       // 确认按钮文本
  cancelText: string;        // 取消按钮文本
  confirmButtonClass: string; // 确认按钮样式类
  resolver: ((value: boolean) => void) | null; // Promise resolver
}
```

### 状态管理

使用 `ref` 和 `reactive` 管理对话框状态：
```typescript
const dialogState = reactive<DialogState>({
  isVisible: false,
  title: '',
  message: '',
  confirmText: '确认',
  cancelText: '取消',
  confirmButtonClass: '',
  resolver: null
});
```

## 正确性属性

*属性是一个特征或行为，应该在系统的所有有效执行中保持为真——本质上是关于系统应该做什么的形式化陈述。属性作为人类可读规范和机器可验证正确性保证之间的桥梁。*

### Property 1: 自定义内容显示
*对于任意*的标题和消息内容，当调用 confirm() 时，对话框应该正确显示这些内容
**验证需求：3.1.3**

### Property 2: 键盘操作响应
*对于任意*打开的对话框实例，按下 Enter 键应该触发确认操作并返回 true，按下 Escape 键应该触发取消操作并返回 false
**验证需求：3.2.3**

### Property 3: 用户选择反映
*对于任意*对话框调用，点击"确认"按钮应该 resolve Promise 为 true，点击"取消"按钮或遮罩层应该 resolve Promise 为 false
**验证需求：3.3.2, 3.3.3**

## 错误处理

### 键盘事件冲突
- 使用事件捕获阶段处理键盘事件
- 调用 `preventDefault()` 和 `stopPropagation()` 防止事件冒泡
- 确保对话框关闭时移除事件监听器

### 多次调用处理
- 如果对话框已经打开，新的 confirm() 调用应该排队等待
- 或者拒绝新的调用并返回 false（更简单的实现）

### 组件卸载
- 在组件卸载时清理所有事件监听器
- 如果对话框打开时组件卸载，自动 resolve 为 false

## 测试策略

### 单元测试
使用 Vitest 和 Vue Test Utils 进行组件测试：

1. **对话框渲染测试**
   - 测试对话框初始状态为隐藏
   - 测试调用 confirm() 后对话框显示
   - 测试自定义标题和消息正确显示
   - 测试按钮文本自定义

2. **用户交互测试**
   - 测试点击确认按钮返回 true
   - 测试点击取消按钮返回 false
   - 测试点击遮罩层返回 false
   - 测试 Enter 键触发确认
   - 测试 Escape 键触发取消

3. **边缘情况测试**
   - 测试空消息内容
   - 测试超长消息内容
   - 测试特殊字符处理

### 属性测试
使用 fast-check 进行属性测试（每个测试至少 100 次迭代）：

1. **Property 1: 自定义内容显示**
   ```typescript
   // Feature: delete-confirmation-dialog, Property 1: 自定义内容显示
   test('对话框应该显示任意自定义内容', async () => {
     fc.assert(
       fc.asyncProperty(
         fc.string(), // 标题
         fc.string(), // 消息
         async (title, message) => {
           const { confirm } = useConfirmDialog();
           const promise = confirm({ title, message });
           // 验证 DOM 中包含标题和消息
           await nextTick();
           expect(document.body.textContent).toContain(title);
           expect(document.body.textContent).toContain(message);
           // 清理
           await clickCancel();
         }
       ),
       { numRuns: 100 }
     );
   });
   ```

2. **Property 2: 键盘操作响应**
   ```typescript
   // Feature: delete-confirmation-dialog, Property 2: 键盘操作响应
   test('键盘操作应该正确响应', async () => {
     fc.assert(
       fc.asyncProperty(
         fc.constantFrom('Enter', 'Escape'),
         async (key) => {
           const { confirm } = useConfirmDialog();
           const promise = confirm({ message: 'Test' });
           await nextTick();
           
           // 模拟键盘事件
           const event = new KeyboardEvent('keydown', { key });
           window.dispatchEvent(event);
           
           const result = await promise;
           expect(result).toBe(key === 'Enter');
         }
       ),
       { numRuns: 100 }
     );
   });
   ```

3. **Property 3: 用户选择反映**
   ```typescript
   // Feature: delete-confirmation-dialog, Property 3: 用户选择反映
   test('用户选择应该正确反映在返回值中', async () => {
     fc.assert(
       fc.asyncProperty(
         fc.constantFrom('confirm', 'cancel', 'backdrop'),
         async (action) => {
           const { confirm } = useConfirmDialog();
           const promise = confirm({ message: 'Test' });
           await nextTick();
           
           // 根据动作类型触发相应操作
           if (action === 'confirm') {
             await clickConfirmButton();
           } else if (action === 'cancel') {
             await clickCancelButton();
           } else {
             await clickBackdrop();
           }
           
           const result = await promise;
           expect(result).toBe(action === 'confirm');
         }
       ),
       { numRuns: 100 }
     );
   });
   ```

### 集成测试
测试在实际组件中的使用：

1. **VisualScriptEditor 集成**
   - 测试 deleteScript() 调用显示对话框
   - 测试确认后脚本被删除
   - 测试取消后脚本保留

2. **ScriptLibrary 集成**
   - 测试 deleteScript() 调用显示对话框
   - 测试确认后脚本从列表中移除

3. **TaskDashboard 集成**
   - 测试 removeTask() 调用显示对话框
   - 测试确认后任务被删除

## 实现细节

### 样式设计

遵循应用现有的设计系统：
- 使用 Tailwind CSS 类
- 颜色：使用 CSS 变量（`--color-*`）
- 字体：使用 `font-display` 类
- 圆角：`rounded-xl` 用于对话框，`rounded-lg` 用于按钮
- 阴影：`shadow-2xl` 用于对话框
- 动画：使用 Vue Transition 组件

### 动画效果

**对话框动画：**
- 淡入：opacity 0 → 1，scale 0.95 → 1
- 淡出：opacity 1 → 0，scale 1 → 0.95
- 持续时间：200ms
- 缓动函数：ease-out

**遮罩层动画：**
- 淡入：opacity 0 → 1
- 淡出：opacity 1 → 0
- 持续时间：200ms

### 可访问性

- 使用 `role="dialog"` 和 `aria-modal="true"`
- 使用 `aria-labelledby` 关联标题
- 使用 `aria-describedby` 关联消息
- 对话框打开时聚焦到确认按钮
- 支持 Tab 键在按钮间切换
- 支持 Escape 键关闭对话框

### Z-index 管理

确保对话框在所有内容之上：
- 遮罩层：`z-[9998]`
- 对话框：`z-[9999]`
- 与 ToastNotification 的 z-index (9999) 相同级别

## 迁移计划

### 第一阶段：创建组件
1. 创建 `ConfirmDialog.vue` 组件
2. 创建 `useConfirmDialog.ts` composable
3. 在 `App.vue` 中添加 `<ConfirmDialog />` 组件

### 第二阶段：替换现有调用
1. 替换 `VisualScriptEditor.vue` 中的 3 处 `confirm()` 调用
2. 替换 `ScriptLibrary.vue` 中的 1 处 `confirm()` 调用
3. 为 `TaskDashboard.vue` 添加确认对话框

### 第三阶段：测试和优化
1. 编写单元测试
2. 编写属性测试
3. 进行集成测试
4. 优化动画和交互体验

## 技术栈

- **框架：** Vue 3 Composition API
- **语言：** TypeScript
- **样式：** Tailwind CSS
- **测试：** Vitest + Vue Test Utils + fast-check
- **运行环境：** Tauri

## 参考

- Vue 3 Composition API: https://vuejs.org/api/composition-api-setup.html
- Vue Transition: https://vuejs.org/guide/built-ins/transition.html
- ARIA Dialog Pattern: https://www.w3.org/WAI/ARIA/apg/patterns/dialog-modal/
