// 极简黑暗模式管理
class DarkMode {
  private isDark = false;

  constructor() {
    // 初始化时检测系统偏好
    this.isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    this.applyTheme();

    // 绑定切换按钮
    const button = document.getElementById('dart_mode_button');
    button?.addEventListener('click', () => this.toggle());
  }

  private applyTheme() {
    document.documentElement.classList.toggle('dark', this.isDark);
  }

  private toggle() {
    this.isDark = !this.isDark;
    this.applyTheme();
  }
}

// 初始化
export function init_dark() {
  new DarkMode();
}
