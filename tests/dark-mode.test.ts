import { describe, it, expect } from 'vitest';
import { readFileSync } from 'fs';
import { join } from 'path';

describe('Dark Mode CSS Variables', () => {
  it('should define all required CSS variables in light mode', () => {
    const indexCss = readFileSync(join(__dirname, '../src/index.css'), 'utf-8');
    
    const requiredVars = [
      '--primary',
      '--background',
      '--surface',
      '--surface-soft',
      '--text-main',
      '--text-muted',
      '--border-main',
      '--font-display',
      '--success',
      '--error',
      '--warning',
      '--info',
      '--success-bg',
      '--error-bg',
      '--warning-bg',
      '--info-bg',
      '--success-border',
      '--error-border',
      '--warning-border',
      '--info-border'
    ];
    
    // Check :root section
    const rootSection = indexCss.match(/:root\s*\{([^}]+)\}/)?.[1] || '';
    
    for (const varName of requiredVars) {
      expect(rootSection).toContain(varName);
    }
  });

  it('should define all required CSS variables in dark mode', () => {
    const indexCss = readFileSync(join(__dirname, '../src/index.css'), 'utf-8');
    
    const requiredVars = [
      '--primary',
      '--background',
      '--surface',
      '--surface-soft',
      '--text-main',
      '--text-muted',
      '--border-main',
      '--success',
      '--error',
      '--warning',
      '--info',
      '--success-bg',
      '--error-bg',
      '--warning-bg',
      '--info-bg',
      '--success-border',
      '--error-border',
      '--warning-border',
      '--info-border'
    ];
    
    // Check :root.dark section
    const darkSection = indexCss.match(/:root\.dark\s*\{([^}]+)\}/)?.[1] || '';
    
    for (const varName of requiredVars) {
      expect(darkSection).toContain(varName);
    }
  });

  it('should have different color values for light and dark modes', () => {
    const indexCss = readFileSync(join(__dirname, '../src/index.css'), 'utf-8');
    
    // Extract light mode primary color
    const lightPrimary = indexCss.match(/:root\s*\{[^}]*--primary:\s*([^;]+);/)?.[1]?.trim();
    
    // Extract dark mode primary color
    const darkPrimary = indexCss.match(/:root\.dark\s*\{[^}]*--primary:\s*([^;]+);/)?.[1]?.trim();
    
    expect(lightPrimary).toBeDefined();
    expect(darkPrimary).toBeDefined();
    expect(lightPrimary).not.toBe(darkPrimary);
  });

  it('should not contain Tailwind directives', () => {
    const indexCss = readFileSync(join(__dirname, '../src/index.css'), 'utf-8');
    
    expect(indexCss).not.toContain('@import "tailwindcss"');
    expect(indexCss).not.toContain('@tailwind');
    expect(indexCss).not.toContain('@apply');
    expect(indexCss).not.toContain('@theme');
    expect(indexCss).not.toContain('@layer');
  });
});

describe('Component Dark Mode Support', () => {
  const vueFiles = [
    'src/App.vue',
    'src/components/VisualScriptEditor.vue',
    'src/components/TaskDashboard.vue',
    'src/components/ScriptLibrary.vue',
    'src/components/SettingsView.vue',
    'src/components/ToastNotification.vue',
    'src/components/ConfirmDialog.vue'
  ];

  vueFiles.forEach(file => {
    it(`${file} should use CSS variables for colors`, () => {
      const content = readFileSync(join(__dirname, '..', file), 'utf-8');
      const styleMatch = content.match(/<style[^>]*>([\s\S]*?)<\/style>/);
      
      if (styleMatch) {
        const styleContent = styleMatch[1];
        
        // Check that CSS variables are used
        expect(styleContent).toMatch(/var\(--/);
        
        // Check for hardcoded theme colors (should use variables instead)
        // Allow rgba, transparent, and opacity-related properties
        const lines = styleContent.split('\n').filter(line => 
          !line.includes('rgba') && 
          !line.includes('transparent') &&
          !line.includes('opacity') &&
          !line.includes('0 0 0') && // Allow black for shadows
          !line.includes('255 255 255') // Allow white for specific effects
        );
        
        // Check for hardcoded hex colors that should be variables
        const themeColorPattern = /(background-color|color|border-color):\s*#[0-9a-fA-F]{6}/;
        const hasHardcodedThemeColors = lines.some(line => themeColorPattern.test(line));
        
        if (hasHardcodedThemeColors) {
          console.warn(`Warning: ${file} may contain hardcoded theme colors`);
        }
      }
    });
  });
});
