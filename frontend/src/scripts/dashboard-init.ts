// 仪表板初始化脚本

import { dashboardStore } from '../stores/dashboard';
import { initAppDetailModal } from '../utils/app-details';
import { updateLastUpdate } from '../utils/dom';
import { loadAllCharts } from '../utils/charts';
import { updateUrlParam, getUrlParams } from '../utils';

/**
 * 初始化搜索功能
 */
function initSearch(): void {
  const searchInput = document.getElementById('searchInput') as HTMLInputElement;
  const searchKeySelect = document.getElementById('searchKey') as HTMLSelectElement;
  const exactMatchCheckbox = document.getElementById('exactMatch') as HTMLInputElement;
  const searchButton = document.getElementById('searchButton');
  const clearSearchButton = document.getElementById('clearSearchButton');

  if (!searchInput || !searchKeySelect || !searchButton) return;

  // 搜索按钮点击事件
  searchButton.addEventListener('click', () => {
    const searchTerm = searchInput.value.trim();
    const searchKey = searchKeySelect.value;
    const exactMatch = exactMatchCheckbox?.checked || false;
    
    dashboardStore.searchApps(searchTerm, searchKey, exactMatch);
    
    // 更新URL参数
    updateUrlParam('search', searchTerm);
    updateUrlParam('search_key', searchKey);
    if (exactMatch) {
      updateUrlParam('exact', 'true');
    } else {
      updateUrlParam('exact', null);
    }
  });

  // 清空搜索按钮点击事件
  if (clearSearchButton) {
    clearSearchButton.addEventListener('click', () => {
      searchInput.value = '';
      exactMatchCheckbox.checked = false;
      dashboardStore.clearSearch();
      
      // 清除URL参数
      updateUrlParam('search', null);
      updateUrlParam('search_key', null);
      updateUrlParam('exact', null);
    });
  }

  // 回车键搜索
  searchInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
      searchButton.click();
    }
  });

  // 搜索字段变化事件
  searchKeySelect.addEventListener('change', () => {
    if (searchInput.value.trim()) {
      searchButton.click();
    }
  });
}

/**
 * 初始化排序功能
 */
function initSorting(): void {
  const sortableHeaders = document.querySelectorAll('th[data-sort]');
  
  sortableHeaders.forEach((header) => {
    header.addEventListener('click', () => {
      const field = header.getAttribute('data-sort');
      if (field) {
        dashboardStore.sortApps(field);
        
        // 更新URL参数
        updateUrlParam('sort', field);
      }
    });
  });
}

/**
 * 初始化刷新功能
 */
function initRefresh(): void {
  const refreshButton = document.getElementById('refreshButton');
  
  if (refreshButton) {
    refreshButton.addEventListener('click', async () => {
      await dashboardStore.refresh();
      updateLastUpdate();
    });
  }
}

/**
 * 初始化导出功能
 */
function initExport(): void {
  const exportButtons = document.querySelectorAll('[data-export]');
  
  exportButtons.forEach((button) => {
    button.addEventListener('click', async () => {
      const format = button.getAttribute('data-export') as 'csv' | 'json' | 'xlsx';
      if (format) {
        await dashboardStore.exportCurrentData(format);
      }
    });
  });
}

/**
 * 初始化键盘快捷键
 */
function initKeyboardShortcuts(): void {
  document.addEventListener('keydown', (e) => {
    // Ctrl+K 或 Cmd+K 聚焦搜索框
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
      e.preventDefault();
      const searchInput = document.getElementById('searchInput') as HTMLInputElement;
      if (searchInput) {
        searchInput.focus();
      }
    }
    
    // Ctrl+R 或 Cmd+R 刷新数据
    if ((e.ctrlKey || e.metaKey) && e.key === 'r') {
      e.preventDefault();
      dashboardStore.refresh();
      updateLastUpdate();
    }
    
    // ESC 清空搜索
    if (e.key === 'Escape') {
      const searchInput = document.getElementById('searchInput') as HTMLInputElement;
      if (searchInput && document.activeElement === searchInput) {
        searchInput.blur();
        const clearButton = document.getElementById('clearSearchButton');
        clearButton?.click();
      }
    }
  });
}

/**
 * 从URL参数恢复状态
 */
function restoreFromUrlParams(): void {
  const params = getUrlParams();
  
  // 恢复应用ID参数
  if (params.app_id) {
    setTimeout(() => {
      if (typeof window.showAppDetail === 'function') {
        window.showAppDetail(params.app_id);
      }
    }, 100);
  }
  
  // 恢复搜索参数
  if (params.search || params.search_key) {
    const searchInput = document.getElementById('searchInput') as HTMLInputElement;
    const searchKeySelect = document.getElementById('searchKey') as HTMLSelectElement;
    const exactMatchCheckbox = document.getElementById('exactMatch') as HTMLInputElement;
    
    if (searchInput && params.search) {
      searchInput.value = params.search;
    }
    
    if (searchKeySelect && params.search_key) {
      searchKeySelect.value = params.search_key;
    }
    
    if (exactMatchCheckbox && params.exact === 'true') {
      exactMatchCheckbox.checked = true;
    }
    
    // 执行搜索
    setTimeout(() => {
      const searchButton = document.getElementById('searchButton');
      searchButton?.click();
    }, 200);
  }
  
  // 恢复排序参数
  if (params.sort) {
    setTimeout(() => {
      dashboardStore.sortApps(params.sort, params.desc === 'true');
    }, 300);
  }
}

/**
 * 初始化错误处理
 */
function initErrorHandling(): void {
  // 全局错误处理
  window.addEventListener('error', (e) => {
    console.error('全局错误:', e.error);
    // 这里可以添加错误上报逻辑
  });
  
  // Promise 错误处理
  window.addEventListener('unhandledrejection', (e) => {
    console.error('未处理的Promise错误:', e.reason);
    // 这里可以添加错误上报逻辑
  });
}

/**
 * 初始化主题切换功能
 */
function initThemeToggle(): void {
  const themeToggle = document.getElementById('themeToggle');
  
  if (themeToggle) {
    // 从localStorage读取主题设置
    const savedTheme = localStorage.getItem('theme') || 'light';
    document.documentElement.setAttribute('data-theme', savedTheme);
    
    themeToggle.addEventListener('click', () => {
      const currentTheme = document.documentElement.getAttribute('data-theme') || 'light';
      const newTheme = currentTheme === 'light' ? 'dark' : 'light';
      
      document.documentElement.setAttribute('data-theme', newTheme);
      localStorage.setItem('theme', newTheme);
      
      // 更新图表主题
      if (typeof window.updateChartTheme === 'function') {
        window.updateChartTheme(newTheme === 'dark');
      }
    });
  }
}

/**
 * 主初始化函数
 */
export async function initDashboard(): Promise<void> {
  try {
    console.log('开始初始化仪表板...');
    
    // 初始化各个功能模块
    initAppDetailModal();
    initSearch();
    initSorting();
    initRefresh();
    initExport();
    initKeyboardShortcuts();
    initAutoRefresh();
    initErrorHandling();
    initThemeToggle();
    
    // 从URL参数恢复状态
    restoreFromUrlParams();
    
    // 加载初始数据
    await Promise.all([
      dashboardStore.loadMarketOverview(),
      dashboardStore.loadApps(),
      dashboardStore.loadChartData(),
    ]);
    
    // 更新最后更新时间
    updateLastUpdate();
    
    console.log('仪表板初始化完成');
    
  } catch (error) {
    console.error('仪表板初始化失败:', error);
    
    // 显示初始化错误
    const errorContainer = document.getElementById('initError');
    if (errorContainer) {
      errorContainer.innerHTML = `
        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
          <strong>初始化失败:</strong> ${error instanceof Error ? error.message : '未知错误'}
          <br>
          <button onclick="location.reload()" class="mt-2 px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700">
            重新加载
          </button>
        </div>
      `;
    }
  }
}


// 导出初始化函数供外部调用
  window.initDashboard = initDashboard;

// 类型声明
declare global {
  interface Window {
    initDashboard?: () => Promise<void>;
    updateChartTheme?: (isDark: boolean) => void;
    destroyAllCharts?: () => void;
    clearAllNotifications?: () => void;
  }
}