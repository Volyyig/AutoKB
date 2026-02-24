# 需求文档：移除 Tailwind CSS 依赖

## 简介

本项目旨在从 AutoKB 桌面自动化应用中完全移除 Tailwind CSS 依赖，将所有组件的样式转换为原生 CSS，同时保持现有的设计系统、视觉效果和深色模式功能。

## 术语表

- **System**: AutoKB 前端应用系统
- **Tailwind_CSS**: 当前使用的 CSS 工具类框架（v4.2.1）
- **Native_CSS**: 原生 CSS 样式（不依赖任何 CSS 框架）
- **CSS_Variables**: CSS 自定义属性（CSS Custom Properties），用于主题系统
- **Utility_Classes**: Tailwind 提供的工具类（如 flex、bg-primary、p-4 等）
- **Component**: Vue 单文件组件（.vue 文件）
- **Dark_Mode**: 深色主题模式
- **Design_System**: 现有的视觉设计系统（颜色、间距、字体等）
- **PostCSS_Config**: PostCSS 配置文件（postcss.config.js）
- **Package_Dependencies**: package.json 中的依赖项

## 需求

### 需求 1：移除 Tailwind CSS 依赖包

**用户故事：** 作为开发者，我希望从项目中移除所有 Tailwind CSS 相关的依赖包，以减少项目体积和构建复杂度。

#### 验收标准

1. WHEN 执行依赖移除操作时，THE System SHALL 从 package.json 的 devDependencies 中移除 @tailwindcss/postcss 包
2. WHEN 执行依赖移除操作时，THE System SHALL 从 package.json 的 devDependencies 中移除 tailwindcss 包
3. WHEN 执行依赖移除操作时，THE System SHALL 保留 autoprefixer 和 postcss 包（用于其他 CSS 处理）
4. WHEN 移除依赖后，THE System SHALL 能够成功执行 pnpm install 而不产生依赖错误

### 需求 2：更新 PostCSS 配置

**用户故事：** 作为开发者，我希望更新 PostCSS 配置以移除 Tailwind 插件，同时保留其他必要的 CSS 处理功能。

#### 验收标准

1. WHEN 更新 PostCSS 配置时，THE System SHALL 从 postcss.config.js 中移除 @tailwindcss/postcss 插件
2. WHEN 更新 PostCSS 配置时，THE System SHALL 保留 autoprefixer 插件配置
3. WHEN 构建项目时，THE System SHALL 能够正常处理 CSS 文件而不依赖 Tailwind

### 需求 3：转换主样式文件

**用户故事：** 作为开发者，我希望将 src/index.css 中的 Tailwind 指令转换为纯 CSS，同时保留和优化 CSS 变量主题系统。

#### 验收标准

1. WHEN 转换主样式文件时，THE System SHALL 移除 @import "tailwindcss" 指令
2. WHEN 转换主样式文件时，THE System SHALL 移除 @theme 指令块
3. WHEN 转换主样式文件时，THE System SHALL 移除 @apply 指令
4. WHEN 转换主样式文件时，THE System SHALL 保留所有 CSS 变量定义（:root 和 :root.dark）
5. WHEN 转换主样式文件时，THE System SHALL 将 body 样式从 @apply 指令转换为标准 CSS 属性
6. WHEN 转换主样式文件时，THE System SHALL 保留所有自定义过渡动画定义

### 需求 4：转换 App.vue 组件样式

**用户故事：** 作为开发者，我希望将 App.vue 中的所有 Tailwind 工具类转换为原生 CSS 类，同时保持相同的视觉效果。

#### 验收标准

1. WHEN 转换 App.vue 样式时，THE System SHALL 将所有 Tailwind 工具类替换为自定义 CSS 类名
2. WHEN 转换 App.vue 样式时，THE System SHALL 在 &lt;style scoped&gt; 块中定义所有必要的 CSS 规则
3. WHEN 转换 App.vue 样式时，THE System SHALL 使用 CSS 变量引用主题颜色
4. WHEN 转换 App.vue 样式时，THE System SHALL 保持布局结构（flex、grid 等）不变
5. WHEN 转换 App.vue 样式时，THE System SHALL 保持所有交互状态样式（hover、focus、active）
6. WHEN 转换 App.vue 样式时，THE System SHALL 保持响应式行为不变

### 需求 5：转换 VisualScriptEditor.vue 组件样式

**用户故事：** 作为开发者，我希望将 VisualScriptEditor.vue 中的所有 Tailwind 工具类转换为原生 CSS 类。

#### 验收标准

1. WHEN 转换 VisualScriptEditor 样式时，THE System SHALL 将所有 Tailwind 工具类替换为语义化的 CSS 类名
2. WHEN 转换 VisualScriptEditor 样式时，THE System SHALL 在 &lt;style scoped&gt; 块中定义所有必要的 CSS 规则
3. WHEN 转换 VisualScriptEditor 样式时，THE System SHALL 使用 CSS 变量引用主题颜色
4. WHEN 转换 VisualScriptEditor 样式时，THE System SHALL 保持复杂布局（fixed、absolute、z-index）不变
5. WHEN 转换 VisualScriptEditor 样式时，THE System SHALL 保持所有动画和过渡效果
6. WHEN 转换 VisualScriptEditor 样式时，THE System SHALL 保持滚动行为和溢出处理

### 需求 6：转换 ScriptLibrary.vue 组件样式

**用户故事：** 作为开发者，我希望将 ScriptLibrary.vue 中的所有 Tailwind 工具类转换为原生 CSS 类。

#### 验收标准

1. WHEN 转换 ScriptLibrary 样式时，THE System SHALL 将所有 Tailwind 工具类替换为自定义 CSS 类名
2. WHEN 转换 ScriptLibrary 样式时，THE System SHALL 在 &lt;style scoped&gt; 块中定义所有必要的 CSS 规则
3. WHEN 转换 ScriptLibrary 样式时，THE System SHALL 使用 CSS 变量引用主题颜色
4. WHEN 转换 ScriptLibrary 样式时，THE System SHALL 保持网格布局和卡片样式
5. WHEN 转换 ScriptLibrary 样式时，THE System SHALL 保持所有交互状态样式

### 需求 7：转换 TaskDashboard.vue 组件样式

**用户故事：** 作为开发者，我希望将 TaskDashboard.vue 中的所有 Tailwind 工具类转换为原生 CSS 类。

#### 验收标准

1. WHEN 转换 TaskDashboard 样式时，THE System SHALL 将所有 Tailwind 工具类替换为自定义 CSS 类名
2. WHEN 转换 TaskDashboard 样式时，THE System SHALL 在 &lt;style scoped&gt; 块中定义所有必要的 CSS 规则
3. WHEN 转换 TaskDashboard 样式时，THE System SHALL 使用 CSS 变量引用主题颜色
4. WHEN 转换 TaskDashboard 样式时，THE System SHALL 保持仪表板布局和统计卡片样式

### 需求 8：转换 SettingsView.vue 组件样式

**用户故事：** 作为开发者，我希望将 SettingsView.vue 中的所有 Tailwind 工具类转换为原生 CSS 类。

#### 验收标准

1. WHEN 转换 SettingsView 样式时，THE System SHALL 将所有 Tailwind 工具类替换为自定义 CSS 类名
2. WHEN 转换 SettingsView 样式时，THE System SHALL 在 &lt;style scoped&gt; 块中定义所有必要的 CSS 规则
3. WHEN 转换 SettingsView 样式时，THE System SHALL 使用 CSS 变量引用主题颜色
4. WHEN 转换 SettingsView 样式时，THE System SHALL 保持表单控件和设置项布局

### 需求 9：转换 ToastNotification.vue 组件样式

**用户故事：** 作为开发者，我希望将 ToastNotification.vue 中的所有 Tailwind 工具类转换为原生 CSS 类。

#### 验收标准

1. WHEN 转换 ToastNotification 样式时，THE System SHALL 将所有 Tailwind 工具类替换为自定义 CSS 类名
2. WHEN 转换 ToastNotification 样式时，THE System SHALL 在 &lt;style scoped&gt; 块中定义所有必要的 CSS 规则
3. WHEN 转换 ToastNotification 样式时，THE System SHALL 使用 CSS 变量引用状态颜色（success、error、warning、info）
4. WHEN 转换 ToastNotification 样式时，THE System SHALL 保持通知动画和定位

### 需求 10：转换 ConfirmDialog.vue 组件样式

**用户故事：** 作为开发者，我希望将 ConfirmDialog.vue 中的所有 Tailwind 工具类转换为原生 CSS 类。

#### 验收标准

1. WHEN 转换 ConfirmDialog 样式时，THE System SHALL 将所有 Tailwind 工具类替换为自定义 CSS 类名
2. WHEN 转换 ConfirmDialog 样式时，THE System SHALL 在 &lt;style scoped&gt; 块中定义所有必要的 CSS 规则
3. WHEN 转换 ConfirmDialog 样式时，THE System SHALL 使用 CSS 变量引用主题颜色
4. WHEN 转换 ConfirmDialog 样式时，THE System SHALL 保持模态框遮罩和居中布局

### 需求 11：验证深色模式功能

**用户故事：** 作为用户，我希望在移除 Tailwind 后深色模式继续正常工作，所有组件在深色模式下显示正确的颜色。

#### 验收标准

1. WHEN 切换到深色模式时，THE System SHALL 应用 :root.dark 中定义的 CSS 变量
2. WHEN 切换到深色模式时，THE System SHALL 在所有组件中正确显示深色主题颜色
3. WHEN 切换到深色模式时，THE System SHALL 保持所有文本的可读性
4. WHEN 切换到深色模式时，THE System SHALL 保持所有边框和阴影的可见性
5. WHEN 在浅色和深色模式之间切换时，THE System SHALL 平滑过渡而不出现闪烁

### 需求 12：验证构建和运行

**用户故事：** 作为开发者，我希望在移除 Tailwind 后项目能够成功构建和运行，没有样式错误或控制台警告。

#### 验收标准

1. WHEN 执行 pnpm dev 时，THE System SHALL 成功启动开发服务器而不产生 CSS 相关错误
2. WHEN 执行 pnpm build 时，THE System SHALL 成功构建项目而不产生 CSS 相关错误
3. WHEN 在浏览器中打开应用时，THE System SHALL 正确渲染所有组件样式
4. WHEN 在浏览器控制台检查时，THE System SHALL 不显示任何 CSS 相关的警告或错误
5. WHEN 检查构建产物时，THE System SHALL 不包含任何 Tailwind CSS 相关的代码

### 需求 13：保持设计系统一致性

**用户故事：** 作为设计师，我希望在移除 Tailwind 后所有视觉效果保持不变，包括颜色、间距、字体、圆角、阴影等。

#### 验收标准

1. WHEN 对比转换前后的界面时，THE System SHALL 保持所有颜色值完全一致
2. WHEN 对比转换前后的界面时，THE System SHALL 保持所有间距（padding、margin）完全一致
3. WHEN 对比转换前后的界面时，THE System SHALL 保持所有字体大小和粗细完全一致
4. WHEN 对比转换前后的界面时，THE System SHALL 保持所有圆角半径完全一致
5. WHEN 对比转换前后的界面时，THE System SHALL 保持所有阴影效果完全一致
6. WHEN 对比转换前后的界面时，THE System SHALL 保持所有过渡动画完全一致

### 需求 14：优化 CSS 组织结构

**用户故事：** 作为开发者，我希望转换后的 CSS 代码具有良好的组织结构和可维护性。

#### 验收标准

1. WHEN 编写组件样式时，THE System SHALL 使用语义化的 CSS 类名（如 .sidebar、.header、.card）
2. WHEN 编写组件样式时，THE System SHALL 将相关样式规则分组并添加注释
3. WHEN 编写组件样式时，THE System SHALL 避免过深的选择器嵌套（最多 3 层）
4. WHEN 编写组件样式时，THE System SHALL 优先使用 CSS 变量而非硬编码的颜色值
5. WHEN 编写组件样式时，THE System SHALL 将可复用的样式提取到全局样式文件中

### 需求 15：文档更新

**用户故事：** 作为开发者，我希望更新项目文档以反映 Tailwind CSS 的移除和新的样式系统。

#### 验收标准

1. WHEN 更新文档时，THE System SHALL 在 README.md 中移除所有 Tailwind CSS 相关的说明
2. WHEN 更新文档时，THE System SHALL 在 README.md 中添加原生 CSS 样式系统的说明
3. WHEN 更新文档时，THE System SHALL 记录 CSS 变量主题系统的使用方法
4. WHEN 更新文档时，THE System SHALL 提供深色模式切换的实现说明
