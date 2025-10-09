

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
    const darkModeToggle = document.getElementById("darkModeToggle");
    if (darkModeToggle) {
      darkModeToggle.addEventListener("click", () => {
        this.toggle();
      });
    }

    // 暗黑模式切换按钮（移动端）
    const darkModeToggleMobile = document.getElementById(
      "darkModeToggleMobile",
    );
    if (darkModeToggleMobile) {
      darkModeToggleMobile.addEventListener("click", () => {
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
    const darkModeToggle = document.getElementById("darkModeToggle");
    const darkModeToggleMobile = document.getElementById(
      "darkModeToggleMobile",
    );

    const updateButton = (button: HTMLElement | null) => {
      if (button) {
        const icon = button.querySelector("i");
        const text = button.querySelector("span");

        if (icon) {
          if (this.isDarkMode) {
            icon.className = "fas fa-sun";
          } else {
            icon.className = "fas fa-moon";
          }
        }

        if (text) {
          text.textContent = this.isDarkMode ? "浅色模式" : "深色模式";
        }
      }
    };

    updateButton(darkModeToggle);
    updateButton(darkModeToggleMobile);
  }
}

// 创建并导出单例实例
export const darkModeManager = DarkModeManager.getInstance();