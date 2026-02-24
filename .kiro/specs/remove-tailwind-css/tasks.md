# 实施计划：移除 Tailwind CSS 依赖

## 概述

本实施计划将指导完成从 AutoKB 项目中移除 Tailwind CSS 依赖的过程。任务按照依赖关系组织，确保每一步都建立在前一步的基础上。整个过程分为 4 个主要阶段：准备工作、核心转换、组件转换和验证测试。

## 任务

- [x] 1. 准备工作和依赖清理
  - 备份当前项目状态（创建 git 分支）
  - 从 package.json 移除 @tailwindcss/postcss 和 tailwindcss 依赖
  - 更新 postcss.config.js，移除 Tailwind 插件
  - 执行 pnpm install 验证依赖安装成功
  - _需求：1.1, 1.2, 1.3, 1.4, 2.1, 2.2_

- [ ] 2. 转换主样式文件（src/index.css）
  - [x] 2.1 移除 Tailwind 指令
    - 移除 @import "tailwindcss" 指令
    - 移除 @theme 指令块
    - 将 body 样式从 @apply 转换为标准 CSS 属性
    - _需求：3.1, 3.2, 3.3, 3.5_
  
  - [ ]* 2.2 验证 CSS 变量完整性（属性测试）
    - **属性 1：CSS 变量完整性**
    - **验证：需求 3.4**
  
  - [x] 2.3 测试开发服务器启动
    - 执行 pnpm dev 验证无 CSS 错误
    - _需求：12.1_

- [x] 3. 检查点 - 确保基础配置正确
  - 确保所有测试通过，询问用户是否有问题

- [ ] 4. 转换 App.vue 组件
  - [x] 4.1 创建语义化 CSS 类
    - 在 &lt;style scoped&gt; 中定义 .app-container, .sidebar, .nav-button, .main-content, .header 等类
    - 将所有 Tailwind 工具类替换为自定义类名
    - 使用 CSS 变量引用主题颜色
    - _需求：4.1, 4.2, 4.3_
  
  - [ ]* 4.2 验证无 Tailwind 类残留（属性测试）
    - **属性 2：无 Tailwind 工具类残留**
    - **验证：需求 4.1**
  
  - [ ]* 4.3 验证 CSS 变量使用（属性测试）
    - **属性 3：CSS 变量优先使用**
    - **验证：需求 4.3**

- [ ] 5. 转换 VisualScriptEditor.vue 组件
  - [x] 5.1 创建编辑器样式类
    - 定义 .visual-editor, .toolbar, .sidebar, .canvas, .node-card 等类
    - 转换所有 Tailwind 类为自定义 CSS
    - 保持复杂定位和层级（fixed, absolute, z-index）
    - _需求：5.1, 5.2, 5.3, 5.4_
  
  - [ ]* 5.2 验证样式转换（单元测试）
    - 测试编辑器布局和交互状态
    - _需求：5.5, 5.6_

- [ ] 6. 转换 TaskDashboard.vue 组件
  - [x] 6.1 创建仪表板样式类
    - 定义 .task-dashboard, .table-container, .table-header, .table-row, .status-badge 等类
    - 转换表格和卡片样式
    - 保持动态状态类（getStatusClass 函数）
    - _需求：7.1, 7.2, 7.3, 7.4_

- [ ] 7. 转换 SettingsView.vue 组件
  - [x] 7.1 创建设置页面样式类
    - 定义 .settings-card, .preset-grid, .color-picker, .form-control 等类
    - 转换网格布局和表单控件样式
    - 保持响应式断点（grid-cols-2 md:grid-cols-4）
    - _需求：8.1, 8.2, 8.3, 8.4_

- [ ] 8. 更新 ToastNotification.vue 组件
  - [x] 8.1 修正 CSS 变量引用
    - 将 var(--color-bg-secondary) 改为 var(--surface)
    - 将 var(--color-text-primary) 改为 var(--text-main)
    - 将 var(--color-accent) 改为 var(--primary)
    - 确保状态颜色变量正确
    - _需求：9.1, 9.2, 9.3_

- [ ] 9. 转换 ConfirmDialog.vue 组件
  - [x] 9.1 创建对话框样式类
    - 定义 .dialog-overlay, .dialog-backdrop, .dialog-container, .dialog-button 等类
    - 转换模态框和按钮样式
    - 保持过渡动画
    - _需求：10.1, 10.2, 10.3, 10.4_

- [ ] 10. 转换 ScriptLibrary.vue 组件
  - [x] 10.1 创建脚本库样式类
    - 定义 .script-library, .script-grid, .script-card 等类
    - 转换网格布局和卡片样式
    - _需求：6.1, 6.2, 6.3, 6.4_

- [x] 11. 检查点 - 确保所有组件转换完成
  - 确保所有测试通过，询问用户是否有问题

- [ ]* 12. 代码质量验证（属性测试）
  - [ ]* 12.1 验证语义化类名
    - **属性 4：语义化类名**
    - **验证：需求 14.1**
  
  - [ ]* 12.2 验证选择器嵌套深度
    - **属性 5：选择器嵌套深度限制**
    - **验证：需求 14.3**
  
  - [ ]* 12.3 验证无硬编码颜色
    - **属性 6：无硬编码主题颜色**
    - **验证：需求 14.4**

- [ ] 13. 深色模式验证
  - [x] 13.1 测试深色模式切换
    - 在浏览器中测试深色模式切换功能
    - 验证所有组件在深色模式下的显示
    - _需求：11.1_
  
  - [ ]* 13.2 深色模式集成测试
    - 使用 Playwright 或 Puppeteer 自动化测试
    - _需求：11.2, 11.3, 11.4, 11.5_

- [ ] 14. 构建和部署验证
  - [x] 14.1 执行生产构建
    - 运行 pnpm build 命令
    - 验证构建成功且无 CSS 错误
    - _需求：12.2_
  
  - [ ]* 14.2 验证构建产物（属性测试）
    - **属性 7：构建产物无 Tailwind 代码**
    - **验证：需求 12.5**
  
  - [x] 14.3 手动测试构建产物
    - 在浏览器中打开 dist/index.html
    - 验证所有功能正常工作
    - _需求：12.3, 12.4_

- [ ] 15. 文档更新
  - [x] 15.1 更新 README.md
    - 移除 Tailwind CSS 相关说明
    - 添加原生 CSS 样式系统说明
    - 记录 CSS 变量主题系统使用方法
    - 提供深色模式切换实现说明
    - _需求：15.1, 15.2, 15.3, 15.4_

- [x] 16. 最终检查点
  - 确保所有测试通过，询问用户是否有问题

## 注意事项

- 标记为 `*` 的任务是可选的，可以跳过以加快 MVP 开发
- 每个任务都引用了具体的需求以确保可追溯性
- 检查点确保增量验证
- 属性测试验证通用正确性属性
- 单元测试验证特定示例和边缘情况
