# 设计文档

## 概述

本设计旨在重构前端主题颜色系统，解决当前 CSS 变量定义与 JavaScript 动态修改之间的不一致性问题。核心策略是建立单一真实来源（index.css 中的 CSS 变量），通过 JavaScript 动态修改这些变量来实现主题切换，并为状态颜色（成功、错误、警告、信息）添加完整的主题支持。

技术栈：Vue 3 + TypeScript + Tailwind CSS 4 + Pinia

## 架构

### 主题系统架构

```
┌─────────────────────────────────────────────────────────┐
│                    应用程序启动                          │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              scriptStore.init()                         │
│  1. 从 localStorage 读取主题配置                        │
│  2. 调用 applyTheme() 应用到 CSS 变量                   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              index.css (单一真实来源)                    │
│  :root { --primary, --success, --error, ... }          │
│  :root.dark { ... }                                     │
│  @theme { 映射 CSS 变量到 Tailwind 类 }                 │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              Vue 组件                                    │
│  使用语义化 Tailwind 类：                                │
│  bg-success, text-error, border-warning, etc.           │
└─────────────────────────────────────────────────────────┘
```

### 主题切换流程

```
用户选择主题预设
       │
       ▼
setTheme(newTheme)
       │
       ▼
更新 theme.value
       │
       ▼
applyTheme()
       │
       ├──> 设置 CSS 变量 (--primary, --success, --error, ...)
       │
       ├──> 切换 dark 类 (document.documentElement.classList)
       │
       └──> 保存到 localStorage
```

## 组件和接口

### 1. 主题配置接口

```typescript
interface ThemeConfig {
  // 主要颜色
  primary: string;
  background: string;
  surface: string;
  surfaceSoft: string;
  textMain: string;
  textMuted: string;
  borderMain: string;
  
  // 状态颜色 - 主色
  success: string;
  error: string;
  warning: string;
  info: string;
  
  // 状态颜色 - 背景
  successBg: string;
  errorBg: string;
  warningBg: string;
  infoBg: string;
  
  // 状态颜色 - 边框
  successBorder: string;
  errorBorder: string;
  warningBorder: string;
  infoBorder: string;
  
  // 模式标识
  isDark: boolean;
}
```

### 2. CSS 变量定义 (index.css)

```css
:root {
  /* 现有变量 */
  --primary: #135bec;
  --background: #f6f6f8;
  --surface: #ffffff;
  --surface-soft: #f8fafc;
  --text-main: #0f172a;
  --text-muted: #64748b;
  --border-main: #e2e8f0;
  
  /* 新增：状态颜色 - 主色 */
  --success: #22c55e;
  --error: #ef4444;
  --warning: #f59e0b;
  --info: #3b82f6;
  
  /* 新增：状态颜色 - 背景 */
  --success-bg: #f0fdf4;
  --error-bg: #fef2f2;
  --warning-bg: #fffbeb;
  --info-bg: #eff6ff;
  
  /* 新增：状态颜色 - 边框 */
  --success-border: #bbf7d0;
  --error-border: #fecaca;
  --warning-border: #fde68a;
  --info-border: #bfdbfe;
}

:root.dark {
  /* 现有变量 */
  --primary: #3b82f6;
  --background: #0f172a;
  --surface: #1e293b;
  --surface-soft: #1e293b80;
  --text-main: #f8fafc;
  --text-muted: #94a3b8;
  --border-main: #1e293b;
  
  /* 新增：状态颜色 - 主色 (深色模式) */
  --success: #4ade80;
  --error: #f87171;
  --warning: #fbbf24;
  --info: #60a5fa;
  
  /* 新增：状态颜色 - 背景 (深色模式) */
  --success-bg: #14532d;
  --error-bg: #7f1d1d;
  --warning-bg: #78350f;
  --info-bg: #1e3a8a;
  
  /* 新增：状态颜色 - 边框 (深色模式) */
  --success-border: #166534;
  --error-border: #991b1b;
  --warning-border: #92400e;
  --info-border: #1e40af;
}

@theme {
  /* 现有映射 */
  --color-primary: var(--primary);
  --color-background-main: var(--background);
  --color-surface-main: var(--surface);
  --color-surface-soft: var(--surface-soft);
  --color-text-main: var(--text-main);
  --color-text-muted: var(--text-muted);
  --color-border-main: var(--border-main);
  
  /* 新增：状态颜色映射 */
  --color-success: var(--success);
  --color-error: var(--error);
  --color-warning: var(--warning);
  --color-info: var(--info);
  
  --color-success-bg: var(--success-bg);
  --color-error-bg: var(--error-bg);
  --color-warning-bg: var(--warning-bg);
  --color-info-bg: var(--info-bg);
  
  --color-success-border: var(--success-border);
  --color-error-border: var(--error-border);
  --color-warning-border: var(--warning-border);
  --color-info-border: var(--info-border);
}
```

### 3. 主题预设更新 (scriptStore.ts)

```typescript
const themePresets: ThemeConfig[] = [
  {
    name: '清新蓝',
    primary: '#135bec',
    background: '#f6f6f8',
    surface: '#ffffff',
    surfaceSoft: '#f8fafc',
    textMain: '#0f172a',
    textMuted: '#64748b',
    borderMain: '#e2e8f0',
    success: '#22c55e',
    error: '#ef4444',
    warning: '#f59e0b',
    info: '#3b82f6',
    successBg: '#f0fdf4',
    errorBg: '#fef2f2',
    warningBg: '#fffbeb',
    infoBg: '#eff6ff',
    successBorder: '#bbf7d0',
    errorBorder: '#fecaca',
    warningBorder: '#fde68a',
    infoBorder: '#bfdbfe',
    isDark: false
  },
  // ... 其他预设类似结构
];
```

### 4. applyTheme 函数更新

```typescript
function applyTheme() {
  const root = document.documentElement;
  
  // 应用主要颜色
  root.style.setProperty('--primary', theme.value.primary);
  root.style.setProperty('--background', theme.value.background);
  root.style.setProperty('--surface', theme.value.surface);
  root.style.setProperty('--surface-soft', theme.value.surfaceSoft);
  root.style.setProperty('--text-main', theme.value.textMain);
  root.style.setProperty('--text-muted', theme.value.textMuted);
  root.style.setProperty('--border-main', theme.value.borderMain);
  
  // 应用状态颜色 - 主色
  root.style.setProperty('--success', theme.value.success);
  root.style.setProperty('--error', theme.value.error);
  root.style.setProperty('--warning', theme.value.warning);
  root.style.setProperty('--info', theme.value.info);
  
  // 应用状态颜色 - 背景
  root.style.setProperty('--success-bg', theme.value.successBg);
  root.style.setProperty('--error-bg', theme.value.errorBg);
  root.style.setProperty('--warning-bg', theme.value.warningBg);
  root.style.setProperty('--info-bg', theme.value.infoBg);
  
  // 应用状态颜色 - 边框
  root.style.setProperty('--success-border', theme.value.successBorder);
  root.style.setProperty('--error-border', theme.value.errorBorder);
  root.style.setProperty('--warning-border', theme.value.warningBorder);
  root.style.setProperty('--info-border', theme.value.infoBorder);
  
  // 切换深色模式类
  if (theme.value.isDark) {
    root.classList.add('dark');
  } else {
    root.classList.remove('dark');
  }
  
  // 保存到 localStorage
  Object.entries(theme.value).forEach(([key, value]) => {
    localStorage.setItem(`theme-${key}`, String(value));
  });
}
```

### 5. 组件颜色迁移映射

| 当前硬编码类 | 替换为主题类 | 用途 |
|------------|------------|------|
| `bg-red-500`, `text-red-500`, `hover:bg-red-50` | `bg-error`, `text-error`, `hover:bg-error-bg` | 删除按钮、错误状态 |
| `bg-green-500`, `text-green-700`, `bg-green-100` | `bg-success`, `text-success`, `bg-success-bg` | 成功状态、激活状态 |
| `bg-amber-50`, `text-amber-500`, `border-amber-100` | `bg-warning-bg`, `text-warning`, `border-warning-border` | 警告提示框 |
| `bg-blue-500` | `bg-info` | 信息提示 |

## 数据模型

### LocalStorage 数据结构

```typescript
// 键值对存储
{
  'theme-primary': string,
  'theme-background': string,
  'theme-surface': string,
  'theme-surfaceSoft': string,
  'theme-textMain': string,
  'theme-textMuted': string,
  'theme-borderMain': string,
  'theme-success': string,
  'theme-error': string,
  'theme-warning': string,
  'theme-info': string,
  'theme-successBg': string,
  'theme-errorBg': string,
  'theme-warningBg': string,
  'theme-infoBg': string,
  'theme-successBorder': string,
  'theme-errorBorder': string,
  'theme-warningBorder': string,
  'theme-infoBorder': string,
  'theme-isDark': 'true' | 'false'
}
```

### 默认值提供机制

```typescript
function loadThemeFromStorage(): ThemeConfig {
  return {
    primary: localStorage.getItem('theme-primary') || '#135bec',
    background: localStorage.getItem('theme-background') || '#f6f6f8',
    surface: localStorage.getItem('theme-surface') || '#ffffff',
    surfaceSoft: localStorage.getItem('theme-surfaceSoft') || '#f8fafc',
    textMain: localStorage.getItem('theme-textMain') || '#0f172a',
    textMuted: localStorage.getItem('theme-textMuted') || '#64748b',
    borderMain: localStorage.getItem('theme-borderMain') || '#e2e8f0',
    success: localStorage.getItem('theme-success') || '#22c55e',
    error: localStorage.getItem('theme-error') || '#ef4444',
    warning: localStorage.getItem('theme-warning') || '#f59e0b',
    info: localStorage.getItem('theme-info') || '#3b82f6',
    successBg: localStorage.getItem('theme-successBg') || '#f0fdf4',
    errorBg: localStorage.getItem('theme-errorBg') || '#fef2f2',
    warningBg: localStorage.getItem('theme-warningBg') || '#fffbeb',
    infoBg: localStorage.getItem('theme-infoBg') || '#eff6ff',
    successBorder: localStorage.getItem('theme-successBorder') || '#bbf7d0',
    errorBorder: localStorage.getItem('theme-errorBorder') || '#fecaca',
    warningBorder: localStorage.getItem('theme-warningBorder') || '#fde68a',
    infoBorder: localStorage.getItem('theme-infoBorder') || '#bfdbfe',
    isDark: localStorage.getItem('theme-isDark') === 'true'
  };
}
```

## 正确性属性

*属性是一个特征或行为，应该在系统的所有有效执行中保持为真——本质上是关于系统应该做什么的形式化陈述。属性作为人类可读规范和机器可验证正确性保证之间的桥梁。*


### 属性 1：主题配置往返一致性

*对于任何*有效的主题配置，当保存到 localStorage 然后重新加载并应用时，所有 CSS 变量的值应该与原始配置相同。

**验证：需求 1.4, 5.5**

### 属性 2：初始加载与用户修改的一致性

*对于任何*主题配置，无论是在初始加载时应用还是在用户修改后应用，最终的 CSS 变量值应该相同。

**验证：需求 1.3**

### 属性 3：状态颜色 CSS 变量完整性

*对于任何*主题模式（浅色或深色），所有状态颜色（success, error, warning, info）的三个变体（主色、背景、边框）的 CSS 变量都应该被定义。

**验证：需求 2.1, 2.2, 2.3, 2.4**

### 属性 4：模式切换时状态颜色更新

*对于任何*主题配置，当切换深色/浅色模式时，所有状态颜色的 CSS 变量（12 个变量）都应该被更新为新模式对应的值。

**验证：需求 2.5**

### 属性 5：主题预设完整性

*对于任何*主题预设，它应该包含所有必需的字段，包括 7 个主要颜色字段和 12 个状态颜色字段。

**验证：需求 2.6, 5.1**

### 属性 6：无硬编码颜色类

*对于任何* Vue 组件文件，它不应该包含硬编码的 Tailwind 颜色类（如 bg-red-500, text-green-700 等），除了主题变量相关的类。

**验证：需求 3.1**

### 属性 7：Tailwind 状态颜色类可用性

*对于任何*状态类型（success, error, warning, info），Tailwind 应该支持该状态的 bg-*, text-*, border-* 类名。

**验证：需求 4.2, 4.3, 4.4, 4.5**

### 属性 8：applyTheme 完整性

*对于任何*主题配置，调用 applyTheme 后，所有主题字段（包括主要颜色和状态颜色）都应该被设置为对应的 CSS 变量。

**验证：需求 5.4**

### 属性 9：向后兼容的默认值

*对于任何*不完整的 localStorage 主题配置（缺少状态颜色字段），加载时应该为所有缺失的字段提供合理的默认值。

**验证：需求 6.1**

## 错误处理

### 1. localStorage 不可用

```typescript
function safeLocalStorageGet(key: string, defaultValue: string): string {
  try {
    return localStorage.getItem(key) || defaultValue;
  } catch (error) {
    console.warn(`无法访问 localStorage: ${error}`);
    return defaultValue;
  }
}

function safeLocalStorageSet(key: string, value: string): void {
  try {
    localStorage.setItem(key, value);
  } catch (error) {
    console.warn(`无法写入 localStorage: ${error}`);
  }
}
```

### 2. 无效的颜色值

```typescript
function isValidColor(color: string): boolean {
  // 验证十六进制颜色格式
  return /^#[0-9A-Fa-f]{6}$/.test(color);
}

function loadThemeFromStorage(): ThemeConfig {
  const primary = safeLocalStorageGet('theme-primary', '#135bec');
  
  // 验证颜色值，如果无效则使用默认值
  return {
    primary: isValidColor(primary) ? primary : '#135bec',
    // ... 其他字段类似处理
  };
}
```

### 3. 主题预设不存在

```typescript
function setTheme(newTheme: Partial<ThemeConfig>) {
  // 合并新主题与当前主题，确保所有字段都有值
  theme.value = {
    ...theme.value,
    ...newTheme
  };
  applyTheme();
}
```

### 4. CSS 变量设置失败

```typescript
function applyTheme() {
  try {
    const root = document.documentElement;
    
    // 尝试设置每个 CSS 变量
    Object.entries(cssVariableMap).forEach(([key, cssVar]) => {
      try {
        root.style.setProperty(cssVar, theme.value[key]);
      } catch (error) {
        console.warn(`无法设置 CSS 变量 ${cssVar}: ${error}`);
      }
    });
    
    // 切换深色模式类
    if (theme.value.isDark) {
      root.classList.add('dark');
    } else {
      root.classList.remove('dark');
    }
  } catch (error) {
    console.error('应用主题失败:', error);
  }
}
```

## 测试策略

### 单元测试

使用 Vitest 进行单元测试，重点测试：

1. **主题配置加载**
   - 测试从 localStorage 加载完整配置
   - 测试从 localStorage 加载不完整配置（缺少状态颜色）
   - 测试 localStorage 不可用时的降级处理

2. **applyTheme 函数**
   - 测试所有 CSS 变量是否正确设置
   - 测试 dark 类的切换
   - 测试 localStorage 保存

3. **主题预设验证**
   - 测试每个预设是否包含所有必需字段
   - 测试预设的颜色值格式是否有效

4. **组件迁移验证**
   - 测试特定组件是否使用了正确的主题类
   - 测试删除按钮使用 error 颜色
   - 测试成功状态使用 success 颜色

### 属性测试

使用 fast-check 进行属性测试（最少 100 次迭代）：

1. **属性 1：主题配置往返一致性**
   ```typescript
   // Feature: theme-color-fix, Property 1: 主题配置往返一致性
   test('theme config round-trip consistency', () => {
     fc.assert(
       fc.property(
         themeConfigArbitrary,
         (config) => {
           // 保存配置
           saveThemeToLocalStorage(config);
           // 重新加载
           const loaded = loadThemeFromStorage();
           // 应用主题
           applyTheme(loaded);
           // 验证 CSS 变量
           return allCssVariablesMatch(config);
         }
       ),
       { numRuns: 100 }
     );
   });
   ```

2. **属性 3：状态颜色 CSS 变量完整性**
   ```typescript
   // Feature: theme-color-fix, Property 3: 状态颜色 CSS 变量完整性
   test('status color CSS variables completeness', () => {
     fc.assert(
       fc.property(
         fc.boolean(), // isDark
         (isDark) => {
           applyTheme({ ...defaultTheme, isDark });
           const statusTypes = ['success', 'error', 'warning', 'info'];
           const variants = ['', '-bg', '-border'];
           
           return statusTypes.every(type =>
             variants.every(variant => {
               const cssVar = `--${type}${variant}`;
               const value = getComputedStyle(document.documentElement)
                 .getPropertyValue(cssVar);
               return value.trim() !== '';
             })
           );
         }
       ),
       { numRuns: 100 }
     );
   });
   ```

3. **属性 5：主题预设完整性**
   ```typescript
   // Feature: theme-color-fix, Property 5: 主题预设完整性
   test('theme preset completeness', () => {
     const requiredFields = [
       'primary', 'background', 'surface', 'surfaceSoft',
       'textMain', 'textMuted', 'borderMain',
       'success', 'error', 'warning', 'info',
       'successBg', 'errorBg', 'warningBg', 'infoBg',
       'successBorder', 'errorBorder', 'warningBorder', 'infoBorder',
       'isDark'
     ];
     
     themePresets.forEach(preset => {
       requiredFields.forEach(field => {
         expect(preset).toHaveProperty(field);
         expect(preset[field]).toBeDefined();
       });
     });
   });
   ```

4. **属性 6：无硬编码颜色类**
   ```typescript
   // Feature: theme-color-fix, Property 6: 无硬编码颜色类
   test('no hard-coded color classes in components', () => {
     const vueFiles = glob.sync('src/**/*.vue');
     const hardCodedColorPattern = /\b(bg|text|border)-(red|green|blue|yellow|amber|purple|pink|indigo)-\d+\b/g;
     
     vueFiles.forEach(file => {
       const content = fs.readFileSync(file, 'utf-8');
       const matches = content.match(hardCodedColorPattern);
       
       if (matches) {
         console.log(`Found hard-coded colors in ${file}:`, matches);
       }
       
       expect(matches).toBeNull();
     });
   });
   ```

5. **属性 9：向后兼容的默认值**
   ```typescript
   // Feature: theme-color-fix, Property 9: 向后兼容的默认值
   test('backward compatible default values', () => {
     fc.assert(
       fc.property(
         fc.record({
           primary: fc.option(fc.hexaString({ minLength: 6, maxLength: 6 })),
           success: fc.option(fc.hexaString({ minLength: 6, maxLength: 6 })),
           // ... 其他字段随机缺失
         }),
         (partialConfig) => {
           // 清空 localStorage
           localStorage.clear();
           // 只保存部分配置
           Object.entries(partialConfig).forEach(([key, value]) => {
             if (value) {
               localStorage.setItem(`theme-${key}`, `#${value}`);
             }
           });
           
           // 加载配置
           const loaded = loadThemeFromStorage();
           
           // 验证所有字段都有值
           const requiredFields = [
             'primary', 'background', 'surface', 'surfaceSoft',
             'textMain', 'textMuted', 'borderMain',
             'success', 'error', 'warning', 'info',
             'successBg', 'errorBg', 'warningBg', 'infoBg',
             'successBorder', 'errorBorder', 'warningBorder', 'infoBorder'
           ];
           
           return requiredFields.every(field => {
             const value = loaded[field];
             return value && typeof value === 'string' && value.length > 0;
           });
         }
       ),
       { numRuns: 100 }
     );
   });
   ```

### 集成测试

1. **主题切换端到端测试**
   - 在浏览器环境中测试主题切换
   - 验证所有组件的颜色都正确更新
   - 测试深色/浅色模式切换

2. **组件视觉回归测试**
   - 使用 Playwright 或 Cypress 截图对比
   - 验证迁移后的组件外观与预期一致

### 测试配置

```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'jsdom',
    setupFiles: ['./tests/setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      include: ['src/**/*.{ts,vue}'],
      exclude: ['src/**/*.spec.ts', 'src/**/*.test.ts']
    }
  }
});
```

### 测试数据生成器

```typescript
// tests/arbitraries.ts
import fc from 'fast-check';

export const hexColorArbitrary = fc.hexaString({ minLength: 6, maxLength: 6 })
  .map(hex => `#${hex}`);

export const themeConfigArbitrary = fc.record({
  primary: hexColorArbitrary,
  background: hexColorArbitrary,
  surface: hexColorArbitrary,
  surfaceSoft: hexColorArbitrary,
  textMain: hexColorArbitrary,
  textMuted: hexColorArbitrary,
  borderMain: hexColorArbitrary,
  success: hexColorArbitrary,
  error: hexColorArbitrary,
  warning: hexColorArbitrary,
  info: hexColorArbitrary,
  successBg: hexColorArbitrary,
  errorBg: hexColorArbitrary,
  warningBg: hexColorArbitrary,
  infoBg: hexColorArbitrary,
  successBorder: hexColorArbitrary,
  errorBorder: hexColorArbitrary,
  warningBorder: hexColorArbitrary,
  infoBorder: hexColorArbitrary,
  isDark: fc.boolean()
});
```
