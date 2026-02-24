# 需求文档

## 简介

本功能旨在修复前端主题颜色系统中的不一致性问题，统一主题变量的定义和应用方式，确保所有组件都使用主题变量而非硬编码颜色，并为状态颜色（成功、错误、警告、信息）添加统一的主题变量支持。

## 术语表

- **Theme_System**: 主题系统，负责管理和应用应用程序的颜色方案
- **CSS_Variable**: CSS 自定义属性，用于在样式表中定义可重用的值
- **Theme_Preset**: 主题预设，预定义的完整主题配置
- **Status_Color**: 状态颜色，用于表示成功、错误、警告、信息等状态的颜色
- **Hard_Coded_Color**: 硬编码颜色，直接在组件中使用的固定颜色类（如 bg-red-500）
- **Dark_Mode**: 深色模式，应用程序的深色主题变体

## 需求

### 需求 1：统一主题变量定义

**用户故事：** 作为开发者，我希望主题变量有唯一的定义来源，以便主题配置清晰且易于维护。

#### 验收标准

1. THE Theme_System SHALL 在 index.css 中定义所有主题 CSS 变量作为唯一真实来源
2. THE Theme_System SHALL 通过 JavaScript 动态修改 CSS 变量来应用主题更改
3. THE Theme_System SHALL 在初始加载和用户修改后使用相同的主题应用机制
4. THE Theme_System SHALL 从 localStorage 读取保存的主题配置并应用到 CSS 变量
5. WHEN 应用程序启动时，THE Theme_System SHALL 在 DOM 渲染前应用保存的主题

### 需求 2：添加状态颜色主题变量

**用户故事：** 作为开发者，我希望为成功、错误、警告、信息等状态定义主题变量，以便这些颜色也能响应主题切换。

#### 验收标准

1. THE Theme_System SHALL 为成功状态定义 CSS 变量（--success, --success-bg, --success-border）
2. THE Theme_System SHALL 为错误状态定义 CSS 变量（--error, --error-bg, --error-border）
3. THE Theme_System SHALL 为警告状态定义 CSS 变量（--warning, --warning-bg, --warning-border）
4. THE Theme_System SHALL 为信息状态定义 CSS 变量（--info, --info-bg, --info-border）
5. WHEN 切换深色/浅色模式时，THE Theme_System SHALL 自动调整所有状态颜色以保持可读性
6. THE Theme_System SHALL 在每个主题预设中包含所有状态颜色的定义

### 需求 3：消除硬编码颜色

**用户故事：** 作为开发者，我希望所有组件都使用主题变量，以便主题能够完全覆盖整个应用程序。

#### 验收标准

1. WHEN 扫描所有 Vue 组件时，THE Theme_System SHALL 确保没有使用硬编码的 Tailwind 颜色类（如 bg-red-500, text-green-700）
2. THE Theme_System SHALL 将所有删除操作的颜色替换为 --error 主题变量
3. THE Theme_System SHALL 将所有成功/激活状态的颜色替换为 --success 主题变量
4. THE Theme_System SHALL 将所有警告/提示的颜色替换为 --warning 主题变量
5. THE Theme_System SHALL 将所有信息提示的颜色替换为 --info 主题变量

### 需求 4：更新 Tailwind 配置

**用户故事：** 作为开发者，我希望 Tailwind CSS 能够识别和使用主题变量，以便可以使用语义化的类名。

#### 验收标准

1. THE Theme_System SHALL 在 @theme 配置中定义所有状态颜色的 Tailwind 颜色类
2. THE Theme_System SHALL 支持 bg-success, text-success, border-success 等语义化类名
3. THE Theme_System SHALL 支持 bg-error, text-error, border-error 等语义化类名
4. THE Theme_System SHALL 支持 bg-warning, text-warning, border-warning 等语义化类名
5. THE Theme_System SHALL 支持 bg-info, text-info, border-info 等语义化类名

### 需求 5：更新主题预设

**用户故事：** 作为用户，我希望所有主题预设都包含完整的状态颜色定义，以便切换主题时所有颜色都能正确显示。

#### 验收标准

1. WHEN 定义主题预设时，THE Theme_System SHALL 为每个预设包含所有状态颜色的值
2. THE Theme_System SHALL 确保浅色主题预设使用适合浅色背景的状态颜色
3. THE Theme_System SHALL 确保深色主题预设使用适合深色背景的状态颜色
4. THE Theme_System SHALL 在 applyTheme 函数中应用所有状态颜色变量
5. THE Theme_System SHALL 在 localStorage 中保存所有状态颜色配置

### 需求 6：保持向后兼容

**用户故事：** 作为用户，我希望现有的主题配置能够继续工作，以便升级后不会丢失我的主题设置。

#### 验收标准

1. WHEN 从 localStorage 读取旧的主题配置时，THE Theme_System SHALL 为缺失的状态颜色提供默认值
2. THE Theme_System SHALL 在首次加载时将完整的主题配置（包括状态颜色）保存到 localStorage
3. THE Theme_System SHALL 保持现有主题变量的命名和结构不变
4. THE Theme_System SHALL 确保现有组件在迁移过程中继续正常工作

### 需求 7：组件迁移

**用户故事：** 作为开发者，我希望系统地将所有组件迁移到使用主题变量，以便确保没有遗漏。

#### 验收标准

1. THE Theme_System SHALL 更新 App.vue 中的状态指示器使用主题变量
2. THE Theme_System SHALL 更新 TaskDashboard.vue 中的任务状态颜色使用主题变量
3. THE Theme_System SHALL 更新 VisualScriptEditor.vue 中的删除按钮和警告框使用主题变量
4. THE Theme_System SHALL 更新 ScriptLibrary.vue 中的删除按钮使用主题变量
5. WHEN 所有组件迁移完成后，THE Theme_System SHALL 确保深色/浅色模式切换时所有颜色都正确响应
