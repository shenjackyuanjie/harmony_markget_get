import "./index.css";
import "./styles/dark-mode.css";

// 导入配置
import {
  currentPage,
  totalPages,
  currentApp,
  currentSort,
  searchTerm,
  searchKey,
  searchExact
} from "./config";
import { PAGE_SIZE } from "./utils/constants";
import { API_BASE } from "./config";

// 导入模块
import { init_dark } from "./modules/dark-mode";
import { setupEventListeners } from "./modules/event-listeners";
import { updateLastUpdate } from "./utils";
import { loadAppList, loadMarketOverview } from "./utils/api";
import { renderAppTable, renderPagination, showTableLoading } from "./utils/dom";
import { getUrlParams } from "./utils/url-params";

// 将函数挂载到全局对象
window.updateUrlParam = (key: string, value: string) => {
  const { updateUrlParam } = require("./utils/url-params");
  updateUrlParam(key, value);
};

window.getUrlParams = () => {
  const { getUrlParams } = require("./utils/url-params");
  return getUrlParams();
};

window.copyToClipboard = (text: string, button?: HTMLElement) => {
  const { copyToClipboard } = require("./utils/clipboard");
  copyToClipboard(text, button);
};


// 初始化应用
async function initializeApp(): Promise<void> {
  console.log("初始化应用...");


  // 初始化暗黑模式
  init_dark();

  // 设置事件监听器
  setupEventListeners();

  // 更新最后更新时间
  updateLastUpdate();

  // 初始化应用数据
  await initializeAppData();

  console.log("应用初始化完成");
}

// 初始化应用数据
async function initializeAppData(): Promise<void> {
  try {
    // 从URL参数获取初始状态
    const urlParams = getUrlParams();
    const page = parseInt(urlParams.page) || 1;
    const sortField = urlParams.sort || currentSort.field;
    const sortDesc = urlParams.desc !== 'false';
    const searchValue = urlParams.search_value || '';
    const searchKey = urlParams.search_key || 'name';
    const searchExact = urlParams.search_exact === 'true';

    // 更新全局状态
    currentPage = page;
    currentSort = { field: sortField, desc: sortDesc };
    searchTerm = searchValue;
    searchKey = searchKey;
    searchExact = searchExact;

    // 显示加载状态
    showTableLoading();

    // 加载市场概览信息
    try {
      const marketInfo = await loadMarketOverview();
      updateMarketInfo(marketInfo);
    } catch (error) {
      console.warn('加载市场概览失败:', error);
    }

    // 加载应用列表
    const response = await loadAppList(
      currentPage,
      currentSort,
      { term: searchTerm, key: searchKey, exact: searchExact }
    );

    const apps = response.data.data || [];
    const totalCount = response.data.total_count || 0;
    totalPages = Math.ceil(totalCount / PAGE_SIZE);

    // 渲染应用表格
    renderAppTable(apps, currentPage, (appId: string) => {
      showAppDetail(appId);
    });

    // 渲染分页
    renderPagination(currentPage, totalPages, (page: number) => {
      loadPageData(page);
    });

  } catch (error) {
    console.error('初始化应用数据失败:', error);
    const tableBody = document.getElementById("appTableBody");
    if (tableBody) {
      tableBody.innerHTML = `
        <tr>
          <td colspan="12" class="text-center py-4 text-red-500">
            加载数据失败，请刷新页面重试
          </td>
        </tr>
      `;
    }
  }
}

// 加载指定页面的数据
async function loadPageData(page: number): Promise<void> {
  try {
    currentPage = page;

    // 更新URL参数
    window.updateUrlParam('page', page.toString());

    // 显示加载状态
    showTableLoading();

    // 加载应用列表
    const response = await loadAppList(
      currentPage,
      currentSort,
      { term: searchTerm, key: searchKey, exact: searchExact }
    );

    const apps = response.data.data || [];
    const totalCount = response.data.total_count || 0;
    totalPages = Math.ceil(totalCount / PAGE_SIZE);

    // 渲染应用表格
    renderAppTable(apps, currentPage, (appId: string) => {
      showAppDetail(appId);
    });

    // 渲染分页
    renderPagination(currentPage, totalPages, (page: number) => {
      loadPageData(page);
    });

  } catch (error) {
    console.error('加载页面数据失败:', error);
    const tableBody = document.getElementById("appTableBody");
    if (tableBody) {
      tableBody.innerHTML = `
        <tr>
          <td colspan="12" class="text-center py-4 text-red-500">
            加载失败，请重试
          </td>
        </tr>
      `;
    }
  }
}

// 更新市场信息
function updateMarketInfo(marketInfo: any): void {
  // 更新应用总数
  const totalAppsElement = document.getElementById("totalApps");
  if (totalAppsElement) {
    totalAppsElement.textContent = marketInfo.total_apps || "0";
  }

  // 更新总下载量
  const totalDownloadsElement = document.getElementById("totalDownloads");
  if (totalDownloadsElement) {
    totalDownloadsElement.textContent = formatNumber(marketInfo.total_downloads || 0);
  }

  // 更新最后更新时间
  const lastUpdateElement = document.getElementById("lastUpdate");
  if (lastUpdateElement && marketInfo.last_update) {
    lastUpdateElement.textContent = new Date(marketInfo.last_update).toLocaleString();
  }
}

// 格式化数字
function formatNumber(num: number): string {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M';
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K';
  }
  return num.toString();
}

// 显示应用详情（占位函数，实际可能需要导入对应的函数）
function showAppDetail(appId: string): void {
  // 这里应该调用应用详情显示函数
  // 可能需要从 app-details.ts 模块导入
  console.log('显示应用详情:', appId);

  // 更新URL参数
  window.updateUrlParam('app_id', appId);

  // 如果有全局的 showAppDetail 函数，调用它
  if ((window as any).showAppDetail) {
    (window as any).showAppDetail(appId);
  }
}

// 当DOM加载完成时初始化应用
if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", () => {
    initializeApp().catch(error => {
      console.error("应用初始化失败:", error);
    });
  });
} else {
  initializeApp().catch(error => {
    console.error("应用初始化失败:", error);
  });
}

// 导出供其他模块使用
export {
  currentPage,
  totalPages,
  currentApp,
  currentSort,
  searchTerm,
  searchKey,
  searchExact,
  PAGE_SIZE,
  API_BASE,
  DarkModeManager,
  darkModeManager
};
