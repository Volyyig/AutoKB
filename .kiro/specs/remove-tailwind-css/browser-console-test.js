/**
 * 深色模式浏览器控制台测试脚本
 * 
 * 使用方法：
 * 1. 在浏览器中打开应用 (http://localhost:1420/)
 * 2. 打开开发者工具控制台 (F12)
 * 3. 复制并粘贴此脚本到控制台
 * 4. 按 Enter 执行
 */

(function() {
  console.log('=== 深色模式测试开始 ===\n');
  
  const results = {
    passed: [],
    failed: [],
    warnings: []
  };
  
  function test(name, condition, message) {
    if (condition) {
      results.passed.push(name);
      console.log(`✅ ${name}`);
    } else {
      results.failed.push(name);
      console.error(`❌ ${name}: ${message}`);
    }
  }
  
  function warn(name, message) {
    results.warnings.push(name);
    console.warn(`⚠️  ${name}: ${message}`);
  }
  
  // 1. 检查 CSS 变量定义
  console.log('\n--- 1. CSS 变量定义检查 ---');
  const root = document.documentElement;
  const computedStyle = getComputedStyle(root);
  
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
  
  requiredVars.forEach(varName => {
    const value = computedStyle.getPropertyValue(varName).trim();
    test(
      `CSS 变量 ${varName} 已定义`,
      value !== '',
      `变量 ${varName} 未定义或为空`
    );
  });
  
  // 2. 检查当前主题模式
  console.log('\n--- 2. 当前主题模式 ---');
  const isDark = root.classList.contains('dark');
  console.log(`当前模式: ${isDark ? '深色' : '浅色'}`);
  
  // 3. 测试主题切换功能
  console.log('\n--- 3. 主题切换测试 ---');
  
  const originalMode = isDark;
  
  // 切换到深色模式
  if (!isDark) {
    root.classList.add('dark');
    console.log('切换到深色模式...');
  }
  
  setTimeout(() => {
    const darkPrimary = computedStyle.getPropertyValue('--primary').trim();
    const darkBackground = computedStyle.getPropertyValue('--background').trim();
    
    console.log(`深色模式 --primary: ${darkPrimary}`);
    console.log(`深色模式 --background: ${darkBackground}`);
    
    test(
      '深色模式 CSS 变量已应用',
      darkPrimary !== '' && darkBackground !== '',
      '深色模式变量未正确应用'
    );
    
    // 切换回浅色模式
    root.classList.remove('dark');
    console.log('切换到浅色模式...');
    
    setTimeout(() => {
      const lightPrimary = computedStyle.getPropertyValue('--primary').trim();
      const lightBackground = computedStyle.getPropertyValue('--background').trim();
      
      console.log(`浅色模式 --primary: ${lightPrimary}`);
      console.log(`浅色模式 --background: ${lightBackground}`);
      
      test(
        '浅色模式 CSS 变量已应用',
        lightPrimary !== '' && lightBackground !== '',
        '浅色模式变量未正确应用'
      );
      
      test(
        '深色和浅色模式颜色不同',
        darkPrimary !== lightPrimary || darkBackground !== lightBackground,
        '深色和浅色模式使用相同的颜色'
      );
      
      // 恢复原始模式
      if (originalMode) {
        root.classList.add('dark');
      }
      
      // 4. 检查组件样式
      console.log('\n--- 4. 组件样式检查 ---');
      
      const sidebar = document.querySelector('.sidebar');
      if (sidebar) {
        const sidebarStyle = getComputedStyle(sidebar);
        const bgColor = sidebarStyle.backgroundColor;
        test(
          '侧边栏背景色已应用',
          bgColor !== 'rgba(0, 0, 0, 0)' && bgColor !== 'transparent',
          `侧边栏背景色: ${bgColor}`
        );
      } else {
        warn('侧边栏元素未找到', '可能在当前页面不可见');
      }
      
      // 5. 检查是否有 Tailwind 类残留
      console.log('\n--- 5. Tailwind 类残留检查 ---');
      
      const allElements = document.querySelectorAll('*');
      const tailwindPatterns = [
        /\bbg-\w+/,
        /\btext-\w+-\d+/,
        /\bp-\d+/,
        /\bm-\d+/,
        /\bflex-\w+/,
        /\bgrid-cols-\d+/,
        /\bw-\d+/,
        /\bh-\d+/
      ];
      
      let tailwindClassesFound = [];
      allElements.forEach(el => {
        const classes = el.className;
        if (typeof classes === 'string') {
          tailwindPatterns.forEach(pattern => {
            const matches = classes.match(pattern);
            if (matches) {
              tailwindClassesFound.push({
                element: el.tagName,
                class: matches[0]
              });
            }
          });
        }
      });
      
      test(
        '无 Tailwind 工具类残留',
        tailwindClassesFound.length === 0,
        `发现 ${tailwindClassesFound.length} 个 Tailwind 类: ${JSON.stringify(tailwindClassesFound.slice(0, 5))}`
      );
      
      // 6. 输出测试结果摘要
      console.log('\n=== 测试结果摘要 ===');
      console.log(`✅ 通过: ${results.passed.length}`);
      console.log(`❌ 失败: ${results.failed.length}`);
      console.log(`⚠️  警告: ${results.warnings.length}`);
      
      if (results.failed.length === 0) {
        console.log('\n🎉 所有测试通过！深色模式功能正常。');
      } else {
        console.log('\n⚠️  部分测试失败，请检查上述错误信息。');
      }
      
      // 返回结果对象供进一步检查
      window.darkModeTestResults = results;
      console.log('\n测试结果已保存到 window.darkModeTestResults');
      
    }, 100);
  }, 100);
  
})();
