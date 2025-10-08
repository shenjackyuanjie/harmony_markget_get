import './index.css';

// index.ts
// 自动根据系统偏好设置初始模式，并监听变化。
// 支持手动覆盖 localStorage: setTheme('dark'|'light'|'auto')
// 当用户选择 'auto' 时，跟随系统偏好。

type Theme = 'dark' | 'light' | 'auto';
const THEME_KEY = 'site-theme';

function applyTheme(theme: Theme){
  const root = document.documentElement;
  if(theme === 'dark'){
    root.classList.add('dark');
    root.style.colorScheme = 'dark';
  } else if(theme === 'light'){
    root.classList.remove('dark');
    root.style.colorScheme = 'light';
  } else {
    const prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
    if(prefersDark){
      root.classList.add('dark');
      root.style.colorScheme = 'dark';
    } else {
      root.classList.remove('dark');
      root.style.colorScheme = 'light';
    }
  }
}

function setTheme(theme: Theme){
  localStorage.setItem(THEME_KEY, theme);
  applyTheme(theme);
}

function initTheme(){
  const saved = (localStorage.getItem(THEME_KEY) || 'auto') as Theme;
  applyTheme(saved);

  // 监听系统偏好变化，仅在用户选择 auto 时生效
  if(window.matchMedia){
    const mq = window.matchMedia('(prefers-color-scheme: dark)');
    mq.addEventListener ? mq.addEventListener('change', onSystemPrefChange) : mq.addListener(onSystemPrefChange);
  }

  function onSystemPrefChange(){
    const current = (localStorage.getItem(THEME_KEY) || 'auto') as Theme;
    if(current === 'auto') applyTheme('auto');
  }
}

// 立即初始化
initTheme();
