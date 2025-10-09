import './index.css';
import './styles/dark-mode.css';

// 黑暗模式管理
class DarkModeManager {
    private static instance: DarkModeManager;
    private isDarkMode: boolean = false;
    private readonly STORAGE_KEY = 'dark-mode-preference';
    private readonly DARK_CLASS = 'dark';
    private readonly TRANSITION_CLASS = 'theme-transition';

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
        const savedPreference = localStorage.getItem(this.STORAGE_KEY);
        const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

        if (savedPreference !== null) {
            this.isDarkMode = savedPreference === 'true';
        } else {
            this.isDarkMode = systemPrefersDark;
        }

        this.applyTheme();
        this.setupEventListeners();
    }

    private setupEventListeners(): void {
        // 监听系统主题变化
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
            // 只有在用户没有手动设置过偏好时才跟随系统
            if (localStorage.getItem(this.STORAGE_KEY) === null) {
                this.isDarkMode = e.matches;
                this.applyTheme();
            }
        });

        // 监听黑暗模式切换按钮
        const darkModeToggle = document.getElementById('dart_mode_button');
        if (darkModeToggle) {
            darkModeToggle.addEventListener('click', () => {
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
        localStorage.setItem(this.STORAGE_KEY, 'true');
        this.applyTheme();
    }

    public disable(): void {
        this.isDarkMode = false;
        localStorage.setItem(this.STORAGE_KEY, 'false');
        this.applyTheme();
    }

    public isDark(): boolean {
        return this.isDarkMode;
    }

    private applyTheme(): void {
        const html = document.documentElement;

        // 添加过渡类
        html.classList.add(this.TRANSITION_CLASS);

        if (this.isDarkMode) {
            html.classList.add(this.DARK_CLASS);
            this.updateDarkModeToggle(true);
        } else {
            html.classList.remove(this.DARK_CLASS);
            this.updateDarkModeToggle(false);
        }

        // 移除过渡类（在过渡完成后）
        setTimeout(() => {
            html.classList.remove(this.TRANSITION_CLASS);
        }, 300);
    }

    private updateDarkModeToggle(isDark: boolean): void {
        const toggle = document.getElementById('darkModeToggle');
        if (!toggle) return;

        const icon = toggle.querySelector('svg');
        const text = toggle.querySelector('.dark-mode-text');

        if (icon) {
            // 更新图标
            icon.innerHTML = isDark ?
                '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"></path>' :
                '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"></path>';
        }

        if (text) {
            // 更新文本
            text.textContent = isDark ? '浅色模式' : '黑暗模式';
        }
    }
}

// 初始化黑暗模式管理器
const darkModeManager = DarkModeManager.getInstance();

// 页面加载完成后初始化
document.addEventListener('DOMContentLoaded', () => {
    console.log('黑暗模式已初始化，当前主题:', darkModeManager.isDark() ? '黑暗' : '浅色');
});
