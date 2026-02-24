# 设计文档：移除 Tailwind CSS 依赖

## 概述

本设计文档描述了如何从 AutoKB 桌面自动化应用中完全移除 Tailwind CSS 依赖，并将所有组件样式转换为原生 CSS。该重构将保持现有的设计系统、视觉效果和深色模式功能，同时减少项目依赖和构建复杂度。

### 目标

1. 完全移除 Tailwind CSS 相关依赖包（@tailwindcss/postcss、tailwindcss）
2. 将所有 Vue 组件中的 Tailwind 工具类转换为原生 CSS
3. 保留并优化现有的 CSS 变量主题系统
4. 确保深色模式继续正常工作
5. 保持所有视觉效果和交互行为不变
6. 提高 CSS 代码的可维护性和语义化

### 范围

- 6 个 Vue 组件需要转换样式
- 1 个主样式文件需要更新
- 2 个配置文件需要修改
- 项目文档需要更新

## 架构

### 当前架构

```
项目结构
├── package.json (包含 Tailwind 依赖)
├── postcss.config.js (配置 Tailwind 插件)
├── src/
│   ├── index.css (使用 @import "tailwindcss", @theme, @apply)
│   ├── App.vue (使用 Tailwind 工具类)
│   └── components/
│       ├── VisualScriptEditor.vue (使用 Tailwind 工具类)
│       ├── ScriptLibrary.vue (使用 Tailwind 工具类)
│       ├── TaskDashboard.vue (使用 Tailwind 工具类)
│       ├── SettingsView.vue (使用 Tailwind 工具类)
│       ├── ToastNotification.vue (使用 Tailwind 工具类)
│       └── ConfirmDialog.vue (使用 Tailwind 工具类)
```

### 目标架构

```
项目结构
├── package.json (移除 Tailwind 依赖)
├── postcss.config.js (仅保留 autoprefixer)
├── src/
│   ├── index.css (纯 CSS 变量 + 全局样式)
│   ├── App.vue (使用自定义 CSS 类)
│   └── components/
│       ├── VisualScriptEditor.vue (使用 scoped CSS)
│       ├── ScriptLibrary.vue (使用 scoped CSS)
│       ├── TaskDashboard.vue (使用 scoped CSS)
│       ├── SettingsView.vue (使用 scoped CSS)
│       ├── ToastNotification.vue (使用 scoped CSS)
│       └── ConfirmDialog.vue (使用 scoped CSS)
```


## 组件和接口

### 1. 依赖管理

#### package.json 修改

**移除的依赖：**
- `@tailwindcss/postcss`: ^4.2.1
- `tailwindcss`: ^4.2.1

**保留的依赖：**
- `autoprefixer`: ^10.4.24 (用于 CSS 浏览器兼容性)
- `postcss`: ^8.5.6 (CSS 处理基础设施)

### 2. PostCSS 配置

#### postcss.config.js 更新

**当前配置：**
```javascript
export default {
    plugins: {
        '@tailwindcss/postcss': {},
        autoprefixer: {},
    },
}
```

**目标配置：**
```javascript
export default {
    plugins: {
        autoprefixer: {},
    },
}
```

### 3. 主样式文件

#### src/index.css 转换

**需要移除的 Tailwind 指令：**
- `@import "tailwindcss"`
- `@theme { ... }` 块
- `@apply` 指令

**需要保留的内容：**
- 所有 CSS 变量定义（:root 和 :root.dark）
- 自定义过渡动画（.page-enter-active 等）
- Material Symbols 字体配置

**转换示例：**

当前的 body 样式：
```css
body {
  @apply bg-background-main font-display text-text-main;
  margin: 0;
  padding: 0;
  overflow: hidden;
  user-select: none;
}
```

转换后的 body 样式：
```css
body {
  background-color: var(--background);
  font-family: var(--font-display);
  color: var(--text-main);
  margin: 0;
  padding: 0;
  overflow: hidden;
  user-select: none;
}
```


### 4. Vue 组件样式转换策略

#### 通用转换原则

1. **类名映射：** 将 Tailwind 工具类转换为语义化的自定义类名
2. **Scoped 样式：** 在每个组件的 `<style scoped>` 块中定义样式
3. **CSS 变量：** 使用 `var(--variable-name)` 引用主题颜色
4. **布局保持：** 确保 flexbox、grid 等布局属性完全一致
5. **交互状态：** 保留所有 hover、focus、active 等伪类样式
6. **响应式：** 保持媒体查询和响应式行为

#### Tailwind 工具类到 CSS 属性映射表

| Tailwind 类 | CSS 属性 | 示例值 |
|------------|---------|--------|
| `flex` | `display: flex` | - |
| `flex-col` | `flex-direction: column` | - |
| `items-center` | `align-items: center` | - |
| `justify-between` | `justify-content: space-between` | - |
| `gap-4` | `gap: 1rem` | 16px |
| `p-4` | `padding: 1rem` | 16px |
| `px-6` | `padding-left: 1.5rem; padding-right: 1.5rem` | 24px |
| `py-2` | `padding-top: 0.5rem; padding-bottom: 0.5rem` | 8px |
| `m-4` | `margin: 1rem` | 16px |
| `w-64` | `width: 16rem` | 256px |
| `h-14` | `height: 3.5rem` | 56px |
| `bg-primary` | `background-color: var(--primary)` | - |
| `text-text-main` | `color: var(--text-main)` | - |
| `border-border-main` | `border-color: var(--border-main)` | - |
| `rounded-lg` | `border-radius: 0.5rem` | 8px |
| `rounded-xl` | `border-radius: 0.75rem` | 12px |
| `shadow-sm` | `box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05)` | - |
| `hover:bg-surface-soft` | `.class:hover { background-color: var(--surface-soft) }` | - |
| `transition-colors` | `transition: color 150ms, background-color 150ms, border-color 150ms` | - |

#### 间距系统

Tailwind 使用 4px 基础单位（0.25rem）：
- `1` = 0.25rem = 4px
- `2` = 0.5rem = 8px
- `3` = 0.75rem = 12px
- `4` = 1rem = 16px
- `6` = 1.5rem = 24px
- `8` = 2rem = 32px


### 5. 组件转换详细设计

#### App.vue 转换

**主要 Tailwind 类使用：**
- 侧边栏：`w-64 flex-shrink-0 border-r border-border-main bg-surface-main flex flex-col`
- 导航按钮：`flex items-center gap-3 px-3 py-2.5 rounded-lg transition-colors`
- 主内容区：`flex-1 flex flex-col overflow-hidden`
- 头部：`h-16 border-b border-border-main bg-surface-main flex items-center justify-between px-8`

**转换策略：**
1. 创建语义化类名：`.sidebar`, `.nav-button`, `.main-content`, `.header`
2. 在 `<style scoped>` 中定义所有样式
3. 保持动态类绑定逻辑（`:class` 绑定）

#### VisualScriptEditor.vue 转换

**主要 Tailwind 类使用：**
- 编辑器容器：`fixed inset-0 z-[1000] bg-background-main flex flex-col`
- 工具栏：`flex h-14 items-center justify-between border-b border-border-main`
- 侧边栏：`w-64 border-r border-border-main bg-surface-main flex flex-col`
- 画布：`flex-1 bg-background-main relative overflow-hidden flex flex-col`

**转换策略：**
1. 使用 `.visual-editor`, `.toolbar`, `.sidebar`, `.canvas` 等类名
2. 保持复杂的定位和层级（fixed, absolute, z-index）
3. 保持滚动和溢出行为

#### TaskDashboard.vue 转换

**主要 Tailwind 类使用：**
- 表格容器：`bg-surface-main rounded-xl border border-border-main shadow-sm`
- 表头：`bg-surface-soft text-text-muted text-xs uppercase tracking-wider`
- 表格行：`hover:bg-surface-soft transition-colors`
- 状态徽章：`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs`

**转换策略：**
1. 使用 `.table-container`, `.table-header`, `.table-row`, `.status-badge` 等类名
2. 保持表格布局和响应式行为
3. 保持动态状态类（通过 `getStatusClass` 函数）

#### SettingsView.vue 转换

**主要 Tailwind 类使用：**
- 设置卡片：`bg-surface-main rounded-xl border border-border-main p-6`
- 网格布局：`grid grid-cols-2 md:grid-cols-4 gap-3`
- 颜色选择器：`w-10 h-10 p-0 border-0 bg-transparent cursor-pointer rounded`

**转换策略：**
1. 使用 `.settings-card`, `.preset-grid`, `.color-picker` 等类名
2. 保持网格布局和响应式断点
3. 保持表单控件样式

#### ToastNotification.vue 转换

**当前状态：** 已经使用自定义 CSS，但引用了一些未定义的 CSS 变量

**需要更新：**
1. 将 `var(--color-bg-secondary)` 改为 `var(--surface)`
2. 将 `var(--color-text-primary)` 改为 `var(--text-main)`
3. 将 `var(--color-accent)` 改为 `var(--primary)`
4. 确保状态颜色变量正确（`var(--success)`, `var(--error)`）

#### ConfirmDialog.vue 转换

**主要 Tailwind 类使用：**
- 遮罩：`fixed inset-0 z-[9998] flex items-center justify-center`
- 背景：`absolute inset-0 bg-black/50 backdrop-blur-sm`
- 对话框：`relative z-[9999] bg-surface-main rounded-xl shadow-2xl w-full max-w-md`
- 按钮：`px-4 py-2 rounded-lg font-medium text-sm`

**转换策略：**
1. 使用 `.dialog-overlay`, `.dialog-backdrop`, `.dialog-container`, `.dialog-button` 等类名
2. 保持模态框定位和层级
3. 保持过渡动画


## 数据模型

### CSS 变量主题系统

#### 当前 CSS 变量定义

**浅色模式（:root）：**
```css
--primary: #135bec;
--background: #f6f6f8;
--surface: #ffffff;
--surface-soft: #f8fafc;
--text-main: #0f172a;
--text-muted: #64748b;
--border-main: #e2e8f0;
--font-display: "Inter", "system-ui", "sans-serif";

/* 状态颜色 */
--success: #22c55e;
--error: #ef4444;
--warning: #f59e0b;
--info: #3b82f6;

/* 状态背景色 */
--success-bg: #f0fdf4;
--error-bg: #fef2f2;
--warning-bg: #fffbeb;
--info-bg: #eff6ff;

/* 状态边框色 */
--success-border: #bbf7d0;
--error-border: #fecaca;
--warning-border: #fde68a;
--info-border: #bfdbfe;
```

**深色模式（:root.dark）：**
```css
--primary: #3b82f6;
--background: #0f172a;
--surface: #1e293b;
--surface-soft: #1e293b80;
--text-main: #f8fafc;
--text-muted: #94a3b8;
--border-main: #1e293b;

/* 状态颜色（深色模式）*/
--success: #4ade80;
--error: #f87171;
--warning: #fbbf24;
--info: #60a5fa;

/* 状态背景色（深色模式）*/
--success-bg: #14532d;
--error-bg: #7f1d1d;
--warning-bg: #78350f;
--info-bg: #1e3a8a;

/* 状态边框色（深色模式）*/
--success-border: #166534;
--error-border: #991b1b;
--warning-border: #92400e;
--info-border: #1e40af;
```

#### 变量使用规范

1. **颜色引用：** 始终使用 `var(--variable-name)` 而非硬编码颜色值
2. **主题切换：** 通过切换 `<html>` 或 `<body>` 的 `dark` 类来切换主题
3. **状态颜色：** 使用专用的状态颜色变量（success、error、warning、info）
4. **透明度：** 使用 CSS 的 `rgb()` 或 `rgba()` 函数配合变量

### 深色模式实现

**当前实现（在 scriptStore.ts 中）：**
```typescript
applyTheme() {
  const root = document.documentElement;
  if (this.theme.isDark) {
    root.classList.add('dark');
  } else {
    root.classList.remove('dark');
  }
  // 应用 CSS 变量
  root.style.setProperty('--primary', this.theme.primary);
  root.style.setProperty('--background', this.theme.background);
  // ... 其他变量
}
```

**保持不变：** 深色模式切换逻辑无需修改，CSS 变量系统已经支持。


## 正确性属性

*属性是一个特征或行为，应该在系统的所有有效执行中保持为真——本质上是关于系统应该做什么的形式化陈述。属性作为人类可读规范和机器可验证正确性保证之间的桥梁。*

### 属性 1：CSS 变量完整性

*对于任何* 主题模式（浅色或深色），所有必需的 CSS 变量应该在 src/index.css 中定义，包括：primary、background、surface、surface-soft、text-main、text-muted、border-main、font-display，以及所有状态颜色（success、error、warning、info）及其背景和边框变体。

**验证：需求 3.4**

### 属性 2：无 Tailwind 工具类残留

*对于任何* Vue 组件文件（.vue），模板中不应包含 Tailwind 特定的工具类模式，如：`bg-{color}`、`text-{size}`、`p-{number}`、`m-{number}`、`flex-{value}`、`grid-{value}`、`w-{number}`、`h-{number}` 等。

**验证：需求 4.1, 5.1, 6.1, 7.1, 8.1, 9.1, 10.1**

### 属性 3：CSS 变量优先使用

*对于任何* 组件的样式定义，颜色值应该使用 CSS 变量（`var(--variable-name)`）而非硬编码的十六进制、RGB 或命名颜色值（除非是透明度或特殊效果）。

**验证：需求 4.3, 5.3, 6.3, 7.3, 8.3, 9.3, 10.3**

### 属性 4：语义化类名

*对于任何* 新定义的 CSS 类名，应该使用语义化命名（描述元素的用途或角色），而非描述样式的命名（如 `.sidebar` 而非 `.w-64-flex-col`）。

**验证：需求 14.1**

### 属性 5：选择器嵌套深度限制

*对于任何* 组件的 CSS 规则，选择器嵌套深度不应超过 3 层，以保持样式的可维护性和性能。

**验证：需求 14.3**

### 属性 6：无硬编码主题颜色

*对于任何* 样式规则中使用的颜色值，如果该颜色属于主题系统（primary、background、surface、text、border、状态颜色），则必须使用对应的 CSS 变量，而非硬编码颜色值。

**验证：需求 14.4**

### 属性 7：构建产物无 Tailwind 代码

*对于任何* 构建输出的 CSS 文件，不应包含 Tailwind CSS 的特征字符串，如：`@tailwind`、`tailwindcss`、`@apply`、`@theme`、`@layer` 等。

**验证：需求 12.5**


## 错误处理

### 1. 依赖移除错误

**场景：** 移除 Tailwind 依赖后，其他依赖可能出现冲突

**处理策略：**
- 在移除依赖前备份 package.json 和 pnpm-lock.yaml
- 执行 `pnpm install` 后检查是否有错误
- 如果出现依赖冲突，使用 `pnpm install --force` 重新安装
- 验证 autoprefixer 和 postcss 仍然正常工作

### 2. CSS 转换错误

**场景：** 转换过程中可能遗漏某些 Tailwind 类或转换不正确

**处理策略：**
- 使用正则表达式搜索所有 `.vue` 文件中的 Tailwind 类模式
- 对比转换前后的视觉效果（截图对比）
- 使用浏览器开发者工具检查计算样式
- 逐个组件进行转换和验证，而非一次性全部转换

### 3. CSS 变量未定义错误

**场景：** 组件引用了未在 index.css 中定义的 CSS 变量

**处理策略：**
- 在浏览器控制台检查 CSS 警告
- 使用 `getComputedStyle` 验证变量值
- 确保所有引用的变量都在 :root 和 :root.dark 中定义
- 为未定义的变量提供回退值：`var(--variable, fallback-value)`

### 4. 深色模式失效错误

**场景：** 转换后深色模式切换不生效或颜色不正确

**处理策略：**
- 验证 `dark` 类是否正确添加到 `<html>` 或 `<body>` 元素
- 检查 :root.dark 选择器是否正确定义
- 确保所有组件都使用 CSS 变量而非硬编码颜色
- 测试深色模式下所有组件的可见性和可读性

### 5. 构建失败错误

**场景：** 移除 Tailwind 后构建过程失败

**处理策略：**
- 检查 PostCSS 配置是否正确
- 确保没有遗留的 Tailwind 指令（@import, @apply, @theme）
- 验证所有 CSS 语法正确
- 使用 `pnpm build --debug` 查看详细错误信息

### 6. 性能回退

**场景：** 转换后 CSS 文件体积增大或渲染性能下降

**处理策略：**
- 提取重复的样式到全局样式文件
- 使用 CSS 压缩工具优化构建产物
- 避免过度使用复杂选择器
- 考虑使用 CSS 模块化或 BEM 命名规范


## 测试策略

### 双重测试方法

本项目将采用单元测试和属性测试相结合的方法，以确保全面覆盖：

- **单元测试：** 验证特定示例、边缘情况和错误条件
- **属性测试：** 验证所有输入的通用属性
- 两者互补且都是必需的（单元测试捕获具体错误，属性测试验证一般正确性）

### 单元测试策略

单元测试应专注于：
- 特定文件的内容验证（如 package.json 不包含 Tailwind 依赖）
- 配置文件的正确性（如 postcss.config.js 配置）
- 构建命令的成功执行
- 特定组件的样式转换正确性

**避免过多单元测试** - 属性测试已经处理了大量输入覆盖。

### 属性测试策略

#### 测试库选择

- **JavaScript/TypeScript：** 使用 `fast-check` 库进行属性测试
- **配置：** 每个属性测试至少运行 100 次迭代

#### 属性测试标签格式

每个属性测试必须使用注释标签引用设计文档中的属性：

```typescript
// Feature: remove-tailwind-css, Property 2: 无 Tailwind 工具类残留
test('Vue components should not contain Tailwind utility classes', () => {
  // 测试实现
});
```

#### 属性测试实现

**属性 1：CSS 变量完整性**
```typescript
// Feature: remove-tailwind-css, Property 1: CSS 变量完整性
test('All required CSS variables are defined in index.css', () => {
  const indexCss = fs.readFileSync('src/index.css', 'utf-8');
  const requiredVars = [
    '--primary', '--background', '--surface', '--surface-soft',
    '--text-main', '--text-muted', '--border-main', '--font-display',
    '--success', '--error', '--warning', '--info',
    '--success-bg', '--error-bg', '--warning-bg', '--info-bg',
    '--success-border', '--error-border', '--warning-border', '--info-border'
  ];
  
  for (const varName of requiredVars) {
    expect(indexCss).toContain(varName);
  }
});
```

**属性 2：无 Tailwind 工具类残留**
```typescript
// Feature: remove-tailwind-css, Property 2: 无 Tailwind 工具类残留
test('Vue components do not contain Tailwind utility classes', () => {
  const vueFiles = glob.sync('src/**/*.vue');
  const tailwindPatterns = [
    /class="[^"]*\bbg-\w+/,
    /class="[^"]*\btext-\w+/,
    /class="[^"]*\bp-\d+/,
    /class="[^"]*\bm-\d+/,
    /class="[^"]*\bflex-\w+/,
    /class="[^"]*\bgrid-\w+/,
    /class="[^"]*\bw-\d+/,
    /class="[^"]*\bh-\d+/
  ];
  
  for (const file of vueFiles) {
    const content = fs.readFileSync(file, 'utf-8');
    for (const pattern of tailwindPatterns) {
      expect(content).not.toMatch(pattern);
    }
  }
});
```

**属性 3：CSS 变量优先使用**
```typescript
// Feature: remove-tailwind-css, Property 3: CSS 变量优先使用
test('Component styles use CSS variables for theme colors', () => {
  const vueFiles = glob.sync('src/**/*.vue');
  const hardcodedColorPattern = /(background-color|color|border-color):\s*#[0-9a-fA-F]{3,6}/;
  
  for (const file of vueFiles) {
    const content = fs.readFileSync(file, 'utf-8');
    const styleMatch = content.match(/<style[^>]*>([\s\S]*?)<\/style>/);
    if (styleMatch) {
      const styleContent = styleMatch[1];
      // 允许透明度和特殊效果使用硬编码颜色
      const lines = styleContent.split('\n').filter(line => 
        !line.includes('rgba') && 
        !line.includes('transparent') &&
        !line.includes('opacity')
      );
      for (const line of lines) {
        expect(line).not.toMatch(hardcodedColorPattern);
      }
    }
  }
});
```

**属性 7：构建产物无 Tailwind 代码**
```typescript
// Feature: remove-tailwind-css, Property 7: 构建产物无 Tailwind 代码
test('Build output does not contain Tailwind code', () => {
  const cssFiles = glob.sync('dist/**/*.css');
  const tailwindSignatures = [
    '@tailwind',
    'tailwindcss',
    '@apply',
    '@theme',
    '@layer'
  ];
  
  for (const file of cssFiles) {
    const content = fs.readFileSync(file, 'utf-8');
    for (const signature of tailwindSignatures) {
      expect(content).not.toContain(signature);
    }
  }
});
```

### 集成测试

**构建测试：**
```bash
# 测试开发服务器启动
pnpm dev &
sleep 5
curl http://localhost:5173 | grep -q "AutoKB"
kill %1

# 测试生产构建
pnpm build
test -f dist/index.html
test -f dist/assets/*.css
```

**视觉回归测试（可选）：**
- 使用 Playwright 或 Puppeteer 截图
- 对比转换前后的截图差异
- 验证深色模式切换效果

### 手动测试清单

1. ✓ 所有页面正常渲染
2. ✓ 侧边栏导航正常工作
3. ✓ 按钮 hover 效果正常
4. ✓ 表单控件样式正确
5. ✓ 模态框和通知正常显示
6. ✓ 深色模式切换正常
7. ✓ 响应式布局正常
8. ✓ 滚动和溢出行为正常

