

// 黑暗模式管理
export class DarkModeManager {
  private static instance: DarkModeManager;
  private isDarkMode: boolean = false;
  private readonly STORAGE_KEY = "dark-mode-preference";
  private readonly DARK_CLASS = "dark";
  private readonly TRANSITION_CLASS = "theme-transition";

  private constructor() {
    this.init();
  }

  public static getInstance(): DarkModeManager {
    if (!DarkModeManager.instance) {
      DarkModeManager.instance = new DarkModeManager();
    }
    return DarkModeManager.instance;
  }

  private init(): void {
    // 从本地存储或系统偏好获取初始状态
    const storedPreference = localStorage.getItem(this.STORAGE_KEY);
    const systemPrefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)",
    ).matches;

    if (storedPreference !== null) {
      this.isDarkMode = storedPreference === "true";
    } else {
      this.isDarkMode = systemPrefersDark;
    }

    this.applyTheme();
    this.setupEventListeners();
  }

  private setupEventListeners(): void {
    // 监听系统主题变化
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", (e) => {
        // 只有在没有用户明确设置时才跟随系统
        if (localStorage.getItem(this.STORAGE_KEY) === null) {
          this.isDarkMode = e.matches;
          this.applyTheme();
        }
      });

    // 暗黑模式切换按钮
    const darkModeToggle = document.getElementById("dart_mode_button");
    if (darkModeToggle) {
      darkModeToggle.addEventListener("click", () => {
        this.toggle();
      });
    }
  }

  public toggle(): void {
    this.isDarkMode = !this.isDarkMode;
    localStorage.setItem(this.STORAGE_KEY, this.isDarkMode.toString());
    this.applyTheme();
  }

  public enable(): void {
    this.isDarkMode = true;
    localStorage.setItem(this.STORAGE_KEY, "true");
    this.applyTheme();
  }

  public disable(): void {
    this.isDarkMode = false;
    localStorage.setItem(this.STORAGE_KEY, "false");
    this.applyTheme();
  }

  public isDark(): boolean {
    return this.isDarkMode;
  }

  private applyTheme(): void {
    const root = document.documentElement;

    // 添加过渡类
    root.classList.add(this.TRANSITION_CLASS);

    if (this.isDarkMode) {
      root.classList.add(this.DARK_CLASS);
    } else {
      root.classList.remove(this.DARK_CLASS);
    }

    // 更新切换按钮状态
    this.updateDarkModeToggle();

    // 移除过渡类（在动画完成后）
    setTimeout(() => {
      root.classList.remove(this.TRANSITION_CLASS);
    }, 300);
  }

  private updateDarkModeToggle(): void {
    const darkModeToggle = document.getElementById("dart_mode_button");

    const updateButton = (button: HTMLElement | null) => {
      if (button) {
        const svg = button.querySelector("svg");

        if (svg) {
          if (this.isDarkMode) {
            // 切换为太阳图标（浅色模式）
            svg.innerHTML = '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>';
          } else {
            // 切换为月亮图标（深色模式）
            svg.innerHTML = '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path>';
          }
        }

        // 更新按钮文本 - 查找最后一个子节点（文本节点）
        const lastChild = button.lastChild;
        if (lastChild && lastChild.nodeType === Node.TEXT_NODE) {
          lastChild.textContent = this.isDarkMode ? "切换浅色模式" : "切换黑暗模式";
        }
      }
    };

    updateButton(darkModeToggle);
  }
}

// 创建并导出单例实例
export const darkModeManager = DarkModeManager.getInstance();
