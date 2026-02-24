# 删除确认对话框 - 实现任务

## 概述

本任务列表将指导实现自定义确认对话框组件，并替换所有现有的原生 `confirm()` 调用。实现将采用 Vue 3 Composition API + TypeScript，确保代码的可维护性和类型安全。

## 任务列表

- [x] 1. 创建确认对话框基础组件
  - [x] 1.1 创建 `src/composables/useConfirmDialog.ts` 文件
    - 定义 `ConfirmOptions` 和 `DialogState` 接口
    - 实现 `confirm()` 函数，返回 Promise<boolean>
    - 实现对话框状态管理（显示/隐藏、内容更新）
    - 实现 `handleConfirm()` 和 `handleCancel()` 方法
    - _需求：3.1.3, 3.3.2, 3.3.3_
  
  - [x] 1.2 创建 `src/components/ConfirmDialog.vue` 组件
    - 实现对话框 UI 结构（遮罩层、对话框容器、标题、消息、按钮）
    - 使用 Tailwind CSS 样式，保持与应用设计一致
    - 添加 Vue Transition 动画效果
    - 绑定 useConfirmDialog 的状态和方法
    - _需求：3.1.1, 3.1.2, 3.2.1_
  
  - [x] 1.3 添加键盘事件处理
    - 监听 `keydown` 事件
    - Enter 键触发确认操作
    - Escape 键触发取消操作
    - 确保事件监听器在对话框关闭时清理
    - _需求：3.2.3_
  
  - [x] 1.4 添加遮罩层点击处理
    - 点击遮罩层触发取消操作
    - 点击对话框内容区域不触发关闭
    - _需求：3.3.3_

- [x] 2. 集成对话框到应用
  - [x] 2.1 在 `src/App.vue` 中添加 ConfirmDialog 组件
    - 导入 ConfirmDialog 组件
    - 在模板中添加 `<ConfirmDialog />` 标签
    - 确保对话框在所有页面中可用
    - _需求：3.1.1_

- [x] 3. 替换 VisualScriptEditor 中的 confirm() 调用
  - [x] 3.1 替换 `deleteScript()` 方法
    - 导入 `useConfirmDialog`
    - 将 `confirm()` 替换为 `await confirm({ ... })`
    - 自定义对话框标题和消息
    - _需求：3.3.1, 3.3.2, 3.3.3_
  
  - [x] 3.2 替换 `deleteGroup()` 方法
    - 将 `confirm()` 替换为 `await confirm({ ... })`
    - 自定义对话框标题和消息
    - _需求：3.3.1, 3.3.2, 3.3.3_
  
  - [x] 3.3 替换 `clearEvents()` 方法
    - 将 `confirm()` 替换为 `await confirm({ ... })`
    - 自定义对话框标题和消息
    - _需求：3.3.1, 3.3.2, 3.3.3_

- [x] 4. 替换 ScriptLibrary 中的 confirm() 调用
  - [x] 4.1 替换 `deleteScript()` 方法
    - 导入 `useConfirmDialog`
    - 将 `confirm()` 替换为 `await confirm({ ... })`
    - 自定义对话框标题和消息
    - _需求：3.3.1, 3.3.2, 3.3.3_

- [x] 5. 为 TaskDashboard 添加确认对话框
  - [x] 5.1 修改 `removeTask()` 方法添加确认
    - 导入 `useConfirmDialog`
    - 在删除前调用 `await confirm({ ... })`
    - 只有确认后才执行 `store.removeTask()`
    - 自定义对话框标题和消息
    - _需求：3.3.1, 3.3.2, 3.3.3_

- [ ] 6. 编写单元测试
  - [ ]* 6.1 测试 useConfirmDialog composable
    - 测试 confirm() 函数返回 Promise
    - 测试对话框状态更新
    - 测试 handleConfirm() 返回 true
    - 测试 handleCancel() 返回 false
    - _需求：3.1.3, 3.3.2, 3.3.3_
  
  - [ ]* 6.2 测试 ConfirmDialog 组件渲染
    - 测试对话框初始状态为隐藏
    - 测试调用 confirm() 后对话框显示
    - 测试自定义标题和消息正确显示
    - 测试按钮文本自定义
    - _需求：3.1.3, 3.2.1_
  
  - [ ]* 6.3 测试用户交互
    - 测试点击确认按钮
    - 测试点击取消按钮
    - 测试点击遮罩层
    - 测试 Enter 键
    - 测试 Escape 键
    - _需求：3.2.3, 3.3.2, 3.3.3_

- [ ] 7. 编写属性测试
  - [ ]* 7.1 编写 Property 1 测试：自定义内容显示
    - **Property 1: 自定义内容显示**
    - **验证需求：3.1.3**
    - 使用 fast-check 生成随机标题和消息
    - 验证对话框正确显示内容
    - 至少 100 次迭代
  
  - [ ]* 7.2 编写 Property 2 测试：键盘操作响应
    - **Property 2: 键盘操作响应**
    - **验证需求：3.2.3**
    - 测试 Enter 和 Escape 键的响应
    - 验证返回值正确
    - 至少 100 次迭代
  
  - [ ]* 7.3 编写 Property 3 测试：用户选择反映
    - **Property 3: 用户选择反映**
    - **验证需求：3.3.2, 3.3.3**
    - 测试确认、取消、遮罩层点击
    - 验证 Promise 返回值正确
    - 至少 100 次迭代

- [ ] 8. 集成测试和验证
  - [ ]* 8.1 测试 VisualScriptEditor 集成
    - 测试 deleteScript() 显示对话框
    - 测试确认后脚本被删除
    - 测试取消后脚本保留
    - _需求：3.3.1, 3.3.2, 3.3.3_
  
  - [ ]* 8.2 测试 ScriptLibrary 集成
    - 测试 deleteScript() 显示对话框
    - 测试确认后脚本从列表移除
    - _需求：3.3.1, 3.3.2, 3.3.3_
  
  - [ ]* 8.3 测试 TaskDashboard 集成
    - 测试 removeTask() 显示对话框
    - 测试确认后任务被删除
    - _需求：3.3.1, 3.3.2, 3.3.3_

- [ ] 9. 最终检查点
  - 确保所有测试通过
  - 验证所有删除操作都使用了确认对话框
  - 检查对话框样式与应用设计一致
  - 测试键盘操作和动画效果
  - 如有问题，请向用户反馈

## 注意事项

- 标记 `*` 的任务为可选测试任务，可以跳过以加快 MVP 开发
- 每个任务都引用了具体的需求编号，便于追溯
- 建议按顺序执行任务，确保每个步骤都正确完成后再进行下一步
- 在第 9 步检查点时，应该运行应用并手动测试所有删除操作
