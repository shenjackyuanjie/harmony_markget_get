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
  searchExact,
  PAGE_SIZE,
  API_BASE
} from "./config";

// 导入模块
import { DarkModeManager, darkModeManager } from "./modules/dark-mode";
import { setupEventListeners } from "./modules/event-listeners";
import { updateLastUpdate } from "./utils";

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

// 初始化模态框状态
function initializeModals(): void {
  const modalIds = [
    "helpModal",
    "searchHelpModal", 
    "contactModal",
    "appDetailModal",
    "submitModal"
  ];
  
  modalIds.forEach(modalId => {
    const modal = document.getElementById(modalId);
    if (modal) {
      modal.classList.add("hidden");
    }
  });
}

// 初始化应用
function initializeApp(): void {
  console.log("初始化应用...");
  
  // 初始化模态框状态
  initializeModals();
  
  // 初始化暗黑模式
  darkModeManager;
  
  // 设置事件监听器
  setupEventListeners();
  
  // 更新最后更新时间
  updateLastUpdate();
  
  console.log("应用初始化完成");
}

// 当DOM加载完成时初始化应用
if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initializeApp);
} else {
  initializeApp();
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